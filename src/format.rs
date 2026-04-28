#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NanEncoding {
    None,
    Ieee,
    Outer,
    Single(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Overflow {
    Infinity,
    Nan,
    Saturate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignMode {
    Signed,
    Unsigned,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZeroMode {
    Signed,
    Unsigned,
    None,
}

pub trait Format: Copy + 'static {
    const NAME: &'static str;
    const STORAGE_BITS: u8;
    const EXPONENT_BITS: u8;
    const MANTISSA_BITS: u8;
    const EXPONENT_BIAS: i32;
    const SIGN: SignMode;
    const ZERO: ZeroMode;
    const NAN: NanEncoding;
    const OVERFLOW: Overflow;

    const ZERO_BITS: u8 = 0;
    const NEG_ZERO_BITS: u8 = Self::SIGN_BIT;
    const SIGN_BIT: u8 = match Self::SIGN {
        SignMode::Signed => 1 << (Self::STORAGE_BITS - 1),
        SignMode::Unsigned => 0,
    };
    const STORAGE_MASK: u8 = if Self::STORAGE_BITS == 8 {
        0xff
    } else {
        (1 << Self::STORAGE_BITS) - 1
    };
    const MANTISSA_MASK: u8 = if Self::MANTISSA_BITS == 0 {
        0
    } else {
        (1 << Self::MANTISSA_BITS) - 1
    };
    const EXPONENT_MASK: u8 =
        ((((1u16 << Self::EXPONENT_BITS) - 1) << Self::MANTISSA_BITS) & 0xff) as u8;
    #[expect(
        clippy::cast_possible_truncation,
        reason = "format exponent fields fit in eight storage bits"
    )]
    const MAX_EXPONENT_FIELD: u8 = ((1u16 << Self::EXPONENT_BITS) - 1) as u8;

    /// Returns `true` if this format can represent infinity.
    ///
    /// A format has infinity when its overflow behavior is
    /// [`Overflow::Infinity`]. Formats with `Overflow::Nan` or
    /// `Overflow::Saturate` do not represent infinity.
    const HAS_INF: bool = matches!(Self::OVERFLOW, Overflow::Infinity);

    /// Returns `true` if this format can represent NaN values.
    ///
    /// A format has NaN if its NaN encoding is anything except
    /// [`NanEncoding::None`].
    const HAS_NAN: bool = !matches!(Self::NAN, NanEncoding::None);

    /// Returns `true` if this format saturates on overflow.
    const HAS_FINITE_ONLY: bool = matches!(Self::OVERFLOW, Overflow::Saturate);
}

#[cfg(test)]
#[expect(
    clippy::assertions_on_constants,
    reason = "tests verify const trait associated constants"
)]
mod tests {
    use super::*;
    use crate::{
        Float4E2M1FnFormat, Float6E2M3FnFormat, Float6E3M2FnFormat, Float8E3M4Format,
        Float8E4M3B11FnuzFormat, Float8E4M3FnFormat, Float8E4M3FnuzFormat, Float8E4M3Format,
        Float8E5M2FnuzFormat, Float8E5M2Format, Float8E8M0FnuFormat,
    };

    // Single NaN with Unsigned zero (E4M3B11FNuz) — has NaN, no Inf, no saturate
    #[test]
    fn single_nan_unsigned_zero_b11_format() {
        assert!(!<Float8E4M3B11FnuzFormat as Format>::HAS_INF);
        assert!(<Float8E4M3B11FnuzFormat as Format>::HAS_NAN);
        assert!(!<Float8E4M3B11FnuzFormat as Format>::HAS_FINITE_ONLY);
    }

    // IEEE-like formats (E5M2, E4M3, E3M4) — has Inf, has NaN, no saturate
    #[test]
    fn ieee_formats_has_inf_nan() {
        assert!(<Float8E5M2Format as Format>::HAS_INF);
        assert!(<Float8E5M2Format as Format>::HAS_NAN);
        assert!(!<Float8E5M2Format as Format>::HAS_FINITE_ONLY);

        assert!(<Float8E4M3Format as Format>::HAS_INF);
        assert!(<Float8E4M3Format as Format>::HAS_NAN);
        assert!(!<Float8E4M3Format as Format>::HAS_FINITE_ONLY);

        assert!(<Float8E3M4Format as Format>::HAS_INF);
        assert!(<Float8E3M4Format as Format>::HAS_NAN);
        assert!(!<Float8E3M4Format as Format>::HAS_FINITE_ONLY);
    }

    // Outer NaN formats (E4M3FN) — has NaN, no Inf, no saturate
    #[test]
    fn outer_nan_formats_has_nan() {
        assert!(!<Float8E4M3FnFormat as Format>::HAS_INF);
        assert!(<Float8E4M3FnFormat as Format>::HAS_NAN);
        assert!(!<Float8E4M3FnFormat as Format>::HAS_FINITE_ONLY);
    }

    // Single NaN, Unsigned zero formats (E4M3FNuz, E5M2FNuz) — has NaN, no Inf
    #[test]
    fn single_nan_unsigned_zero_formats() {
        assert!(!<Float8E4M3FnuzFormat as Format>::HAS_INF);
        assert!(<Float8E4M3FnuzFormat as Format>::HAS_NAN);
        assert!(!<Float8E4M3FnuzFormat as Format>::HAS_FINITE_ONLY);

        assert!(!<Float8E5M2FnuzFormat as Format>::HAS_INF);
        assert!(<Float8E5M2FnuzFormat as Format>::HAS_NAN);
        assert!(!<Float8E5M2FnuzFormat as Format>::HAS_FINITE_ONLY);
    }

    // Single NaN, unsigned sign format (E8M0FNU) — has NaN, no Inf
    #[test]
    fn unsigned_e8m0_format() {
        assert!(!<Float8E8M0FnuFormat as Format>::HAS_INF);
        assert!(<Float8E8M0FnuFormat as Format>::HAS_NAN);
        assert!(!<Float8E8M0FnuFormat as Format>::HAS_FINITE_ONLY);
    }

    // Saturate formats (F4, F6) — no Inf, no NaN, finite only
    #[test]
    fn saturate_formats_finite_only() {
        assert!(!<Float4E2M1FnFormat as Format>::HAS_INF);
        assert!(!<Float4E2M1FnFormat as Format>::HAS_NAN);
        assert!(<Float4E2M1FnFormat as Format>::HAS_FINITE_ONLY);

        assert!(!<Float6E2M3FnFormat as Format>::HAS_INF);
        assert!(!<Float6E2M3FnFormat as Format>::HAS_NAN);
        assert!(<Float6E2M3FnFormat as Format>::HAS_FINITE_ONLY);

        assert!(!<Float6E3M2FnFormat as Format>::HAS_INF);
        assert!(!<Float6E3M2FnFormat as Format>::HAS_NAN);
        assert!(<Float6E3M2FnFormat as Format>::HAS_FINITE_ONLY);
    }
}
