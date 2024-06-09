use cove::prelude::*;

#[test]
fn lossless() {
    assert_eq!(0u128.cast::<i8>().closest(), 0i8);
    assert_eq!((-99f64).cast::<i16>().closest(), -99i16);
}

#[test]
fn saturating() {
    assert_eq!(u128::MAX.cast::<i32>().closest(), i32::MAX);
    assert_eq!((-300f32).cast::<usize>().closest(), 0usize);
}

#[test]
fn rounding() {
    assert_eq!(5.5f32.cast::<u8>().closest(), 6u8);
    assert_eq!(5.49f32.cast::<u8>().closest(), 5u8);
}