use microfloat::{f4e2m1fn, f6e2m3fn, f6e3m2fn, f8e3m4, f8e4m3, f8e4m3b11fnuz, f8e4m3fn, f8e4m3fnuz, f8e5m2, f8e5m2fnuz, f8e8m0fnu};

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
