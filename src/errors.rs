//! Provides error types returned from [`Cast::cast`](crate::casts::Cast) for casts provided by
//! cove
//! 
//! Cove uses three error types in its casts:
//!
//! * [`LosslessCastError`]: for casts which are guaranteed lossless from their types alone
//!     * Acts as a marker type: cannot actually be constructed 
//! * [`LossyCastError`]: for lossy casts which are able to represent the lossy value as the target 
//! type
//!     * Used in most of cove's casts
//!     * Allows for retrieving the origin and target values via the `from` and `to` member fields:
//!     ```
//!     # use cove::prelude::*;
//!     assert_eq!(260u32.cast::<u8>().unwrap_err().from, 260u32);
//!     assert_eq!(260u32.cast::<u8>().unwrap_err().to, 4u8);
//!     ```
//!     * Provides a descriptive message
//!         * e.g. `"Numerical cast was lossy [260 (u32) -> 4 (u8)]"`
//! * [`FailedCastError`]: for lossy casts which are unable to represent
//!     the lossy value as the target type
//!     * Used for certain `NonZero*` casts, where representing e.g.
//!         [`NonZeroUsize`](core::num::NonZeroUsize) in the error could invoke undefined behavior
//!     * Allows for retrieving the origin (but not target) value via the `from` member field:
//!     ```
//!     # use cove::prelude::*;
//!     # use std::num::NonZeroU8;
//!     assert_eq!(0u32.cast::<NonZeroU8>().unwrap_err().from, 0u32);
//!     ```
//!     * Provides as descriptive an error message as possible
//!         * e.g. `"Numerical cast failed [0 (u32) -> (core::num::nonzero::NonZeroU8)]"`

use core::fmt::{Debug, Display, Formatter};
use core::marker::PhantomData;

/// Indicates that a cast between numeric types could not possibly have lost data, as deduced from 
/// the types alone.
/// 
/// This is used for cove's casts which cannot lose data on the target platform, even if they 
/// could on a different platform (at which point the cast would fail to compile). This error is 
/// similar to [`Infallible`](core::convert::Infallible) in that it cannot be instantiated and 
/// serves instead as a marker which carries the source and target type information.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LosslessCastError<CastFrom, CastTo>(PhantomData<(CastFrom, CastTo)>);

impl<CastFrom: Display, CastTo> Display for LosslessCastError<CastFrom, CastTo> {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> core::fmt::Result {
        // This is safe because LosslessCastError cannot be instantiated; it is really just a holder
        // of type information.
        unsafe {core::hint::unreachable_unchecked()}
    }
}

#[cfg(feature = "std")]
impl<CastFrom: Debug + Display, CastTo: Debug>
std::error::Error for LosslessCastError<CastFrom, CastTo> {}

// -- LossyCastError -- //

/// Indicates that a cast between numeric types lost data.
///
/// This is used for a majority of cove's casts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LossyCastError<CastFrom, CastTo> {
    /// The original value before the cast
    pub from: CastFrom,

    /// The lossy value after the cast
    pub to: CastTo
}

impl<CastFrom: Display, CastTo: Display> Display for LossyCastError<CastFrom, CastTo> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "Numerical cast was lossy [{} ({}) -> {} ({})]",
            self.from, core::any::type_name::<CastFrom>(),
            self.to, core::any::type_name::<CastTo>()
        )
    }
}

#[cfg(feature = "std")]
impl<CastFrom: Debug + Display, CastTo: Debug + Display>
std::error::Error for LossyCastError<CastFrom, CastTo> {}

// -- FailedCastError -- //
/// Indicates that a cast between numeric types would have lost data but could not even create the
/// lossy value.
///
/// This is generally used for casts from primitives to the `NonZero*` family in [`core::num`], as 
/// there is no way to create the associated `NonZero*` in the face of a `0` value without invoking 
/// undefined behavior.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FailedCastError<CastFrom, CastTo> {
    /// The original value before the cast
    pub from: CastFrom,

    // -- Implementation -- //
    to: PhantomData<CastTo>
}

impl<CastFrom, CastTo> FailedCastError<CastFrom, CastTo> {
    /// Creates a new [`FailedCastError`] from the provided `source`
    pub fn new(source: CastFrom) -> Self {
        Self {
            from: source,
            to: PhantomData
        }
    }
}

impl<CastFrom: Display, CastTo> Display for FailedCastError<CastFrom, CastTo> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "Numerical cast failed [{} ({}) -> ({})]",
            self.from,
            core::any::type_name::<CastFrom>(),
            core::any::type_name::<CastTo>()
        )
    }
}

#[cfg(feature = "std")]
impl<CastFrom: Debug + Display, CastTo: Debug>
std::error::Error for FailedCastError<CastFrom, CastTo> {}