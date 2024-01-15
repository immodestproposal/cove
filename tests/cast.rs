//! These tests cover the base Cast trait

use cove::prelude::*;

use core::num::{
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
};

#[test]
#[allow(clippy::cast_possible_truncation)]
fn int_lossless() {
    // Unsigned to unsigned
    assert_eq!(98usize.cast::<u8>().unwrap(), 98u8);
    assert_eq!(62000u64.cast::<u32>().unwrap(), 62000u32);
    assert_eq!(12123u32.cast::<NonZeroU16>().unwrap(), NonZeroU16::new(12123).unwrap());
    assert_eq!(129u16.cast::<NonZeroU128>().unwrap(), NonZeroU128::new(129).unwrap());

    // Unsigned to signed
    assert_eq!(12345u32.cast::<i16>().unwrap(), 12345i16);
    assert_eq!(33usize.cast::<i8>().unwrap(), 33i8);
    assert_eq!(18u128.cast::<NonZeroI32>().unwrap(), NonZeroI32::new(18).unwrap());

    // Signed to signed
    assert_eq!((-82i16).cast::<i8>().unwrap(), -82i8);
    assert_eq!(7000i128.cast::<i64>().unwrap(), 7000i64);
    assert_eq!(NonZeroI64::new(-3000).unwrap().cast::<i16>().unwrap(), -3000i16);

    // Signed to unsigned
    assert_eq!(13i16.cast::<u8>().unwrap(), 13u8);
    assert_eq!(8123i64.cast::<u32>().unwrap(), 8123u32);
    assert_eq!(
        NonZeroIsize::new(9000).unwrap().cast::<NonZeroU32>().unwrap(),
        NonZeroU32::new(9000).unwrap()
    );
}

#[test]
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn int_lossy() {
    // Unsigned to unsigned
    assert_eq!(70000u32.cast::<u16>().unwrap_err().to, 4464u16);
    assert_eq!(401u128.cast::<u8>().unwrap_err().to, 145u8);
    assert_eq!(0u64.cast::<NonZeroUsize>().unwrap_err().from, 0u64);

    // Unsigned to signed
    assert_eq!(1_000_000_000_000u64.cast::<i32>().unwrap_err().to, 1_000_000_000_000u64 as i32);
    assert_eq!(320usize.cast::<i8>().unwrap_err().to, 320usize as i8);
    assert_eq!(NonZeroU8::new(200).unwrap().cast::<i8>().unwrap_err().to, 200u8 as i8);

    // Signed to signed
    assert_eq!((-973i128).cast::<i8>().unwrap_err().to, -973i128 as i8);
    assert_eq!(82010i64.cast::<i16>().unwrap_err().to, 82010i64 as i16);
    assert_eq!(
        NonZeroI128::new(i128::MAX).unwrap().cast::<NonZeroI16>().unwrap_err().to,
        NonZeroI16::new(i128::MAX as i16).unwrap()
    );

    // Signed to unsigned
    assert_eq!((-90i32).cast::<u16>().unwrap_err().to, -90i32 as u16);
    assert_eq!(800i16.cast::<u8>().unwrap_err().to, 800i16 as u8);
    assert_eq!(0isize.cast::<NonZeroU64>().unwrap_err().from, 0isize);
}

#[test]
#[allow(
    clippy::cast_possible_truncation, clippy::unnecessary_cast, clippy::cast_lossless,
    clippy::float_cmp
)]
fn float_lossless() {
    // Narrowing
    assert_eq!((-3.5f64).cast::<f32>().unwrap(), -3.5f64 as f32);
    assert_eq!(80.0f64.cast::<f32>().unwrap(), 80.0f64 as f32);

    // Widening
    assert_eq!(f32::MAX.cast::<f64>().unwrap(), f32::MAX as f64);
    assert_eq!(f32::MIN.cast::<f64>().unwrap(), f32::MIN as f64);

    // To unsigned
    assert_eq!(7f32.cast::<u32>().unwrap(), 7u32);
    assert_eq!(89991f64.cast::<NonZeroU32>().unwrap(), NonZeroU32::new(89991).unwrap());

    // To signed
    assert_eq!(980f32.cast::<isize>().unwrap(), 980isize);
    assert_eq!((-87f64).cast::<NonZeroI8>().unwrap(), NonZeroI8::new(-87).unwrap());

    // From unsigned
    assert_eq!(NonZeroUsize::new(7770).unwrap().cast::<f32>().unwrap(), 7770usize as f32);
    assert_eq!(10_705_040u64.cast::<f64>().unwrap(), 10_705_040f64);

    // From signed
    assert_eq!((-99199i32).cast::<f32>().unwrap(), -99199f32);
    assert_eq!(NonZeroI128::new(88).unwrap().cast::<f64>().unwrap(), 88f64);
}

#[test]
#[allow(
    clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss,
    clippy::float_cmp
)]
fn float_lossy() {
    // Narrowing
    assert_eq!(f64::MIN.cast::<f32>().unwrap_err().to, f64::MIN as f32);
    assert_eq!(f64::MAX.cast::<f32>().unwrap_err().to, f64::MAX as f32);

    // To unsigned
    assert_eq!(9.78f32.cast::<u32>().unwrap_err().to, 9.78f32 as u32);
    assert_eq!(0f64.cast::<NonZeroU128>().unwrap_err().from, 0f64);
    assert_eq!((-77f64).cast::<u64>().unwrap_err().to, -77f64 as u64);

    // To signed
    assert_eq!(6000.33f64.cast::<i8>().unwrap_err().to, 6000.33f64 as i8);
    assert_eq!((-5060.122f32).cast::<NonZeroI16>().unwrap_err().from, -5060.122f32);
    assert_eq!((-893.442f32).cast::<i32>().unwrap_err().to, -893i32);

    // From unsigned
    u32::MAX.cast::<f32>().unwrap_err();
    assert_eq!(u32::MAX.cast::<f32>().unwrap_err().to, u32::MAX as f32);
    assert_eq!(NonZeroU128::MAX.cast::<f64>().unwrap_err().to, u128::MAX as f64);

    // From signed
    assert_eq!(i64::MIN.cast::<f32>().unwrap(), i64::MIN as f32);
    assert_eq!((i64::MIN + 1).cast::<f32>().unwrap_err().to, (i64::MIN + 1) as f32);
    assert_eq!(i64::MAX.cast::<f64>().unwrap_err().to, i64::MAX as f64);
}