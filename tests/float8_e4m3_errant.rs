#![allow(clippy::float_cmp)]
use float8::F8E4M3;
use microfloat::f8e4m3fn;
use std::num::FpCategory;

// ============================================================================
// float8 F8E4M3 behavior vs microfloat f8e4m3fn behavior
//
// float8::F8E4M3 uses NVIDIA __NV_SATFINITE saturation semantics, matching the behavior defined in https://gitlab.com/nvidia/headers/cuda-individual/cudart/-/raw/main/cuda_fp8.hpp.
//
// In NVIDIA's model, __nv_saturation_t controls overflow behavior:
//   - __NV_SATFINITE: overflow clamps to max finite value
//   - __NV_NOSAT: overflow produces IEEE NaN or Inf
//
// float8 implements __NV_SATFINITE, where "INFINITY" is the overflow sentinel (FP8_MAXNORM), not a true infinity.
// microfloat implements __NV_NOSAT (IEEE / OCP compliant) instead.
//
// float8 also has classify() bug where Zero and NaN are considered Subnormal
// ============================================================================

// E4M3 bit layout: S.EEEE.MMM (1 + 4 + 3 = 8 bits)
const fn e4m3_bits(sign: u8, exp: u8, mant: u8) -> u8 {
    (sign << 7) | ((exp & 0xF) << 3) | (mant & 0x7)
}

#[test]
fn float8_f8e4m3_classify_zero() {
    // float8 errant: classify() returns Subnormal for Zero
    assert_eq!(F8E4M3::ZERO.classify(), FpCategory::Subnormal);
    assert_eq!(F8E4M3::NEG_ZERO.classify(), FpCategory::Subnormal);
    // microfloat correct: classify() returns Zero
    assert_eq!(f8e4m3fn::ZERO.classify(), FpCategory::Zero);
    assert_eq!(f8e4m3fn::NEG_ZERO.classify(), FpCategory::Zero);
}

#[test]
fn float8_f8e4m3_classify_nan() {
    // float8 errant: classify() returns Subnormal for NaN
    assert_eq!(F8E4M3::NAN.classify(), FpCategory::Subnormal);
    assert!(F8E4M3::NAN.is_nan());
    assert!(!F8E4M3::NAN.is_finite());
    assert!(!F8E4M3::NAN.is_normal());
    // float8 classify() checks !is_normal() before is_nan()
    // microfloat correct: classify() returns Nan
    assert_eq!(f8e4m3fn::NAN.classify(), FpCategory::Nan);
    assert!(f8e4m3fn::NAN.is_nan());
}

#[test]
fn float8_f8e4m3_satfinite_infinity_constant() {
    // NVIDIA __NV_SATFINITE: FP8_MAXNORM = 0x7E = exp=15, mant=6 = 448.0
    // This is the overflow sentinel (not a true infinity).
    // See cuda_fp8.hpp: FP8_MAXNORM = 0x7E; res = FP8_MAXNORM;
    //
    // IEEE 754 E4M3: INFINITY = exp=15, mant=0 = e4m3_bits(0, 15, 0)
    // OCP E4M3: no infinity (N/A)
    assert_eq!(F8E4M3::INFINITY.to_bits(), e4m3_bits(0b0, 0b1111, 0b110));
    assert_eq!(F8E4M3::INFINITY.to_f32(), 448.0);
    assert!(F8E4M3::INFINITY.is_infinite());
    let inf = F8E4M3::from_bits(e4m3_bits(0b0, 0b1111, 0b110));
    assert!(inf.is_infinite());
    assert_eq!(inf.to_f32(), 448.0);
    // microfloat (__NV_NOSAT / OCP): no infinity, 448.0 is finite
    assert!(!f8e4m3fn::has_inf());
    let val = f8e4m3fn::from_bits(e4m3_bits(0b0, 0b1111, 0b110));
    assert_eq!(val.to_f32(), 448.0);
    assert!(val.is_finite());
    assert!(!val.is_infinite());
}

#[test]
fn float8_f8e4m3_max_448() {
    // __NV_SATFINITE: from_f32(448.0) clamps to FP8_MAXNORM = 0x7E
    // which is the same as the "INFINITY" constant.
    let val = F8E4M3::from_f32(448.0);
    assert_eq!(val.to_bits(), e4m3_bits(0b0, 0b1111, 0b110));
    assert_eq!(val.to_bits(), F8E4M3::INFINITY.to_bits());
    assert!(val.is_infinite());
    assert_eq!(val.to_f32(), 448.0);
    // microfloat: MAX = 448.0 is finite (__NV_NOSAT / OCP compliant)
    assert_eq!(f8e4m3fn::MAX.to_bits(), e4m3_bits(0b0, 0b1111, 0b110));
    assert_eq!(f8e4m3fn::MAX.to_f32(), 448.0);
    assert!(f8e4m3fn::MAX.is_finite());
    assert!(!f8e4m3fn::MAX.is_infinite());
}

#[test]
fn float8_f8e4m3_overflow_to_nan() {
    // __NV_NOSAT / OCP E4M3: overflow maps to NaN (no infinity)
    assert!(f8e4m3fn::from_f32(500.0).to_f32().is_nan());
    assert!(f8e4m3fn::from_f32(f32::INFINITY).to_f32().is_nan());
    assert!(f8e4m3fn::from_f32(f32::NEG_INFINITY).to_f32().is_nan());
}

#[test]
fn float8_f8e4m3_ieee_infinity_not_recognized() {
    // IEEE 754 E4M3 infinity: exp=15, mant=0 = e4m3_bits(0, 15, 0)
    // Neither float8 (__NV_SATFINITE) nor microfloat (__NV_NOSAT) use
    // e4m3_bits(0, 15, 0) as infinity.
    let ieee_inf = F8E4M3::from_bits(e4m3_bits(0b0, 0b1111, 0b000));
    assert!(!ieee_inf.is_infinite());
    assert_eq!(ieee_inf.to_f32(), 256.0);
    // microfloat: e4m3_bits(0, 15, 0) decodes as 256.0, which is correct
    let mf_val = f8e4m3fn::from_bits(e4m3_bits(0b0, 0b1111, 0b000));
    assert_eq!(mf_val.to_f32(), 256.0);
    assert!(mf_val.is_finite());
}
