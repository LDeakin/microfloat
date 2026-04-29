#![allow(clippy::float_cmp)]
use float8::F8E5M2;
use microfloat::f8e5m2;
use std::num::FpCategory;

// ============================================================================
// float8 F8E5M2 behavior vs microfloat f8e5m2 behavior
//
// float8::F8E5M2 uses NVIDIA __NV_SATFINITE saturation semantics, matching the behavior defined in https://gitlab.com/nvidia/headers/cuda-individual/cudart/-/raw/main/cuda_fp8.hpp.
//
// In NVIDIA's model, __nv_saturation_t controls overflow behavior:
//   - __NV_SATFINITE: overflow clamps to max finite value
//   - __NV_NOSAT: overflow produces IEEE NaN or Inf
//
// float8 implements __NV_SATFINITE, where "INFINITY" is the overflow sentinel (FP8_MAXNORM), not a true infinity.
// microfloat implements __NV_NOSAT (IEEE compliant) instead.
//
// float8 also has classify() bug where Zero and NaN are considered Subnormal
// ============================================================================

// E5M2 bit layout: S.EEEEE.MM (1 + 5 + 2 = 8 bits)
const fn e5m2_bits(sign: u8, exp: u8, mant: u8) -> u8 {
    (sign << 7) | ((exp & 0x1F) << 2) | (mant & 0x3)
}

#[test]
fn float8_f8e5m2_classify_zero() {
    // NVIDIA classify() bug (independent of saturation mode):
    // classify() checks !is_normal() before is_nan(), so Zero
    // (exp=0) returns Subnormal instead of Zero.
    assert_eq!(F8E5M2::ZERO.classify(), FpCategory::Subnormal);
    assert_eq!(F8E5M2::NEG_ZERO.classify(), FpCategory::Subnormal);
    // microfloat correct: classify() returns Zero
    assert_eq!(f8e5m2::ZERO.classify(), FpCategory::Zero);
    assert_eq!(f8e5m2::NEG_ZERO.classify(), FpCategory::Zero);
}

#[test]
fn float8_f8e5m2_classify_nan() {
    // NVIDIA classify() bug: classify() checks !is_normal() before is_nan(),
    // so NaN (exp=31) returns Subnormal instead of Nan.
    assert_eq!(F8E5M2::NAN.classify(), FpCategory::Subnormal);
    assert!(F8E5M2::NAN.is_nan());
    assert!(!F8E5M2::NAN.is_finite());
    assert!(!F8E5M2::NAN.is_normal());
    // microfloat correct: classify() returns Nan
    assert_eq!(f8e5m2::NAN.classify(), FpCategory::Nan);
    assert!(f8e5m2::NAN.is_nan());
}

#[test]
fn float8_f8e5m2_satfinite_infinity_constant() {
    // NVIDIA __NV_SATFINITE: FP8_MAXNORM = 0x7B = exp=30, mant=3 = 57344.0
    // This is the overflow sentinel (not a true infinity).
    // See cuda_fp8.hpp: FP8_MAXNORM = 0x7B; res = FP8_MAXNORM;
    //
    // IEEE 754 E5M2: INFINITY = exp=31, mant=0 = e5m2_bits(0, 31, 0)
    assert_eq!(F8E5M2::INFINITY.to_bits(), e5m2_bits(0b0, 0b11110, 0b11));
    assert_eq!(F8E5M2::INFINITY.to_f32(), 57344.0);
    assert!(F8E5M2::INFINITY.is_infinite());
    let inf = F8E5M2::from_bits(e5m2_bits(0b0, 0b11110, 0b11));
    assert!(inf.is_infinite());
    assert_eq!(inf.to_f32(), 57344.0);
    // microfloat (__NV_NOSAT / IEEE): INFINITY at e5m2_bits(0, 31, 0)
    assert!(f8e5m2::has_inf());
    assert_eq!(f8e5m2::INFINITY.to_bits(), e5m2_bits(0b0, 0b11111, 0b00));
    assert!(f8e5m2::INFINITY.is_infinite());
    assert!(f8e5m2::NEG_INFINITY.is_infinite());
}

#[test]
fn float8_f8e5m2_max_57344() {
    // IEEE 754 E5M2: MAX = exp=30, mant=3 = e5m2_bits(0, 30, 3) = 57344.0
    // __NV_SATFINITE: MAX = exp=30, mant=2 = e5m2_bits(0, 30, 2) = 49152.0
    // The MAX constant is one mantissa bit less than IEEE max.
    // float8 INFINITY is actually IEEE E5M2's max finite.
    assert_eq!(F8E5M2::MAX.to_bits(), e5m2_bits(0b0, 0b11110, 0b10));
    assert_eq!(F8E5M2::MAX.to_f32(), 49152.0);
    assert_eq!(F8E5M2::INFINITY.to_bits(), e5m2_bits(0b0, 0b11110, 0b11));
    assert_eq!(F8E5M2::INFINITY.to_f32(), 57344.0);
    // microfloat: MAX = 57344.0 is finite (__NV_NOSAT / IEEE compliant)
    assert_eq!(f8e5m2::MAX.to_bits(), e5m2_bits(0b0, 0b11110, 0b11));
    assert_eq!(f8e5m2::MAX.to_f32(), 57344.0);
    assert!(f8e5m2::MAX.is_finite());
    assert!(!f8e5m2::MAX.is_infinite());
}

#[test]
fn float8_f8e5m2_ieee_infinity_not_recognized() {
    // IEEE 754 E5M2 infinity: exp=31, mant=0 = e5m2_bits(0, 31, 0)
    // __NV_SATFINITE: uses e5m2_bits(0, 30, 3) for infinity, not e5m2_bits(0, 31, 0)
    let ieee_inf = F8E5M2::from_bits(e5m2_bits(0b0, 0b11111, 0b00));
    assert!(!ieee_inf.is_infinite());
    // Yet it decodes to f32::INFINITY through the f16 conversion (inconsistent)
    assert_eq!(ieee_inf.to_f32(), f32::INFINITY);
    // microfloat (__NV_NOSAT / IEEE): IEEE infinity is recognized as infinite
    let mf_inf = f8e5m2::from_bits(e5m2_bits(0b0, 0b11111, 0b00));
    assert!(mf_inf.is_infinite());
    assert_eq!(f8e5m2::INFINITY.to_bits(), e5m2_bits(0b0, 0b11111, 0b00));
}
