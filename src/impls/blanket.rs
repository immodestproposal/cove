use crate::cast::{AssumeLossless, Closest, Lossless, Lossy, LossyCastError, Saturated};
use super::LosslessCast;
use core::fmt::Display;

impl<CastFrom: Display, CastTo: Display> AssumeLossless<CastTo>
for Result<CastTo, LossyCastError<CastFrom, CastTo>> {
    #[inline]
    fn assume_lossless(self) -> CastTo {
        self.unwrap_or_else(|error| {
            // Should not arrive here; panic in a debug build
            debug_assert!(
                false,
                "Lossy cast was assumed to be lossless [{} ({}) -> {} ({})]",
                error.from, stringify!(CastFrom),
                error.to, stringify!(CastTo)
            );

            // Use the lossy value
            error.to
        })
    }
}

impl<CastFrom, CastTo> Closest<CastTo> for Result<CastTo, LossyCastError<CastFrom, CastTo>>
where LossyCastError<CastFrom, CastTo> : Closest<CastTo> {
    #[inline]
    fn closest(self) -> CastTo {
        self.unwrap_or_else(Closest::closest)
    }
}

impl<CastFrom, CastTo> Lossless<CastTo> for Result<CastTo, LossyCastError<CastFrom, CastTo>>
where Result<CastTo, LossyCastError<CastFrom, CastTo>> : LosslessCast {
    #[inline]
    fn lossless(self) -> CastTo {
        debug_assert!(
            self.is_ok(),
            "Implementation error: implemented Lossless for invalid types [{} -> {}]",
            stringify!(CastFrom),
            stringify!(CastTo)
        );

        unsafe {self.unwrap_unchecked()}
    }
}

impl<CastFrom, CastTo> Lossy<CastTo> for Result<CastTo, LossyCastError<CastFrom, CastTo>> {
    #[inline]
    fn lossy(self) -> CastTo {
        self.unwrap_or_else(|error| error.to)
    }
}

impl<CastFrom, CastTo> Saturated<CastTo> for Result<CastTo, LossyCastError<CastFrom, CastTo>>
where LossyCastError<CastFrom, CastTo> : Saturated<CastTo> {
    #[inline]
    fn saturated(self) -> CastTo {
        self.unwrap_or_else(Saturated::saturated)
    }
}
