/// Helper trait for marking whether two types have the same size. This should ideally be easy 
/// to express with a type bound (i.e. a where clause), but support for that is still unstable as
/// of the time of writing this. This trait provides a workaround, but unfortunately only for types
/// we already know about; extending Cove will require more work on the part of the implementer. 
pub trait SameSize<T> {}

macro_rules! same_size {
    // The actual impl
    ($lhs:ty => {$($rhs:ty),+}) => {
        $(impl SameSize<$rhs> for $lhs {})*    
    };
  
    // Iteratively generate impls
    ($first:ty, $($lhs:ty),+ => $rhs:tt) => {
        same_size!($first => $rhs);
        $(same_size!($lhs => $rhs);)*
    };
    
    // Generate impls in n-squared fashion
    ($($ty:ty),*) => {
        same_size!($($ty),* => {$($ty),*});
    }
}

same_size!(u8, i8, core::num::NonZeroU8, core::num::NonZeroI8);

#[cfg(target_pointer_width = "16")]
mod platform_dependent {
    use super::*;

    same_size!(u32,  i32,  f32, core::num::NonZeroU32,  core::num::NonZeroI32);
    same_size!(u64,  i64,  f64, core::num::NonZeroU64,  core::num::NonZeroI64);
    same_size!(u128, i128,      core::num::NonZeroU128, core::num::NonZeroI128);

    same_size!(
        u16, i16, usize, isize, 
        core::num::NonZeroU16,   core::num::NonZeroI16,
        core::num::NonZeroUsize, core::num::NonZeroIsize 
    );
}

#[cfg(target_pointer_width = "32")]
mod platform_dependent {
    use super::*;

    same_size!(u16,  i16,       core::num::NonZeroU16,  core::num::NonZeroI16);
    same_size!(u64,  i64,  f64, core::num::NonZeroU64,  core::num::NonZeroI64);
    same_size!(u128, i128,      core::num::NonZeroU128, core::num::NonZeroI128);

    same_size!(
        u32, i32, f32, usize, isize, 
        core::num::NonZeroU32,   core::num::NonZeroI32,
        core::num::NonZeroUsize, core::num::NonZeroIsize 
    );
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;
    
    same_size!(u16,  i16,       core::num::NonZeroU16,  core::num::NonZeroI16);
    same_size!(u32,  i32,  f32, core::num::NonZeroU32,  core::num::NonZeroI32);
    same_size!(u128, i128,      core::num::NonZeroU128, core::num::NonZeroI128);
    
    same_size!(
        u64, i64, f64, usize, isize, 
        core::num::NonZeroU64,   core::num::NonZeroI64,
        core::num::NonZeroUsize, core::num::NonZeroIsize 
    );
}

#[cfg(target_pointer_width = "128")]
mod platform_dependent {
    use super::*;

    same_size!(u16,  i16,       core::num::NonZeroU16,  core::num::NonZeroI16);
    same_size!(u32,  i32,  f32, core::num::NonZeroU32,  core::num::NonZeroI32);
    same_size!(u64,  i64,  f64, core::num::NonZeroU64,  core::num::NonZeroI64);

    same_size!(
        u128, i128, usize, isize, 
        core::num::NonZeroU128,  core::num::NonZeroI128,
        core::num::NonZeroUsize, core::num::NonZeroIsize 
    );
}