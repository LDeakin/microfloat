#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NanEncoding {
    None,
    Ieee,
    Outer,
    Single(u8),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Overflow {
    Infinity,
    Nan,
    Saturate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignMode {
    Signed,
    Unsigned,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZeroMode {
    Signed,
    Unsigned,
    None,
}

pub trait Format: Copy + 'static {
    const NAME: &'static str;
    const STORAGE_BITS: u8;
    const EXPONENT_BITS: u8;
    const MANTISSA_BITS: u8;
    const EXPONENT_BIAS: i32;
    const SIGN: SignMode;
    const ZERO: ZeroMode;
    const NAN: NanEncoding;
    const OVERFLOW: Overflow;

    const ZERO_BITS: u8 = 0;
    const NEG_ZERO_BITS: u8 = Self::SIGN_BIT;
    const SIGN_BIT: u8 = match Self::SIGN {
        SignMode::Signed => 1 << (Self::STORAGE_BITS - 1),
        SignMode::Unsigned => 0,
    };
    const STORAGE_MASK: u8 = if Self::STORAGE_BITS == 8 {
        0xff
    } else {
        (1 << Self::STORAGE_BITS) - 1
    };
    const MANTISSA_MASK: u8 = if Self::MANTISSA_BITS == 0 {
        0
    } else {
        (1 << Self::MANTISSA_BITS) - 1
    };
    const EXPONENT_MASK: u8 =
        ((((1u16 << Self::EXPONENT_BITS) - 1) << Self::MANTISSA_BITS) & 0xff) as u8;
    #[expect(
        clippy::cast_possible_truncation,
        reason = "format exponent fields fit in eight storage bits"
    )]
    const MAX_EXPONENT_FIELD: u8 = ((1u16 << Self::EXPONENT_BITS) - 1) as u8;
}
