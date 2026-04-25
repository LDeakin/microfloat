use microfloat::{f4e2m1fn, f8e4m3, f8e4m3b11fnuz, f8e8m0fnu};

#[test]
fn remainder_special_values() {
    assert!((f8e4m3::NAN % f8e4m3::ONE).is_nan());
    assert!((f8e4m3::INFINITY % f8e4m3::from_f32(2.0)).is_nan());
    assert_eq!(
        (f8e4m3::ZERO % f8e4m3::NEG_INFINITY).to_bits(),
        f8e4m3::NEG_ZERO.to_bits()
    );
    assert_eq!(f8e4m3::ONE % f8e4m3::INFINITY, f8e4m3::ONE);
    assert_eq!(f8e4m3::from_f32(-1.0) % f8e4m3::INFINITY, f8e4m3::INFINITY);
}

#[test]
fn zero_division_and_remainder_nan_modes() {
    assert_eq!(f4e2m1fn::ZERO / f4e2m1fn::ZERO, f4e2m1fn::ZERO);
    assert_eq!(
        (f8e4m3::ZERO / f8e4m3::ZERO).to_bits(),
        f8e4m3::from_bits(0xfc).to_bits()
    );
    assert_eq!(
        (f8e4m3b11fnuz::ZERO / f8e4m3b11fnuz::ZERO).to_bits(),
        f8e4m3b11fnuz::NAN.to_bits()
    );

    assert_eq!(f4e2m1fn::ONE % f4e2m1fn::ZERO, f4e2m1fn::NEG_ZERO);
    assert_eq!(
        (f8e4m3b11fnuz::ONE % f8e4m3b11fnuz::ZERO).to_bits(),
        f8e4m3b11fnuz::NAN.to_bits()
    );
    assert_eq!(f8e8m0fnu::ZERO / f8e8m0fnu::ZERO, f8e8m0fnu::ONE);
    assert_eq!((f8e4m3::ONE % f8e4m3::NAN).to_bits(), f8e4m3::NAN.to_bits());
    assert_eq!((f8e4m3::NAN % f8e4m3::NAN).to_bits(), f8e4m3::NAN.to_bits());
    assert_eq!(
        (f8e4m3::ONE % f8e4m3::from_f32(-0.5)).to_bits(),
        f8e4m3::NEG_ZERO.to_bits()
    );
}
