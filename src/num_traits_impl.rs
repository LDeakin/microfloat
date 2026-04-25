use core::num::FpCategory;

use crate::{
    f4e2m1fn, f6e2m3fn, f6e3m2fn, f8e3m4, f8e4m3, f8e4m3b11fnuz, f8e4m3fn, f8e4m3fnuz, f8e5m2,
    f8e5m2fnuz, f8e8m0fnu,
};
use num_traits::{
    Bounded, FromPrimitive, Num, NumCast, One, Signed, ToPrimitive, Zero, float::FloatCore,
};

macro_rules! impl_common_num_traits {
    ($($type:ty),* $(,)?) => {
        $(
            impl ToPrimitive for $type {
                fn to_i64(&self) -> Option<i64> {
                    <$type>::to_f32(*self).to_i64()
                }

                fn to_u64(&self) -> Option<u64> {
                    <$type>::to_f32(*self).to_u64()
                }

                fn to_f32(&self) -> Option<f32> {
                    Some(<$type>::to_f32(*self))
                }

                fn to_f64(&self) -> Option<f64> {
                    Some(<$type>::to_f64(*self))
                }
            }

            impl FromPrimitive for $type {
                fn from_i64(n: i64) -> Option<Self> {
                    n.to_f32().map(Self::from_f32)
                }

                fn from_u64(n: u64) -> Option<Self> {
                    n.to_f32().map(Self::from_f32)
                }

                fn from_f32(n: f32) -> Option<Self> {
                    Some(Self::from_f32(n))
                }

                fn from_f64(n: f64) -> Option<Self> {
                    Some(Self::from_f64(n))
                }
            }

            impl NumCast for $type {
                fn from<T: ToPrimitive>(n: T) -> Option<Self> {
                    n.to_f32().map(Self::from_f32)
                }
            }

            impl Bounded for $type {
                fn min_value() -> Self {
                    Self::MIN
                }

                fn max_value() -> Self {
                    Self::MAX
                }
            }
        )*
    };
}

macro_rules! impl_zero_one_num_signed {
    ($($type:ty),* $(,)?) => {
        $(
            impl Zero for $type {
                fn zero() -> Self {
                    Self::ZERO
                }

                fn is_zero(&self) -> bool {
                    self.classify() == FpCategory::Zero
                }
            }

            impl One for $type {
                fn one() -> Self {
                    Self::ONE
                }
            }

            impl Num for $type {
                type FromStrRadixErr = <f32 as Num>::FromStrRadixErr;

                fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                    f32::from_str_radix(str, radix).map(Self::from_f32)
                }
            }

            impl Signed for $type {
                fn abs(&self) -> Self {
                    <$type>::abs(*self)
                }

                fn abs_sub(&self, other: &Self) -> Self {
                    if *self <= *other {
                        Self::ZERO
                    } else {
                        *self - *other
                    }
                }

                fn signum(&self) -> Self {
                    <$type>::signum(*self)
                }

                fn is_positive(&self) -> bool {
                    self.is_sign_positive() && !self.is_zero()
                }

                fn is_negative(&self) -> bool {
                    self.is_sign_negative() && !self.is_zero()
                }
            }

            impl FloatCore for $type {
                fn infinity() -> Self {
                    Self::INFINITY
                }

                fn neg_infinity() -> Self {
                    Self::NEG_INFINITY
                }

                fn nan() -> Self {
                    Self::NAN
                }

                fn neg_zero() -> Self {
                    Self::NEG_ZERO
                }

                fn min_value() -> Self {
                    Self::MIN
                }

                fn min_positive_value() -> Self {
                    Self::MIN_POSITIVE
                }

                fn epsilon() -> Self {
                    Self::EPSILON
                }

                fn max_value() -> Self {
                    Self::MAX
                }

                fn is_nan(self) -> bool {
                    <$type>::is_nan(self)
                }

                fn is_infinite(self) -> bool {
                    <$type>::is_infinite(self)
                }

                fn is_finite(self) -> bool {
                    <$type>::is_finite(self)
                }

                fn is_normal(self) -> bool {
                    <$type>::is_normal(self)
                }

                fn classify(self) -> FpCategory {
                    <$type>::classify(self)
                }

                fn to_degrees(self) -> Self {
                    Self::from_f32(<$type>::to_f32(self).to_degrees())
                }

                fn to_radians(self) -> Self {
                    Self::from_f32(<$type>::to_f32(self).to_radians())
                }

                fn integer_decode(self) -> (u64, i16, i8) {
                    <f32 as FloatCore>::integer_decode(<$type>::to_f32(self))
                }
            }
        )*
    };
}

impl_common_num_traits!(
    f8e3m4,
    f8e4m3,
    f8e4m3b11fnuz,
    f8e4m3fn,
    f8e4m3fnuz,
    f8e5m2,
    f8e5m2fnuz,
    f8e8m0fnu,
    f4e2m1fn,
    f6e2m3fn,
    f6e3m2fn,
);

impl_zero_one_num_signed!(
    f8e3m4,
    f8e4m3,
    f8e4m3b11fnuz,
    f8e4m3fn,
    f8e4m3fnuz,
    f8e5m2,
    f8e5m2fnuz,
    f4e2m1fn,
    f6e2m3fn,
    f6e3m2fn,
);
