use microfloat::f6e3m2fn;
use proptest::prelude::*;

prop_compose! {
    fn any_f6e3m2fn() (bits in 0u8..) -> f6e3m2fn {
        f6e3m2fn::from_bits(bits)
    }
}

#[test]
fn roundtrip_from_bits_to_bits() {
    proptest!(|(bits in 0u8..)| {
        let v = f6e3m2fn::from_bits(bits);
        assert_eq!(v.to_bits(), bits);
    });
}

#[test]
fn default_is_zero() {
    let default = f6e3m2fn::default();
    assert!(default.to_f32() == 0.0 || default.to_f32() == -0.0);
}

#[test]
fn add_zero_identity() {
    proptest!(|(v in any_f6e3m2fn())| {
        if v.is_finite() {
            let result = v + f6e3m2fn::ZERO;
            if result.is_finite() {
                let diff = (result.to_f32() - v.to_f32()).abs();
                assert!(diff <= 1.0);
            }
        }
    });
}

#[test]
fn mul_one_identity() {
    proptest!(|(v in any_f6e3m2fn())| {
        if v.is_finite() {
            let result = v * f6e3m2fn::ONE;
            if result.is_finite() {
                let diff = (result.to_f32() - v.to_f32()).abs();
                assert!(diff <= 1.0);
            }
        }
    });
}

#[test]
fn neg_twice_identity() {
    proptest!(|(v in any_f6e3m2fn())| {
        if v.is_finite() {
            let result = -(-v);
            let diff = (result.to_f32() - v.to_f32()).abs();
            assert!(diff <= 1.0);
        }
    });
}

#[test]
fn abs_non_negative() {
    proptest!(|(v in any_f6e3m2fn())| {
        if v.is_finite() {
            let abs = v.abs();
            assert!(!abs.is_sign_negative());
        }
    });
}

#[test]
fn floor_ceil_ordering() {
    proptest!(|(v in any_f6e3m2fn())| {
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
    proptest!(|(v in any_f6e3m2fn())| {
        if !v.is_nan() {
            assert_eq!(v.total_cmp(&v), core::cmp::Ordering::Equal);
        }
    });
}

#[test]
fn one_is_finite() {
    assert!(f6e3m2fn::ONE.is_finite());
    assert!(f6e3m2fn::ONE.is_normal());
}

#[test]
fn display_does_not_panic() {
    proptest!(|(v in any_f6e3m2fn())| {
        let _ = format!("{v}");
    });
}

#[test]
fn compound_assign_eq_binary_add() {
    proptest!(|(a in any_f6e3m2fn(), b in any_f6e3m2fn())| {
        let mut x = a;
        x += b;
        assert_eq!(x, a + b);
    });
}

#[test]
fn compound_assign_eq_binary_sub() {
    proptest!(|(a in any_f6e3m2fn(), b in any_f6e3m2fn())| {
        let mut x = a;
        x -= b;
        assert_eq!(x, a - b);
    });
}

#[test]
fn compound_assign_eq_binary_mul() {
    proptest!(|(a in any_f6e3m2fn(), b in any_f6e3m2fn())| {
        let mut x = a;
        x *= b;
        assert_eq!(x, a * b);
    });
}

#[test]
fn compound_assign_eq_binary_div() {
    proptest!(|(a in any_f6e3m2fn(), b in any_f6e3m2fn())| {
        let mut x = a;
        x /= b;
        assert_eq!(x, a / b);
    });
}

#[test]
fn compound_assign_eq_binary_rem() {
    proptest!(|(a in any_f6e3m2fn(), b in any_f6e3m2fn())| {
        let mut x = a;
        x %= b;
        assert_eq!(x, a % b);
    });
}

#[test]
fn iter_sum() {
    proptest!(|(a in any_f6e3m2fn(), b in any_f6e3m2fn(), c in any_f6e3m2fn())| {
        let values = [a, b, c];
        assert_eq!(values.into_iter().sum::<f6e3m2fn>(), a + b + c);
        assert_eq!(values.iter().sum::<f6e3m2fn>(), a + b + c);
    });
}

#[test]
fn iter_product() {
    proptest!(|(a in any_f6e3m2fn(), b in any_f6e3m2fn(), c in any_f6e3m2fn())| {
        let values = [a, b, c];
        assert_eq!(values.into_iter().product::<f6e3m2fn>(), a * b * c);
        assert_eq!(values.iter().product::<f6e3m2fn>(), a * b * c);
    });
}
