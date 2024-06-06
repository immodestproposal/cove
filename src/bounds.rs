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
//! The function signature is verbose and awkward, and requires an undesirable level of familiarity 
//! with Cove's implementation. Now compare the above example with this one, rewritten to use the 
//! bounding trait [`CastTo`]:
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
//! 
//! That syntactic cleanup is the goal of this module. Note that using bounding traits is not 
//! expected to be a common use case and thus these traits are not included in Cove's prelude.
//! 
//! # Support
//! The different bounding trait cover different use cases and supported types, according to the 
//! tables. Note that the supported types are those provided out-of-the-box by Cove, but external 
//! types may also be supported (see [`CastImpl`] for details on extending Cove support).
//! 
//! Target types supported by each bounding trait:
//! 
//! | Trait                 | Supported Target Types        | 
//! | ---                   | ---                           |
//! | [`CastTo`]            | all primitives                |
//! | [`CastToClosest`]     | all primitives + `NonZero*`   |
//! | [`CastToLossless`]    | all primitives + `NonZero*` where guaranteed lossless by types alone |  
//! 
//! Casting traits supported by each bounding trait:
//!   
//! | Trait              | [`Cast`] | [`AssumedLossless`] | [`Closest`] | [`Lossy`] | [`Lossless`] | 
//! | ---                | ---      | ---                 | ---         | ---       | ---          |
//! | [`CastTo`]         | ✔       | ✔                   | ✔          | ✔         |              | 
//! | [`CastToClosest`]  | ✔       |                     | ✔           |           |              |
//! | [`CastToLossless`] | ✔       |                     |             |           | ✔            |

use crate::base::CastImpl;
use crate::casts::{AssumedLossless, Cast, Closest, Lossless, Lossy};

/// Provides a convenience subtrait for use with bounding generic function parameters
/// 
/// This is the "go-to" bounding trait since it covers the most common use cases. If this does 
/// not cover your use case, consider one of the other bounding traits:
/// 
/// * [`CastToClosest`]: only supports [`Cast`] and [`Closest`] casting traits, but covers the 
/// `NonZero*` family of numbers in addition to the primitives
/// * [`CastToLossless`]: only supports a subset of source and target types, but guarantees 
/// a lossless cast at compilation time via the [`Lossless`] trait
///
/// # Examples
///
/// Using [`Cast`] without a follow-on:
/// ```
/// use cove::prelude::*;
/// use cove::bounds::CastTo;
///
/// fn is_cast_lossless(x: impl CastTo<u8>) -> bool {
///     x.cast().is_ok()
/// }
///
/// assert!(is_cast_lossless(7i8));
/// assert!(!is_cast_lossless(300u16));
/// ```
///
/// Using [`Lossy`]:
///
/// ```
/// use cove::prelude::*;
/// use cove::bounds::CastTo;
///
/// /// Returns true if `lhs` and `rhs` are equal after being lossily casted to u32, false otherwise
/// fn lossy_are_equal(lhs: impl CastTo<u32>, rhs: impl CastTo<u32>) -> bool {
///     lhs.cast().lossy() == rhs.cast().lossy()
/// }
///
/// assert!(lossy_are_equal(10.3f32, 10u64));
/// assert!(!lossy_are_equal(-200i16, -202i32));
/// ```
///
/// Using [`AssumedLossless`]:
///
/// ```
/// use cove::prelude::*;
/// use cove::bounds::CastTo;
///
/// /// Casts `x` to i128; if this is lossy, it will panic in a debug build (and just be silently
/// /// lossy in a release build)
/// fn foo(x: impl CastTo<i128>) -> i128 {
///     x.cast().assumed_lossless()
/// }
/// 
/// assert_eq!(foo(9800), 9800i128);
/// ```
///
/// Using [`Closest`]:
///
/// ```
/// use cove::prelude::*;
/// use cove::bounds::CastTo;
/// 
/// /// Casts `x` to a u16, yielding the closest possible value
/// fn close_enough(x: impl CastTo<u16>) -> u16 {
///     x.cast().closest()
/// }
/// 
/// assert_eq!(close_enough(99u8), 99u16);
/// assert_eq!(close_enough(-5i8), 0u16);
/// assert_eq!(close_enough(4.5f32), 5u16);
/// assert_eq!(close_enough(u32::MAX), u16::MAX);
/// ```
pub trait CastTo<T> : Cast + CastImpl<T, Error = <Self as CastTo<T>>::_Error> {
    /// This associated type is intended for internal use only; it is part of a workaround for Rust
    /// not yet (as of 1.78.0) supporting trait aliases in stable, nor elaborating where clauses to 
    /// subtraits. Both are open issues, hence the workaround.
    type _Error: Copy + AssumedLossless<T> + Closest<T> + Lossy<T>;
}

/// Provides a convenience subtrait for use with bounding generic function parameters
///
/// This bounding trait can handle casts to the `NonZero*` family of numbers (in addition to the 
/// primitives) but supports only the [`Cast`] and [`Closest`] casting traits. Unless you need 
/// support for a `NonZero*` target type, consider using [`CastTo`] or [`CastToLossless`] instead.
///
/// # Examples
///
/// Using [`Cast`] without a follow-on:
/// ```
/// use cove::prelude::*;
/// use cove::bounds::CastToClosest;
///
/// fn is_cast_lossless(x: impl CastToClosest<i64>) -> bool {
///     x.cast().is_ok()
/// }
///
/// assert!(is_cast_lossless(-100i128));
/// assert!(!is_cast_lossless(98.2f64));
/// ```
///
/// Using [`Closest`]:
///
/// ```
/// use cove::prelude::*;
/// use cove::bounds::CastToClosest;
///
/// /// Casts `x` to a u16, yielding the closest possible value
/// fn close_enough(x: impl CastToClosest<f32>) -> f32 {
///     x.cast().closest()
/// }
///
/// assert_eq!(close_enough(88u8), 88f32);
/// assert_eq!(close_enough(-1024i16), -1024f32);
/// assert_eq!(close_enough(f64::MAX), f32::MAX);
/// assert_eq!(close_enough(f64::NEG_INFINITY), f32::NEG_INFINITY);
/// assert_eq!(close_enough(u32::MAX), u32::MAX as f32);
/// assert_eq!(close_enough(u128::MAX), f32::MAX);
/// ```
pub trait CastToClosest<T> : Cast + CastImpl<T, Error = <Self as CastToClosest<T>>::_Error> {
    /// This associated type is intended for internal use only; it is part of a workaround for Rust
    /// not yet (as of 1.78.0) supporting trait aliases in stable, nor elaborating where clauses to 
    /// subtraits. Both are open issues, hence the workaround.
    type _Error: Copy + Closest<T>;
}

/// Provides a convenience subtrait for use with bounding generic function parameters
///
/// This bounding trait only supports casts which are guaranteed to be lossless at compilation time 
/// as deduced from the types alone, such as i64 -> isize on a 64-bit platform or u8 -> u32 on any 
/// platform. This is powerful when applicable but ultimately limited in scope; if your use case 
/// does not match consider using consider using [`CastTo`] or [`CastToClosest`] instead.
///
/// # Examples
/// ```
/// use cove::prelude::*;
/// use cove::bounds::CastToLossless;
///
/// /// Casts `x` to a usize losslessly
/// fn foo(x: impl CastToLossless<usize>) -> usize {
///     x.cast().lossless()
/// }
///
/// // u8 -> usize is lossless on all platforms
/// assert_eq!(foo(8u8), 8usize);
///
/// #[cfg(target_pointer_width = "64")] {
///     // This will only compile on 64-bit platforms; on a 32-bit platform this will fail to build
///     assert_eq!(foo(u64::MAX), usize::MAX);
/// }
/// ```
pub trait CastToLossless<T>: Cast + CastImpl<T, Error = <Self as CastToLossless<T>>::_Error> {
    /// This associated type is intended for internal use only; it is part of a workaround for Rust
    /// not yet (as of 1.78.0) supporting trait aliases in stable, nor elaborating where clauses to 
    /// subtraits. Both are open issues, hence the workaround.
    type _Error: Copy + Lossless<T>;
}