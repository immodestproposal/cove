//! Provides convenience traits for bounding generic types
//!
//! Cove's casting traits are purpose-built for easy syntax when performing casts and follow-on 
//! casts. A side effect of optimizing for these use cases is less convenient syntax when 
//! attempting to use the traits as generic type bounds.
//!
//! Consider this artificial example of a function which will accept any type which can be 
//! lossily cast to an f32:
//! ```
//! use cove::base::CastImpl;
//! use cove::prelude::*;
//! 
//! fn foo<E: Lossy<f32>, T: Cast + CastImpl<f32, Error = E>>(x: T) -> f32 {
//!     x.cast().lossy()
//! }
//! 
//! assert_eq!(foo(38i128), 38f32);
//! ```
//! 
//! This is verbose and awkward, and requires an undesirable level of familiarity with Cove's 
//! implementation. This module aims to alleviate this issue by providing convenience traits
//! designed for bounding generic types. This is not expected to be a common use case, and thus
//! these traits are not included in Cove's prelude.
//! 
//! By way of illustration, compare the above example with this one, rewritten to use the bounding 
//! trait [`CastTo`]:
//! ```
//! use cove::bounds::CastTo;
//! use cove::prelude::*;
//! 
//! fn foo(x: impl CastTo<f32>) -> f32 {
//!     x.cast().lossy()
//! }
//!
//! assert_eq!(foo(38i128), 38f32);
//! ``` 

use crate::base::CastImpl;
use crate::casts::{AssumedLossless, Cast, Closest, Lossy};

/// Provides a convenience subtrait for use with bounding generic function parameters
///
/// # Support
/// [`CastTo`] is supported as a blanket implementation for all casts which can yield an error 
/// type which implements [`Copy`](core::marker::Copy), [`AssumedLossless`], [`Closest`], and 
/// [`Lossy`]. In practice this means casts to primitive types, and is expected to be the most
/// common bounding trait.
///
/// # Examples
/// Using [`Lossy`]:
/// ```
/// use cove::prelude::*;
/// use cove::bounds::CastTo;
///
/// // An artificial example function using CastTo as a bounds to compare disparate types
/// fn lossy_are_equal(lhs: impl CastTo<u32>, rhs: impl CastTo<u32>) -> bool {
///     lhs.cast().lossy() == rhs.cast().lossy()
/// }
///
/// assert!(lossy_are_equal(10.3f32, 10u64));
/// assert!(!lossy_are_equal(-200i16, -202i32));
/// ```
/// ```
/// use cove::prelude::*;
/// use cove::bounds::CastTo;
/// 
/// fn foo(x: impl CastTo<i128>) -> i128 {
///     x.cast().assumed_lossless()
/// }
/// 
/// assert_eq!(foo(9800), 9800i128);
/// ```
pub trait CastTo<T> : Cast + CastImpl<T, Error = <Self as CastTo<T>>::_Error> {
    /// This associated type is intended for internal use only; it is part of a workaround for Rust
    /// not yet (as of 1.78.0) supporting trait aliases in stable, nor elaborating where clauses to 
    /// subtraits. Both are open issues, hence the workaround.
    type _Error: Copy + AssumedLossless<T> + Closest<T> + Lossy<T>;
}

///
///
/// This next example shows a more verbose version, to account for those cases which cannot use 
/// [`CastTo`]:
/// ```
/// use cove::prelude::*;
/// use cove::base::CastImpl;
/// use cove::bounds::CastTo;
/// use cove::errors::FailedCastError;
/// use core::num::NonZeroI8;
///
/// // An artificial example function which finds the closest NonZeroI8 to the input and checks
/// // whether its value is 8.
/// fn closest_is_8_verbose<
///     T: Cast + CastImpl<NonZeroI8, Error = FailedCastError<T, NonZeroI8>>
/// >
/// (x: T) -> bool where FailedCastError<T, NonZeroI8> : Closest<NonZeroI8> {
///     x.cast().closest() == NonZeroI8::new(8).unwrap()
/// }
///
/// assert!(closest_is_8_verbose(8.1f64));
/// assert!(!closest_is_8_verbose(70u32));
///
/// // Compare the above to this version using CastTo; the NonZeroI8 has been swapped for an i8,
/// // since NonZeroI8 if not a primitive and does not support CastTo.
///
/// fn closest_is_8_succint(x: impl CastTo<i8>) -> bool {
///     x.cast().closest() == 8i8
/// }
///
/// assert!(closest_is_8_succint(8.1f64));
/// assert!(!closest_is_8_succint(70u32));
/// ```
pub trait CastToClosest<T> : Cast + CastImpl<T, Error = <Self as CastToClosest<T>>::_Error> {
    type _Error: Copy + Closest<T>;
}

impl<TO, ERROR: Copy + Closest<TO>, FROM: Cast + CastImpl<TO, Error = ERROR>> CastToClosest<TO> for
FROM {
    type _Error = ERROR;
}