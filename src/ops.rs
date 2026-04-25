use core::fmt;
use core::num::FpCategory;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use core::str::FromStr;

use crate::bits::{nan_bits, negate_bits};
use crate::format::{Format, NanEncoding};
use crate::micro::MicroFloat;

impl<F: Format> fmt::Debug for MicroFloat<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(F::NAME).field(&self.to_f32()).finish()
    }
}

impl<F: Format> fmt::Display for MicroFloat<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_f32(), f)
    }
}

impl<F: Format> fmt::LowerExp for MicroFloat<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerExp::fmt(&self.to_f32(), f)
    }
}

impl<F: Format> fmt::UpperExp for MicroFloat<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperExp::fmt(&self.to_f32(), f)
    }
}

impl<F: Format> FromStr for MicroFloat<F> {
    type Err = <f32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        f32::from_str(s).map(Self::from_f32)
    }
}

impl<F: Format> PartialEq for MicroFloat<F> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else if self.classify() == FpCategory::Zero && other.classify() == FpCategory::Zero {
            true
        } else {
            self.bits == other.bits
        }
    }
}

impl<F: Format> PartialOrd for MicroFloat<F> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if self.is_nan() || other.is_nan() {
            None
        } else {
            self.to_f32().partial_cmp(&other.to_f32())
        }
    }
}

impl<F: Format> Neg for MicroFloat<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if matches!(F::NAN, NanEncoding::Single(_)) && self.is_nan() {
            return self;
        }
        Self::from_bits(negate_bits::<F>(self.bits))
    }
}

impl<F: Format> Add for MicroFloat<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_nan() {
            self
        } else if rhs.is_nan() {
            Self::from_bits(nan_bits::<F>(false))
        } else {
            Self::from_f32(self.to_f32() + rhs.to_f32())
        }
    }
}

impl<F: Format> AddAssign for MicroFloat<F> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<F: Format> Sub for MicroFloat<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.is_nan() {
            self
        } else if rhs.is_nan() {
            Self::from_bits(nan_bits::<F>(false))
        } else {
            Self::from_f32(self.to_f32() - rhs.to_f32())
        }
    }
}

impl<F: Format> SubAssign for MicroFloat<F> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<F: Format> Mul for MicroFloat<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if rhs.is_nan() {
            rhs
        } else if self.is_nan() {
            self
        } else {
            Self::from_f32(self.to_f32() * rhs.to_f32())
        }
    }
}

impl<F: Format> MulAssign for MicroFloat<F> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<F: Format> Div for MicroFloat<F> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.classify() == FpCategory::Zero && rhs.classify() == FpCategory::Zero {
            return match F::NAN {
                NanEncoding::None => Self::ZERO,
                NanEncoding::Ieee | NanEncoding::Outer => Self::from_bits(nan_bits::<F>(true)),
                NanEncoding::Single(_) => Self::NAN,
            };
        }
        Self::from_f32(self.to_f32() / rhs.to_f32())
    }
}

impl<F: Format> DivAssign for MicroFloat<F> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<F: Format> Rem for MicroFloat<F> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.classify() == FpCategory::Zero {
            return match F::NAN {
                NanEncoding::None => Self::NEG_ZERO,
                NanEncoding::Ieee | NanEncoding::Outer => Self::from_bits(nan_bits::<F>(false)),
                NanEncoding::Single(_) => Self::NAN,
            };
        }
        if self.is_nan() && rhs.is_nan() {
            return Self::from_bits(nan_bits::<F>(
                self.is_sign_negative() && rhs.is_sign_negative(),
            ));
        }
        if rhs.is_nan() {
            return rhs;
        }
        if self.is_nan() {
            return self;
        }
        Self::from_f32(numpy_remainder(self.to_f32(), rhs.to_f32()))
    }
}

impl<F: Format> RemAssign for MicroFloat<F> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

fn numpy_remainder(lhs: f32, rhs: f32) -> f32 {
    // NOTE: Covered by MicroFloat::rem
    // if lhs.is_nan() || rhs.is_nan() || rhs == 0.0 {
    //     return f32::NAN;
    // }

    if lhs.is_infinite() && rhs.is_finite() {
        return f32::from_bits(0xffc0_0000);
    }
    if rhs.is_infinite() && lhs.is_finite() {
        if lhs == 0.0 {
            return if rhs.is_sign_negative() { -0.0 } else { 0.0 };
        }
        if lhs.is_sign_negative() == rhs.is_sign_negative() {
            lhs
        } else {
            rhs
        }
    } else {
        let result = lhs - libm::floorf(lhs / rhs) * rhs;
        if result == 0.0 {
            if rhs.is_sign_negative() { -0.0 } else { 0.0 }
        } else {
            result
        }
    }
}
