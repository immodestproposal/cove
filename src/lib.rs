#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

// This is allowed across the board because clippy is incorrect; matching on bool is more 
// compact and easier to read than if/else. Obviously this is subjective, but for exactly 
// that reason clippy shouldn't be trying to force its authors' personal preferences on the 
// community as a whole.
#![allow(clippy::match_bool)]

//! # Cove: Casts Of Varying Elegance
//! A collection of extension traits to improve the safety and maintainability of numerical casts.
//!
//! Cove's primary goals are:
//! * **clarity**: the programmer's intention for a cast is clear from the name
//! * **correctness**: suspicious casts via `as` can be reduced or eliminated altogether
//! * **performance**: in release builds, cove's casts generally compile down to the same
//! assembly as manual implementations
//!
//! ## Quick Usage
//! ```
//! use cove::prelude::*;
//! use core::num::{NonZeroI8, NonZeroI32};
//!
//! // Check whether a cast is lossy at runtime
//! assert_eq!(8i16.cast::<u8>()?, 8u8);
//! assert!(0u128.cast::<NonZeroI8>().is_err());
//!
//! // Of course, turbofish disambiguation is unnecessary if the compiler can deduce the type:
//! fn foo(x: u8) -> u8 {x}
//! assert_eq!(foo(2i16.cast()?), 2u8);
//!
//! // If the cast ends up being lossy, you can usually still use the lossy value if you like:
//! assert_eq!(9.2f64.cast::<usize>().unwrap_err().to, 9usize);
//!
//! // ...or more concisely:
//! assert_eq!(9.2f64.cast::<usize>().lossy(), 9usize);
//!
//! // Perhaps you don't mind if the cast is lossy, but you'd like to get as close as possible:
//! assert_eq!(300u32.cast::<u8>().closest(), 255u8);
//! assert_eq!((-7isize).cast::<u16>().closest(), 0u16);
//! assert_eq!(-4.6f32.cast::<i16>().closest(), -5i16);
//! assert_eq!(-0.0f64.cast::<NonZeroI32>().closest(), NonZeroI32::new(-1).unwrap());
//!
//! // If you are supremely confident a cast is lossless you can always use unwrap_unchecked:
//! assert_eq!(unsafe {90u32.cast::<u8>().unwrap_unchecked()}, 90);
//!
//! // ...but if the unsafeness makes you uncomfortable you might prefer cove's assumed_lossless,
//! // which will use a debug_assert instead of unsafe (and just risk lossiness in release builds):
//! assert_eq!(90u32.cast::<u8>().assumed_lossless(), 90);
//!
//! # Ok::<(), cove::errors::LossyCastError<i16, u8>>(())
//! ```
#![cfg_attr(target_pointer_width = "64", doc = "```")]
#![cfg_attr(not(target_pointer_width = "64"), doc = "```compile_fail")]
//! use cove::prelude::*;
//! use core::num::{NonZeroU16, NonZeroU64};
//!
//! // If the types guarantee a lossless cast, you can of course always use `From`/`Into`:
//! assert_eq!(NonZeroU64::from(NonZeroU16::new(12).unwrap()), NonZeroU64::new(12).unwrap());
//!
//! // ...but what if those traits aren't provided because the cast could be lossy on some other
//! // platform? If you don't mind losing portability, try out cove's `lossless`. This will only
//! // compile on platforms where usize is at least 64 bits:
//! assert_eq!(31u64.cast::<usize>().lossless(), 31usize);
//! ```

//! ## Motivation
//! Given the existence of [`From`]/[`Into`]/[`TryFrom`]/[`TryInto`] and the `as` keyword, it is
//! natural to ask what value additional numeric casting functionality could provide. The
//! motivation is simple: the existing mechanisms, while perfectly serviceable, are
//! general-purpose tools which do not align precisely to the use cases for numeric casts. This
//! creates an opportunity for improvements; though each improvement is minor, in codebases rife
//! with casts they can collectively have an outsized effect.
//!
//! ### [`From`]/[`Into`]
//! The [`From`]/[`Into`] traits are implemented for numeric casts which are guaranteed to be
//! lossless on all supported platforms based on their types alone. This is a strong guarantee,
//! and if these traits fit your use case you should think hard before picking anything else,
//! including cove's casts. However, such a strong guarantee naturally comes with a limited scope;
//! for the many use cases which do not conform, other casting mechanisms are required.
//!
//! ### [`TryFrom`]/[`TryInto`]
//! The [`TryFrom`]/[`TryInto`] traits are provided for numeric casts which might be lossy, to
//! allow for testing of this lossiness at runtime. This covers many of the use cases unaddressed
//! by [`From`]/[`Into`], but not all. For example:
//!
//! * Some conversions which might be desired are not provided, such as from floating points to
//!     integers
//! * If the cast is lossy but you want to use whatever it produces anyway, [`TryFrom`]/[`TryInto`]
//!     can't help
//! * If the cast is lossy but you want as close as it can get, [`TryFrom`]/[`TryInto`] can't help
//! * If the cast is lossy and you want good error messages, [`TryFrom`]/[`TryInto`]'s errors tend
//!     to disappoint
//! * If you know a cast is lossless, you are stuck with suboptimal options:
//!     * Risk the unsafeness of [`unwrap_unchecked`](Result::unwrap_unchecked)
//!     * Absorb the performance cost of [`unwrap`](Result::unwrap)
//!     * Absorb the performance cost and polluted interface implied by returning a [`Result`]
//!
//! ### `as` Keyword
//! The use cases not covered by [`From`]/[`Into`]/[`TryFrom`]/[`TryInto`] are generally left to the
//! `as` keyword. This is unfortunately a fairly blunt instrument which requires paying careful
//! attention to the semantics of numeric casts to ensure correct use. For this reason, usage of
//! `as` for numeric casts often triggers complaints from linters, such as when using clippy in
//! pedantic mode.
//!
//! Since `as` is not a trait it is quite difficult to use it in generic contexts. Moreover, due to
//! being overloaded for other type casts it can be more challenging to search its usages for
//! possible sources of numeric cast bugs. In general it is a good idea to avoid the `as` keyword
//! for numeric casts, at least in the presence of better options. This crate aims to provide those
//! better options.
//!
//! ## Usage
//! Cove provides a [`prelude`] module to assist with importing the requisite extension traits.
//! Most applications of cove will not require `use`ing anything more.
//!
//! All cove casts begin with a call to [`Cast::cast`](casts::Cast::cast):
//! ```
//! use cove::prelude::*;
//!
//! // Turbofish disambiguation of the target type is required in this example, but not
//! // necessarily in other cases.
//! assert_eq!(10u32.cast::<i32>()?, 10i32);
//! # Ok::<(), cove::errors::LossyCastError<u32, i32>>(())
//! ```
//! Just as with [`TryFrom`]/[`TryInto`], this basic usage returns a [`Result`] which may be
//! interrogated like any [`Result`]. While the returned error is generally a little more useful
//! than that returned by [`TryFrom`]/[`TryInto`], the full value of the cove casts is not realized
//! until the next step: using the follow-on extension traits.
//!
//! ### Follow-on Extension Traits
//! Cove defines a number of extension traits which are implemented for the [`Result`] returned
//! from calling [`Cast::cast`](casts::Cast::cast) and well as for its contained error types. A
//! common cove usage, therefore, involves calling [`Cast::cast`](casts::Cast::cast) and then
//! immediately calling one of the follow-on extension traits on its [`Result`]:
//! ```
//! use cove::prelude::*;
//!
//! assert_eq!(8u64.cast::<u16>().assumed_lossless(), 8u16);
//! assert_eq!((-8i64).cast::<u16>().closest(), 0u16);
//! ```
//!
//! An overview of the available follow-on extension traits is provided here; see the
//! documentation for each trait for more in-depth details and semantics:
//! * [`Lossless`](casts::Lossless): for compile-time lossless casts based on types alone (e.g.
//! widening conversions)
//!     * Will not compile for casts which could be lossy based on their types
//!     * Does not guarantee portability; compiling on a target platform does not imply compiling on
//!         all platforms
//!     * Akin to [`From`]/[`Into`] but trades off portability guarantees for a broader scope (e.g.
//!         support for [`usize`]/[`isize`])
//!     * Zero-overhead: generally optimizes to the same assembly as the `as` keyword
//! * [`Lossy`](casts::Lossy): for casts where lossiness is acceptable with no general guarantees
//!     on the accuracy
//!     * Most akin to the `as` keyword but self-documents the intent and works in generic contexts
//!     * Very situational: consider one of the other extension traits instead
//!     * Zero-overhead: generally optimizes to the same assembly as the `as` keyword
//! * [`AssumedLossless`](casts::AssumedLossless): for casts asserted to be lossless at runtime
//!     * Will panic in dev builds if the cast is lossy; will just be silently lossy in release
//!         builds
//!     * Most akin to [`Result::unwrap_unchecked`] but offers an alternative to unsafeness
//!     * Zero-overhead: generally optimizes to the same assembly as the `as` keyword
//! * [`Closest`](casts::Closest): for casts which can be lossy provided they get as close as the
//!     types allow
//!     * Yields the closest possible cast, which might not be very close at all:
//!     ```
//!     # use cove::prelude::*;
//!     assert_eq!(1_000_000_000u64.cast::<u8>().closest(), 255u8);
//!     ```
//!     * **NOT** zero-overhead: generally involves at least one branch over the `as` keyword
//!
//! ### Cast Errors
//! Cove's [`Cast`](casts::Cast) trait uses an associated error type for flexibility. In
//! practice, cove provides two error types which are actually used for casts:
//!
//! * [`LossyCastError`](errors::LossyCastError): for lossy casts which are able to represent the
//! lossy value as the target type
//!     * Used in most of cove's casts
//!     * Allows for retrieving the origin and target values via the `from` and `to` member fields:
//!     ```
//!     # use cove::prelude::*;
//!     assert_eq!(260u32.cast::<u8>().unwrap_err().from, 260u32);
//!     assert_eq!(260u32.cast::<u8>().unwrap_err().to, 4u8);
//!     ```
//!     * Provides a descriptive message
//!         * e.g. `"Numerical cast was lossy [260 (u32) -> 4 (u8)]"`
//! * [`FailedCastError`](errors::FailedCastError): for lossy casts which are unable to represent
//!     the lossy value as the target type
//!     * Used for certain NonZero casts, where representing e.g.
//!         [`NonZeroUsize`](core::num::NonZeroUsize) in the error type could invoke undefined
//!         behavior
//!     * Allows for retrieving the origin (but not target) value via the `from` member field:
//!     ```
//!     # use cove::prelude::*;
//!     # use std::num::NonZeroU8;
//!     assert_eq!(0u32.cast::<NonZeroU8>().unwrap_err().from, 0u32);
//!     ```
//!     * Provides as descriptive an error message as possible
//!         * e.g. `"Numerical cast failed [0 (u32) -> (core::num::nonzero::NonZeroU8)]"`
//!
//! Note that it is not necessary to interact explicitly with these error types in many cases,
//! such as when using the follow-on extension traits; thus, they are not included in the prelude.
//!
//! ### Features
//! Cove supports one feature, `std`, which is included in the default features. Enabling this
//! feature (or rather, failing to disable it) enables support for the Rust standard library.
//! If this is disabled, cove depends only on the Rust core library.
//!
//! Enabling `std` causes cove's error types to implement [`std::error::Error`]; otherwise they do
//! not, as at the time of writing [`core::error::Error`] is unstable. In addition, some cast
//! implementations are controlled by this feature, as the rust standard library allows for
//! optimizations via intrinsics not available in stable [`core`].
//!
//! ### Supported Casts
//! Not all follow-on cast types make sense for all numerical conversions; attempting to use an
//! unsupported cast will result in a compilation error. Refer to the documentation of the
//! individual casts for details, but as quick rules of thumb:
//!
//! * [`Cast`](casts::Cast) and [`Closest`](casts::Closest) are supported for all casts between all
//!     primitive numerical types as well as the NonZero* family of non-zero integers from
//!     [`core::num`].
//! * [`Lossy`](casts::Lossy) and [`AssumedLossless`](casts::AssumedLossless) are supported
//!     whenever the target type is a primitive.
//! * [`Lossless`](casts::Lossless) is supported whenever [`From`]/[`Into`] is supported as well
//!     as to/from [`usize`] / [`isize`] / [`NonZeroUsize`](core::num::NonZeroUsize) /
//!     [`NonZeroIsize`](core::num::NonZeroIsize) when this is guaranteed lossless on the target
//!     platform.
//!
//! ### Extending Support
//! Extending cove's casts to new types involves implementing [`base::CastImpl`]; see the
//! documentation for [`base`] for more details.
//! 
//! ### Generic Bounds
//! As with all traits, Cove's casting traits may be used as bounds on generic parameters to a 
//! function. Cove provides the convenience subtrait [`casts::CastTo`] to simplify this in the 
//! most common cases; see its documentation for an example. 
//!
//! ### Guidelines
//! It might seem challenging to determine which type of cast to use in which circumstances.
//! While one size rarely fits all in software, here are some quick guidelines which might be
//! useful:
//!
//! * If [`From`]/[`Into`] are provided for your use case, use those instead of any of cove's casts
//! * Otherwise, if you are writing an interface to be consumed by a third party:
//!     * Consider whether you really want any form of fallible casting in the interface; it
//!         might be better to just take the target type
//!     * If possible, favor [`TryFrom`]/[`TryInto`] over any of cove's casts to avoid introducing
//!         interface dependencies
//! * Otherwise, favor cove's casts over [`TryFrom`]/[`TryInto`] or the `as` keyword:
//!     * Favor [`Lossless`](casts::Lossless) if provided for your use case and you'd rather
//!         detect portability errors at compile time than runtime
//!     * Favor [`AssumedLossless`](casts::AssumedLossless) if confident the cast will always be
//!         lossless
//!     * Favor [`Cast`](casts::Cast) with error handling if only lossless casts should proceed
//!     * Favor [`Closest`](casts::Closest) when best-effort lossiness is acceptable
//!     * Use [`Lossy`](casts::Lossy) in niche circumstances; favor this over the `as` keyword
//!         * Exception: in some const contexts it may be necessary to use the `as` keyword,
//!              since const trait support is limited
//!
//! ## Performance
//! Cove's primary mission is to improve the casting situation by replacing as many use cases for
//! the `as` keyword as possible. Since one of the reasons to use `as` is performance, cove
//! strives to provide implementations which can compete on runtime speed, so that there is no
//! need for the programmer to choose between safer, self-documenting casts and speedy ones.
//!
//! Several of the casts provided in this crate can be expected to optimize to the same
//! assembly as the `as` keyword in release builds. For example, consider this function:
//!
//! ```
//! #[inline(never)]
//! fn cast_u32_to_u8(value: u32) {
//!     // core::hint::black_box(value as u8);
//!     // core::hint::black_box(value.cast::<u8>().lossy());
//!     // core::hint::black_box(value.cast::<u8>().assumed_lossless());
//! }
//! ```
//!
//! Commenting in each of these lines in turn and compiling the function in release with Rust
//! 1.72.0 on stable-x86_64-pc-windows-msvc yields the exact same assembly for all three:
//!
//! ```ignore
//! push rax
//! mov byte ptr [rsp + 7], cl
//! lea rax, [rsp + 7]
//! pop rax
//! ret
//! ```
//!
//! Optimizer results are subject to variation by version and platform and can never be completely
//! relied upon, but the core point remains: there is no need to a priori favor `as` over cove's
//! casts strictly for performance.
//!
//! Consult the documentation on each casting trait for performance notes. Also refer to `asm.rs`
//! in cove's `examples` directory for assistance with testing assembly generation for your platform.

// TODO: tests (both std and no_std): also cross-compiled for pointer widths
// TODO: do we still need both Lossless and LosslessCast?
// TODO: documentation:
// TODO:    * small example of using traits in a generic context
// TODO:    * re-read all docs for correctness
// TODO:    * readme
// TODO: make sure all casts documented as zero-overhead have been covered in the asm example
// TODO: fill out cargo.toml more (badges, keywords, etc)
// TODO: solicit feedback, possibly take feedback, publish a 1.0

mod impls;

pub mod base;
pub mod bounds;
pub mod casts;
pub mod errors;
pub mod prelude;
