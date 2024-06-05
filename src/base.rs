//! Provides the base trait needed to support casting through the [`Cast`](crate::casts::Cast) trait
//!
//! This module is for extending casting support; if you only need to use existing casts this
//! module will be irrelevant to you, but it is needed for extending cove's casts to new types.
//!
//! Casting functionality is split between [`CastImpl`] and [`Cast`](crate::casts::Cast) because
//! [`CastImpl`] is more flexible to implement while [`Cast`](crate::casts::Cast) is more
//! ergonomic to use. Without this divide it would be difficult to provide easy turbofish-based type
//! resolution in the face of ambiguity. For example, the following:
//!
//! ```
//! # use cove::prelude::*;
//! assert_eq!(5u16.cast::<u8>()?, 5u8);
//! # Ok::<(), Box<cove::errors::LossyCastError<u16, u8>>>(())
//! ```
//!
//! would have to be expressed in two lines like this:
//!
//! ```
//! # use cove::base::CastImpl;
//! let foo: u8 = 5u16.cast_impl()?;
//! assert_eq!(foo, 5u8);
//! # Ok::<(), Box<cove::errors::LossyCastError<u16, u8>>>(())
//! ```
//!
//! ...or else one rather awkward line:
//!
//! ```
//! # use cove::base::CastImpl;
//! assert_eq!(<u16 as CastImpl<u8>>::cast_impl(5)?, 5u8);
//! # Ok::<(), Box<cove::errors::LossyCastError<u16, u8>>>(())
//! ```
//!
//! Implement [`CastImpl`] to extend casting functionality to new types, and the follow-on extension
//! traits ([`AssumedLossless`](crate::casts::AssumedLossless) /
//! [`Closest`](crate::casts::Closest) / [`Lossless`](crate::casts::Lossless) /
//! [`Lossy`](crate::casts::Lossy)) as appropriate. Upon implementing [`CastImpl`], be sure to also
//! implement [`Cast`](crate::casts::Cast) using the default implementation; essentially, just mark
//! the type as implementing [`Cast`](crate::casts::Cast).
//!
//! **Example of extending casting functionality:**
//!
//! ```
//! use cove::prelude::*;
//! use cove::base::CastImpl;
//! use cove::errors::LossyCastError;
//!
//! // Define a newtype to serve as an example of extending casting support
//! #[derive(Debug, PartialEq)]
//! struct Wrapper(u16);
//!
//! // Mark the newtype as supporting Cast
//! impl Cast for Wrapper {}
//!
//! // Provide casting from the newtype to u8 by implementing CastImpl
//! impl CastImpl<u8> for Wrapper {
//!     // Here we re-use cove's LossyCastError, but that is not required
//!     type Error = LossyCastError<Self, u8>;
//!
//!     // Provide the actual implementation
//!     fn cast_impl(self) -> Result<u8, Self::Error> {
//!         // For this implementation we delegate to the u16 -> u8 cast implementation and just
//!         // adapt the error type
//!         self.0.cast::<u8>().map_err(|error| LossyCastError {
//!             from: Self(error.from),
//!             to: error.to
//!         })
//!     }
//! }
//!
//! // Now we can cast from our Wrapper to u8
//! assert_eq!(Wrapper(8).cast::<u8>().unwrap(), 8u8);
//! assert_eq!(Wrapper(300).cast::<u8>().unwrap_err().to, 44u8);
//!
//! // Because we used LossyCastError, the Lossy and AssumedLossless traits work automatically.
//! // Note that AssumedLossless also requires Wrapper to implement Debug, which it does.
//! assert_eq!(Wrapper(8).cast::<u8>().assumed_lossless(), 8u8);
//! assert_eq!(Wrapper(300).cast::<u8>().lossy(), 44u8);
//!
//! // If Closest or Lossless is desired it may be necessary to use a different error type;
//! // otherwise it will be difficult to implement those extension traits due to Rust's orphaning
//! // rules. To leverage Cove's blanket implementations, be sure to implement the follow-on 
//! // extension traits on the error type.
//! ```

/// Provides the base trait for [`Cast`](crate::casts::Cast); implement this to extend
/// [`Cast`](crate::casts::Cast) to new types.
///
/// See the [module documentation](crate::base) for an example.
pub trait CastImpl<T> {
    /// Specifies the error type returned from [`cast_impl`](CastImpl::cast_impl) and by
    /// extension from [`Cast::cast`](crate::casts::Cast::cast). Note that some blanket
    /// implementations for the follow-on extension traits may apply if this is one of the error
    /// types provided by this crate ([`LossyCastError`](crate::errors::LossyCastError) /
    /// [`FailedCastError`](crate::errors::FailedCastError)).
    type Error;

    /// Casts `self` to type `T`; see [`Cast::cast`](crate::casts::Cast::cast) for details and
    /// invariants to uphold.
    ///
    /// # Errors
    /// Returns `Err` if the cast is lossy; that is, if the casted value is not equal to `self`
    fn cast_impl(self) -> Result<T, Self::Error>;
}