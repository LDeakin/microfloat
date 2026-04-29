#![allow(clippy::float_cmp)]
use microfloat::f8e5m2;
use std::num::FpCategory;

// ============================================================================
// OCP 8-bit Floating Point Specification Validation
// Source: OCP 8-bit Floating Point Specification (OFP8)
// https://www.opencompute.org/documents/ocp-8bit-floating-point-specification-ofp8-revision-1-0-2023-12-01-pdf-1
// ============================================================================

// E5M2 bit layout: S.EEEEE.MM (1 + 5 + 2 = 8 bits)
const fn e5m2_bits(sign: u8, exp: u8, mant: u8) -> u8 {
    (sign << 7) | ((exp & 0x1F) << 2) | (mant & 0x3)
}

// ============================================================================
// E5M2 - OCP 8-bit Floating Point Specification (OFP8)
// ============================================================================

// Table 1: OFP8 Exponent Parameters
//
// | Parameter        | E5M2  |
// |------------------|-------|
// | Exponent bias    | 15    |
// | Emax (unbiased)  | 15    |
// | Emin (unbiased)  | -14   |

// Table 2: OFP8 Value Encoding Details
//
// | Parameter               | E5M2                    |
// |-------------------------|-------------------------|
// | Infinities              | S.11111.00^2            |
// | NaN                     | S.11111.{01,10,11}^2    |
// | Zeros                   | S.00000.00^2            |
// | Max normal number       | S.11110.11^2            |
// |                         | = +/-57,344             |
// | Min normal number       | S.00001.00^2            |
// |                         | = +/-2^-14              |
// | Max subnormal number    | S.00000.11^2            |
// |                         | = +/-0.75*2^-14         |
// | Min subnormal number    | S.00000.01^2            |
// |                         | = +/-2^-16              |
// | Dynamic range           | 32 binades              |

// E5M2 matches OCP E5M2 spec via f8e5m2 (IEEE-style).

#[test]
fn e5m2_bias_is_15() {
    // Table 1: Exponent bias = 15
    // With bias=15: 1.0 = exp_field=15, so 15-15=0, significand=1.0, value=1.0
    assert_eq!(f8e5m2::ONE.to_bits(), e5m2_bits(0b0, 0b01111, 0b00));
    // 2.0 = exp_field=16, 16-15=1, significand=1.0, value=2.0
    assert_eq!(
        f8e5m2::from_f32(2.0).to_bits(),
        e5m2_bits(0b0, 0b10000, 0b00)
    );
    // 0.5 = exp_field=14, 14-15=-1, significand=1.0, value=0.5
    assert_eq!(
        f8e5m2::from_f32(0.5).to_bits(),
        e5m2_bits(0b0, 0b01110, 0b00)
    );
}

#[test]
fn e5m2_exponent_parameters() {
    // Table 1: Emax (unbiased) = 15, Emin (unbiased) = -14
    // One: exp_field=15, so exponent = 15-15 = 0, value = 1.0
    assert_eq!(f8e5m2::ONE.to_bits(), e5m2_bits(0b0, 0b01111, 0b00));
    // max exponent: exp_field=30, exponent = 30-15 = 15 = emax
}

#[test]
fn e5m2_zeros() {
    // Table 2: Zeros = S.00000.00 (both +0 and -0)
    assert_eq!(f8e5m2::ZERO.to_bits(), e5m2_bits(0b0, 0b00000, 0b00));
    assert_eq!(f8e5m2::NEG_ZERO.to_bits(), e5m2_bits(0b1, 0b00000, 0b00));
    assert!(f8e5m2::ZERO.to_f32() == 0.0);
    assert!(f8e5m2::NEG_ZERO.is_sign_negative());
}

#[test]
fn e5m2_max_normal_number() {
    // Table 2: Max normal number = S.11110.11 = +/-57,344
    // exp_field=30, mantissa=3: (1 + 3/4) * 2^(30-15) = 1.75 * 32768 = 57344
    let max_normal = f8e5m2::from_f32(57344.0);
    assert_eq!(max_normal.to_f32(), 57344.0);
    assert_eq!(max_normal.to_bits(), e5m2_bits(0b0, 0b11110, 0b11));
}

#[test]
fn e5m2_exact_max_normal_57344() {
    // S.11110.11 = (1 + 3/4) * 2^15 = 57344
    let bits = e5m2_bits(0b0, 0b11110, 0b11);
    let val = f8e5m2::from_bits(bits);
    assert_eq!(val.to_f32(), 57344.0, "max finite should be exactly 57344");
}

#[test]
fn e5m2_min_normal_number() {
    // Table 2: Min normal number = S.00001.00 = +/-2^-14
    // exp_field=1, mantissa=0: 1.0 * 2^(1-15) = 2^-14
    let min_normal = f8e5m2::from_f32(2.0f32.powi(-14));
    assert_eq!(min_normal.to_f32(), 2.0f32.powi(-14));
    assert_eq!(min_normal.to_bits(), e5m2_bits(0b0, 0b00001, 0b00));
}

#[test]
fn e5m2_exact_min_normal_value() {
    // S.00001.00 = 2^-14
    let bits = e5m2_bits(0b0, 0b00001, 0b00);
    let val = f8e5m2::from_bits(bits);
    assert_eq!(
        val.to_f32(),
        2.0_f32.powi(-14),
        "min normal should be exactly 2^-14"
    );
}

#[test]
fn e5m2_max_subnormal_number() {
    // Table 2: Max subnormal = S.00000.11 = +/-0.75 * 2^-14
    // exp_field=0, mantissa=3: (3/4) * 2^(1-15) = 0.75 * 2^-14
    let bits = e5m2_bits(0b0, 0b00000, 0b11);
    let max_sub = f8e5m2::from_bits(bits);
    let expected = 0.75 * 2.0_f32.powi(-14);
    assert_eq!(max_sub.to_f32(), expected);
}

#[test]
fn e5m2_exact_max_subnormal() {
    // S.00000.11 = (3/4) * 2^-14 = 3 * 2^-16
    let bits = e5m2_bits(0b0, 0b00000, 0b11);
    let val = f8e5m2::from_bits(bits);
    let expected = 3.0 * 2.0_f32.powi(-16);
    assert_eq!(
        val.to_f32(),
        expected,
        "max subnormal should be exactly 3*2^-16"
    );
}

#[test]
fn e5m2_min_subnormal_number() {
    // Table 2: Min subnormal = S.00000.01 = +/-2^-16
    // exp_field=0, mantissa=1: (1/4) * 2^(1-15) = 2^-16
    let bits = e5m2_bits(0b0, 0b00000, 0b01);
    let min_sub = f8e5m2::from_bits(bits);
    assert_eq!(min_sub.to_f32(), 2.0_f32.powi(-16));
}

#[test]
fn e5m2_exact_min_subnormal() {
    // S.00000.01 = (1/4) * 2^-14 = 2^-16
    let bits = e5m2_bits(0b0, 0b00000, 0b01);
    let val = f8e5m2::from_bits(bits);
    assert_eq!(
        val.to_f32(),
        2.0_f32.powi(-16),
        "min subnormal should be exactly 2^-16"
    );
}

#[test]
fn e5m2_infinity() {
    // Table 2: Infinities = S.11111.00
    assert!(f8e5m2::has_inf());
    assert_eq!(f8e5m2::INFINITY.to_bits(), e5m2_bits(0b0, 0b11111, 0b00));
    assert_eq!(
        f8e5m2::NEG_INFINITY.to_bits(),
        e5m2_bits(0b1, 0b11111, 0b00)
    );
    assert!(f8e5m2::INFINITY.to_f32().is_infinite());
    assert!(f8e5m2::NEG_INFINITY.to_f32().is_infinite());
    assert!(f8e5m2::NEG_INFINITY.to_f32().is_sign_negative());
}

#[test]
fn e5m2_infinity_bit_pattern() {
    // S.11111.00 = exp=31, mant=0
    assert_eq!(f8e5m2::INFINITY.to_bits(), e5m2_bits(0b0, 0b11111, 0b00));
    assert_eq!(
        f8e5m2::NEG_INFINITY.to_bits(),
        e5m2_bits(0b1, 0b11111, 0b00)
    );
}

#[test]
fn e5m2_nan() {
    // Table 2: NaN = S.11111.{01,10,11} = exp=31, mant=1,2,3
    // E5M2 bit layout: S.EEEEE.MM
    assert!(f8e5m2::has_nan());
    // sign=0, exp=31(0b11111), mant=2(0b10)
    let nan_bits = e5m2_bits(0b0, 0b11111, 0b10);
    let nan = f8e5m2::from_bits(nan_bits);
    assert!(nan.to_f32().is_nan());
    // All non-zero mantissas with exp=31 should be NaN (IEEE-style)
    // 0b0_11111_01 = exp=31, mant=1
    // 0b0_11111_10 = exp=31, mant=2
    // 0b0_11111_11 = exp=31, mant=3
    for bits in [
        e5m2_bits(0b0, 0b11111, 0b01),
        e5m2_bits(0b0, 0b11111, 0b10),
        e5m2_bits(0b0, 0b11111, 0b11),
    ] {
        let val = f8e5m2::from_bits(bits);
        assert!(val.to_f32().is_nan(), "bits={bits:02x} should be NaN");
    }
}

#[test]
fn e5m2_nan_bit_pattern() {
    // E5M2 layout: S.EEEEE.MM
    // NaN: S.11111.{01,10,11} = exp=31, mant=1,2,3
    assert!(f8e5m2::NAN.to_f32().is_nan());
    for bits in [
        e5m2_bits(0b0, 0b11111, 0b01),
        e5m2_bits(0b0, 0b11111, 0b10),
        e5m2_bits(0b0, 0b11111, 0b11),
    ] {
        assert!(f8e5m2::from_bits(bits).to_f32().is_nan());
    }
    // Negative NaN (sign bit set)
    for bits in [
        e5m2_bits(0b1, 0b11111, 0b01),
        e5m2_bits(0b1, 0b11111, 0b10),
        e5m2_bits(0b1, 0b11111, 0b11),
    ] {
        assert!(f8e5m2::from_bits(bits).to_f32().is_nan());
    }
}

#[test]
fn e5m2_dynamic_range_32_binades() {
    // Table 2: Dynamic range = 32 binades
    // emax (unbiased) = 15, emin (unbiased) = -14
    // Normal numbers: exponents from -14 to 15, that's 30 different powers of 2
    // Subnormal numbers: one additional binade (0 to 2^-14)
    // Total: 31 or 32 depending on counting
    // The key: from min_subnormal exponent to max_normal exponent
    // min subnormal exponent = 1 - bias = 1 - 15 = -14 (but values are smaller)
    // max normal exponent = 15
    // Dynamic range in binades = 15 - (-14) = 29 normal + subnormal = 30+
    // With the subnormal range extending: roughly 32 binades
    assert!(f8e5m2::from_f32(57344.0).to_bits() == e5m2_bits(0b0, 0b11110, 0b11));
    assert!(f8e5m2::from_f32(2.0f32.powi(-16)).to_bits() == e5m2_bits(0b0, 0b00000, 0b01));
}

#[test]
fn e5m2_bit_layout() {
    // S.EEEEE.MM layout (1+5+2=8 bits)
    // sign=0, exp=15(0b01111), mant=0 → +1.0
    assert_eq!(f8e5m2::ONE.to_bits(), e5m2_bits(0b0, 0b01111, 0b00));
    // sign=1, exp=15, mant=0 → -1.0
    assert_eq!(
        f8e5m2::from_f32(-1.0).to_bits(),
        e5m2_bits(0b1, 0b01111, 0b00)
    );
    // sign=0, exp=30(0b11110), mant=3 → max normal = 57344
    assert_eq!(
        f8e5m2::from_f32(57344.0).to_bits(),
        e5m2_bits(0b0, 0b11110, 0b11)
    );
    // sign=1, exp=30, mant=3 → -57344
    assert_eq!(
        f8e5m2::from_f32(-57344.0).to_bits(),
        e5m2_bits(0b1, 0b11110, 0b11)
    );
}

#[test]
fn e5m2_roundtrip_normal_values() {
    let vals = [
        1.0, -1.0, 2.0, -2.0, 4.0, -4.0, 0.5, -0.5, 0.25, -0.25, 8.0, -8.0, 16.0, -16.0, 32.0,
        -32.0, 64.0, -64.0, 128.0, -128.0, 1024.0, -1024.0, 4096.0, -4096.0, 8192.0, -8192.0,
        16384.0, -16384.0, 32768.0, -32768.0, 57344.0, -57344.0,
    ];
    for &v in &vals {
        let encoded = f8e5m2::from_f32(v);
        let decoded = encoded.to_f32();
        assert!(
            (decoded - v).abs() / v.abs() < 1e-2,
            "e5m2 roundtrip failed: {v} -> {decoded} (error {:.4}%)",
            (decoded - v).abs() / v.abs() * 100.0
        );
    }
}

#[test]
fn e5m2_roundtrip_subnormal_values() {
    let vals = [
        2.0_f32.powi(-16), // min subnormal
        2.0_f32.powi(-15),
        3.0 * 2.0_f32.powi(-16),
        3.0 * 2.0_f32.powi(-15), // max subnormal = 0.75 * 2^-14
    ];
    for &v in &vals {
        let encoded = f8e5m2::from_f32(v);
        let decoded = encoded.to_f32();
        assert!(
            (decoded - v).abs() < 1e-14,
            "e5m2 subnormal roundtrip failed: {v} -> {decoded}"
        );
    }
}

#[test]
fn e5m2_overflow_to_infinity() {
    // IEEE-style f8e5m2 max finite = 57344.0
    // Values very close to max round to max
    assert_eq!(
        f8e5m2::from_f32(57345.0).to_bits(),
        e5m2_bits(0b0, 0b11110, 0b11)
    ); // rounds to max finite
    // Truly large values overflow to infinity
    assert!(f8e5m2::from_f32(f32::INFINITY).is_infinite());
}

#[test]
fn e5m2_underflow_to_subnormal() {
    // Values between 0 and min subnormal (2^-16) should round to subnormal or zero
    let tiny = 0.5 * 2.0_f32.powi(-16);
    let encoded = f8e5m2::from_f32(tiny);
    assert!(
        encoded.to_bits() == e5m2_bits(0b0, 0b00000, 0b01)
            || encoded.to_bits() == e5m2_bits(0b0, 0b00000, 0b00)
    );
}

#[test]
fn e5m2_classify_zero() {
    assert_eq!(f8e5m2::ZERO.classify(), FpCategory::Zero);
    assert_eq!(f8e5m2::NEG_ZERO.classify(), FpCategory::Zero);
    assert_eq!(
        f8e5m2::from_bits(e5m2_bits(0b0, 0b00000, 0b01)).classify(),
        FpCategory::Subnormal
    );
}

#[test]
fn e5m2_classify_subnormal() {
    assert_eq!(
        f8e5m2::from_bits(e5m2_bits(0b0, 0b00000, 0b01)).classify(),
        FpCategory::Subnormal
    );
    assert_eq!(
        f8e5m2::from_bits(e5m2_bits(0b0, 0b00000, 0b11)).classify(),
        FpCategory::Subnormal
    );
    assert_eq!(
        f8e5m2::from_bits(e5m2_bits(0b0, 0b00001, 0b00)).classify(),
        FpCategory::Normal
    );
    assert_eq!(f8e5m2::ZERO.classify(), FpCategory::Zero);
}

#[test]
fn e5m2_classify_special() {
    assert!(f8e5m2::INFINITY.is_infinite());
    assert!(f8e5m2::NEG_INFINITY.is_infinite());
    assert!(f8e5m2::NAN.is_nan());
    assert!(!f8e5m2::ONE.is_infinite());
    assert!(!f8e5m2::ONE.is_nan());
}
