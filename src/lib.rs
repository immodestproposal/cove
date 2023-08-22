#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

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

// TODO: NonZero support, tests (both std and no_std), performance recheck
// TODO: example of extending Cast (reference it in the CastImpl docs)
// TODO: re-document everything: include up-front quick usage examples and also performance notes
// TODO: fill out cargo.toml more, fill out readme
// TODO: solicit feedback, possibly take feedback, publish a 1.0

mod cast;
mod impls;

pub mod base;

pub use cast::*;