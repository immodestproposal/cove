//! Provides error types returned from [`Cast::cast`](crate::casts::Cast) for casts provided by
//! cove. See the [`crate error documentation`](crate#cast-errors) for an overview.

use core::fmt::{Debug, Display, Formatter};
use core::marker::PhantomData;

// -- LossyCastError -- //

/// Indicates that a cast between numeric types lost data.
///
/// This is used for most of cove's casts, and enables usage of various follow-on traits; see the
/// [`crate documentation`](crate) for an overview.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LossyCastError<CastFrom, CastImpl> {
    /// The original value before the cast
    pub from: CastFrom,

    /// The lossy value after the cast
    pub to: CastImpl
}

impl<CastFrom: Display, CastImpl: Display> Display for LossyCastError<CastFrom, CastImpl> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "Numerical cast was lossy [{} ({}) -> {} ({})]",
            self.from, core::any::type_name::<CastFrom>(),
            self.to, core::any::type_name::<CastImpl>()
        )
    }
}

#[cfg(feature = "std")]
impl<CastFrom: Debug + Display, CastImpl: Debug + Display>
std::error::Error for LossyCastError<CastFrom, CastImpl> {}

// -- FailedCastError -- //
/// Indicates that a cast between numeric types would have lost data but could not even create the
/// lossy value.
///
/// This is generally used for casts from primitives to the `NonZero*` family in
/// [`core::num`], as there is no way to create the associated `NonZero*` in the face of a `0` value
/// without invoking undefined behavior. This error enables usage of various follow-on traits; see
/// the [`crate documentation`](crate) for an overview.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FailedCastError<CastFrom, CastImpl> {
    /// The original value before the cast
    pub from: CastFrom,

    // -- Implementation -- //
    to: PhantomData<CastImpl>
}

impl<CastFrom, CastImpl> FailedCastError<CastFrom, CastImpl> {
    /// Creates a new [`FailedCastError`] from the provided `source`
    pub fn new(source: CastFrom) -> Self {
        Self {
            from: source,
            to: PhantomData
        }
    }
}

impl<CastFrom: Display, CastImpl> Display for FailedCastError<CastFrom, CastImpl> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "Numerical cast failed [{} ({}) -> ({})]",
            self.from,
            core::any::type_name::<CastFrom>(),
            core::any::type_name::<CastImpl>()
        )
    }
}

#[cfg(feature = "std")]
impl<CastFrom: Debug + Display, CastImpl: Debug>
std::error::Error for FailedCastError<CastFrom, CastImpl> {}