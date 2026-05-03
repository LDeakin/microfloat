use core::cmp::Ordering;
use core::marker::PhantomData;
use core::num::FpCategory;

use crate::bits::{
    abs_bits, classify_bits, decode_f32, encode_f32, infinity_bits, nan_bits, neg_zero_bits,
    negate_bits, next_down_bits, next_up_bits, one_bits, total_key,
};
use crate::format::{Format, NanEncoding, SignMode};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct MicroFloat<F: Format> {
    pub bits: u8,
    _format: PhantomData<F>,
}

impl<F: Format> MicroFloat<F> {
    pub const ZERO: Self = Self::from_bits(0);
    pub const NEG_ZERO: Self = Self::from_bits(neg_zero_bits::<F>());
    pub const ONE: Self = Self::from_bits(one_bits::<F>());
    pub const NEG_ONE: Self = Self::from_bits(negate_bits::<F>(one_bits::<F>()));
    pub const INFINITY: Self = Self::from_bits(infinity_bits::<F>(false));
    pub const NAN: Self = Self::from_bits(nan_bits::<F>(false));

    pub const fn from_bits(bits: u8) -> Self {
        Self {
            bits,
            _format: PhantomData,
        }
    }

    pub const fn to_bits(self) -> u8 {
        self.bits
    }

    pub const fn from_le_bytes(bytes: [u8; 1]) -> Self {
        Self::from_bits(u8::from_le_bytes(bytes))
    }

    pub const fn from_be_bytes(bytes: [u8; 1]) -> Self {
        Self::from_bits(u8::from_be_bytes(bytes))
    }

    pub const fn from_ne_bytes(bytes: [u8; 1]) -> Self {
        Self::from_bits(u8::from_ne_bytes(bytes))
    }

    pub const fn to_le_bytes(self) -> [u8; 1] {
        self.bits.to_le_bytes()
    }

    pub const fn to_be_bytes(self) -> [u8; 1] {
        self.bits.to_be_bytes()
    }

    pub const fn to_ne_bytes(self) -> [u8; 1] {
        self.bits.to_ne_bytes()
    }

    pub fn from_f32(value: f32) -> Self {
        Self::from_bits(encode_f32::<F>(value))
    }

    pub fn from_f64(value: f64) -> Self {
        #[expect(
            clippy::cast_possible_truncation,
            reason = "conversion intentionally follows f64-to-f32 rounding before encoding"
        )]
        Self::from_bits(encode_f32::<F>(value as f32))
    }

    pub fn to_f32(self) -> f32 {
        decode_f32::<F>(self.bits)
    }

    pub fn to_f64(self) -> f64 {
        f64::from(self.to_f32())
    }

    pub fn is_nan(self) -> bool {
        classify_bits::<F>(self.bits).is_nan
    }

    pub fn is_infinite(self) -> bool {
        classify_bits::<F>(self.bits).is_infinite
    }

    pub fn is_finite(self) -> bool {
        !self.is_nan() && !self.is_infinite()
    }

    pub fn is_normal(self) -> bool {
        matches!(self.classify(), FpCategory::Normal)
    }

    pub fn classify(self) -> FpCategory {
        let class = classify_bits::<F>(self.bits);
        if class.is_nan {
            FpCategory::Nan
        } else if class.is_infinite {
            FpCategory::Infinite
        } else if class.is_zero {
            FpCategory::Zero
        } else if class.is_subnormal {
            FpCategory::Subnormal
        } else {
            FpCategory::Normal
        }
    }

    pub const fn is_sign_positive(self) -> bool {
        !self.is_sign_negative()
    }

    pub const fn is_sign_negative(self) -> bool {
        match F::SIGN {
            SignMode::Unsigned => false,
            SignMode::Signed => {
                if F::STORAGE_BITS < 8 {
                    self.bits >= F::SIGN_BIT
                } else {
                    (self.bits & F::SIGN_BIT) != 0
                }
            }
        }
    }

    pub fn copysign(self, sign: Self) -> Self {
        if matches!(F::NAN, NanEncoding::Single(_)) && self.is_nan() {
            return self;
        }
        if F::SIGN == SignMode::Unsigned {
            self
        } else if sign.is_sign_negative() {
            Self::from_bits(abs_bits::<F>(self.bits) | F::SIGN_BIT)
        } else {
            Self::from_bits(abs_bits::<F>(self.bits))
        }
    }

    pub fn signum(self) -> Self {
        if self.is_nan() || self.classify() == FpCategory::Zero {
            self
        } else if self.is_sign_negative() {
            Self::NEG_ONE
        } else {
            Self::ONE
        }
    }

    pub fn abs(self) -> Self {
        if matches!(F::NAN, NanEncoding::Single(_)) && self.is_nan() {
            return self;
        }
        Self::from_bits(abs_bits::<F>(self.bits))
    }

    pub const fn next_up(self) -> Self {
        Self::from_bits(next_up_bits::<F>(self.bits))
    }

    pub const fn next_down(self) -> Self {
        Self::from_bits(next_down_bits::<F>(self.bits))
    }

    pub fn floor(self) -> Self {
        unary_result(self, libm::floorf(self.to_f32()))
    }

    pub fn ceil(self) -> Self {
        unary_result(self, libm::ceilf(self.to_f32()))
    }

    pub fn trunc(self) -> Self {
        unary_result(self, libm::truncf(self.to_f32()))
    }

    pub fn round_ties_even(self) -> Self {
        unary_result(self, libm::rintf(self.to_f32()))
    }

    pub fn recip(self) -> Self {
        unary_result(self, 1.0 / self.to_f32())
    }

    pub fn powf(self, n: Self) -> Self {
        if self.is_nan() {
            if n.classify() == FpCategory::Zero {
                return Self::ONE;
            }
            if self.is_sign_negative() && is_odd_integer(n.to_f32()) {
                return Self::NAN;
            }
            return self;
        }
        if !F::HAS_NAN
            && self.is_sign_negative()
            && self.classify() != FpCategory::Zero
            && !is_integer(n.to_f32())
        {
            return Self::ZERO;
        }
        Self::from_f32(libm::powf(self.to_f32(), n.to_f32()))
    }

    pub fn sqrt(self) -> Self {
        if !F::HAS_NAN && self.is_sign_negative() && self.classify() != FpCategory::Zero {
            return Self::ZERO;
        }
        unary_result(self, libm::sqrtf(self.to_f32()))
    }

    pub fn exp(self) -> Self {
        unary_result(self, libm::expf(self.to_f32()))
    }

    pub fn exp2(self) -> Self {
        unary_result(self, libm::exp2f(self.to_f32()))
    }

    pub fn exp_m1(self) -> Self {
        unary_result(self, libm::expm1f(self.to_f32()))
    }

    pub fn ln(self) -> Self {
        if !F::HAS_NAN && self.is_sign_negative() && self.classify() != FpCategory::Zero {
            return Self::ZERO;
        }
        unary_result(self, libm::logf(self.to_f32()))
    }

    pub fn ln_1p(self) -> Self {
        if !F::HAS_NAN && self.to_f32() < -1.0 {
            return Self::ZERO;
        }
        unary_result(self, libm::log1pf(self.to_f32()))
    }

    pub fn log2(self) -> Self {
        if !F::HAS_NAN && self.is_sign_negative() && self.classify() != FpCategory::Zero {
            return Self::ZERO;
        }
        unary_result(self, libm::log2f(self.to_f32()))
    }

    pub fn log10(self) -> Self {
        if !self.is_nan()
            && matches!(F::NAN, NanEncoding::Ieee | NanEncoding::Outer)
            && self.is_sign_negative()
            && self.classify() != FpCategory::Zero
        {
            return Self::NAN;
        }
        unary_result(self, libm::log10f(self.to_f32()))
    }

    pub fn cbrt(self) -> Self {
        unary_result(self, libm::cbrtf(self.to_f32()))
    }

    pub fn hypot(self, other: Self) -> Self {
        if self.is_infinite() || other.is_infinite() {
            return Self::INFINITY;
        }
        if self.is_nan() {
            return self;
        }
        if other.is_nan() {
            return other;
        }
        Self::from_f32(libm::hypotf(self.to_f32(), other.to_f32()))
    }

    pub fn sin(self) -> Self {
        unary_result(self, libm::sinf(self.to_f32()))
    }

    pub fn cos(self) -> Self {
        unary_result(self, libm::cosf(self.to_f32()))
    }

    pub fn tan(self) -> Self {
        unary_result(self, libm::tanf(self.to_f32()))
    }

    pub fn asin(self) -> Self {
        if matches!(F::NAN, NanEncoding::Ieee | NanEncoding::Outer) && self.to_f32().abs() > 1.0 {
            return Self::NAN;
        }
        unary_result(self, libm::asinf(self.to_f32()))
    }

    pub fn acos(self) -> Self {
        if matches!(F::NAN, NanEncoding::Ieee | NanEncoding::Outer) && self.to_f32().abs() > 1.0 {
            return Self::NAN;
        }
        unary_result(self, libm::acosf(self.to_f32()))
    }

    pub fn atan(self) -> Self {
        unary_result(self, libm::atanf(self.to_f32()))
    }

    pub fn atan2(self, other: Self) -> Self {
        if self.is_nan() {
            return self;
        }
        if other.is_nan() {
            return other;
        }
        Self::from_f32(libm::atan2f(self.to_f32(), other.to_f32()))
    }

    pub fn sinh(self) -> Self {
        unary_result(self, libm::sinhf(self.to_f32()))
    }

    pub fn cosh(self) -> Self {
        unary_result(self, libm::coshf(self.to_f32()))
    }

    pub fn tanh(self) -> Self {
        unary_result(self, libm::tanhf(self.to_f32()))
    }

    pub fn min(self, other: Self) -> Self {
        if self.is_nan() && other.is_nan() {
            self
        } else if self.is_nan() {
            other
        } else if other.is_nan() || self < other {
            self
        } else {
            other
        }
    }

    pub fn max(self, other: Self) -> Self {
        if self.is_nan() && other.is_nan() {
            self
        } else if self.is_nan() {
            other
        } else if other.is_nan() || self > other {
            self
        } else {
            other
        }
    }

    /// Restricts `self` to the interval `[min, max]`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`, `min` is NaN, or `max` is `NaN`.
    pub fn clamp(self, min: Self, max: Self) -> Self {
        assert!(
            !min.is_nan() && !max.is_nan(),
            "`min` and `max` must not be `NaN`"
        );
        assert!(min <= max, "`min` must be less than or equal to `max`");
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    #[expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "signature matches f32::total_cmp for API compatibility"
    )]
    pub fn total_cmp(&self, other: &Self) -> Ordering {
        total_key::<F>(self.bits).cmp(&total_key::<F>(other.bits))
    }
}

fn unary_result<F: Format>(input: MicroFloat<F>, result: f32) -> MicroFloat<F> {
    if input.is_nan() {
        input
    } else {
        MicroFloat::from_f32(result)
    }
}

fn is_odd_integer(value: f32) -> bool {
    if !value.is_finite() || !is_integer(value) {
        return false;
    }
    let half = libm::truncf(value * 0.5);
    value - half * 2.0 != 0.0
}

fn is_integer(value: f32) -> bool {
    libm::truncf(value).to_bits() == value.to_bits()
}
