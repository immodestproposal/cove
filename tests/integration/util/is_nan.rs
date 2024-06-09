/// Helper trait to identify whether a number is NaN
pub trait IsNaN {
    /// Returns true if `self` is NaN, false otherwise
    fn is_nan(&self) -> bool;
}

impl IsNaN for f32 {
    fn is_nan(&self) -> bool {
        f32::is_nan(*self)
    }
}

impl IsNaN for f64 {
    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }
}

// Marks the provided types as never NaN
macro_rules! never_nan {
    ($($int:ty),*) => {
        $(
            impl IsNaN for $int {
                fn is_nan(&self) -> bool {
                    false
                }
            }
        )*
    }
}

never_nan!(
    u8, u16, u32, u64, u128, usize, 
    i8, i16, i32, i64, i128, isize,
    core::num::NonZeroU8, core::num::NonZeroU16, core::num::NonZeroU32, 
    core::num::NonZeroU64, core::num::NonZeroU128, core::num::NonZeroUsize,
    core::num::NonZeroI8, core::num::NonZeroI16, core::num::NonZeroI32,
    core::num::NonZeroI64, core::num::NonZeroI128, core::num::NonZeroIsize
);