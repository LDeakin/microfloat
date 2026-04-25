use microfloat::f4e2m1fn;
use proptest::prelude::*;

prop_compose! {
    fn any_f4e2m1fn() (bits in 0u8..) -> f4e2m1fn {
        f4e2m1fn::from_bits(bits)
    }
}

#[test]
fn roundtrip_from_bits_to_bits() {
    proptest!(|(bits in 0u8..)| {
        let v = f4e2m1fn::from_bits(bits);
        assert_eq!(v.to_bits(), bits);
    });
}

#[test]
fn default_is_zero() {
    let default = f4e2m1fn::default();
    assert!(default.to_f32() == 0.0 || default.to_f32() == -0.0);
}

#[test]
fn add_zero_identity() {
    proptest!(|(v in any_f4e2m1fn())| {
        if v.is_finite() {
            let result = v + f4e2m1fn::ZERO;
            if result.is_finite() {
                let diff = (result.to_f32() - v.to_f32()).abs();
                assert!(diff <= 1.0);
            }
        }
    });
}

#[test]
fn mul_one_identity() {
    proptest!(|(v in any_f4e2m1fn())| {
        if v.is_finite() {
            let result = v * f4e2m1fn::ONE;
            if result.is_finite() {
                let diff = (result.to_f32() - v.to_f32()).abs();
                assert!(diff <= 1.0);
            }
        }
    });
}

#[test]
fn neg_twice_identity() {
    proptest!(|(v in any_f4e2m1fn())| {
        if v.is_finite() {
            let result = -(-v);
            let diff = (result.to_f32() - v.to_f32()).abs();
            assert!(diff <= 1.0);
        }
    });
}

#[test]
fn abs_non_negative() {
    proptest!(|(v in any_f4e2m1fn())| {
        if v.is_finite() {
            let abs = v.abs();
            assert!(!abs.is_sign_negative());
        }
    });
}

#[test]
fn floor_ceil_ordering() {
    proptest!(|(v in any_f4e2m1fn())| {
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
    proptest!(|(v in any_f4e2m1fn())| {
        if !v.is_nan() {
            assert_eq!(v.total_cmp(&v), core::cmp::Ordering::Equal);
        }
    });
}

#[test]
fn one_is_finite() {
    assert!(f4e2m1fn::ONE.is_finite());
    assert!(f4e2m1fn::ONE.is_normal());
}

#[test]
fn display_does_not_panic() {
    proptest!(|(v in any_f4e2m1fn())| {
        let _ = format!("{v}");
    });
}

#[test]
fn compound_assign_eq_binary_add() {
    proptest!(|(a in any_f4e2m1fn(), b in any_f4e2m1fn())| {
        let mut x = a;
        x += b;
        assert_eq!(x, a + b);
    });
}

#[test]
fn compound_assign_eq_binary_sub() {
    proptest!(|(a in any_f4e2m1fn(), b in any_f4e2m1fn())| {
        let mut x = a;
        x -= b;
        assert_eq!(x, a - b);
    });
}

#[test]
fn compound_assign_eq_binary_mul() {
    proptest!(|(a in any_f4e2m1fn(), b in any_f4e2m1fn())| {
        let mut x = a;
        x *= b;
        assert_eq!(x, a * b);
    });
}

#[test]
fn compound_assign_eq_binary_div() {
    proptest!(|(a in any_f4e2m1fn(), b in any_f4e2m1fn())| {
        let mut x = a;
        x /= b;
        assert_eq!(x, a / b);
    });
}

#[test]
fn compound_assign_eq_binary_rem() {
    proptest!(|(a in any_f4e2m1fn(), b in any_f4e2m1fn())| {
        let mut x = a;
        x %= b;
        assert_eq!(x, a % b);
    });
}

#[test]
fn iter_sum() {
    proptest!(|(a in any_f4e2m1fn(), b in any_f4e2m1fn(), c in any_f4e2m1fn())| {
        let values = [a, b, c];
        assert_eq!(values.into_iter().sum::<f4e2m1fn>(), a + b + c);
        assert_eq!(values.iter().sum::<f4e2m1fn>(), a + b + c);
    });
}

#[test]
fn iter_product() {
    proptest!(|(a in any_f4e2m1fn(), b in any_f4e2m1fn(), c in any_f4e2m1fn())| {
        let values = [a, b, c];
        assert_eq!(values.into_iter().product::<f4e2m1fn>(), a * b * c);
        assert_eq!(values.iter().product::<f4e2m1fn>(), a * b * c);
    });
}
