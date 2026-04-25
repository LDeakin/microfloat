#![cfg(feature = "num-traits")]

use microfloat::*;
use num_traits::{FromPrimitive, Num, NumCast, One, Signed, ToPrimitive, Zero, float::FloatCore};

macro_rules! common_num_traits_tests {
    ($($module:ident: $type:ty;)*) => {
        $(
            mod $module {
                use super::*;

                #[test]
                fn bounded() {
                    assert_eq!(<$type as num_traits::Bounded>::min_value(), <$type>::MIN);
                    assert_eq!(<$type as num_traits::Bounded>::max_value(), <$type>::MAX);
                }
            }
        )*
    };
}

common_num_traits_tests! {
    float8_e3m4: f8e3m4;
    float8_e4m3: f8e4m3;
    float8_e4m3b11fnuz: f8e4m3b11fnuz;
    float8_e4m3fn: f8e4m3fn;
    float8_e4m3fnuz: f8e4m3fnuz;
    float8_e5m2: f8e5m2;
    float8_e5m2fnuz: f8e5m2fnuz;
    float8_e8m0fnu: f8e8m0fnu;
    float4_e2m1fn: f4e2m1fn;
    float6_e2m3fn: f6e2m3fn;
    float6_e3m2fn: f6e3m2fn;
}

macro_rules! float_core_constant_tests {
    ($($module:ident: $type:ty;)*) => {
        $(
            mod $module {
                use super::*;

                #[test]
                fn special_constants() {
                    assert_eq!(<$type as FloatCore>::nan().to_bits(), <$type>::NAN.to_bits());
                    assert_eq!(
                        <$type as FloatCore>::infinity().to_bits(),
                        <$type>::INFINITY.to_bits()
                    );
                    assert_eq!(
                        <$type as FloatCore>::neg_infinity().to_bits(),
                        <$type>::NEG_INFINITY.to_bits()
                    );
                    assert_eq!(
                        <$type as FloatCore>::neg_zero().to_bits(),
                        <$type>::NEG_ZERO.to_bits()
                    );
                }
            }
        )*
    };
}

float_core_constant_tests! {
    float_core_float8_e3m4: f8e3m4;
    float_core_float8_e4m3: f8e4m3;
    float_core_float8_e4m3b11fnuz: f8e4m3b11fnuz;
    float_core_float8_e4m3fn: f8e4m3fn;
    float_core_float8_e4m3fnuz: f8e4m3fnuz;
    float_core_float8_e5m2: f8e5m2;
    float_core_float8_e5m2fnuz: f8e5m2fnuz;
    float_core_float4_e2m1fn: f4e2m1fn;
    float_core_float6_e2m3fn: f6e2m3fn;
    float_core_float6_e3m2fn: f6e3m2fn;
}

#[test]
fn conversions_and_methods() {
    let value = f8e4m3::from_f32(2.0);

    assert_eq!(value.to_i64(), Some(2));
    assert_eq!(value.to_u64(), Some(2));
    assert_eq!(ToPrimitive::to_f32(&value), Some(2.0));
    assert_eq!(ToPrimitive::to_f64(&value), Some(2.0));

    assert_eq!(f8e4m3::from_i64(2), Some(value));
    assert_eq!(f8e4m3::from_u64(2), Some(value));
    assert_eq!(<f8e4m3 as FromPrimitive>::from_f32(2.0), Some(value));
    assert_eq!(<f8e4m3 as FromPrimitive>::from_f64(2.0), Some(value));
    assert_eq!(<f8e4m3 as NumCast>::from(2.0f32), Some(value));

    assert_eq!(f8e4m3::zero(), f8e4m3::ZERO);
    assert!(f8e4m3::ZERO.is_zero());
    assert!(!value.is_zero());
    assert_eq!(f8e4m3::one(), f8e4m3::ONE);
    assert_eq!(<f8e4m3 as Num>::from_str_radix("2", 10).unwrap(), value);

    assert_eq!(Signed::abs(&f8e4m3::from_f32(-2.0)), value);
    assert_eq!(value.abs_sub(&f8e4m3::ONE), f8e4m3::ONE);
    assert_eq!(f8e4m3::ONE.abs_sub(&value), f8e4m3::ZERO);
    assert_eq!(Signed::signum(&f8e4m3::from_f32(-2.0)), f8e4m3::NEG_ONE);
    assert!(value.is_positive());
    assert!(!f8e4m3::ZERO.is_positive());
    assert!(f8e4m3::from_f32(-2.0).is_negative());
    assert!(!f8e4m3::ZERO.is_negative());

    assert_eq!(<f8e4m3 as FloatCore>::min_value(), f8e4m3::MIN);
    assert_eq!(
        <f8e4m3 as FloatCore>::min_positive_value(),
        f8e4m3::MIN_POSITIVE
    );
    assert_eq!(<f8e4m3 as FloatCore>::epsilon(), f8e4m3::EPSILON);
    assert_eq!(<f8e4m3 as FloatCore>::max_value(), f8e4m3::MAX);
    assert!(FloatCore::is_nan(f8e4m3::NAN));
    assert!(FloatCore::is_infinite(f8e4m3::INFINITY));
    assert!(FloatCore::is_finite(value));
    assert!(FloatCore::is_normal(value));
    assert_eq!(FloatCore::classify(value), core::num::FpCategory::Normal);
    assert_eq!(value.to_degrees(), f8e4m3::from_f32(2.0f32.to_degrees()));
    assert_eq!(value.to_radians(), f8e4m3::from_f32(2.0f32.to_radians()));
    assert_eq!(
        value.integer_decode(),
        <f32 as FloatCore>::integer_decode(2.0)
    );
}
