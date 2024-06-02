//! This module provides blanket implementations of certain casting traits where applicable

use crate::base::CastImpl;
use crate::casts::{AssumedLossless, Cast, CastTo, Closest, Lossless, Lossy};
use crate::errors::{FailedCastError, LossyCastError};
use super::LosslessCast;
use core::fmt::Debug;

impl<CastFrom: Debug, CastImpl: Debug> AssumedLossless<CastImpl>
for Result<CastImpl, LossyCastError<CastFrom, CastImpl>> {
    #[inline]
    fn assumed_lossless(self) -> CastImpl {
        self.unwrap_or_else(|error| {
            // Should not arrive here; panic in a debug build
            debug_assert!(
                false,
                "Lossy cast was assumed to be lossless [{:?} ({}) -> {:?} ({})]",
                error.from, stringify!(CastFrom),
                error.to, stringify!(CastImpl)
            );

            // Use the lossy value
            error.to
        })
    }
}

impl<CastFrom, CastImpl> Closest<CastImpl> for Result<CastImpl, LossyCastError<CastFrom, CastImpl>>
where LossyCastError<CastFrom, CastImpl> : Closest<CastImpl> {
    #[inline]
    fn closest(self) -> CastImpl {
        self.unwrap_or_else(Closest::closest)
    }
}

impl<CastFrom, CastImpl> Closest<CastImpl> for Result<CastImpl, FailedCastError<CastFrom, CastImpl>>
    where FailedCastError<CastFrom, CastImpl> : Closest<CastImpl> {
    #[inline]
    fn closest(self) -> CastImpl {
        self.unwrap_or_else(Closest::closest)
    }
}

impl<CastFrom, CastImpl> Lossless<CastImpl> for Result<CastImpl, LossyCastError<CastFrom, CastImpl>>
where Result<CastImpl, LossyCastError<CastFrom, CastImpl>> : LosslessCast {
    #[inline]
    fn lossless(self) -> CastImpl {
        debug_assert!(
            self.is_ok(),
            "Implementation error: implemented Lossless for invalid types [{} -> {}]",
            stringify!(CastFrom),
            stringify!(CastImpl)
        );

        unsafe {self.unwrap_unchecked()}
    }
}

impl<CastFrom, CastImpl> Lossy<CastImpl> for Result<CastImpl, LossyCastError<CastFrom, CastImpl>> {
    #[inline]
    fn lossy(self) -> CastImpl {
        self.unwrap_or_else(|error| error.to)
    }
}

impl<TO, FROM: Cast + CastImpl<TO, Error = LossyCastError<FROM, TO>> + Sized> CastTo<TO> for FROM {}