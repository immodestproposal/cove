//! This module provides blanket implementations of certain casting traits where applicable

use crate::base::CastImpl;
use crate::bounds::{CastTo, CastToClosest, CastToLossless};
use crate::casts::{AssumedLossless, Cast, Closest, Lossless, Lossy};
use crate::errors::{LosslessCastError, LossyCastError};
use core::fmt::Debug;

// Blanket implementation for AssumedLossless applied to all LosslessCastErrors. We need to
// implement this even though it is impossible to construct a LosslessCastError in order to 
// trigger the blanket implementation for Results.
impl<CastFrom: Debug, CastTo: Debug> AssumedLossless<CastTo>
for LosslessCastError<CastFrom, CastTo> {
    #[inline]
    fn assumed_lossless(self) -> CastTo {
        // This is safe because LosslessCastError cannot be instantiated
        unsafe {std::hint::unreachable_unchecked()}
    }
}

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

// Blanket implementation for Closest applied to all LosslessCastErrors. We need to
// implement this even though it is impossible to construct a LosslessCastError in order to 
// trigger the blanket implementation for Results.
impl<CastFrom: Debug, CastTo: Debug> Closest<CastTo> for LosslessCastError<CastFrom, CastTo> {
    #[inline]
    fn closest(self) -> CastTo {
        // This is safe because LosslessCastError cannot be instantiated
        unsafe {std::hint::unreachable_unchecked()}
    }
}

// Blanket implementation for Results containing Err variants which implement Closest
impl<T, Error: Closest<T>> Closest<T> for Result<T, Error> {
    #[inline]
    fn closest(self) -> T {
        self.unwrap_or_else(Closest::closest)
    }
}

// Blanket implementation for Lossless applied to all LosslessCastErrors. We need to
// implement this even though it is impossible to construct a LosslessCastError in order to 
// trigger the blanket implementation for CastToLossless.
unsafe impl<CastFrom: Debug, CastTo: Debug> Lossless<CastTo> for LosslessCastError<CastFrom, 
CastTo> {
    #[inline]
    fn lossless(self) -> CastTo {
        // This is safe because LosslessCastError cannot be instantiated
        unsafe {std::hint::unreachable_unchecked()}
    }
}

// Blanket implementation for Lossless for Results containing Err variants which implement Lossless
unsafe impl<T, Error: Lossless<T>> Lossless<T> for Result<T, Error> {
    #[inline]
    fn lossless(self) -> T {
        // Provide a debug assertion to catch implementation errors
        debug_assert!(
            self.is_ok(),
            "Implementation error: \
            implemented Lossless for invalid cast error [Target: {}][Error: {}]",
            core::any::type_name::<T>(),
            core::any::type_name::<Error>()
        );
        
        // This is safe because Error implements Lossless; for this not to be safe, the Error type
        // must have unsafely implemented Lossless and not maintained its safety guarantees, which
        // is a bug.
        unsafe {self.unwrap_unchecked()}
    }
}

// Blanket implementation for Lossy applied to all LosslessCastErrors. We need to
// implement this even though it is impossible to construct a LosslessCastError in order to 
// trigger the blanket implementation for Results.
impl<CastFrom: Debug, CastTo: Debug> Lossy<CastTo> for LosslessCastError<CastFrom, CastTo> {
    #[inline]
    fn lossy(self) -> CastTo {
        // This is safe because LosslessCastError cannot be instantiated
        unsafe {std::hint::unreachable_unchecked()}
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
impl<TO, ERROR: Copy + Closest<TO>, FROM: Cast + CastImpl<TO, Error = ERROR>> CastToClosest<TO> 
for FROM {
    type _Error = ERROR;
}

// Blanket implementation for the CastToLossless subtrait
impl<TO, ERROR: Copy + Lossless<TO>, FROM: Cast + CastImpl<TO, Error = ERROR>> CastToLossless<TO> 
for FROM {
    type _Error = ERROR;
}