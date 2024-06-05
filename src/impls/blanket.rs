//! This module provides blanket implementations of certain casting traits where applicable

use crate::bounds::{CastTo, CastToClosest};
use crate::casts::{AssumedLossless, Cast, Closest, Lossless, Lossy};
use crate::errors::LossyCastError;
use super::LosslessCast;
use core::fmt::Debug;
use crate::base::CastImpl;

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

// Blanket implementation for Results containing Err variants which implement LosslessCast
impl<T, Error: LosslessCast> Lossless<T> for Result<T, Error> {
    #[inline]
    fn lossless(self) -> T {
        debug_assert!(
            self.is_ok(),
            "Implementation error: implemented Lossless for invalid types [{} -> {}]",
            stringify!(CastFrom),
            stringify!(CastImpl)
        );

        unsafe {self.unwrap_unchecked()}
    }
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