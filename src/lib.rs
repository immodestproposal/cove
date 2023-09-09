#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! # Cove: Casts Of Varying Elegance
//! A collection of extension traits for improving the safety and maintainability of numerical
//! casts.
//!
//! Cove's primary goals are:
//! * **clarity**: the programmer's intention for a cast is clear from the name
//! * **correctness**: suspicious casts via `as` can be reduced or eliminated altogether
//! * **performance**: in release builds, cove's casts generally compile down to the same
//! assembly as manual implementations
//!
//! ## Quick Usage
//! ```
//! use cove::*;
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
//! // Perhaps you don't mind if the cast is lossy, but you'd like to saturate to the target type:
//! assert_eq!(300u32.cast::<u8>().saturated(), 255u8);
//! assert_eq!((-7isize).cast::<u16>().saturated(), 0u16);
//!
//! // ...or maybe an estimate is acceptable:
//! assert_eq!(-4.4f32.cast::<i16>().estimated(), -4i16);
//! assert_eq!(-0.0f64.cast::<NonZeroI32>().estimated(), NonZeroI32::new(-1).unwrap());
//!
//! // If you are supremely confident a cast is lossless you can always use `unwrap_unchecked`
//! // off the returned `Result`:
//! assert_eq!(unsafe {90u32.cast::<u8>().unwrap_unchecked()}, 90);
//!
//! // ...but if that makes you uncomfortable you might prefer cove's `assumed_lossless`, which will
//! // use a debug assertion instead of unsafe (and just be lossy in release builds):
//! assert_eq!(90u32.cast::<u8>().assumed_lossless(), 90);
//!
//! # Ok::<(), LossyCastError<i16, u8>>(())
//! ```

#![cfg_attr(target_pointer_width = "64", doc = "```")]
#![cfg_attr(not(target_pointer_width = "64"), doc = "```compile_fail")]
//! use cove::*;
//! use core::num::{NonZeroU16, NonZeroU64};
//!
//! // If a number's type guarantees a lossless cast, you can of course always use `From`/`Into`:
//! assert_eq!(NonZeroU64::from(NonZeroU16::new(12).unwrap()), NonZeroU64::new(12).unwrap());
//!
//! // ...but what if those traits aren't provided because the cast could be lossy on some other
//! // platform? If you don't mind losing portability, try out cove's `lossless`. This will only
//! // compile on platforms where usize is at least 64 bits:
//! assert_eq!(31u64.cast::<usize>().lossless(), 31usize);
//! ```

//! ## Design and Motivation
//! This crate provides extension traits for casting between numerical types, especially
//! primitives and other cheaply-cloneable numeric types. Many of these traits parallel existing
//! mechanisms such as [`From`]/[`Into`] or [`TryFrom`]/[`TryInto`], but offer differing semantics
//! tailored to common use cases.
//!
//! # Motivation
//! Given the existence of [`From`]/[`Into`]/[`TryFrom`]/[`TryInto`] and the `as` keyword, it is
//! natural to ask what value additional numeric casting functionality could provide. The
//! motivation is simple: the existing mechanisms, while perfectly serviceable, are
//! general-purpose tools which do not align precisely to the use cases for numeric casts. This
//! creates an opportunity for improvements; though each improvement is minor, in codebases rife
//! with casts they can collectively have an outsized effect.
//!
//! ### Definitions
//! This crate uses the following terms:
//!
//! * `numeric cast`: a type cast from one numerical type to another
//! * `lossless cast`: a numeric cast where the casted value does not change when its type changes
//! * `lossy cast`: a numeric cast where the casted value changes when its type changes
//!
//! ### Use Cases
//! Numeric casts can be sorted into the following use cases:
//!
//! * The compiler can prove that the cast is lossless from types alone, regardless of the target
//! platform. For example, a u8 ⮕ u16 cast must be lossless on all platforms. Let's call this
//! `lossless: portable`.
//! * The compiler can prove that the cast is lossless from types alone on the target platform,
//! but it may not be on a different platform. For example, a u64 ⮕ usize cast must be lossless
//! on a 64-bit platform, but it may not be on a 32-bit platform. Let's call this `lossless:
//! non-portable`.
//! * The cast is not provably lossless but it does not matter whether it is. Let's call this
//! `lossy: no detection`.
//! * The cast is not provably lossless. It is important to detect whether it is lossy and, if so,
//! to obtain the details of the lossiness (such as the lossy value itself). Let's call this `lossy:
//! detection, details`.
//! * The cast is not provably lossless. It is important to detect whether it is lossy; however, the
//! details of the lossiness (such as the lossy value itself) are unimportant. Let's call this
//! `lossy: detection, no details`.
//!
//! The existing standard mechanisms for casting cleanly cover some of these use cases, but not
//! all, as shown here:
//!
//! | Use Case                      | Clean Standard Mechanism                          |
//! | ---                           | ---                                               |
//! | lossless: portable            | [`From`]/[`Into`]                                 |
//! | lossless: non-portable        | ???                                               |
//! | lossy: no detection           | `as` keyword                                      |
//! | lossy: detection, details     | ???                                               |
//! | lossy: detection, no details  | [`TryFrom`]/[`TryInto`] (except floating point)   |
//!
//! The missing rows can be fulfilled less cleanly through use of workarounds such as the `as`
//! keyword, which (as noted) is also the best standard mechanism for lossy casts without details.
//! Using this keyword for numeric casts is problematic for a few reasons:
//!
//! * It is a blunt instrument, forcing a conversion that the programmer has to verify is
//! acceptable. This makes it a potential source of bugs.
//! * It isn't self-documenting; that is, if a maintainer sees a lossy numerical cast via `as`,
//! it may not be clear whether the original programmer noticed and deemed the potential lossiness
//! acceptable or merely overlooked it.
//! * It is overloaded for all manner of casts, not just numerical ones. This increases the
//! search space when hunting for invalid numerical casts with a simple text search.
//! * It is not a trait, and therefore can be difficult to employ in generic contexts.
//!
//! As a consequence of these issues, it is usually a good idea to avoid the `as` keyword for
//! numerical casts, at least in the presence of better options. This crate aims to provide those
//! better options.
//!
//! With this crate in play, the revised table looks like this:
//!
//! | Use Case                      | Recommended Mechanism                 |
//! | ---                           | ---                                   |
//! | lossless: portable            | [`From`]/[`Into`]                     |
//! | lossless: non-portable        | [`LosslessCast`]                      |
//! | lossy: no detection           | [`LossyCast`]                         |
//! | lossy: detection, details     | [`Cast`]                              |
//! | lossy: detection, no details  | [`Cast`] (supports floating point)    |
//!
//! ### Floating Point
//! Perhaps surprisingly, the standard traits [`TryFrom`]/[`TryInto`] are not supported for many
//! conversions between integer types and floating point types. This crate supports casting between
//! these types.
//!
//! ### Saturate
//! [`Cast`] supports saturating its result through the [`Saturate`] trait, which is defined by
//! default for integer to integer conversions. This provides a convenient yet explicit mechanism
//! for casting to the closest target value.
//!
//! ### `NonZero`
//! Built-in support for casting to and from the `std::num::NonZero`* family is planned but not yet
//! implemented. It may be implemented externally by extending the base casting traits in
//! [`cove::base`](crate::base).

// TODO: tests (both std and no_std)
// TODO: re-document everything:
// TODO:    * performance notes
// TODO:    * small example of using traits in a generic context
// TODO:    * full example of extending Cast (reference it from the CastImpl docs)
// TODO:    * comparison with standard casting methodologies
// TODO:    * table of support for each follow-on extension trait
// TODO:    * lib.rs, cast.rs
// TODO: make sure all casts documented as zero-overhead have been covered in the asm example
// TODO: fill out cargo.toml more, fill out readme
// TODO: solicit feedback, possibly take feedback, publish a 1.0

mod cast;
mod impls;

pub mod base;

pub use cast::*;