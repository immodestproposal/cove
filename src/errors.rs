use core::fmt::{Debug, Display, Formatter};
use core::marker::PhantomData;

// -- LossyCastError -- //

/// Indicates that a cast between numeric types lost data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LossyCastError<CastFrom, CastTo> {
    /// The original value before the cast
    pub from: CastFrom,

    /// The value after the cast
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FailedCastError<CastFrom, CastTo> {
    /// The original value before the cast
    pub from: CastFrom,

    // -- Implementation -- //
    to: PhantomData<CastTo>
}

impl<CastFrom, CastTo> FailedCastError<CastFrom, CastTo> {
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