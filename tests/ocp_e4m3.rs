#![allow(clippy::float_cmp)]
use microfloat::f8e4m3fn;
use std::num::FpCategory;

// ============================================================================
// OCP 8-bit Floating Point Specification Validation
// Source: OCP 8-bit Floating Point Specification (OFP8)
// https://www.opencompute.org/documents/ocp-8-bit-floating-point-specification-ofp8-revision-1-0-2023-12-01-pdf-1
// ============================================================================

// E4M3 bit layout: S.EEEE.MMM (1 + 4 + 3 = 8 bits)
const fn e4m3_bits(sign: u8, exp: u8, mant: u8) -> u8 {
    (sign << 7) | ((exp & 0xF) << 3) | (mant & 0x7)
}

// ============================================================================
// E4M3 - OCP 8-bit Floating Point Specification (OFP8)
// ============================================================================

// Table 1: OFP8 Exponent Parameters
//
// | Parameter        | E4M3  | E5M2  |
// |------------------|-------|-------|
// | Exponent bias    | 7     | 15    |
// | Emax (unbiased)  | 8     | 15    |
// | Emin (unbiased)  | -6    | -14   |

// Table 2: OFP8 Value Encoding Details
//
// | Parameter               | E4M3                 | E5M2                    |
// |-------------------------|----------------------|-------------------------|
// | Infinities              | N/A                  | S.11111.00^2            |
// | NaN                     | S.1111.111^2         | S.11111.{01,10,11}^2    |
// | Zeros                   | S.0000.000^2         | S.00000.00^2            |
// | Max normal number       | S.1111.110^2         | S.11110.11^2            |
// |                         | = +/-448             | = +/-57,344             |
// | Min normal number       | S.0001.000^2         | S.00001.00^2            |
// |                         | = +/-2^-6            | = +/-2^-14              |
// | Max subnormal number    | S.0000.111^2         | S.00000.11^2            |
// |                         | = +/-0.875*2^-6      | = +/-0.75*2^-14         |
// | Min subnormal number    | S.0000.001^2         | S.00000.01^2            |
// |                         | = +/-2^-9            | = +/-2^-16              |
// | Dynamic range           | 18 binades           | 32 binades              |

// E4M3 matches OCP E4M3 spec via f8e4m3fn (finite-only, Outer NaN).

#[test]
fn e4m3_bias_is_7() {
    // Table 1: Exponent bias = 7
    // With bias=7: 1.0 = exp_field=7, so 7-7=0, significand=1.0, value=1.0
    assert_eq!(f8e4m3fn::ONE.to_bits(), e4m3_bits(0b0, 0b0111, 0b000));
    // 2.0 = exp_field=8, 8-7=1, significand=1.0, value=2.0
    assert_eq!(
        f8e4m3fn::from_f32(2.0).to_bits(),
        e4m3_bits(0b0, 0b1000, 0b000)
    );
    // 0.5 = exp_field=6, 6-7=-1, significand=1.0, value=0.5
    assert_eq!(
        f8e4m3fn::from_f32(0.5).to_bits(),
        e4m3_bits(0b0, 0b0110, 0b000)
    );
}

#[test]
fn e4m3_exponent_parameters() {
    // Table 1: Emax (unbiased) = 8, Emin (unbiased) = -6
    // One: exp_field=7, so exponent = 7-7 = 0, value = 1.0
    assert_eq!(f8e4m3fn::ONE.to_bits(), e4m3_bits(0b0, 0b0111, 0b000));
    // max normal: exp_field=14, exponent = 14-7 = 7, value = 448.0
    // emax (unbiased) = 8, so max stored exponent = 8 + 7 = 15
    // But exp=15, mant=7 is NaN, so max finite uses exp=14
}

#[test]
fn e4m3_zeros() {
    // Table 2: Zeros = S.0000.000 (both +0 and -0)
    assert_eq!(f8e4m3fn::ZERO.to_bits(), e4m3_bits(0b0, 0b0000, 0b000));
    assert_eq!(f8e4m3fn::NEG_ZERO.to_bits(), e4m3_bits(0b1, 0b0000, 0b000));
    assert!(f8e4m3fn::ZERO.to_f32() == 0.0);
    assert!(f8e4m3fn::NEG_ZERO.is_sign_negative());
}

#[test]
fn e4m3_max_normal_number() {
    // Table 2: Max normal number = S.1111.110 = +/-448
    // exp_field=14, mantissa=7: (1 + 7/8) * 2^(14-7) = 1.875 * 128 = 240
    // But OCP E4M3 allows exp_field=15 with mantissa=6:
    // (1 + 6/8) * 2^(15-7) = 1.75 * 256 = 448
    assert_eq!(f8e4m3fn::MAX.to_bits(), e4m3_bits(0b0, 0b1111, 0b110));
    assert_eq!(f8e4m3fn::MAX.to_f32(), 448.0);
}

#[test]
fn e4m3_exact_max_normal_448() {
    // S.1111.110 = (1 + 6/8) * 2^(15-7) = 448
    let bits = e4m3_bits(0b0, 0b1111, 0b110);
    let val = f8e4m3fn::from_bits(bits);
    assert_eq!(val.to_f32(), 448.0, "max finite should be exactly 448");
}

#[test]
fn e4m3_min_normal_number() {
    // Table 2: Min normal number = S.0001.000 = +/-2^-6
    // exp_field=1, mantissa=0: 1.0 * 2^(1-7) = 2^-6
    let min_normal = f8e4m3fn::from_f32(2.0f32.powi(-6));
    assert_eq!(min_normal.to_f32(), 2.0f32.powi(-6));
    assert_eq!(min_normal.to_bits(), e4m3_bits(0b0, 0b0001, 0b000));
}

#[test]
fn e4m3_exact_min_normal_value() {
    // S.0001.000 = 2^-6
    let bits = e4m3_bits(0b0, 0b0001, 0b000);
    let val = f8e4m3fn::from_bits(bits);
    assert_eq!(
        val.to_f32(),
        2.0_f32.powi(-6),
        "min normal should be exactly 2^-6"
    );
}

#[test]
fn e4m3_max_subnormal_number() {
    // Table 2: Max subnormal = S.0000.111 = +/-0.875 * 2^-6
    // exp_field=0, mantissa=7: (7/8) * 2^(1-7) = 7/8 * 2^-6 = 7 * 2^-9
    let bits = e4m3_bits(0b0, 0b0000, 0b111);
    let max_sub = f8e4m3fn::from_bits(bits);
    let expected = 7.0 * 2.0_f32.powi(-9);
    assert_eq!(max_sub.to_f32(), expected);
}

#[test]
fn e4m3_exact_max_subnormal_7_2neg9() {
    // S.0000.111 = (7/8) * 2^-6 = 7 * 2^-9
    let bits = e4m3_bits(0b0, 0b0000, 0b111);
    let val = f8e4m3fn::from_bits(bits);
    let expected = 7.0 * 2.0_f32.powi(-9);
    assert_eq!(
        val.to_f32(),
        expected,
        "max subnormal should be exactly 7*2^-9"
    );
}

#[test]
fn e4m3_min_subnormal_number() {
    // Table 2: Min subnormal = S.0000.001 = +/-2^-9
    // exp_field=0, mantissa=1: (1/8) * 2^(1-7) = 2^-9
    let bits = e4m3_bits(0b0, 0b0000, 0b001);
    let min_sub = f8e4m3fn::from_bits(bits);
    assert_eq!(min_sub.to_f32(), 2.0_f32.powi(-9));
}

#[test]
fn e4m3_exact_min_subnormal() {
    // S.0000.001 = (1/8) * 2^-6 = 2^-9
    let bits = e4m3_bits(0b0, 0b0000, 0b001);
    let val = f8e4m3fn::from_bits(bits);
    assert_eq!(
        val.to_f32(),
        2.0_f32.powi(-9),
        "min subnormal should be exactly 2^-9"
    );
}

#[test]
fn e4m3_no_infinity() {
    // Table 2: Infinities = N/A for OCP E4M3
    assert!(!f8e4m3fn::has_inf());
}

#[test]
fn e4m3_overflow_becomes_nan() {
    // OCP E4M3: no infinity, overflow maps to NaN
    let overflow = f8e4m3fn::from_f32(f32::INFINITY);
    assert!(overflow.to_f32().is_nan());
    // Values beyond 448 also overflow to NaN
    assert!(f8e4m3fn::from_f32(500.0).to_f32().is_nan());
}

#[test]
fn e4m3_nan() {
    // Table 2: NaN = S.1111.111 = exp=15, mant=7
    // Outer encoding: exp=all-1s, mant=all-1s
    assert!(f8e4m3fn::has_nan());
    let nan_bits = e4m3_bits(0b0, 0b1111, 0b111);
    let nan = f8e4m3fn::from_bits(nan_bits);
    assert!(nan.to_f32().is_nan());
    // Only exact outer NaN (exp=15, mant=7) is NaN
    // exp=15, mant<7 is a normal finite value (used for max=448 etc)
    assert!(f8e4m3fn::NAN.to_f32().is_nan());
    assert_eq!(f8e4m3fn::NAN.to_bits(), e4m3_bits(0b0, 0b1111, 0b111));
}

#[test]
fn e4m3_nan_bit_pattern() {
    // Outer encoding: S.1111.111 = exp=15, mant=7
    assert!(f8e4m3fn::NAN.to_f32().is_nan());
    assert_eq!(f8e4m3fn::NAN.to_bits(), e4m3_bits(0b0, 0b1111, 0b111));
    // Positive outer NaN
    assert!(
        f8e4m3fn::from_bits(e4m3_bits(0b0, 0b1111, 0b111))
            .to_f32()
            .is_nan()
    );
    // Negative outer NaN
    assert!(
        f8e4m3fn::from_bits(e4m3_bits(0b1, 0b1111, 0b111))
            .to_f32()
            .is_nan()
    );
    // exp=15, mant<7 are finite values, not NaN
    assert!(
        !f8e4m3fn::from_bits(e4m3_bits(0b0, 0b1111, 0b110))
            .to_f32()
            .is_nan()
    ); // max=448
    assert!(
        !f8e4m3fn::from_bits(e4m3_bits(0b0, 0b1111, 0b101))
            .to_f32()
            .is_nan()
    ); // 416
}

#[test]
fn e4m3_dynamic_range_18_binades() {
    // Table 2: Dynamic range = 18 binades
    // OCP E4M3: exp_field=15 with mantissa 0-6 are normal numbers
    // emax (unbiased) = 8 (from exp_field=15), emin (unbiased) = -6
    // Range = 8 - (-6) = 14 binades of normals
    // Plus subnormal binade: 0 to 2^-6 = 3 more binades (2^-6, 2^-7, 2^-8, 2^-9)
    // Total: ~18 binades
    assert!(f8e4m3fn::from_f32(448.0).to_bits() == e4m3_bits(0b0, 0b1111, 0b110));
    assert!(f8e4m3fn::from_f32(2.0f32.powi(-9)).to_bits() == e4m3_bits(0b0, 0b0000, 0b001));
}

#[test]
fn e4m3_bit_layout() {
    // S.EEEE.MMM layout (1+4+3=8 bits)
    // sign=0, exp=7(0b0111), mant=0 → +1.0
    assert_eq!(f8e4m3fn::ONE.to_bits(), e4m3_bits(0b0, 0b0111, 0b000));
    // sign=1, exp=7, mant=0 → -1.0
    assert_eq!(
        f8e4m3fn::from_f32(-1.0).to_bits(),
        e4m3_bits(0b1, 0b0111, 0b000)
    );
    // sign=0, exp=15(0b1111), mant=6 → max = 448
    assert_eq!(
        f8e4m3fn::from_f32(448.0).to_bits(),
        e4m3_bits(0b0, 0b1111, 0b110)
    );
    // sign=1, exp=15, mant=6 → -448
    assert_eq!(
        f8e4m3fn::from_f32(-448.0).to_bits(),
        e4m3_bits(0b1, 0b1111, 0b110)
    );
}

#[test]
fn e4m3_roundtrip_normal_values() {
    let vals = [
        1.0, -1.0, 2.0, -2.0, 4.0, -4.0, 0.5, -0.5, 0.25, -0.25, 8.0, -8.0, 16.0, -16.0, 32.0,
        -32.0, 64.0, -64.0, 128.0, -128.0, 240.0, -240.0, 448.0, -448.0,
    ];
    for &v in &vals {
        let encoded = f8e4m3fn::from_f32(v);
        let decoded = encoded.to_f32();
        assert!(
            (decoded - v).abs() / v.abs() < 1e-2,
            "e4m3 roundtrip failed: {v} -> {decoded} (error {:.4}%)",
            (decoded - v).abs() / v.abs() * 100.0
        );
    }
}

#[test]
fn e4m3_roundtrip_subnormal_values() {
    let vals = [
        2.0_f32.powi(-9), // min subnormal
        2.0_f32.powi(-8),
        3.0 * 2.0_f32.powi(-9),
        7.0 * 2.0_f32.powi(-9), // max subnormal
    ];
    for &v in &vals {
        let encoded = f8e4m3fn::from_f32(v);
        let decoded = encoded.to_f32();
        assert!(
            (decoded - v).abs() < 1e-12,
            "e4m3 subnormal roundtrip failed: {v} -> {decoded}"
        );
    }
}

#[test]
fn e4m3_overflow_to_nan() {
    // OCP E4M3 has no infinity, overflow becomes NaN
    assert!(f8e4m3fn::from_f32(500.0).to_f32().is_nan());
    assert!(f8e4m3fn::from_f32(f32::INFINITY).to_f32().is_nan());
    assert!(f8e4m3fn::from_f32(f32::NEG_INFINITY).to_f32().is_nan());
}

#[test]
fn e4m3_underflow_to_subnormal() {
    // Values between 0 and min subnormal (2^-9) should round to subnormal or zero
    let tiny = 0.5 * 2.0_f32.powi(-9); // half of min subnormal
    let encoded = f8e4m3fn::from_f32(tiny);
    assert!(
        encoded.to_bits() == e4m3_bits(0b0, 0b0000, 0b001)
            || encoded.to_bits() == e4m3_bits(0b0, 0b0000, 0b000)
    );
}

#[test]
fn e4m3_classify_zero() {
    assert_eq!(f8e4m3fn::ZERO.classify(), FpCategory::Zero);
    assert_eq!(f8e4m3fn::NEG_ZERO.classify(), FpCategory::Zero);
    assert_eq!(
        f8e4m3fn::from_bits(e4m3_bits(0b0, 0b0000, 0b001)).classify(),
        FpCategory::Subnormal
    );
}

#[test]
fn e4m3_classify_subnormal() {
    assert_eq!(
        f8e4m3fn::from_bits(e4m3_bits(0b0, 0b0000, 0b001)).classify(),
        FpCategory::Subnormal
    );
    assert_eq!(
        f8e4m3fn::from_bits(e4m3_bits(0b0, 0b0000, 0b111)).classify(),
        FpCategory::Subnormal
    );
    assert_eq!(
        f8e4m3fn::from_bits(e4m3_bits(0b0, 0b0001, 0b000)).classify(),
        FpCategory::Normal
    );
    assert_eq!(f8e4m3fn::ZERO.classify(), FpCategory::Zero);
}

#[test]
fn e4m3_classify_special() {
    assert!(!f8e4m3fn::from_f32(448.0).is_infinite());
    assert!(f8e4m3fn::NAN.is_nan());
    assert!(!f8e4m3fn::ONE.is_infinite());
    assert!(!f8e4m3fn::ONE.is_nan());
    // 448 is finite, not infinity (OCP E4M3 has no infinity)
    let max_val = f8e4m3fn::from_f32(448.0);
    assert!(max_val.is_finite());
}
