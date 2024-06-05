//! This module provides blanket implementations of certain casting traits where applicable

use crate::base::CastImpl;
use crate::bounds::{CastTo, CastToClosest, CastToLossless};
use crate::casts::{AssumedLossless, Cast, Closest, Lossless, Lossy};
use crate::errors::{FailedCastError, LossyCastError};
use super::LosslessCast;
use core::fmt::Debug;

// Blanket implementation for AssumedLossless applied to all LossyCastErrors
impl<CastFrom: Debug, CastTo: Debug> AssumedLossless<CastTo>
for LossyCastError<CastFrom, CastTo> {
    #[inline]
    fn assumed_lossless(self) -> CastTo {
        // Should not arrive here; panic in a debug build
        debug_assert!(
            false,
            "Lossy cast was assumed to be lossless [{:?} ({}) -> {:?} ({})]",
            self.from, core::any::type_name::<CastFrom>(),
            self.to, core::any::type_name::<CastTo>()
        );

        // Use the lossy value
        self.to
    }
}

// Blanket implementation for Results containing Err variants which implement AssumedLossless
impl<T, Error: AssumedLossless<T>> AssumedLossless<T> for Result<T, Error> {
    #[inline]
    fn assumed_lossless(self) -> T {
        self.unwrap_or_else(AssumedLossless::assumed_lossless)
    }
}

// Blanket implementation for Results containing Err variants which implement Closest
impl<T, Error: Closest<T>> Closest<T> for Result<T, Error> {
    #[inline]
    fn closest(self) -> T {
        self.unwrap_or_else(Closest::closest)
    }
}

// Blanket implementation for Results containing LossyCastErrors which implement LosslessCast.
// We do this specifically for LossyCastErrors instead of all implementing Errors because we want
// the CastFrom and CastTo types to populate the debug_assert message.
impl<CastFrom, CastTo> Lossless<CastTo> for Result<CastTo, LossyCastError<CastFrom, CastTo>> where
    LossyCastError<CastFrom, CastTo> : LosslessCast {
    #[inline]
    fn lossless(self) -> CastTo {
        unsafe {lossless_impl::<CastFrom, _, _>(self)}
    }
}

// Blanket implementation for Results containing FailedCastErrors which implement LosslessCast.
// We do this specifically for FailedCastErrors instead of all implementing Errors because we want
// the CastFrom and CastTo types to populate the debug_assert message.
impl<CastFrom, CastTo> Lossless<CastTo> for Result<CastTo, FailedCastError<CastFrom, CastTo>> where
    FailedCastError<CastFrom, CastTo> : LosslessCast {
    #[inline]
    fn lossless(self) -> CastTo {
        unsafe {lossless_impl::<CastFrom, _, _>(self)}
    }
}

/// Helper function for the Lossless cast implementations
/// 
/// # Safety
/// Must only be called for `result`s which are guaranteed Ok(())
unsafe fn lossless_impl<CastFrom, CastTo, Error>(result: Result<CastTo, Error>) -> CastTo {
    // Panic in debug builds if this is implemented incorrectly; this implies a bug in Cove
    debug_assert!(
        result.is_ok(),
        "Implementation error: implemented Lossless for invalid types [{} -> {}]",
        core::any::type_name::<CastFrom>(),
        core::any::type_name::<CastTo>()
    );

    // Just unwrap the Ok variant, as the result cannot be Err
    result.unwrap_unchecked()
}

// Blanket implementation for Lossy applied to all LossyCastErrors
impl<CastFrom, CastTo> Lossy<CastTo> for LossyCastError<CastFrom, CastTo> {
    #[inline]
    fn lossy(self) -> CastTo {
        self.to
    }
}

// Blanket implementation for Results containing Err variants which implement Lossy
impl<T, Error: Lossy<T>> Lossy<T> for Result<T, Error> {
    #[inline]
    fn lossy(self) -> T {
        self.unwrap_or_else(Lossy::lossy)
    }
}

// Blanket implementation for the CastTo subtrait
impl<
    TO,
    ERROR: Copy + AssumedLossless<TO> + Closest<TO> + Lossy<TO>,
    FROM: Cast + CastImpl<TO, Error = ERROR>
> CastTo<TO> for FROM {
    type _Error = ERROR;
}

// Blanket implementation for the CastToClosest subtrait
impl<
    TO, 
    ERROR: Copy + Closest<TO>, 
    FROM: Cast + CastImpl<TO, Error = ERROR>
> CastToClosest<TO> for FROM {
    type _Error = ERROR;
}

// Blanket implementation for the CastToLossless subtrait
impl<
    TO,
    ERROR: Copy + LosslessCast,
    FROM: Cast + CastImpl<TO, Error = ERROR>
> CastToLossless<TO> for FROM {
    type _Error = ERROR;
}