use microfloat::{
    f4e2m1fn, f6e2m3fn, f6e3m2fn, f8e3m4, f8e4m3, f8e4m3b11fnuz, f8e4m3fn, f8e4m3fnuz, f8e5m2,
    f8e5m2fnuz, f8e8m0fnu,
};

trait StepFloat: Copy + core::fmt::Debug + PartialEq {
    const INFINITY: Self;
    const NEG_INFINITY: Self;

    fn from_bits(bits: u8) -> Self;
    fn to_bits(self) -> u8;
    fn to_f32(self) -> f32;
    fn is_nan(self) -> bool;
    fn next_up(self) -> Self;
    fn next_down(self) -> Self;
}

macro_rules! impl_step_float {
    ($($type:ty),* $(,)?) => {
        $(
            impl StepFloat for $type {
                const INFINITY: Self = <$type>::INFINITY;
                const NEG_INFINITY: Self = <$type>::NEG_INFINITY;

                fn from_bits(bits: u8) -> Self {
                    <$type>::from_bits(bits)
                }

                fn to_bits(self) -> u8 {
                    <$type>::to_bits(self)
                }

                fn to_f32(self) -> f32 {
                    <$type>::to_f32(self)
                }

                fn is_nan(self) -> bool {
                    <$type>::is_nan(self)
                }

                fn next_up(self) -> Self {
                    <$type>::next_up(self)
                }

                fn next_down(self) -> Self {
                    <$type>::next_down(self)
                }
            }
        )*
    };
}

impl_step_float!(
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

#[test]
fn special_constants_cover_format_modes() {
    assert_eq!(f8e8m0fnu::ONE.to_bits(), 0x7f);
    assert_eq!(f8e4m3::ONE.to_bits(), 0x38);
    assert_eq!(f8e4m3fnuz::NEG_ZERO.to_bits(), 0x00);
    assert_eq!(f8e8m0fnu::NEG_ZERO.to_bits(), f8e8m0fnu::NAN.to_bits());
    assert_eq!(f8e4m3::NEG_ZERO.to_bits(), 0x80);

    assert_eq!(f8e8m0fnu::MAX.to_bits(), 0xfe);
    assert_eq!(f8e4m3b11fnuz::MAX.to_bits(), 0x7f);
    assert_eq!(f4e2m1fn::INFINITY.to_bits(), f4e2m1fn::MAX.to_bits());
    assert_eq!(f8e4m3fn::NEG_INFINITY.to_bits(), 0xff);
    assert_eq!(f4e2m1fn::NAN.to_bits(), f4e2m1fn::NEG_ZERO.to_bits());
}

#[test]
fn conversion_boundaries_cover_format_modes() {
    assert!(f8e8m0fnu::NAN.to_f32().is_nan());
    assert_eq!(f8e4m3fnuz::from_f32(-0.0).to_bits(), 0x00);
    assert_eq!(f8e8m0fnu::from_f32(1.0e-39).to_bits(), 0x00);
    assert_eq!(f8e8m0fnu::from_f32(16.0).to_bits(), 0x83);
    assert!(!f8e8m0fnu::from_bits(0x80).is_sign_negative());
}

#[test]
fn clamp_panics_when_min_gt_max() {
    let _ = std::panic::catch_unwind(|| f8e4m3::ZERO.clamp(f8e4m3::ONE, f8e4m3::from_f32(0.5)));
}

#[test]
fn clamp_panics_when_min_is_nan() {
    let _ = std::panic::catch_unwind(|| f8e4m3::ZERO.clamp(f8e4m3::NAN, f8e4m3::ONE));
}

#[test]
fn clamp_panics_when_max_is_nan() {
    let _ = std::panic::catch_unwind(|| f8e4m3::ZERO.clamp(f8e4m3::ONE, f8e4m3::NAN));
}

#[test]
fn format_query_helpers() {
    assert!(f8e3m4::has_inf());
    assert!(f8e3m4::has_nan());
    assert!(!f8e3m4::is_finite_only());

    assert!(f8e4m3::has_inf());
    assert!(f8e4m3::has_nan());
    assert!(!f8e4m3::is_finite_only());

    assert!(f8e5m2::has_inf());
    assert!(f8e5m2::has_nan());
    assert!(!f8e5m2::is_finite_only());

    assert!(!f8e4m3fn::has_inf());
    assert!(f8e4m3fn::has_nan());
    assert!(!f8e4m3fn::is_finite_only());

    assert!(!f8e4m3fnuz::has_inf());
    assert!(f8e4m3fnuz::has_nan());
    assert!(!f8e4m3fnuz::is_finite_only());

    assert!(!f8e5m2fnuz::has_inf());
    assert!(f8e5m2fnuz::has_nan());
    assert!(!f8e5m2fnuz::is_finite_only());

    assert!(!f8e4m3b11fnuz::has_inf());
    assert!(f8e4m3b11fnuz::has_nan());
    assert!(!f8e4m3b11fnuz::is_finite_only());

    assert!(!f8e8m0fnu::has_inf());
    assert!(f8e8m0fnu::has_nan());
    assert!(!f8e8m0fnu::is_finite_only());

    assert!(!f4e2m1fn::has_inf());
    assert!(!f4e2m1fn::has_nan());
    assert!(f4e2m1fn::is_finite_only());

    assert!(!f6e2m3fn::has_inf());
    assert!(!f6e2m3fn::has_nan());
    assert!(f6e2m3fn::is_finite_only());

    assert!(!f6e3m2fn::has_inf());
    assert!(!f6e3m2fn::has_nan());
    assert!(f6e3m2fn::is_finite_only());
}

#[test]
fn next_methods_cover_every_canonical_value() {
    assert_next_methods::<f8e3m4>("f8e3m4", 0xff);
    assert_next_methods::<f8e4m3>("f8e4m3", 0xff);
    assert_next_methods::<f8e4m3b11fnuz>("f8e4m3b11fnuz", 0xff);
    assert_next_methods::<f8e4m3fn>("f8e4m3fn", 0xff);
    assert_next_methods::<f8e4m3fnuz>("f8e4m3fnuz", 0xff);
    assert_next_methods::<f8e5m2>("f8e5m2", 0xff);
    assert_next_methods::<f8e5m2fnuz>("f8e5m2fnuz", 0xff);
    assert_next_methods::<f8e8m0fnu>("f8e8m0fnu", 0xff);
    assert_next_methods::<f4e2m1fn>("f4e2m1fn", 0x0f);
    assert_next_methods::<f6e2m3fn>("f6e2m3fn", 0x3f);
    assert_next_methods::<f6e3m2fn>("f6e3m2fn", 0x3f);
}

#[test]
fn next_methods_cover_named_edges() {
    assert_eq!(f8e4m3::NEG_ZERO.next_up().to_bits(), 0x01);
    assert_eq!(f8e4m3::ZERO.next_down().to_bits(), 0x81);
    assert_eq!(
        f8e4m3::INFINITY.next_down().to_bits(),
        f8e4m3::MAX.to_bits()
    );
    assert_eq!(
        f8e4m3::NEG_INFINITY.next_up().to_bits(),
        f8e4m3::MIN.to_bits()
    );
    assert_eq!(
        f8e4m3::INFINITY.next_up().to_bits(),
        f8e4m3::INFINITY.to_bits()
    );
    assert_eq!(
        f8e4m3::NEG_INFINITY.next_down().to_bits(),
        f8e4m3::NEG_INFINITY.to_bits()
    );
    assert_eq!(f8e4m3::NAN.next_up().to_bits(), f8e4m3::NAN.to_bits());
    assert_eq!(f8e4m3::NAN.next_down().to_bits(), f8e4m3::NAN.to_bits());

    assert_eq!(
        f8e4m3fnuz::MAX.next_up().to_bits(),
        f8e4m3fnuz::NAN.to_bits()
    );
    assert_eq!(
        f8e4m3fnuz::MIN.next_down().to_bits(),
        f8e4m3fnuz::NAN.to_bits()
    );
    assert_eq!(f4e2m1fn::MAX.next_up().to_bits(), f4e2m1fn::MAX.to_bits());
    assert_eq!(f4e2m1fn::MIN.next_down().to_bits(), f4e2m1fn::MIN.to_bits());
    assert_eq!(f8e8m0fnu::MAX.next_up().to_bits(), f8e8m0fnu::NAN.to_bits());
    assert_eq!(
        f8e8m0fnu::from_bits(0x00).next_down().to_bits(),
        f8e8m0fnu::NAN.to_bits()
    );
}

fn assert_next_methods<T: StepFloat>(name: &str, max_bits: u8) {
    let sorted = sorted_representable_values::<T>(max_bits);
    for raw in 0..=max_bits {
        let value = T::from_bits(raw);
        let next_up = value.next_up();
        let expected_up = expected_next_up(value, &sorted);
        assert_eq!(
            next_up.to_bits(),
            expected_up.to_bits(),
            "{name} next_up raw {raw:#04x}: got {:#04x}, expected {:#04x}",
            next_up.to_bits(),
            expected_up.to_bits()
        );

        let next_down = value.next_down();
        let expected_down = expected_next_down(value, &sorted);
        assert_eq!(
            next_down.to_bits(),
            expected_down.to_bits(),
            "{name} next_down raw {raw:#04x}: got {:#04x}, expected {:#04x}",
            next_down.to_bits(),
            expected_down.to_bits()
        );
    }
}

fn sorted_representable_values<T: StepFloat>(max_bits: u8) -> Vec<T> {
    let mut values = Vec::new();
    for raw in 0..=max_bits {
        let value = T::from_bits(raw);
        if !value.is_nan() {
            values.push(value);
        }
    }
    values.sort_by(|lhs, rhs| lhs.to_f32().total_cmp(&rhs.to_f32()));
    values
}

fn expected_next_up<T: StepFloat>(value: T, sorted: &[T]) -> T {
    if value.is_nan() {
        return value;
    }

    let as_f32 = value.to_f32();
    if is_positive_infinity(as_f32) {
        return value;
    }
    if is_zero(as_f32) {
        return first_greater_than_zero(sorted).unwrap_or(T::INFINITY);
    }
    let index = sorted_index(value, sorted);
    sorted.get(index + 1).copied().unwrap_or(T::INFINITY)
}

fn expected_next_down<T: StepFloat>(value: T, sorted: &[T]) -> T {
    if value.is_nan() {
        return value;
    }

    let as_f32 = value.to_f32();
    if is_negative_infinity(as_f32) {
        return value;
    }
    if is_zero(as_f32) {
        return last_less_than_zero(sorted).unwrap_or(T::NEG_INFINITY);
    }
    let index = sorted_index(value, sorted);
    index
        .checked_sub(1)
        .and_then(|previous| sorted.get(previous))
        .copied()
        .unwrap_or(T::NEG_INFINITY)
}

fn sorted_index<T: StepFloat>(value: T, sorted: &[T]) -> usize {
    sorted
        .iter()
        .position(|candidate| candidate.to_bits() == value.to_bits())
        .expect("non-NaN canonical value must be present in sorted representable values")
}

fn first_greater_than_zero<T: StepFloat>(sorted: &[T]) -> Option<T> {
    sorted.iter().copied().find(|value| value.to_f32() > 0.0)
}

fn last_less_than_zero<T: StepFloat>(sorted: &[T]) -> Option<T> {
    sorted
        .iter()
        .copied()
        .rev()
        .find(|value| value.to_f32() < 0.0)
}

fn is_zero(value: f32) -> bool {
    matches!(value.classify(), core::num::FpCategory::Zero)
}

fn is_positive_infinity(value: f32) -> bool {
    value.is_infinite() && value.is_sign_positive()
}

fn is_negative_infinity(value: f32) -> bool {
    value.is_infinite() && value.is_sign_negative()
}
