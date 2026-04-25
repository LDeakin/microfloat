use microfloat::f8e4m3;
use proptest::prelude::*;

prop_compose! {
    fn any_f8e4m3() (bits in 0u8..) -> f8e4m3 {
        f8e4m3::from_bits(bits)
    }
}

#[test]
fn roundtrip_from_bits_to_bits() {
    proptest!(|(bits in 0u8..)| {
        let v = f8e4m3::from_bits(bits);
        assert_eq!(v.to_bits(), bits);
    });
}

#[test]
fn from_bits_zero_is_zero() {
    let zero = f8e4m3::from_bits(0);
    assert!(zero.to_f32() == 0.0 || zero.to_f32() == -0.0);
}

#[test]
fn default_is_zero() {
    let default = f8e4m3::default();
    assert!(default.to_f32() == 0.0 || default.to_f32() == -0.0);
}

#[test]
fn add_zero_identity() {
    proptest!(|(v in any_f8e4m3())| {
        if v.is_finite() {
            let result = v + f8e4m3::ZERO;
            if result.is_finite() {
                let diff = (result.to_f32() - v.to_f32()).abs();
                assert!(diff <= 1.0);
            }
        }
    });
}

#[test]
fn mul_one_identity() {
    proptest!(|(v in any_f8e4m3())| {
        if v.is_finite() {
            let result = v * f8e4m3::ONE;
            if result.is_finite() {
                let diff = (result.to_f32() - v.to_f32()).abs();
                assert!(diff <= 1.0);
            }
        }
    });
}

#[test]
fn neg_twice_identity() {
    proptest!(|(v in any_f8e4m3())| {
        if v.is_finite() {
            let result = -(-v);
            let diff = (result.to_f32() - v.to_f32()).abs();
            assert!(diff <= 1.0);
        }
    });
}

#[test]
fn abs_non_negative() {
    proptest!(|(v in any_f8e4m3())| {
        if v.is_finite() {
            let abs = v.abs();
            assert!(!abs.is_sign_negative());
        }
    });
}

#[test]
fn floor_ceil_ordering() {
    proptest!(|(v in any_f8e4m3())| {
        if v.is_finite() {
            let f = v.floor();
            let c = v.ceil();
            let fv = v.to_f32();
            if f.is_finite() {
                assert!(f.to_f32() <= fv);
            }
            if c.is_finite() {
                assert!(c.to_f32() >= fv);
            }
        }
    });
}

#[test]
fn total_cmp_reflexive() {
    proptest!(|(v in any_f8e4m3())| {
        if !v.is_nan() {
            assert_eq!(v.total_cmp(&v), core::cmp::Ordering::Equal);
        }
    });
}

#[test]
fn nan_is_nan() {
    assert!(f8e4m3::NAN.is_nan());
}

#[test]
fn infinity_is_infinite() {
    assert!(f8e4m3::INFINITY.is_infinite());
}

#[test]
fn one_is_finite() {
    assert!(f8e4m3::ONE.is_finite());
    assert!(f8e4m3::ONE.is_normal());
}

#[test]
fn display_does_not_panic() {
    proptest!(|(v in any_f8e4m3())| {
        let _ = format!("{v}");
    });
}

#[test]
fn nan_propagates_on_add() {
    proptest!(|(v in any_f8e4m3())| {
        assert!((v + f8e4m3::NAN).is_nan());
        assert!((f8e4m3::NAN + v).is_nan());
    });
}

#[test]
fn nan_propagates_on_sub() {
    proptest!(|(v in any_f8e4m3())| {
        assert!((v - f8e4m3::NAN).is_nan());
        assert!((f8e4m3::NAN - v).is_nan());
    });
}

#[test]
fn nan_propagates_on_mul() {
    proptest!(|(v in any_f8e4m3())| {
        assert!((v * f8e4m3::NAN).is_nan());
        assert!((f8e4m3::NAN * v).is_nan());
    });
}

#[test]
fn compound_assign_eq_binary_add() {
    proptest!(|(a in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()))| {
        let mut x = a;
        x += b;
        prop_assert!(x == a + b || (x.is_nan() && (a + b).is_nan()));
    });
}

#[test]
fn compound_assign_eq_binary_sub() {
    proptest!(|(a in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()))| {
        let mut x = a;
        x -= b;
        prop_assert!(x == a - b || (x.is_nan() && (a - b).is_nan()));
    });
}

#[test]
fn compound_assign_eq_binary_mul() {
    proptest!(|(a in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()))| {
        let mut x = a;
        x *= b;
        prop_assert!(x == a * b || (x.is_nan() && (a * b).is_nan()));
    });
}

#[test]
fn compound_assign_eq_binary_div() {
    proptest!(|(a in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()))| {
        let mut x = a;
        x /= b;
        prop_assert!(x == a / b || (x.is_nan() && (a / b).is_nan()));
    });
}

#[test]
fn compound_assign_eq_binary_rem() {
    proptest!(|(a in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()))| {
        let mut x = a;
        x %= b;
        prop_assert!(x == a % b || (x.is_nan() && (a % b).is_nan()));
    });
}

#[test]
fn iter_sum() {
    proptest!(|(a in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()), c in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()))| {
        let values = [a, b, c];
        prop_assert!(values.into_iter().sum::<f8e4m3>() == a + b + c || (values.into_iter().sum::<f8e4m3>().is_nan() && (a + b + c).is_nan()));
        prop_assert!(values.iter().sum::<f8e4m3>() == a + b + c || (values.iter().sum::<f8e4m3>().is_nan() && (a + b + c).is_nan()));
    });
}

#[test]
fn iter_product() {
    proptest!(|(a in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()), c in any_f8e4m3().prop_filter("non-nan", |v| !v.is_nan()))| {
        let values = [a, b, c];
        prop_assert!(values.into_iter().product::<f8e4m3>() == a * b * c || (values.into_iter().product::<f8e4m3>().is_nan() && (a * b * c).is_nan()));
        prop_assert!(values.iter().product::<f8e4m3>() == a * b * c || (values.iter().product::<f8e4m3>().is_nan() && (a * b * c).is_nan()));
    });
}
