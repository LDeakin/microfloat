use microfloat::*;

mod fixtures {
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(
        clippy::all,
        clippy::pedantic,
        reason = "generated compatibility fixtures are checked as data, not hand-written code"
    )]
    include!("fixtures/generated.rs");
}

trait CaseFloat:
    Copy
    + core::fmt::Debug
    + PartialEq
    + PartialOrd
    + core::ops::Neg<Output = Self>
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Div<Output = Self>
    + core::ops::Rem<Output = Self>
{
    const NAN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NEG_ZERO: Self;
    const MIN: Self;
    const MAX: Self;

    fn from_bits(bits: u8) -> Self;
    fn to_bits(self) -> u8;
    fn from_f32(value: f32) -> Self;
    fn to_f32(self) -> f32;
    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
    fn is_sign_positive(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn copysign(self, sign: Self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn trunc(self) -> Self;
    fn round_ties_even(self) -> Self;
    fn recip(self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn exp_m1(self) -> Self;
    fn ln(self) -> Self;
    fn ln_1p(self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn cbrt(self) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

macro_rules! impl_case_float {
    ($($type:ty),* $(,)?) => {
        $(
            impl CaseFloat for $type {
                const NAN: Self = <$type>::NAN;
                const INFINITY: Self = <$type>::INFINITY;
                const NEG_INFINITY: Self = <$type>::NEG_INFINITY;
                const NEG_ZERO: Self = <$type>::NEG_ZERO;
                const MIN: Self = <$type>::MIN;
                const MAX: Self = <$type>::MAX;

                fn from_bits(bits: u8) -> Self {
                    <$type>::from_bits(bits)
                }

                fn to_bits(self) -> u8 {
                    <$type>::to_bits(self)
                }

                fn from_f32(value: f32) -> Self {
                    <$type>::from_f32(value)
                }

                fn to_f32(self) -> f32 {
                    <$type>::to_f32(self)
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

                fn is_sign_positive(self) -> bool {
                    <$type>::is_sign_positive(self)
                }

                fn is_sign_negative(self) -> bool {
                    <$type>::is_sign_negative(self)
                }

                fn abs(self) -> Self {
                    <$type>::abs(self)
                }

                fn signum(self) -> Self {
                    <$type>::signum(self)
                }

                fn copysign(self, sign: Self) -> Self {
                    <$type>::copysign(self, sign)
                }

                fn floor(self) -> Self {
                    <$type>::floor(self)
                }

                fn ceil(self) -> Self {
                    <$type>::ceil(self)
                }

                fn trunc(self) -> Self {
                    <$type>::trunc(self)
                }

                fn round_ties_even(self) -> Self {
                    <$type>::round_ties_even(self)
                }

                fn recip(self) -> Self {
                    <$type>::recip(self)
                }

                fn powf(self, n: Self) -> Self {
                    <$type>::powf(self, n)
                }

                fn sqrt(self) -> Self {
                    <$type>::sqrt(self)
                }

                fn exp(self) -> Self {
                    <$type>::exp(self)
                }

                fn exp2(self) -> Self {
                    <$type>::exp2(self)
                }

                fn exp_m1(self) -> Self {
                    <$type>::exp_m1(self)
                }

                fn ln(self) -> Self {
                    <$type>::ln(self)
                }

                fn ln_1p(self) -> Self {
                    <$type>::ln_1p(self)
                }

                fn log2(self) -> Self {
                    <$type>::log2(self)
                }

                fn log10(self) -> Self {
                    <$type>::log10(self)
                }

                fn cbrt(self) -> Self {
                    <$type>::cbrt(self)
                }

                fn hypot(self, other: Self) -> Self {
                    <$type>::hypot(self, other)
                }

                fn sin(self) -> Self {
                    <$type>::sin(self)
                }

                fn cos(self) -> Self {
                    <$type>::cos(self)
                }

                fn tan(self) -> Self {
                    <$type>::tan(self)
                }

                fn asin(self) -> Self {
                    <$type>::asin(self)
                }

                fn acos(self) -> Self {
                    <$type>::acos(self)
                }

                fn atan(self) -> Self {
                    <$type>::atan(self)
                }

                fn atan2(self, other: Self) -> Self {
                    <$type>::atan2(self, other)
                }

                fn sinh(self) -> Self {
                    <$type>::sinh(self)
                }

                fn cosh(self) -> Self {
                    <$type>::cosh(self)
                }

                fn tanh(self) -> Self {
                    <$type>::tanh(self)
                }

                fn min(self, other: Self) -> Self {
                    <$type>::min(self, other)
                }

                fn max(self, other: Self) -> Self {
                    <$type>::max(self, other)
                }
            }
        )*
    };
}

impl_case_float!(
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

fn assert_special_constants<T: CaseFloat>(fixture: &fixtures::FormatFixture) {
    assert_eq!(
        T::NAN.to_bits(),
        fixture.nan_bits,
        "{} NAN constant: got {:#04x}, expected {:#04x}",
        fixture.rust_type,
        T::NAN.to_bits(),
        fixture.nan_bits
    );
    assert_eq!(
        T::INFINITY.to_bits(),
        fixture.infinity_bits,
        "{} INFINITY constant: got {:#04x}, expected {:#04x}",
        fixture.rust_type,
        T::INFINITY.to_bits(),
        fixture.infinity_bits
    );
    assert_eq!(
        T::NEG_INFINITY.to_bits(),
        fixture.neg_infinity_bits,
        "{} NEG_INFINITY constant: got {:#04x}, expected {:#04x}",
        fixture.rust_type,
        T::NEG_INFINITY.to_bits(),
        fixture.neg_infinity_bits
    );
    assert_eq!(
        T::NEG_ZERO.to_bits(),
        fixture.neg_zero_bits,
        "{} NEG_ZERO constant: got {:#04x}, expected {:#04x}",
        fixture.rust_type,
        T::NEG_ZERO.to_bits(),
        fixture.neg_zero_bits
    );
    assert_eq!(
        T::MIN.to_bits(),
        fixture.min_bits,
        "{} MIN constant: got {:#04x}, expected {:#04x}",
        fixture.rust_type,
        T::MIN.to_bits(),
        fixture.min_bits
    );
    assert_eq!(
        T::MAX.to_bits(),
        fixture.max_bits,
        "{} MAX constant: got {:#04x}, expected {:#04x}",
        fixture.rust_type,
        T::MAX.to_bits(),
        fixture.max_bits
    );
}

fn assert_decode<T: CaseFloat>(fixture: &fixtures::FormatFixture) {
    for (raw, expected_bits) in fixture.decode_f32_bits.iter().copied().enumerate() {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "decode fixture indices cover exactly one byte of raw inputs"
        )]
        let got = T::from_bits(raw as u8).to_f32().to_bits();
        assert_eq!(
            got, expected_bits,
            "{} decode raw {raw:#04x}: got {got:#010x}, expected {expected_bits:#010x}",
            fixture.rust_type
        );
    }
}

fn assert_raw_roundtrip<T: CaseFloat>(fixture: &fixtures::FormatFixture) {
    for raw in 0..=255u8 {
        assert_eq!(
            T::from_bits(raw).to_bits(),
            raw,
            "{} raw roundtrip {raw:#04x}",
            fixture.rust_type
        );
    }
}

fn assert_predicate<T, Predicate, Expected>(
    fixture: &fixtures::FormatFixture,
    name: &str,
    predicate: Predicate,
    expected: Expected,
) where
    T: CaseFloat,
    Predicate: Fn(T) -> bool,
    Expected: Fn(&fixtures::FormatFixture, u8) -> bool,
{
    for raw in 0..=255u8 {
        let value = T::from_bits(raw);
        let got = predicate(value);
        let expected = expected(fixture, raw);
        assert_eq!(
            got, expected,
            "{} {name} raw {raw:#04x}: got {got}, expected {expected}",
            fixture.rust_type
        );
    }
}

fn assert_conversions<T: CaseFloat>(fixture: &fixtures::FormatFixture) {
    for &(input, expected) in fixture.conversions {
        let got = T::from_f32(input).to_bits();
        assert_eq!(
            got, expected,
            "{} from_f32({input:?}): got {got:#04x}, expected {expected:#04x}",
            fixture.rust_type
        );
    }
}

fn assert_arithmetic_method<T, Apply, Expected>(
    fixture: &fixtures::FormatFixture,
    name: &str,
    apply: Apply,
    expected: Expected,
) where
    T: CaseFloat,
    Apply: Fn(T, T) -> T,
    Expected: Fn((f32, f32, u8, u8, u8, u8, u8)) -> u8,
{
    for &(lhs, rhs, add, sub, mul, div, rem) in fixture.arithmetic {
        let l = T::from_f32(lhs);
        let r = T::from_f32(rhs);
        let got = apply(l, r).to_bits();
        let expected = expected((lhs, rhs, add, sub, mul, div, rem));
        assert_eq!(
            got, expected,
            "{} {name} {lhs:?} {rhs:?}: got {got:#04x}, expected {expected:#04x}",
            fixture.rust_type
        );
    }
}

fn assert_unary_method<T, Apply, Expected>(
    fixture: &fixtures::FormatFixture,
    name: &str,
    apply: Apply,
    expected: Expected,
) where
    T: CaseFloat,
    Apply: Fn(T) -> T,
    Expected: Fn(fixtures::UnaryMethodsFixture) -> u8,
{
    for row in fixture.unary_methods {
        let value = T::from_f32(row.input);
        let got = apply(value).to_bits();
        let expected = expected(*row);
        assert_eq!(
            got, expected,
            "{} {name} {:?}: got {got:#04x}, expected {expected:#04x}",
            fixture.rust_type, row.input
        );
    }
}

fn assert_binary_method<T, Apply, Expected>(
    fixture: &fixtures::FormatFixture,
    name: &str,
    apply: Apply,
    expected: Expected,
) where
    T: CaseFloat,
    Apply: Fn(T, T) -> T,
    Expected: Fn(fixtures::BinaryMethodsFixture) -> u8,
{
    for row in fixture.binary_methods {
        let lhs = T::from_f32(row.lhs);
        let rhs = T::from_f32(row.rhs);
        let got = apply(lhs, rhs).to_bits();
        let expected = expected(*row);
        assert_eq!(
            got, expected,
            "{} {name} {:?} {:?}: got {got:#04x}, expected {expected:#04x}",
            fixture.rust_type, row.lhs, row.rhs
        );
    }
}

fn assert_comparison<T, Apply, Expected>(
    fixture: &fixtures::FormatFixture,
    name: &str,
    apply: Apply,
    expected: Expected,
) where
    T: CaseFloat,
    Apply: Fn(T, T) -> bool,
    Expected: Fn(fixtures::ComparisonFixture) -> bool,
{
    for row in fixture.comparisons {
        let lhs = T::from_f32(row.lhs);
        let rhs = T::from_f32(row.rhs);
        let got = apply(lhs, rhs);
        let expected = expected(*row);
        assert_eq!(
            got, expected,
            "{} {name} {:?} {:?}: got {got}, expected {expected}",
            fixture.rust_type, row.lhs, row.rhs
        );
    }
}

macro_rules! compatibility_tests {
    ($($module:ident: $type:ty, $fixture:ident;)*) => {
        $(
            mod $module {
                use super::*;

                fn fixture() -> &'static fixtures::FormatFixture {
                    &fixtures::$fixture
                }

                #[test]
                fn special_constants() {
                    assert_special_constants::<$type>(fixture());
                }

                #[test]
                fn raw_roundtrip() {
                    assert_raw_roundtrip::<$type>(fixture());
                }

                #[test]
                fn decode() {
                    assert_decode::<$type>(fixture());
                }

                #[test]
                fn is_nan() {
                    assert_predicate::<$type, _, _>(
                        fixture(),
                        "is_nan",
                        <$type as CaseFloat>::is_nan,
                        |fixture, raw| fixture.is_nan[usize::from(raw)],
                    );
                }

                #[test]
                fn is_infinite() {
                    assert_predicate::<$type, _, _>(
                        fixture(),
                        "is_infinite",
                        <$type as CaseFloat>::is_infinite,
                        |fixture, raw| fixture.is_infinite[usize::from(raw)],
                    );
                }

                #[test]
                fn is_finite() {
                    assert_predicate::<$type, _, _>(
                        fixture(),
                        "is_finite",
                        <$type as CaseFloat>::is_finite,
                        |fixture, raw| fixture.is_finite[usize::from(raw)],
                    );
                }

                #[test]
                fn is_sign_negative() {
                    assert_predicate::<$type, _, _>(
                        fixture(),
                        "is_sign_negative",
                        <$type as CaseFloat>::is_sign_negative,
                        |fixture, raw| fixture.is_sign_negative[usize::from(raw)],
                    );
                }

                #[test]
                fn is_sign_positive() {
                    assert_predicate::<$type, _, _>(
                        fixture(),
                        "is_sign_positive",
                        <$type as CaseFloat>::is_sign_positive,
                        |fixture, raw| !fixture.is_sign_negative[usize::from(raw)],
                    );
                }

                #[test]
                fn from_f32() {
                    assert_conversions::<$type>(fixture());
                }

                #[test]
                fn add() {
                    assert_arithmetic_method::<$type, _, _>(
                        fixture(),
                        "add",
                        |lhs, rhs| lhs + rhs,
                        |(_, _, expected, _, _, _, _)| expected,
                    );
                }

                #[test]
                fn sub() {
                    assert_arithmetic_method::<$type, _, _>(
                        fixture(),
                        "sub",
                        |lhs, rhs| lhs - rhs,
                        |(_, _, _, expected, _, _, _)| expected,
                    );
                }

                #[test]
                fn mul() {
                    assert_arithmetic_method::<$type, _, _>(
                        fixture(),
                        "mul",
                        |lhs, rhs| lhs * rhs,
                        |(_, _, _, _, expected, _, _)| expected,
                    );
                }

                #[test]
                fn div() {
                    assert_arithmetic_method::<$type, _, _>(
                        fixture(),
                        "div",
                        |lhs, rhs| lhs / rhs,
                        |(_, _, _, _, _, expected, _)| expected,
                    );
                }

                #[test]
                fn rem() {
                    assert_arithmetic_method::<$type, _, _>(
                        fixture(),
                        "rem",
                        |lhs, rhs| lhs % rhs,
                        |(_, _, _, _, _, _, expected)| expected,
                    );
                }

                #[test]
                fn neg() {
                    assert_unary_method::<$type, _, _>(fixture(), "neg", |value| -value, |row| row.neg);
                }

                #[test]
                fn abs() {
                    assert_unary_method::<$type, _, _>(fixture(), "abs", <$type as CaseFloat>::abs, |row| row.abs);
                }

                #[test]
                fn signum() {
                    assert_unary_method::<$type, _, _>(fixture(), "signum", <$type as CaseFloat>::signum, |row| row.sign);
                }

                #[test]
                fn floor() {
                    assert_unary_method::<$type, _, _>(fixture(), "floor", <$type as CaseFloat>::floor, |row| row.floor);
                }

                #[test]
                fn ceil() {
                    assert_unary_method::<$type, _, _>(fixture(), "ceil", <$type as CaseFloat>::ceil, |row| row.ceil);
                }

                #[test]
                fn trunc() {
                    assert_unary_method::<$type, _, _>(fixture(), "trunc", <$type as CaseFloat>::trunc, |row| row.trunc);
                }

                #[test]
                fn round_ties_even() {
                    assert_unary_method::<$type, _, _>(
                        fixture(),
                        "round_ties_even",
                        <$type as CaseFloat>::round_ties_even,
                        |row| row.round_ties_even,
                    );
                }

                #[test]
                fn recip() {
                    assert_unary_method::<$type, _, _>(fixture(), "recip", <$type as CaseFloat>::recip, |row| row.recip);
                }

                #[test]
                fn sqrt() {
                    assert_unary_method::<$type, _, _>(fixture(), "sqrt", <$type as CaseFloat>::sqrt, |row| row.sqrt);
                }

                #[test]
                fn exp() {
                    assert_unary_method::<$type, _, _>(fixture(), "exp", <$type as CaseFloat>::exp, |row| row.exp);
                }

                #[test]
                fn exp2() {
                    assert_unary_method::<$type, _, _>(fixture(), "exp2", <$type as CaseFloat>::exp2, |row| row.exp2);
                }

                #[test]
                fn exp_m1() {
                    assert_unary_method::<$type, _, _>(fixture(), "exp_m1", <$type as CaseFloat>::exp_m1, |row| row.exp_m1);
                }

                #[test]
                fn ln() {
                    assert_unary_method::<$type, _, _>(fixture(), "ln", <$type as CaseFloat>::ln, |row| row.ln);
                }

                #[test]
                fn ln_1p() {
                    assert_unary_method::<$type, _, _>(fixture(), "ln_1p", <$type as CaseFloat>::ln_1p, |row| row.ln_1p);
                }

                #[test]
                fn log2() {
                    assert_unary_method::<$type, _, _>(fixture(), "log2", <$type as CaseFloat>::log2, |row| row.log2);
                }

                #[test]
                fn log10() {
                    assert_unary_method::<$type, _, _>(fixture(), "log10", <$type as CaseFloat>::log10, |row| row.log10);
                }

                #[test]
                fn cbrt() {
                    assert_unary_method::<$type, _, _>(fixture(), "cbrt", <$type as CaseFloat>::cbrt, |row| row.cbrt);
                }

                #[test]
                fn sin() {
                    assert_unary_method::<$type, _, _>(fixture(), "sin", <$type as CaseFloat>::sin, |row| row.sin);
                }

                #[test]
                fn cos() {
                    assert_unary_method::<$type, _, _>(fixture(), "cos", <$type as CaseFloat>::cos, |row| row.cos);
                }

                #[test]
                fn tan() {
                    assert_unary_method::<$type, _, _>(fixture(), "tan", <$type as CaseFloat>::tan, |row| row.tan);
                }

                #[test]
                fn asin() {
                    assert_unary_method::<$type, _, _>(fixture(), "asin", <$type as CaseFloat>::asin, |row| row.asin);
                }

                #[test]
                fn acos() {
                    assert_unary_method::<$type, _, _>(fixture(), "acos", <$type as CaseFloat>::acos, |row| row.acos);
                }

                #[test]
                fn atan() {
                    assert_unary_method::<$type, _, _>(fixture(), "atan", <$type as CaseFloat>::atan, |row| row.atan);
                }

                #[test]
                fn sinh() {
                    assert_unary_method::<$type, _, _>(fixture(), "sinh", <$type as CaseFloat>::sinh, |row| row.sinh);
                }

                #[test]
                fn cosh() {
                    assert_unary_method::<$type, _, _>(fixture(), "cosh", <$type as CaseFloat>::cosh, |row| row.cosh);
                }

                #[test]
                fn tanh() {
                    assert_unary_method::<$type, _, _>(fixture(), "tanh", <$type as CaseFloat>::tanh, |row| row.tanh);
                }

                #[test]
                fn copysign() {
                    assert_binary_method::<$type, _, _>(
                        fixture(),
                        "copysign",
                        <$type as CaseFloat>::copysign,
                        |row| row.copysign,
                    );
                }

                #[test]
                fn min() {
                    assert_binary_method::<$type, _, _>(fixture(), "min", <$type as CaseFloat>::min, |row| row.min);
                }

                #[test]
                fn max() {
                    assert_binary_method::<$type, _, _>(fixture(), "max", <$type as CaseFloat>::max, |row| row.max);
                }

                #[test]
                fn powf() {
                    assert_binary_method::<$type, _, _>(fixture(), "powf", <$type as CaseFloat>::powf, |row| row.powf);
                }

                #[test]
                fn hypot() {
                    assert_binary_method::<$type, _, _>(fixture(), "hypot", <$type as CaseFloat>::hypot, |row| row.hypot);
                }

                #[test]
                fn atan2() {
                    assert_binary_method::<$type, _, _>(fixture(), "atan2", <$type as CaseFloat>::atan2, |row| row.atan2);
                }

                #[test]
                fn lt() {
                    assert_comparison::<$type, _, _>(fixture(), "lt", |lhs, rhs| lhs < rhs, |row| row.lt);
                }

                #[test]
                fn le() {
                    assert_comparison::<$type, _, _>(fixture(), "le", |lhs, rhs| lhs <= rhs, |row| row.le);
                }

                #[test]
                fn eq() {
                    assert_comparison::<$type, _, _>(fixture(), "eq", |lhs, rhs| lhs == rhs, |row| row.eq);
                }

                #[test]
                fn ne() {
                    assert_comparison::<$type, _, _>(fixture(), "ne", |lhs, rhs| lhs != rhs, |row| row.ne);
                }

                #[test]
                fn ge() {
                    assert_comparison::<$type, _, _>(fixture(), "ge", |lhs, rhs| lhs >= rhs, |row| row.ge);
                }

                #[test]
                fn gt() {
                    assert_comparison::<$type, _, _>(fixture(), "gt", |lhs, rhs| lhs > rhs, |row| row.gt);
                }
            }
        )*
    };
}

compatibility_tests! {
    float8_e3m4: f8e3m4, FLOAT8E3M4_FIXTURE;
    float8_e4m3: f8e4m3, FLOAT8E4M3_FIXTURE;
    float8_e4m3b11fnuz: f8e4m3b11fnuz, FLOAT8E4M3B11FNUZ_FIXTURE;
    float8_e4m3fn: f8e4m3fn, FLOAT8E4M3FN_FIXTURE;
    float8_e4m3fnuz: f8e4m3fnuz, FLOAT8E4M3FNUZ_FIXTURE;
    float8_e5m2: f8e5m2, FLOAT8E5M2_FIXTURE;
    float8_e5m2fnuz: f8e5m2fnuz, FLOAT8E5M2FNUZ_FIXTURE;
    float8_e8m0fnu: f8e8m0fnu, FLOAT8E8M0FNU_FIXTURE;
    float4_e2m1fn: f4e2m1fn, FLOAT4E2M1FN_FIXTURE;
    float6_e2m3fn: f6e2m3fn, FLOAT6E2M3FN_FIXTURE;
    float6_e3m2fn: f6e3m2fn, FLOAT6E3M2FN_FIXTURE;
}

#[test]
fn public_api_helpers() {
    use core::num::FpCategory;

    let value = f8e4m3::from_f64(1.5);

    assert_eq!(f8e4m3::default(), f8e4m3::ZERO);
    assert_eq!(value.to_f64().to_bits(), 1.5f64.to_bits());
    assert_eq!(f8e4m3::from_le_bytes([value.to_bits()]), value);
    assert_eq!(f8e4m3::from_be_bytes([value.to_bits()]), value);
    assert_eq!(f8e4m3::from_ne_bytes([value.to_bits()]), value);
    assert_eq!(value.to_le_bytes(), [value.to_bits()]);
    assert_eq!(value.to_be_bytes(), [value.to_bits()]);
    assert_eq!(value.to_ne_bytes(), [value.to_bits()]);

    assert_eq!(f8e4m3::ZERO.classify(), FpCategory::Zero);
    assert_eq!(f8e4m3::from_bits(0x01).classify(), FpCategory::Subnormal);
    assert_eq!(f8e4m3::ONE.classify(), FpCategory::Normal);
    assert_eq!(f8e4m3::INFINITY.classify(), FpCategory::Infinite);
    assert_eq!(f8e4m3::NAN.classify(), FpCategory::Nan);
    assert!(f8e4m3::ONE.is_normal());

    assert_eq!(
        f8e4m3::from_f32(3.0).clamp(f8e4m3::ONE, f8e4m3::from_f32(2.0)),
        f8e4m3::from_f32(2.0)
    );
    assert_eq!(
        f8e4m3::ZERO.clamp(f8e4m3::ONE, f8e4m3::from_f32(2.0)),
        f8e4m3::ONE
    );
    assert_eq!(value.clamp(f8e4m3::ONE, f8e4m3::from_f32(2.0)), value);
    assert!(f8e4m3::NEG_ZERO.total_cmp(&f8e4m3::ZERO).is_lt());
    assert!(f4e2m1fn::NEG_ZERO.total_cmp(&f4e2m1fn::ZERO).is_lt());

    assert_eq!(format!("{value}"), "1.5");
    assert_eq!(format!("{value:?}"), "f8e4m3(1.5)");
    assert_eq!(format!("{value:e}"), "1.5e0");
    assert_eq!(format!("{value:E}"), "1.5E0");
    assert_eq!("1.5".parse::<f8e4m3>().unwrap(), value);
}
