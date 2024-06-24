
//! Provides an explanation for why cove is useful
//!
//! # Motivation: `cove` vs `core`
//! Given the existence of [`From`]/[`Into`]/[`TryFrom`]/[`TryInto`] and the `as` keyword, it is
//! natural to ask what value additional numeric casting functionality could provide. The
//! motivation is simple: the existing mechanisms, while perfectly serviceable, are
//! general-purpose tools which do not align precisely to the use cases for numeric casts. This
//! creates an opportunity for improvements; though each improvement is minor, in codebases rife
//! with casts they can collectively have an outsized effect.
//!
//! ## [`From`]/[`Into`]
//! The [`From`]/[`Into`] traits are implemented for numeric casts which are guaranteed to be
//! lossless on all supported platforms based on their types alone. This is a strong guarantee,
//! and if these traits fit your use case you should think hard before picking anything else,
//! including cove's casts. However, such a strong guarantee naturally comes with a limited scope;
//! for the many use cases which do not conform, other casting mechanisms are required.
//!
//! ## [`TryFrom`]/[`TryInto`]
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
//! ## `as` Keyword
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
//! # Motivation: `cove` vs `conv`
//! Given the existence of the excellent `conv` crate, one might wonder why cove was created at
//! all. The answer is simple: cove's author did not know about the `conv` crate when he 
//! started work on cove. By the time he realized its existence, much of the work on cove had 
//! been completed and it seemed a shame to stop. 
//! 
//! So why should you use cove instead of `conv`? The short answer is that each chooses to 
//! emphasize different functionality and makes different tradeoffs to do so. This section 
//! explores those differences.
//! 
//! **Caveat**: this comparison was written by the author of `cove`. It makes an earnest attempt 
//! at a fair comparison, but the author is simply not as familiar with `conv` as `cove` and thus
//! could misrepresent `conv`. Please use your own judgment (and feel free to inform the author 
//! of any places he errs!).
//! 
//! ## Similarities
//! First off, at their core both crates solve the same problems:
//! 
//! * Both crates improve clarity and correctness of numerical value casting
//! * Both crates provide traits which can be used in generic contexts related to casting
//! * Both crates are extensible to new types
//! * Both crates focus on performant codegen
//! * Both crates provide casts for numerical primitives
//! * Both crates support `no_std`
//! 
//! ## Differences
//! The first, most obvious difference between the crates is mileage: `conv` has been around much 
//! longer than `cove` and seen far more usage. That said, it never released a 1.x.x version, 
//! meaning it is technically an unstable API from a semver perspective. Given its wide adoption, 
//! this is probably a non-issue in practice.
//! 
//! While `conv` has fairly minimal dependencies, cove has none at all, even including 
//! dev-dependencies and build-dependencies. Both can optionally depend on `std`. 
//!
//! ### Supported Types
//! There is a difference in which types are supported out-of-the-box for each crate. In 
//! particular, `conv` supports casts to/from `char`, while cove does not. It is the author's 
//! opinion that `char` represents sufficiently different semantics from numerical types that it 
//! should not be conflated with them. On the other hand, cove supports casts to/from the `NonZero*` 
//! family of integers in [`core::num`], while `conv` does not.
//! 
//! ### Casting Semantics
//! Another difference is that `conv` offers precise semantics on rounding floating points, 
//! providing options to round towards zero, towards positive or negative infinity, towards the 
//! closest number, or to use the default scheme (which will generally be similar to rounding 
//! towards zero). By contrast, cove offers the default scheme (i.e. rounding towards zero) and
//! towards the closest number, but not rounding towards positive or negative infinity.
//! 
//! Unlike `conv`, cove offers support for bitwise casting, which focuses on the bit representation 
//! of numerical types rather than their mathematical value. This has applications in FFI as well
//! as some niche use cases (e.g., generating random floats from an LCG).
//! 
//! As noted in its documentation, `conv` takes the stance that while exact conversions from 
//! floats to int are possible, it is misleading to advertise it with an implementation; 
//! consequently, only approximation casting is supported in that case (though the "approximate" 
//! value might well be exactly equal). It is cove's author's opinion that there is nothing 
//! misleading about returning an exact conversion if the cast was truly lossless and an error if
//! it was lossy; consequently, cove takes a different stance, choosing to unify the interfaces 
//! into one overarching cast and thereby simplify the mental model.
//! 
//! Similarly, for int-to-float casts `conv` takes the stance that errors should be returned once
//! the input values exceed the point at which the output type can precisely represent all 
//! inputs, even if the actual value can be converted losslessly. For example:
//! 
//! ```ignore
//! // conv declares both of these to be a PosOverflow error, even though the second can be
//! // converted losslessly:
//! assert!(f32::value_from(16_777_217u32).is_err());
//! assert!(f32::value_from(16_777_218u32).is_err());
//! ```
//! 
//! Cove takes the opposite stance:
//!
//! ```
//! # use cove::prelude::*;
//! // cove agrees with conv on the lossy cast being an error...
//! assert!(16_777_217u32.cast::<f32>().is_err());
//! 
//! // ...but disagrees on the second cast since it can be converted losslessly.
//! assert_eq!(16_777_218u32.cast::<f32>(), Ok(16_777_218.0f32));
//! ```
//!
//! ### Casting Syntax
//! There are some syntactical distinctions between the crates, but they aren't huge. Consider these 
//! examples copied from `conv`'s documentation:
//!
//! ```ignore
//! assert_eq!(41.0f32.approx(), Ok(41u8));
//! assert_eq!(u8::value_from(256i16).unwrap_or_saturate(), 255u8);
//! ```
//! 
//! The analogous casts in cove look like so:
//! 
//! ```
//! # use cove::prelude::*;
//! assert_eq!(41.0f32.cast(), Ok(41u8));
//! assert_eq!(256i16.cast::<u8>().closest(), 255);
//! ```
//! 
//! Cove's design made an effort to always lead with a call to `cast` for consistency and apply 
//! transforms to the result via follow-on extension traits. A similar technique is used in `conv`, 
//! but since there are multiple initial calls possible there end up being many times more 
//! possible combinations to mentally manage than with cove.
//!
//! ### Bounding Syntax
//! The syntax of the two crates differs more heavily when it comes to generic bounds. Consider 
//! the task of bounding a function which saturates an input to a u8 as best as possible via `conv`:
//! 
//! ```ignore
//! // It took some time to figure out how to bound the function; it is also possible there is a 
//! // better way, but that isn't clear at the moment.
//! fn foo<E: Into<RangeErrorKind>, T: ApproxInto<u8, Err = E> + Saturated>(x: T) -> u8 {
//!     x.approx_into().unwrap_or_saturate()
//! }
//! 
//! assert_eq!(foo(300u16), 255u8);
//! ```
//! 
//! By contrast, cove provides traits specifically to assist with bounding:
//! 
//! ```
//! # use cove::prelude::*;
//! # use cove::bounds::CastToClosest;
//! fn foo(x: impl CastToClosest<u8>) -> u8 {
//!     x.cast().closest()
//! }
//! 
//! assert_eq!(foo(300u16), 255u8);
//! ```
//! 
//! ### Errors
//! The two crates took similar but somewhat different approaches to errors. The `conv` crate 
//! supports three times as many error types as cove, including several enum types which logically 
//! contain even more subtypes of errors. This allows consuming code to take different actions 
//! based on the type -- for example, if it needed to do something different for a positive overflow 
//! as opposed to a negative overflow. 
//! 
//! Cove opted instead for a simpler mental model coupled with improved non-type diagnostics; that 
//! is, it focuses on error message quality rather than discerning errors via the type system. By 
//! way of illustration, consider the cast from `16_777_217u32` to `f32` used in an earlier example.
//!
//! Printing the resulting error via `Display` yields:
//! * **conv:** `conversion resulted in positive overflow`
//! * **cove:** `Numerical cast was lossy [16777217 (u32) -> 16777216 (f32)]`
//!
//! Printing the resulting error via `Debug` yields:
//! * **conv:** `PosOverflow(..)`
//! * **cove:** `LossyCastError { from: 16777217, to: 16777216.0 }`
//! 
//! # Overall
//! So which to use: cove, `conv`, or just the basic features of `core`? As with everything in 
//! software, there is no substitute for understanding the tradeoffs and how they apply to your 
//! particular situation. As a rule of thumb, the author recommends using either crate over the 
//! raw functionality of `core` unless you can get away purely with [`From`]/[`Into`]. Use `conv`
//! if you need to round floats towards infinity, for its `char` support, if you agree with its 
//! design philosophy around int ↔ float conversions and errors, or if you just like its proven 
//! track record. Use cove for its bounding syntax, for its bitwise cast and `NonZero*` support, if 
//! you agree with its design philosophy regarding int ↔ float conversions and errors, or if you 
//! just like its simpler mental model.