//! This module provides the base traits needed to support casting through the interface traits
//! [`Cast`](crate::Cast) / [`LossyCast`](crate::LossyCast) / [`LosslessCast`](crate::LosslessCast).
//! If you only need to use existing casting support these base traits will be irrelevant to you,
//! but they are important for extending casting support to new types.
//!
//! Casting functionality is split between the base and interface traits because the base traits
//! are more flexible to implement while the interface traits are more ergonomic to use. Without
//! this divide it would be difficult to provide easy turbofish-based type resolution in the face
//! of ambiguity. For example, the following:
//!
//! ```
//! use cove::LossyCast;
//!
//! assert_eq!(5u16.lossy_cast::<u8>(), 5u8);
//! ```
//!
//! would have to be expressed like this:
//!
//! ```
//! use cove::base::LossyCastImpl;
//!
//! // Type resolution takes two lines...
//! let foo: u8 = 5u16.lossy_cast_impl();
//! assert_eq!(foo, 5u8);
//!
//! // ...or else one rather verbose line
//! assert_eq!(<u16 as LossyCastImpl<u8>>::lossy_cast_impl(5), 5u8);
//! ```
//!
//! It is neither necessary nor advised to implement the interface traits directly; instead,
//! implement these base traits to extend casting functionality to new types.

use crate::CastError;

/// Provides the base trait for [`Cast`](crate::Cast); implement this to extend
/// [`Cast`](crate::Cast) to new types
pub trait CastImpl<T> {
    /// Returns `Ok` if the cast is lossless and `Err` if any amount of data was lost. See
    /// [`Cast::cast`](crate::Cast::cast) for details and invariants to uphold.
    fn cast_impl(self) -> Result<T, CastError<Self, T>> where Self: Sized;
}

/// Provides the base trait for [`LossyCast`](crate::LossyCast); implement this to extend
/// [`LossyCast`](crate::LossyCast) to new types
pub trait LossyCastImpl<T> {
    /// Casts this numerical type to type `T`, ignoring any errors. See
    /// [`LossyCast::lossy_cast`](crate::LossyCast::lossy_cast) for details and invariants to
    /// uphold. Also note the expectation that this operation should be cheap at runtime, ideally
    /// free.
    fn lossy_cast_impl(self) -> T;
}

/// Provides the base trait for [`LosslessCast`](crate::LosslessCast); implement this to extend
/// [`LosslessCast`](crate::LosslessCast) to new types
pub trait LosslessCastImpl<T> {
    /// Casts this numerical type to type `T` in a fashion guaranteed to be lossless on the target
    /// platform. See [`LosslessCast::lossless_cast`](crate::LosslessCast::lossless_cast) for
    /// details and invariants to uphold. Also note the expectation that this operation should be
    /// cheap at runtime, ideally free.
    fn lossless_cast_impl(self) -> T;
}