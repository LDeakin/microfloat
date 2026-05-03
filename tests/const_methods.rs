use microfloat::{
    f4e2m1fn, f6e2m3fn, f6e3m2fn, f8e3m4, f8e4m3, f8e4m3b11fnuz, f8e4m3fn, f8e4m3fnuz, f8e5m2,
    f8e5m2fnuz, f8e8m0fnu,
};

macro_rules! assert_const_methods_compile {
    ($($type:ty),* $(,)?) => {
        $(
            const _: () = {
                let value = <$type>::from_bits(0x01);
                let sign = <$type>::NEG_ONE;
                let bits = value.to_bits();
                let _ = <$type>::from_le_bytes([bits]);
                let _ = <$type>::from_be_bytes([bits]);
                let _ = <$type>::from_ne_bytes([bits]);
                let _ = value.to_le_bytes();
                let _ = value.to_be_bytes();
                let _ = value.to_ne_bytes();
                let _ = <$type>::has_inf();
                let _ = <$type>::has_nan();
                let _ = <$type>::is_finite_only();
                let _ = value.is_nan();
                let _ = value.is_infinite();
                let _ = value.is_finite();
                let _ = value.is_normal();
                let _ = value.classify();
                let _ = value.is_sign_positive();
                let _ = value.is_sign_negative();
                let _ = value.copysign(sign);
                let _ = value.signum();
                let _ = value.abs();
                let _ = value.next_up();
                let _ = value.next_down();
            };
        )*
    };
}

assert_const_methods_compile!(
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
fn const_methods_compile() {}
