use cove::prelude::*;

use core::num::{
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
};

#[test]
fn nonzero_assumed_lossless() {
    // Narrowing: NonZero -> NonZero
    assert_eq!(NonZeroI32::new(7).unwrap().cast::<NonZeroU16>().assumed_lossless().get(), 7u16);
    
    // Narrowing: NonZero -> primitive
    assert_eq!(
        NonZeroU32::new(3001).unwrap().cast::<NonZeroI16>().assumed_lossless().get(), 
        3001i16
    );
}

#[test]
#[allow(
    clippy::cast_sign_loss, clippy::float_cmp, 
    clippy::cast_precision_loss, clippy::cast_possible_wrap
)]
fn nonzero_bitwise() {
    // NonZero -> primitive
    assert_eq!(NonZeroU8::new(7).unwrap().cast::<u8>().bitwise(), 7u8);
    assert_eq!(NonZeroI8::new(-7).unwrap().cast::<u8>().bitwise(), -7i8 as u8);
    assert_eq!(
        NonZeroI64::new(i64::MIN).unwrap().cast::<f64>().bitwise(),
        f64::from_ne_bytes(i64::MIN.to_ne_bytes())
    );

    assert_ne!(NonZeroI64::new(i64::MIN).unwrap().cast::<f64>().bitwise(), i64::MIN as f64);
    
    // NonZero -> NonZero
    assert_eq!(
        NonZeroU16::new(u16::MAX).unwrap().cast::<NonZeroI16>().bitwise(),
        NonZeroI16::new(u16::MAX as i16).unwrap()
    );
}

#[test]
#[cfg(target_pointer_width = "16")]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn nonzero_bitwise_16() {
    assert_eq!(NonZeroUsize::new(4500).unwrap().cast::<i16>().bitwise(), 4500i16);
    assert_eq!(NonZeroIsize::new(isize::MIN).unwrap().cast::<u16>().bitwise(), isize::MIN as u16);
    assert_eq!(NonZeroU16::new(8080).unwrap().cast::<usize>().bitwise(), 8080usize);
    assert_eq!(
        NonZeroI16::new(-301).unwrap().cast::<NonZeroUsize>().bitwise().get(),
        -301i16 as usize
    );
}

#[test]
#[cfg(target_pointer_width = "32")]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn nonzero_bitwise_32() {
    assert_eq!(NonZeroUsize::new(45000).unwrap().cast::<i32>().bitwise(), 45000i32);
    assert_eq!(NonZeroIsize::new(isize::MIN).unwrap().cast::<u32>().bitwise(), isize::MIN as u32);
    assert_eq!(NonZeroU32::new(8080).unwrap().cast::<usize>().bitwise(), 8080usize);
    assert_eq!(
        NonZeroI32::new(-301).unwrap().cast::<NonZeroUsize>().bitwise().get(),
        -301i32 as usize
    );
}

#[test]
#[cfg(target_pointer_width = "64")]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn nonzero_bitwise_64() {
    assert_eq!(NonZeroUsize::new(45000).unwrap().cast::<i64>().bitwise(), 45000i64);
    assert_eq!(NonZeroIsize::new(isize::MIN).unwrap().cast::<u64>().bitwise(), isize::MIN as u64);
    assert_eq!(NonZeroU64::new(8080).unwrap().cast::<usize>().bitwise(), 8080usize);
    assert_eq!(
        NonZeroI64::new(-301).unwrap().cast::<NonZeroUsize>().bitwise().get(), 
        -301i64 as usize
    );
}

#[test]
#[cfg(target_pointer_width = "128")]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn nonzero_bitwise_128() {
    assert_eq!(NonZeroUsize::new(45000).unwrap().cast::<i128>().bitwise(), 45000i128);
    assert_eq!(NonZeroIsize::new(isize::MIN).unwrap().cast::<u128>().bitwise(), isize::MIN as u128);
    assert_eq!(NonZeroU128::new(8080).unwrap().cast::<usize>().bitwise(), 8080usize);
    assert_eq!(
        NonZeroI128::new(-301).unwrap().cast::<NonZeroUsize>().bitwise().get(),
        -301i128 as usize
    );
}

#[test]
fn nonzero_closest() {
    // Narrowing: NonZero -> NonZero
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<NonZeroU8>().closest().get(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<NonZeroU8>().closest().get(), 255u8);

    // Narrowing: NonZero -> primitive
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<u8>().closest(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<u8>().closest(), 255u8);

    // Narrowing: primitive -> NonZero
    assert_eq!(3u16.cast::<NonZeroU8>().closest(), NonZeroU8::new(3).unwrap());
    assert_eq!(261u16.cast::<NonZeroU8>().closest(), NonZeroU8::new(255).unwrap());
    assert_eq!(0u16.cast::<NonZeroU8>().closest(), NonZeroU8::new(1).unwrap());

    // Floating point
    assert_eq!(8.0f32.cast::<NonZeroI32>().closest(), NonZeroI32::new(8).unwrap());
}

#[test]
fn nonzero_lossy() {
    // Narrowing: NonZero -> NonZero
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<NonZeroU8>().lossy().get(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<NonZeroU8>().lossy().get(), 5u8);

    // Narrowing: NonZero -> primitive
    assert_eq!(NonZeroU16::new(3).unwrap().cast::<u8>().lossy(), 3u8);
    assert_eq!(NonZeroU16::new(261).unwrap().cast::<u8>().lossy(), 5u8);
}

#[test]
#[allow(clippy::cast_lossless)]
fn lossless() {
    assert_eq!(
        NonZeroU64::new(u64::MAX).unwrap().cast::<NonZeroU128>().lossless().get(), 
        u64::MAX as u128
    );
    
    assert_eq!(NonZeroI8::new(-3i8).unwrap().cast::<NonZeroIsize>().lossless().get(), -3isize);
    assert_eq!(NonZeroUsize::new(1).unwrap().cast::<usize>().lossless(), 1usize);
    
    assert_eq!(
        NonZeroI64::new(i64::MIN).unwrap().cast::<NonZeroI128>().lossless().get(),
        i64::MIN as i128
    );

    // Casts that should not compile
    //let _failure = 0u64.cast::<NonZeroU64>().lossless();
}