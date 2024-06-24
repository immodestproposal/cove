//! These tests cover the `Closest` trait for primitives to primitives. This is a hard one for 
//! which to provide random testing. 

use std::num::NonZeroI8;
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

#[test]
#[allow(clippy::float_cmp)]
fn infinity() {
    assert_eq!(f64::INFINITY.cast::<f64>().closest(), f64::INFINITY);
    assert_eq!(f64::INFINITY.cast::<f32>().closest(), f32::INFINITY);
    assert_eq!(f64::NEG_INFINITY.cast::<f64>().closest(), f64::NEG_INFINITY);
    assert_eq!(f64::NEG_INFINITY.cast::<f32>().closest(), f32::NEG_INFINITY);

    assert_eq!(f32::INFINITY.cast::<f64>().closest(), f64::INFINITY);
    assert_eq!(f32::INFINITY.cast::<f32>().closest(), f32::INFINITY);
    assert_eq!(f32::NEG_INFINITY.cast::<f64>().closest(), f64::NEG_INFINITY);
    assert_eq!(f32::NEG_INFINITY.cast::<f32>().closest(), f32::NEG_INFINITY);

    assert_eq!(f64::INFINITY.cast::<u32>().closest(), u32::MAX);
    assert_eq!(f32::NEG_INFINITY.cast::<i16>().closest(), i16::MIN);
}

#[test]
#[allow(clippy::float_cmp)]
fn overflow() {
    assert_eq!(u128::MAX.cast::<f32>().closest(), f32::MAX);
    assert_eq!(f64::MIN.cast::<f32>().closest(), f32::MIN);
}

#[test]
fn nan() {
    assert!(f64::NAN.cast::<f64>().closest().is_nan());
    assert!(f64::NAN.cast::<f32>().closest().is_nan());
    assert!(f32::NAN.cast::<f64>().closest().is_nan());
    assert!(f32::NAN.cast::<f32>().closest().is_nan());
    
    assert_eq!(f32::NAN.cast::<i8>().closest(), 0i8);
    assert_eq!(f32::NAN.cast::<NonZeroI8>().closest().get(), 1i8);
    assert_eq!((-f32::NAN).cast::<NonZeroI8>().closest().get(), -1i8);
}