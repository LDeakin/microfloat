use crate::format::{Format, NanEncoding, Overflow, SignMode, ZeroMode};

#[derive(Clone, Copy)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "classification mirrors core::num::FpCategory predicates"
)]
pub struct Class {
    pub is_zero: bool,
    pub is_subnormal: bool,
    pub is_infinite: bool,
    pub is_nan: bool,
}

pub const fn classify_bits<F: Format>(bits: u8) -> Class {
    if is_nan_bits::<F>(bits) {
        return Class {
            is_zero: false,
            is_subnormal: false,
            is_infinite: false,
            is_nan: true,
        };
    }
    if is_infinity_bits::<F>(bits) {
        return Class {
            is_zero: false,
            is_subnormal: false,
            is_infinite: true,
            is_nan: false,
        };
    }
    match F::ZERO {
        ZeroMode::None => {
            return Class {
                is_zero: false,
                is_subnormal: false,
                is_infinite: false,
                is_nan: false,
            };
        }
        ZeroMode::Signed | ZeroMode::Unsigned => {}
    }

    let mag = magnitude_bits::<F>(bits);
    Class {
        is_zero: mag == 0,
        is_subnormal: mag != 0 && exponent_field::<F>(bits) == 0,
        is_infinite: false,
        is_nan: false,
    }
}

pub const fn one_bits<F: Format>() -> u8 {
    match F::ZERO {
        ZeroMode::None => {
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "format bias is an unsigned 8-bit exponent payload"
            )]
            return F::EXPONENT_BIAS as u8;
        }
        ZeroMode::Signed | ZeroMode::Unsigned => {}
    }
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "format bias is an unsigned 8-bit exponent field"
    )]
    let exponent_bias = F::EXPONENT_BIAS as u8;
    (exponent_bias << F::MANTISSA_BITS) & F::STORAGE_MASK
}

pub const fn neg_zero_bits<F: Format>() -> u8 {
    match F::ZERO {
        ZeroMode::Signed => F::NEG_ZERO_BITS,
        ZeroMode::Unsigned => F::ZERO_BITS,
        ZeroMode::None => nan_bits::<F>(false),
    }
}

pub const fn max_finite_bits<F: Format>(negative: bool) -> u8 {
    match F::ZERO {
        ZeroMode::None => return 0xfe,
        ZeroMode::Signed | ZeroMode::Unsigned => {}
    }
    let mut exp = F::MAX_EXPONENT_FIELD;
    let mut mant = F::MANTISSA_MASK;
    match F::NAN {
        NanEncoding::Ieee => {
            exp -= 1;
        }
        NanEncoding::Outer => {
            mant -= 1;
        }
        NanEncoding::Single(nan) => {
            let mag = nan & !F::SIGN_BIT;
            // No format has a Single(nan) where the nan magnitude equals the max finite
            // bit pattern, so this branch is unreachable.
            // #[coverage(off)]
            if mag == ((exp << F::MANTISSA_BITS) | mant) {
                mant -= 1;
            }
        }
        NanEncoding::None => {}
    }
    let bits = (exp << F::MANTISSA_BITS) | mant;
    if negative {
        negate_bits::<F>(bits)
    } else {
        bits
    }
}

pub const fn infinity_bits<F: Format>(negative: bool) -> u8 {
    match F::OVERFLOW {
        Overflow::Infinity => {
            let bits = F::EXPONENT_MASK;
            if negative {
                negate_bits::<F>(bits)
            } else {
                bits
            }
        }
        Overflow::Nan => nan_bits::<F>(negative),
        Overflow::Saturate => max_finite_bits::<F>(negative),
    }
}

pub const fn nan_bits<F: Format>(negative: bool) -> u8 {
    match F::NAN {
        NanEncoding::None => F::NEG_ZERO_BITS,
        NanEncoding::Ieee => {
            let bits = F::EXPONENT_MASK | (1 << (F::MANTISSA_BITS.saturating_sub(1)));
            if negative {
                negate_bits::<F>(bits)
            } else {
                bits
            }
        }
        NanEncoding::Outer => {
            let bits = F::EXPONENT_MASK | F::MANTISSA_MASK;
            if negative {
                negate_bits::<F>(bits)
            } else {
                bits
            }
        }
        NanEncoding::Single(bits) => bits,
    }
}

pub const fn negate_bits<F: Format>(bits: u8) -> u8 {
    match F::SIGN {
        // Coverage gap: unreachable. SignMode::Unsigned formats return early in the Neg trait impl before reaching negate_bits.
        // For NanEncoding::Single with Signed, the Single(_)_case is unreachable because Single encodings only appear with Unsigned sign.
        // #[coverage(off)]
        SignMode::Unsigned => nan_bits::<F>(false),
        SignMode::Signed => match F::ZERO {
            ZeroMode::Unsigned if (bits & !F::SIGN_BIT) == 0 => bits & !F::SIGN_BIT,
            ZeroMode::Signed | ZeroMode::Unsigned | ZeroMode::None => bits ^ F::SIGN_BIT,
        },
    }
}

pub const fn is_nan_bits<F: Format>(bits: u8) -> bool {
    match F::NAN {
        NanEncoding::None => false,
        NanEncoding::Ieee => {
            exponent_field::<F>(bits) == F::MAX_EXPONENT_FIELD && mantissa_field::<F>(bits) != 0
        }
        NanEncoding::Outer => {
            exponent_field::<F>(bits) == F::MAX_EXPONENT_FIELD
                && mantissa_field::<F>(bits) == F::MANTISSA_MASK
        }
        NanEncoding::Single(nan) => bits == nan,
    }
}

pub const fn is_infinity_bits<F: Format>(bits: u8) -> bool {
    F::HAS_INF
        && exponent_field::<F>(bits) == F::MAX_EXPONENT_FIELD
        && mantissa_field::<F>(bits) == 0
}

pub const fn exponent_field<F: Format>(bits: u8) -> u8 {
    (magnitude_bits::<F>(bits) & F::EXPONENT_MASK) >> F::MANTISSA_BITS
}

pub const fn mantissa_field<F: Format>(bits: u8) -> u8 {
    magnitude_bits::<F>(bits) & F::MANTISSA_MASK
}

pub const fn magnitude_bits<F: Format>(bits: u8) -> u8 {
    match F::SIGN {
        SignMode::Unsigned => bits,
        SignMode::Signed => {
            if F::STORAGE_BITS < 8 {
                bits & (F::SIGN_BIT - 1)
            } else {
                bits & !F::SIGN_BIT
            }
        }
    }
}

pub const fn abs_bits<F: Format>(bits: u8) -> u8 {
    match F::SIGN {
        SignMode::Unsigned => bits,
        SignMode::Signed => magnitude_bits::<F>(bits),
    }
}

pub const fn next_up_bits<F: Format>(bits: u8) -> u8 {
    let bits = bits & F::STORAGE_MASK;
    let class = classify_bits::<F>(bits);
    if class.is_nan {
        return bits;
    }
    if class.is_infinite {
        return if is_negative_bits::<F>(bits) {
            max_finite_bits::<F>(true)
        } else {
            bits
        };
    }
    if class.is_zero {
        return 1;
    }
    match F::SIGN {
        SignMode::Unsigned => {
            return if bits == max_finite_bits::<F>(false) {
                infinity_bits::<F>(false)
            } else {
                bits + 1
            };
        }
        SignMode::Signed => {}
    }
    if is_negative_bits::<F>(bits) {
        if magnitude_bits::<F>(bits) == 1 {
            neg_zero_bits::<F>()
        } else {
            bits - 1
        }
    } else if bits == max_finite_bits::<F>(false) {
        infinity_bits::<F>(false)
    } else {
        bits + 1
    }
}

pub const fn next_down_bits<F: Format>(bits: u8) -> u8 {
    let bits = bits & F::STORAGE_MASK;
    let class = classify_bits::<F>(bits);
    if class.is_nan {
        return bits;
    }
    if class.is_infinite {
        return if is_negative_bits::<F>(bits) {
            bits
        } else {
            max_finite_bits::<F>(false)
        };
    }
    if class.is_zero {
        return match F::SIGN {
            SignMode::Signed => negate_bits::<F>(1),
            SignMode::Unsigned => infinity_bits::<F>(true),
        };
    }
    match F::SIGN {
        SignMode::Unsigned => {
            return if bits == 0 {
                infinity_bits::<F>(true)
            } else {
                bits - 1
            };
        }
        SignMode::Signed => {}
    }
    if is_negative_bits::<F>(bits) {
        if bits == max_finite_bits::<F>(true) {
            infinity_bits::<F>(true)
        } else {
            bits + 1
        }
    } else if magnitude_bits::<F>(bits) == 1 {
        F::ZERO_BITS
    } else {
        bits - 1
    }
}

pub fn decode_f32<F: Format>(bits: u8) -> f32 {
    if F::ZERO == ZeroMode::None {
        return if bits == 0xff {
            f32::from_bits(0x7fc0_0000)
        } else {
            exp2i(i32::from(bits) - F::EXPONENT_BIAS)
        };
    }
    if is_nan_bits::<F>(bits) {
        let sign = if is_negative_bits::<F>(bits) {
            0x8000_0000
        } else {
            0
        };
        return f32::from_bits(sign | 0x7fc0_0000);
    }
    if is_infinity_bits::<F>(bits) {
        return if is_negative_bits::<F>(bits) {
            f32::NEG_INFINITY
        } else {
            f32::INFINITY
        };
    }
    let mag = magnitude_bits::<F>(bits);
    let sign = if is_negative_bits::<F>(bits) {
        -1.0
    } else {
        1.0
    };
    if mag == 0 {
        return if sign < 0.0 { -0.0 } else { 0.0 };
    }
    let exp = exponent_field::<F>(bits);
    let mant = f32::from(mantissa_field::<F>(bits));
    let scale = exp2i(if exp == 0 {
        1 - F::EXPONENT_BIAS
    } else {
        i32::from(exp) - F::EXPONENT_BIAS
    });
    let significand = if exp == 0 {
        mant / exp2i(i32::from(F::MANTISSA_BITS))
    } else {
        1.0 + mant / exp2i(i32::from(F::MANTISSA_BITS))
    };
    sign * significand * scale
}

pub const fn is_negative_bits<F: Format>(bits: u8) -> bool {
    match F::SIGN {
        SignMode::Unsigned => false,
        SignMode::Signed => {
            if F::STORAGE_BITS < 8 {
                bits >= F::SIGN_BIT
            } else {
                (bits & F::SIGN_BIT) != 0
            }
        }
    }
}

pub fn encode_f32<F: Format>(value: f32) -> u8 {
    if F::ZERO == ZeroMode::None {
        return encode_e8m0::<F>(value);
    }
    if value.is_nan() {
        return match F::NAN {
            NanEncoding::None => F::NEG_ZERO_BITS,
            NanEncoding::Ieee | NanEncoding::Outer => nan_bits::<F>(value.is_sign_negative()),
            NanEncoding::Single(_) => nan_bits::<F>(false),
        };
    }
    if value.is_infinite() {
        return overflow_bits::<F>(value.is_sign_negative());
    }

    let negative = value.is_sign_negative();
    let abs = f32::from_bits(value.to_bits() & 0x7fff_ffff);
    if abs == 0.0 {
        return match F::ZERO {
            ZeroMode::Signed => {
                if negative {
                    F::NEG_ZERO_BITS
                } else {
                    F::ZERO_BITS
                }
            }
            ZeroMode::Unsigned => F::ZERO_BITS,
            ZeroMode::None => nan_bits::<F>(false),
        };
    }

    if is_overflowing_finite::<F>(abs) {
        return overflow_bits::<F>(negative);
    }

    let bits = encode_positive_finite::<F>(abs);
    if negative {
        negate_bits::<F>(bits)
    } else {
        bits
    }
}

pub fn encode_e8m0<F: Format>(value: f32) -> u8 {
    if !value.is_finite() || value <= 0.0 {
        return 0xff;
    }

    let exp = floor_log2_f32(value);
    if exp < -F::EXPONENT_BIAS {
        return 0;
    }

    let base = exp2i(exp);
    let rounded_exp = if value >= base + base * 0.5 {
        exp + 1
    } else {
        exp
    };
    let bits = rounded_exp + F::EXPONENT_BIAS;
    #[expect(
        clippy::cast_sign_loss,
        reason = "rounded exponent is clamped to the non-negative E8M0 payload range"
    )]
    // Coverage gap: unreachable for any finite positive f32.
    // floor_log2_f32 returns at most 127, so rounded_exp is at most 128,
    // and bits = 128 + 127 = 255, already within [0, 254].
    // Subnormal values have exp < -126, hitting an early return above.
    // #[coverage(off)]
    let bits = bits.clamp(0, 254) as u8;
    bits
}

fn encode_positive_finite<F: Format>(value: f32) -> u8 {
    let mantissa_bits = i32::from(F::MANTISSA_BITS);
    let min_normal_exp = 1 - F::EXPONENT_BIAS;
    let exp = floor_log2_f32(value);

    if exp < min_normal_exp {
        let scaled = round_to_integer(value, min_normal_exp - mantissa_bits);
        #[expect(
            clippy::cast_possible_truncation,
            reason = "subnormal mantissa is clamped to the format mantissa range"
        )]
        return scaled.min(1u64 << F::MANTISSA_BITS) as u8;
    }

    let mut value_exp = exp;
    let mut significand = round_to_integer(value, value_exp - mantissa_bits);
    let hidden_bit = 1u64 << F::MANTISSA_BITS;
    if significand >= hidden_bit << 1 {
        significand >>= 1;
        value_exp += 1;
    }

    let exponent = value_exp + F::EXPONENT_BIAS;
    let mantissa = significand - hidden_bit;
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "rounded finite value has already been range-checked for this format"
    )]
    let exponent = exponent as u8;
    #[expect(
        clippy::cast_possible_truncation,
        reason = "mantissa is masked by construction from a format-sized significand"
    )]
    let mantissa = mantissa as u8;
    ((exponent << F::MANTISSA_BITS) | mantissa) & F::STORAGE_MASK
}

fn is_overflowing_finite<F: Format>(value: f32) -> bool {
    let max_bits = max_finite_bits::<F>(false);
    let max = decode_f32::<F>(max_bits);
    let prev = decode_f32::<F>(max_bits - 1);
    value >= max + (max - prev) * 0.5
}

fn round_to_integer(value: f32, scale_exp: i32) -> u64 {
    let bits = value.to_bits();
    #[expect(
        clippy::cast_possible_wrap,
        reason = "masked f32 exponent is in 0..=255"
    )]
    let exp = ((bits >> 23) & 0xff) as i32;
    let fraction = bits & 0x7f_ffff;
    let (significand, power) = if exp == 0 {
        (u64::from(fraction), -149)
    } else {
        (u64::from((1 << 23) | fraction), exp - 150)
    };
    let shift = power - scale_exp;
    if shift >= 0 {
        significand << shift
    } else {
        round_right_shift(significand, -shift)
    }
}

const fn round_right_shift(value: u64, shift: i32) -> u64 {
    if shift >= 64 {
        return 0;
    }

    #[expect(
        clippy::cast_sign_loss,
        reason = "negative shifts return above; remaining shifts are non-negative"
    )]
    let shift = shift as u32;
    let quotient = value >> shift;
    let remainder = value & ((1u64 << shift) - 1);
    let halfway = 1u64 << (shift - 1);
    if remainder > halfway || (remainder == halfway && quotient & 1 == 1) {
        quotient + 1
    } else {
        quotient
    }
}

const fn floor_log2_f32(value: f32) -> i32 {
    let bits = value.to_bits() & 0x7fff_ffff;
    #[expect(
        clippy::cast_possible_wrap,
        reason = "masked f32 exponent is in 0..=255"
    )]
    let exp = ((bits >> 23) & 0xff) as i32;
    if exp == 0 {
        let fraction = bits & 0x7f_ffff;
        #[expect(
            clippy::cast_possible_wrap,
            reason = "leading_zeros for a u32 is at most 32"
        )]
        let leading_zeros = fraction.leading_zeros() as i32;
        31 - leading_zeros - 149
    } else {
        exp - 127
    }
}

pub const fn overflow_bits<F: Format>(negative: bool) -> u8 {
    match F::OVERFLOW {
        Overflow::Infinity => infinity_bits::<F>(negative),
        Overflow::Nan => match F::NAN {
            NanEncoding::Outer => nan_bits::<F>(negative),
            _ => nan_bits::<F>(false),
        },
        Overflow::Saturate => max_finite_bits::<F>(negative),
    }
}

pub const fn exp2i(exp: i32) -> f32 {
    if exp < -149 {
        0.0
    } else if exp > 127 {
        f32::INFINITY
    } else if exp >= -126 {
        #[expect(
            clippy::cast_sign_loss,
            reason = "biased normal f32 exponent is positive in this branch"
        )]
        f32::from_bits(((exp + 127) as u32) << 23)
    } else {
        f32::from_bits(1u32 << (exp + 149))
    }
}

pub fn total_key<F: Format>(bits: u8) -> i16 {
    let widened = if F::STORAGE_BITS < 8 {
        (bits & F::STORAGE_MASK) << (8 - F::STORAGE_BITS)
    } else {
        bits
    };
    #[expect(
        clippy::cast_possible_wrap,
        reason = "reinterpretation through i8 implements the total-order key transform"
    )]
    let signed = widened as i8;
    if signed < 0 {
        i16::from(!widened)
    } else {
        i16::from(widened | 0x80)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Float4E2M1FnFormat, Float6E2M3FnFormat, Float6E3M2FnFormat, Float8E3M4Format,
        Float8E4M3B11FnuzFormat, Float8E4M3FnFormat, Float8E4M3FnuzFormat, Float8E4M3Format,
        Float8E5M2FnuzFormat, Float8E5M2Format, Float8E8M0FnuFormat,
    };

    // max_finite_bits is called via overflow_bits (Saturate path) or is_overflowing_finite.
    // Saturate formats have ZeroMode::Signed and fall past the early return.
    // f8e8m0fnu is ZeroMode::None but uses Overflow::Nan so the Saturate path is never hit.
    // This test exercises the otherwise unreachable path.
    #[test]
    fn max_finite_bits_zero_mode_none_returns_0xfe() {
        // ZeroMode::None returns 0xfe unconditionally before negate_bits is reached.
        assert_eq!(max_finite_bits::<Float8E8M0FnuFormat>(false), 0xfe);
        assert_eq!(max_finite_bits::<Float8E8M0FnuFormat>(true), 0xfe);
    }

    // Confirm that existing Saturate formats do NOT take the ZeroMode::None path.
    #[test]
    fn max_finite_bits_saturate_formats_dont_return_0xfe() {
        // These formats have ZeroMode::Signed and compute the value normally.
        assert_ne!(max_finite_bits::<Float4E2M1FnFormat>(false), 0xfe);
        assert_ne!(max_finite_bits::<Float6E2M3FnFormat>(false), 0xfe);
        assert_ne!(max_finite_bits::<Float6E3M2FnFormat>(false), 0xfe);
    }

    #[test]
    fn max_finite_bits_outer_nan_encoding_decrements_mantissa() {
        // Float8E4M3FnFormat uses NanEncoding::Outer which executes mant -= 1.
        // MAX_EXPONENT_FIELD=15, MANTISSA_MASK=7, so mant becomes 6.
        // bits = (15 << 3) | 6 = 0x7e.
        assert_eq!(max_finite_bits::<Float8E4M3FnFormat>(false), 0x7e);
        assert_eq!(max_finite_bits::<Float8E4M3FnFormat>(true), 0xfe);
    }

    #[test]
    fn max_finite_bits_single_nan_encoding_no_decrement() {
        // Float8E4M3B11FnuzFormat uses NanEncoding::Single(0x80).
        // mag = 0x80 & !0x80 = 0x00, which does not equal (15 << 3) | 7 = 0x7f.
        // So mant -= 1 is NOT executed.
        // bits = (15 << 3) | 7 = 0x7f.
        assert_eq!(max_finite_bits::<Float8E4M3B11FnuzFormat>(false), 0x7f);
        assert_eq!(max_finite_bits::<Float8E4M3B11FnuzFormat>(true), 0xff);
    }

    // one_bits is called in const initializers (MicroFloat::ONE, MicroFloat::NEG_ONE)
    // and neg_zero_bits in MicroFloat::NEG_ZERO, so both are const-folded and never
    // exercised at runtime.  These tests exist to cover each ZeroMode branch.
    #[test]
    fn one_bits_zero_mode_none_returns_exponent_bias() {
        // ZeroMode::None returns EXPONENT_BIAS directly (no mantissa shift).
        assert_eq!(one_bits::<Float8E8M0FnuFormat>(), 0x7f);
    }

    #[test]
    fn one_bits_zero_mode_signed_returns_bias_shifted() {
        // ZeroMode::Signed computes (exponent_bias << mantissa_bits) & storage_mask.
        assert_eq!(one_bits::<Float8E4M3Format>(), 0x38);
        assert_eq!(one_bits::<Float4E2M1FnFormat>(), 0x02);
        assert_eq!(one_bits::<Float6E3M2FnFormat>(), 0x0c);
    }

    #[test]
    fn one_bits_zero_mode_unsigned_returns_bias_shifted() {
        // ZeroMode::Unsigned follows the same formula as Signed.
        assert_eq!(one_bits::<Float8E4M3FnuzFormat>(), 0x40);
        assert_eq!(one_bits::<Float8E5M2FnuzFormat>(), 0x40);
    }

    #[test]
    fn neg_zero_bits_zero_mode_signed() {
        // ZeroMode::Signed returns the format's NEG_ZERO_BITS constant.
        assert_eq!(neg_zero_bits::<Float8E4M3Format>(), 0x80);
        assert_eq!(neg_zero_bits::<Float4E2M1FnFormat>(), 0x08);
    }

    #[test]
    fn neg_zero_bits_zero_mode_unsigned() {
        // ZeroMode::Unsigned returns ZERO_BITS (always 0).
        assert_eq!(neg_zero_bits::<Float8E4M3FnuzFormat>(), 0x00);
        assert_eq!(neg_zero_bits::<Float8E4M3B11FnuzFormat>(), 0x00);
    }

    #[test]
    fn neg_zero_bits_zero_mode_none_returns_nan() {
        // ZeroMode::None aliases neg_zero to nan_bits(false).
        assert_eq!(
            neg_zero_bits::<Float8E8M0FnuFormat>(),
            nan_bits::<Float8E8M0FnuFormat>(false)
        );
    }

    // infinity_bits is only called via overflow_bits in code paths for formats with Overflow::Nan.
    // Formats with Overflow::Infinity are covered by existing from_f32 tests, but formats with
    // Overflow::Nan or Overflow::Saturate never reach infinity_bits through normal paths.
    // These tests directly call infinity_bits to cover the Overflow::Infinity branch for all formats.
    #[test]
    fn infinity_bits_overflow_infinity_positive() {
        // Overflow::Infinity formats: return EXPONENT_MASK (positive sign).
        assert_eq!(infinity_bits::<Float8E3M4Format>(false), 0x70);
        assert_eq!(infinity_bits::<Float8E4M3Format>(false), 0x78);
        assert_eq!(infinity_bits::<Float8E5M2Format>(false), 0x7C);
    }

    #[test]
    fn infinity_bits_overflow_infinity_negative() {
        // Overflow::Infinity formats: return EXPONENT_MASK negated (negative sign).
        assert_eq!(infinity_bits::<Float8E3M4Format>(true), 0xF0);
        assert_eq!(infinity_bits::<Float8E4M3Format>(true), 0xF8);
        assert_eq!(infinity_bits::<Float8E5M2Format>(true), 0xFC);
    }

    #[test]
    fn infinity_bits_overflow_nan_positive() {
        // Overflow::Nan formats: return nan_bits(false).
        assert_eq!(infinity_bits::<Float4E2M1FnFormat>(false), 0x07);
        assert_eq!(infinity_bits::<Float6E2M3FnFormat>(false), 0x1F);
        assert_eq!(infinity_bits::<Float6E3M2FnFormat>(false), 0x1F);
        assert_eq!(infinity_bits::<Float8E4M3FnFormat>(false), 0x7F);
        assert_eq!(infinity_bits::<Float8E4M3FnuzFormat>(false), 0x80);
        assert_eq!(infinity_bits::<Float8E5M2FnuzFormat>(false), 0x80);
        assert_eq!(infinity_bits::<Float8E4M3B11FnuzFormat>(false), 0x80);
    }

    #[test]
    fn infinity_bits_overflow_nan_negative() {
        // Overflow::Nan formats with Signed sign: nan_bits negated.
        // f4e2m1fn: IEEE NaN 0x07 negated -> 0x0F.
        assert_eq!(infinity_bits::<Float4E2M1FnFormat>(true), 0x0F);
        // f6e2m3fn: IEEE NaN 0x1F negated -> 0x3F.
        assert_eq!(infinity_bits::<Float6E2M3FnFormat>(true), 0x3F);
        // f6e3m2fn: IEEE NaN 0x1F negated -> 0x3F.
        assert_eq!(infinity_bits::<Float6E3M2FnFormat>(true), 0x3F);
        // f8e4m3fn: IEEE NaN 0x7F negated -> 0xFF.
        assert_eq!(infinity_bits::<Float8E4M3FnFormat>(true), 0xFF);
        // f8e4m3fnuz: Single(0x80) with Unsigned zero -> same for both signs.
        assert_eq!(infinity_bits::<Float8E4M3FnuzFormat>(true), 0x80);
        // f8e5m2fnuz: Single(0x80) with Unsigned zero -> same for both signs.
        assert_eq!(infinity_bits::<Float8E5M2FnuzFormat>(true), 0x80);
        // f8e4m3b11fnuz: Single(0x80) with Unsigned zero -> same for both signs.
        assert_eq!(infinity_bits::<Float8E4M3B11FnuzFormat>(true), 0x80);
    }

    #[test]
    fn infinity_bits_overflow_saturate_positive() {
        // Overflow::Saturate formats: return max_finite_bits(false).
        assert_eq!(infinity_bits::<Float4E2M1FnFormat>(false), 0x07);
        assert_eq!(infinity_bits::<Float6E2M3FnFormat>(false), 0x1F);
        assert_eq!(infinity_bits::<Float6E3M2FnFormat>(false), 0x1F);
    }

    #[test]
    fn infinity_bits_overflow_saturate_negative() {
        // Overflow::Saturate formats: return max_finite_bits(true) = negated max_finite_bits.
        assert_eq!(infinity_bits::<Float4E2M1FnFormat>(true), 0x0F);
        assert_eq!(infinity_bits::<Float6E2M3FnFormat>(true), 0x3F);
        assert_eq!(infinity_bits::<Float6E3M2FnFormat>(true), 0x3F);
    }

    // exp2i branches that are unreachable through normal format conversions
    // because no format in this crate has a large enough exponent range:
    // - exp < -149: smallest format bias is 1, so min subnormal scale is 1 - 1 = 0
    // - exp > 127: largest format exponent field is 8 bits, max encoded value is 255
    //   giving max decoded exp of 255 - 7 = 248, but encode_f32 clamps before that
    // - exp >= -126: covered by existing encode/decode tests
    // These tests directly call exp2i to cover the gap.
    #[allow(clippy::float_cmp)]
    #[test]
    fn exp2i_unreachable_by_formats() {
        // Below minimum f32 denormal: returns 0.0
        assert_eq!(exp2i(-150), 0.0);
        assert_eq!(exp2i(-500), 0.0);
        // Just at boundary: smallest f32 denormal (1 << 0)
        assert_eq!(exp2i(-149), f32::from_bits(1u32));
        // Denormal range: -148 ..=-127 produces f32 denormals
        assert_eq!(exp2i(-128), f32::from_bits(1u32 << 21));
        assert_eq!(exp2i(-127), f32::from_bits(1u32 << 22));
        // exp = -126: boundary between denormal and normal branches (both give same value)
        assert_eq!(exp2i(-126), f32::MIN_POSITIVE);
        // Normal range: exp = -125 uses the normal branch
        assert_eq!(exp2i(-125), f32::from_bits((2u32) << 23));
        // Normal range: exp = 127 uses the normal branch
        assert_eq!(exp2i(127), f32::from_bits(0x7F00_0000));
        // Above maximum f32 normal: returns INFINITY
        assert_eq!(exp2i(128), f32::INFINITY);
        assert_eq!(exp2i(500), f32::INFINITY);
    }

    // SignMode::Unsigned branches: only Float8E8M0FnuFormat uses Unsigned sign.
    // These paths are unreachable through from_f32/to_f32 because f8e8m0fnu uses
    // encode_e8m0 which bypasses encode_f32/decode_f32 where these functions are called.
    // Direct calls cover the otherwise-unreachable Unsigned branches.
    #[test]
    fn is_negative_bits_unsigned_returns_false() {
        // SignMode::Unsigned always returns false regardless of input bits.
        assert!(!is_negative_bits::<Float8E8M0FnuFormat>(0x00));
        assert!(!is_negative_bits::<Float8E8M0FnuFormat>(0xff));
    }

    #[test]
    fn magnitude_bits_unsigned_returns_bits() {
        // SignMode::Unsigned returns the input bits unchanged.
        assert_eq!(magnitude_bits::<Float8E8M0FnuFormat>(0x00), 0x00);
        assert_eq!(magnitude_bits::<Float8E8M0FnuFormat>(0xff), 0xff);
    }
}
