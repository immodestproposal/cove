//! This module provides the base trait needed to support casting through the [`Cast`](crate::Cast)
//! trait. If you only need to use existing casting support this base trait will be irrelevant to
//! you, but it is important for extending casting support to new types.
//!
//! Casting functionality is split between [`CastImpl`] and [`Cast`](crate::Cast) because
//! [`CastImpl`] is more flexible to implement while [`Cast`](crate::Cast) is more ergonomic to
//! use. Without this divide it would be difficult to provide easy turbofish-based type resolution
//! in the face of ambiguity. For example, the following:
//!
//! ```
//! use cove::prelude::*;
//!
//! assert_eq!(5u16.cast::<u8>()?, 5u8);
//!
//! # Ok::<(), Box<cove::errors::LossyCastError<u16, u8>>>(())
//! ```
//!
//! would have to be expressed like this:
//!
//! ```
//! use cove::base::CastImpl;
//!
//! // Type resolution takes two lines...
//! let foo: u8 = 5u16.cast_impl()?;
//! assert_eq!(foo, 5u8);
//!
//! // ...or else one rather awkward line
//! assert_eq!(<u16 as CastImpl<u8>>::cast_impl(5)?, 5u8);
//!
//! # Ok::<(), Box<cove::errors::LossyCastError<u16, u8>>>(())
//! ```
//!
//! It is neither necessary nor advised to implement [`Cast`](crate::Cast) directly; instead,
//! implement [`CastImpl`] to extend casting functionality to new types, and the follow-on extension
//! traits ([`AssumedLossless`](crate::AssumedLossless) / [`Estimated`](crate::Estimated) /
//! [`Lossless`](crate::Lossless) / [`Lossy`](crate::Lossy) / [`Saturated`](crate::Saturated)) as
//! appropriate.

/// Provides the base trait for [`Cast`](crate::Cast); implement this to extend
/// [`Cast`](crate::Cast) to new types.
pub trait CastImpl<T> {
    /// Specifies the error type returned from [`cast_impl`](CastImpl::cast_impl). Note that some
    /// blanket implementations for the follow-on extension traits may apply if this is one of the
    /// error types provided by this crate ([`LossyCastError`](crate::LossyCastError) /
    /// [`FailedCastError`](crate::FailedCastError)).
    type Error;

    /// Casts `self` to type `T`; see [`Cast::cast`](crate::Cast::cast) for details and invariants
    /// to uphold.
    ///
    /// # Errors
    /// Returns `Err` if the cast is lossy; that is, if the casted value is not equal to `self`
    fn cast_impl(self) -> Result<T, Self::Error>;
}