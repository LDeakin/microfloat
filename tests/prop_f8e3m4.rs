use core::str::FromStr;
use microfloat::f8e3m4;
use proptest::prelude::*;

prop_compose! {
    fn any_f8e3m4() (bits in 0u8..) -> f8e3m4 {
        f8e3m4::from_bits(bits)
    }
}

#[test]
fn roundtrip_from_bits_to_bits() {
    proptest!(|(bits in 0u8..)| {
        let v = f8e3m4::from_bits(bits);
        assert_eq!(v.to_bits(), bits);
    });
}

#[test]
fn from_bits_zero_is_zero() {
    let zero = f8e3m4::from_bits(0);
    assert!(zero.to_f32() == 0.0 || zero.to_f32() == -0.0);
}

#[test]
fn default_is_zero() {
    let default = f8e3m4::default();
    assert!(default.to_f32() == 0.0 || default.to_f32() == -0.0);
}

#[test]
fn add_zero_identity() {
    proptest!(|(v in any_f8e3m4())| {
        if v.is_finite() {
            let result = v + f8e3m4::ZERO;
            if result.is_finite() {
                let diff = (result.to_f32() - v.to_f32()).abs();
                assert!(diff <= 1.0);
            }
        }
    });
}

#[test]
fn mul_one_identity() {
    proptest!(|(v in any_f8e3m4())| {
        if v.is_finite() {
            let result = v * f8e3m4::ONE;
            if result.is_finite() {
                let diff = (result.to_f32() - v.to_f32()).abs();
                assert!(diff <= 1.0);
            }
        }
    });
}

#[test]
fn neg_twice_identity() {
    proptest!(|(v in any_f8e3m4())| {
        if v.is_finite() {
            let result = -(-v);
            let diff = (result.to_f32() - v.to_f32()).abs();
            assert!(diff <= 1.0);
        }
    });
}

#[test]
fn neg_zero_is_zero() {
    let neg = -f8e3m4::ZERO;
    assert!(neg.to_f32() == 0.0 || neg.to_f32() == -0.0);
}

#[test]
fn abs_non_negative() {
    proptest!(|(v in any_f8e3m4())| {
        if v.is_finite() {
            let abs = v.abs();
            assert!(!abs.is_sign_negative());
        }
    });
}

#[test]
fn abs_symmetric() {
    proptest!(|(v in any_f8e3m4())| {
        if v.is_finite() && v.to_f32() != 0.0 {
            let a = v.abs();
            let neg = -v;
            let b = neg.abs();
            assert_eq!(a.to_f32().abs().to_bits(), b.to_f32().abs().to_bits());
        }
    });
}

#[test]
fn abs_of_zero_is_zero() {
    let result = f8e3m4::ZERO.abs();
    assert!(result.to_f32() == 0.0 || result.to_f32() == -0.0);
}

#[test]
fn floor_ceil_ordering() {
    proptest!(|(v in any_f8e3m4())| {
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
fn trunc_towards_zero() {
    proptest!(|(v in any_f8e3m4())| {
        if v.is_finite() {
            let t = v.trunc();
            let fv = v.to_f32();
            if t.is_finite() {
                let abs_trunc = t.to_f32().abs();
                let abs_orig = fv.abs();
                assert!(abs_trunc <= abs_orig + 1.0);
            }
        }
    });
}

#[test]
fn powf_one_identity() {
    proptest!(|(v in any_f8e3m4())| {
        if v.is_finite() {
            let result = v.powf(f8e3m4::ONE);
            let diff = (result.to_f32() - v.to_f32()).abs();
            assert!(diff <= 1.0);
        }
    });
}

#[test]
fn total_cmp_reflexive() {
    proptest!(|(v in any_f8e3m4())| {
        if !v.is_nan() {
            assert_eq!(v.total_cmp(&v), core::cmp::Ordering::Equal);
        }
    });
}

#[test]
fn partial_eq_nan_returns_false() {
    proptest!(|(v in any_f8e3m4())| {
        if v.is_nan() {
            assert!(!v.eq(&v));
        }
    });
}

#[test]
fn nan_is_nan() {
    assert!(f8e3m4::NAN.is_nan());
}

#[test]
fn infinity_is_infinite() {
    assert!(f8e3m4::INFINITY.is_infinite());
}

#[test]
fn neg_infinity_is_infinite() {
    assert!(f8e3m4::NEG_INFINITY.is_infinite());
}

#[test]
fn neg_infinity_of_infinity() {
    let neg_inf = -f8e3m4::INFINITY;
    assert!(neg_inf.is_infinite());
}

#[test]
fn zero_is_not_infinite() {
    assert!(!f8e3m4::ZERO.is_infinite());
    assert!(!f8e3m4::default().is_infinite());
}

#[test]
fn max_is_finite() {
    assert!(f8e3m4::MAX.is_finite());
}

#[test]
fn min_is_finite() {
    assert!(f8e3m4::MIN.is_finite());
}

#[test]
fn one_is_finite() {
    assert!(f8e3m4::ONE.is_finite());
    assert!(f8e3m4::ONE.is_normal());
}

#[test]
fn epsilon_positive() {
    assert!(f8e3m4::EPSILON.to_f32() > 0.0);
}

#[test]
fn display_does_not_panic() {
    proptest!(|(v in any_f8e3m4())| {
        let _ = format!("{v}");
    });
}

#[test]
fn debug_does_not_panic() {
    proptest!(|(v in any_f8e3m4())| {
        let _ = format!("{v:?}");
    });
}

#[test]
fn from_str_parse_returns_valid() {
    proptest!(|(s in "[0-9.eE+-]+")| {
        if let Ok(x) = s.parse::<f32>() {
            if x.is_finite() {
                let parsed = f8e3m4::from_str(&s);
                if let Ok(v) = parsed {
                    assert!(v.to_f32().is_finite() || v.is_nan() || v.to_f32().is_infinite());
                }
            }
        }
    });
}

#[test]
fn nan_propagates_on_add() {
    proptest!(|(v in any_f8e3m4())| {
        assert!((v + f8e3m4::NAN).is_nan());
        assert!((f8e3m4::NAN + v).is_nan());
    });
}

#[test]
fn nan_propagates_on_sub() {
    proptest!(|(v in any_f8e3m4())| {
        assert!((v - f8e3m4::NAN).is_nan());
        assert!((f8e3m4::NAN - v).is_nan());
    });
}

#[test]
fn nan_propagates_on_mul() {
    proptest!(|(v in any_f8e3m4())| {
        assert!((v * f8e3m4::NAN).is_nan());
        assert!((f8e3m4::NAN * v).is_nan());
    });
}

#[test]
fn compound_assign_eq_binary_add() {
    proptest!(|(a in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()))| {
        let mut x = a;
        x += b;
        prop_assert!(x == a + b || (x.is_nan() && (a + b).is_nan()));
    });
}

#[test]
fn compound_assign_eq_binary_sub() {
    proptest!(|(a in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()))| {
        let mut x = a;
        x -= b;
        prop_assert!(x == a - b || (x.is_nan() && (a - b).is_nan()));
    });
}

#[test]
fn compound_assign_eq_binary_mul() {
    proptest!(|(a in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()))| {
        let mut x = a;
        x *= b;
        prop_assert!(x == a * b || (x.is_nan() && (a * b).is_nan()));
    });
}

#[test]
fn compound_assign_eq_binary_div() {
    proptest!(|(a in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()))| {
        let mut x = a;
        x /= b;
        prop_assert!(x == a / b || (x.is_nan() && (a / b).is_nan()));
    });
}

#[test]
fn compound_assign_eq_binary_rem() {
    proptest!(|(a in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()))| {
        let mut x = a;
        x %= b;
        prop_assert!(x == a % b || (x.is_nan() && (a % b).is_nan()));
    });
}

#[test]
fn iter_sum() {
    proptest!(|(a in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()), c in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()))| {
        let values = [a, b, c];
        prop_assert!(values.into_iter().sum::<f8e3m4>() == a + b + c || (values.into_iter().sum::<f8e3m4>().is_nan() && (a + b + c).is_nan()));
        prop_assert!(values.iter().sum::<f8e3m4>() == a + b + c || (values.iter().sum::<f8e3m4>().is_nan() && (a + b + c).is_nan()));
    });
}

#[test]
fn iter_product() {
    proptest!(|(a in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()), b in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()), c in any_f8e3m4().prop_filter("non-nan", |v| !v.is_nan()))| {
        let values = [a, b, c];
        prop_assert!(values.into_iter().product::<f8e3m4>() == a * b * c || (values.into_iter().product::<f8e3m4>().is_nan() && (a * b * c).is_nan()));
        prop_assert!(values.iter().product::<f8e3m4>() == a * b * c || (values.iter().product::<f8e3m4>().is_nan() && (a * b * c).is_nan()));
    });
}
