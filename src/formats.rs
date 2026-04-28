macro_rules! doc_bits {
    ($description:literal, $bits:expr) => {
        concat!($description, "\n\nRaw bits: `", stringify!($bits), "`.")
    };
}

macro_rules! doc_zero {
    (Signed, $bits:expr) => {
        doc_bits!("Positive zero (`+0.0`).", $bits)
    };
    (Unsigned, $bits:expr) => {
        doc_bits!("Unsigned zero.", $bits)
    };
    (None, $bits:expr) => {
        doc_bits!(
            "This format has no zero; this is the smallest positive finite scale value.",
            $bits
        )
    };
}

macro_rules! doc_neg_zero {
    (Signed, $bits:expr) => {
        doc_bits!("Negative zero (`-0.0`).", $bits)
    };
    (Unsigned, $bits:expr) => {
        doc_bits!("This format has unsigned zero; this aliases `ZERO`.", $bits)
    };
    (None, $bits:expr) => {
        doc_bits!("This format has no zero; this aliases `NAN`.", $bits)
    };
}

macro_rules! doc_neg_one {
    (Signed, $bits:expr) => {
        doc_bits!("Negative one (`-1.0`).", $bits)
    };
    (Unsigned, $bits:expr) => {
        doc_bits!("This format is unsigned; this aliases `NAN`.", $bits)
    };
}

macro_rules! doc_subnormal {
    (None, $description:literal, $bits:expr) => {
        concat!(
            "This format has no subnormal values; ",
            $description,
            "\n\nRaw bits: `",
            stringify!($bits),
            "`."
        )
    };
    ($zero:ident, $description:literal, $bits:expr) => {
        doc_bits!($description, $bits)
    };
}

macro_rules! doc_min {
    (Unsigned, $bits:expr) => {
        doc_bits!("Minimum finite value.", $bits)
    };
    (Signed, $bits:expr) => {
        doc_bits!("Most negative finite value.", $bits)
    };
}

macro_rules! doc_infinity {
    (Infinity, $bits:expr) => {
        doc_bits!("Positive infinity.", $bits)
    };
    (Nan, $bits:expr) => {
        doc_bits!(
            "This format has no infinity; converting positive infinity or overflowing in the positive direction produces this NaN value.",
            $bits
        )
    };
    (Saturate, $bits:expr) => {
        doc_bits!(
            "This format has no infinity; converting positive infinity or overflowing in the positive direction saturates to `MAX`.",
            $bits
        )
    };
}

macro_rules! doc_neg_infinity {
    (Infinity, $bits:expr) => {
        doc_bits!("Negative infinity.", $bits)
    };
    (Nan, $bits:expr) => {
        doc_bits!(
            "This format has no infinity; converting negative infinity or overflowing in the negative direction produces this NaN value.",
            $bits
        )
    };
    (Saturate, $bits:expr) => {
        doc_bits!(
            "This format has no infinity; converting negative infinity or overflowing in the negative direction saturates to `MIN`.",
            $bits
        )
    };
}

macro_rules! doc_nan {
    (NanEncoding::None, $bits:expr) => {
        doc_bits!(
            "This format has no NaN values; this aliases `NEG_ZERO`.",
            $bits
        )
    };
    ($nan:expr, $bits:expr) => {
        doc_bits!("Canonical NaN value.", $bits)
    };
}

macro_rules! define_format {
    (
        $(#[$meta:meta])*
        $vis:vis struct $type:ident, $format:ident {
            storage: $storage:expr,
            exponent: $exponent:expr,
            mantissa: $mantissa:expr,
            digits: $digits:expr,
            bias: $bias:expr,
            sign: $sign:ident,
            zero: $zero:ident,
            nan: $nan:expr,
            overflow: $overflow:ident,
            bits: {
                neg_zero: $neg_zero_bits:expr,
                one: $one_bits:expr,
                neg_one: $neg_one_bits:expr,
                min_positive_subnormal: $min_positive_subnormal_bits:expr,
                max_subnormal: $max_subnormal_bits:expr,
                min_positive: $min_positive_bits:expr,
                min: $min_bits:expr,
                max: $max_bits:expr,
                epsilon: $epsilon_bits:expr,
                infinity: $infinity_bits:expr,
                neg_infinity: $neg_infinity_bits:expr,
                nan: $nan_bits:expr
            }
        }
    ) => {
        #[derive(Clone, Copy, Debug)]
        #[allow(clippy::redundant_pub_crate)]
        pub(crate) struct $format;

        impl Format for $format {
            const NAME: &'static str = stringify!($type);
            const STORAGE_BITS: u8 = $storage;
            const EXPONENT_BITS: u8 = $exponent;
            const MANTISSA_BITS: u8 = $mantissa;
            const EXPONENT_BIAS: i32 = $bias;
            const SIGN: SignMode = SignMode::$sign;
            const ZERO: ZeroMode = ZeroMode::$zero;
            const NAN: NanEncoding = $nan;
            const OVERFLOW: Overflow = Overflow::$overflow;
        }

        $(#[$meta])*
        #[derive(Clone, Copy)]
        #[allow(non_camel_case_types)]
        #[must_use]
        #[repr(transparent)]
        pub struct $type(MicroFloat<$format>);

        impl $type {
            #[doc = "Radix of the floating point representation."]
            pub const RADIX: u32 = 2;

            #[doc = "Number of storage bits used by this format."]
            pub const STORAGE_BITS: u32 = $storage;

            #[doc = "Number of exponent bits in this format."]
            pub const EXPONENT_BITS: u32 = $exponent;

            #[doc = "Number of stored mantissa bits in this format."]
            pub const MANTISSA_BITS: u32 = $mantissa;

            #[doc = "Number of significant base-2 digits in this format."]
            pub const DIGITS: u32 = $digits;

            #[doc = "Number of significant base-2 mantissa digits in this format."]
            pub const MANTISSA_DIGITS: u32 = $digits;

            #[doc = "Exponent bias used by this format."]
            pub const EXPONENT_BIAS: i32 = $bias;

            #[doc = doc_zero!($zero, 0x00)]
            pub const ZERO: Self = Self::from_bits(0x00);

            #[doc = doc_neg_zero!($zero, $neg_zero_bits)]
            pub const NEG_ZERO: Self = Self::from_bits($neg_zero_bits);

            #[doc = doc_bits!("One (`1.0`).", $one_bits)]
            pub const ONE: Self = Self::from_bits($one_bits);

            #[doc = doc_neg_one!($sign, $neg_one_bits)]
            pub const NEG_ONE: Self = Self::from_bits($neg_one_bits);

            #[doc = doc_subnormal!(
                $zero,
                "Minimum positive subnormal value.",
                $min_positive_subnormal_bits
            )]
            pub const MIN_POSITIVE_SUBNORMAL: Self =
                Self::from_bits($min_positive_subnormal_bits);

            #[doc = doc_subnormal!($zero, "Maximum subnormal value.", $max_subnormal_bits)]
            pub const MAX_SUBNORMAL: Self = Self::from_bits($max_subnormal_bits);

            #[doc = doc_subnormal!($zero, "Minimum positive normal value.", $min_positive_bits)]
            pub const MIN_POSITIVE: Self = Self::from_bits($min_positive_bits);

            #[doc = doc_min!($sign, $min_bits)]
            pub const MIN: Self = Self::from_bits($min_bits);

            #[doc = doc_bits!("Maximum finite value.", $max_bits)]
            pub const MAX: Self = Self::from_bits($max_bits);

            #[doc = doc_bits!("Difference between `1.0` and the next larger representable value.", $epsilon_bits)]
            pub const EPSILON: Self = Self::from_bits($epsilon_bits);

            #[doc = doc_infinity!($overflow, $infinity_bits)]
            pub const INFINITY: Self = Self::from_bits($infinity_bits);

            #[doc = doc_neg_infinity!($overflow, $neg_infinity_bits)]
            pub const NEG_INFINITY: Self = Self::from_bits($neg_infinity_bits);

            #[doc = doc_nan!($nan, $nan_bits)]
            pub const NAN: Self = Self::from_bits($nan_bits);

            /// Creates a value from its raw storage bits.
            ///
            /// All eight bits are preserved in the byte. For sub-byte formats,
            /// byte values outside the canonical storage range are decoded compatibly
            /// with `ml-dtypes` raw-byte views and may alias canonical values.
            pub const fn from_bits(bits: u8) -> Self {
                Self(MicroFloat::<$format>::from_bits(bits))
            }

            /// Returns the raw storage bits of this value.
            pub const fn to_bits(self) -> u8 {
                self.0.to_bits()
            }

            /// Creates a value from its little-endian byte representation.
            pub const fn from_le_bytes(bytes: [u8; 1]) -> Self {
                Self(MicroFloat::<$format>::from_le_bytes(bytes))
            }

            /// Creates a value from its big-endian byte representation.
            pub const fn from_be_bytes(bytes: [u8; 1]) -> Self {
                Self(MicroFloat::<$format>::from_be_bytes(bytes))
            }

            /// Creates a value from its native-endian byte representation.
            pub const fn from_ne_bytes(bytes: [u8; 1]) -> Self {
                Self(MicroFloat::<$format>::from_ne_bytes(bytes))
            }

            /// Returns the little-endian byte representation of this value.
            pub const fn to_le_bytes(self) -> [u8; 1] {
                self.0.to_le_bytes()
            }

            /// Returns the big-endian byte representation of this value.
            pub const fn to_be_bytes(self) -> [u8; 1] {
                self.0.to_be_bytes()
            }

            /// Returns the native-endian byte representation of this value.
            pub const fn to_ne_bytes(self) -> [u8; 1] {
                self.0.to_ne_bytes()
            }

            /// Returns `true` if this format can represent infinity.
            pub const fn has_inf() -> bool {
                <$format as crate::format::Format>::HAS_INF
            }

            /// Returns `true` if this format can represent NaN values.
            pub const fn has_nan() -> bool {
                <$format as crate::format::Format>::HAS_NAN
            }

            /// Returns `true` if this format saturates on overflow (finite-only).
            pub const fn is_finite_only() -> bool {
                <$format as crate::format::Format>::HAS_FINITE_ONLY
            }

            /// Converts an `f32` to this format.
            ///
            /// The result is rounded to the nearest representable value. Values outside the
            /// finite range follow this format's overflow behavior.
            pub fn from_f32(value: f32) -> Self {
                Self(MicroFloat::<$format>::from_f32(value))
            }

            /// Converts an `f64` to this format.
            ///
            /// The value is first rounded to `f32`, then converted to this format.
            pub fn from_f64(value: f64) -> Self {
                Self(MicroFloat::<$format>::from_f64(value))
            }

            /// Converts this value to `f32`.
            pub fn to_f32(self) -> f32 {
                self.0.to_f32()
            }

            /// Converts this value to `f64`.
            pub fn to_f64(self) -> f64 {
                self.0.to_f64()
            }

            /// Returns `true` if this value is NaN.
            pub fn is_nan(self) -> bool {
                self.0.is_nan()
            }

            /// Returns `true` if this value is positive or negative infinity.
            pub fn is_infinite(self) -> bool {
                self.0.is_infinite()
            }

            /// Returns `true` if this value is neither infinite nor NaN.
            pub fn is_finite(self) -> bool {
                self.0.is_finite()
            }

            /// Returns `true` if this value is finite, nonzero, and not subnormal.
            pub fn is_normal(self) -> bool {
                self.0.is_normal()
            }

            /// Returns the floating point category of this value.
            pub fn classify(self) -> core::num::FpCategory {
                self.0.classify()
            }

            /// Returns `true` if this value has a positive sign.
            ///
            /// Unsigned formats always return `true`.
            pub const fn is_sign_positive(self) -> bool {
                self.0.is_sign_positive()
            }

            /// Returns `true` if this value has a negative sign.
            ///
            /// Unsigned formats always return `false`.
            pub const fn is_sign_negative(self) -> bool {
                self.0.is_sign_negative()
            }

            /// Returns a value with the magnitude of `self` and the sign of `sign`.
            pub fn copysign(self, sign: Self) -> Self {
                Self(self.0.copysign(sign.0))
            }

            /// Returns a number representing the sign of `self`.
            ///
            /// NaN and zero values are returned unchanged.
            pub fn signum(self) -> Self {
                Self(self.0.signum())
            }

            /// Returns the absolute value of `self`.
            pub fn abs(self) -> Self {
                Self(self.0.abs())
            }

            /// Returns the greatest integer less than or equal to `self`, rounded to this format.
            pub fn floor(self) -> Self {
                Self(self.0.floor())
            }

            /// Returns the smallest integer greater than or equal to `self`, rounded to this format.
            pub fn ceil(self) -> Self {
                Self(self.0.ceil())
            }

            /// Returns the integer part of `self`, rounded toward zero.
            pub fn trunc(self) -> Self {
                Self(self.0.trunc())
            }

            /// Returns `self` rounded to the nearest integer, with halfway cases rounded to even.
            pub fn round_ties_even(self) -> Self {
                Self(self.0.round_ties_even())
            }

            /// Returns the reciprocal `1 / self`, rounded to this format.
            pub fn recip(self) -> Self {
                Self(self.0.recip())
            }

            /// Raises `self` to the floating point power `n`, rounded to this format.
            pub fn powf(self, n: Self) -> Self {
                Self(self.0.powf(n.0))
            }

            /// Returns the square root of `self`, rounded to this format.
            pub fn sqrt(self) -> Self {
                Self(self.0.sqrt())
            }

            /// Returns `e^(self)`, rounded to this format.
            pub fn exp(self) -> Self {
                Self(self.0.exp())
            }

            /// Returns `2^(self)`, rounded to this format.
            pub fn exp2(self) -> Self {
                Self(self.0.exp2())
            }

            /// Returns `e^(self) - 1`, rounded to this format.
            pub fn exp_m1(self) -> Self {
                Self(self.0.exp_m1())
            }

            /// Returns the natural logarithm of `self`, rounded to this format.
            pub fn ln(self) -> Self {
                Self(self.0.ln())
            }

            /// Returns `ln(1 + self)`, rounded to this format.
            pub fn ln_1p(self) -> Self {
                Self(self.0.ln_1p())
            }

            /// Returns the base-2 logarithm of `self`, rounded to this format.
            pub fn log2(self) -> Self {
                Self(self.0.log2())
            }

            /// Returns the base-10 logarithm of `self`, rounded to this format.
            pub fn log10(self) -> Self {
                Self(self.0.log10())
            }

            /// Returns the cube root of `self`, rounded to this format.
            pub fn cbrt(self) -> Self {
                Self(self.0.cbrt())
            }

            /// Returns `sqrt(self^2 + other^2)`, rounded to this format.
            pub fn hypot(self, other: Self) -> Self {
                Self(self.0.hypot(other.0))
            }

            /// Computes the sine of `self` in radians, rounded to this format.
            pub fn sin(self) -> Self {
                Self(self.0.sin())
            }

            /// Computes the cosine of `self` in radians, rounded to this format.
            pub fn cos(self) -> Self {
                Self(self.0.cos())
            }

            /// Computes the tangent of `self` in radians, rounded to this format.
            pub fn tan(self) -> Self {
                Self(self.0.tan())
            }

            /// Computes the arcsine of `self`, in radians, rounded to this format.
            pub fn asin(self) -> Self {
                Self(self.0.asin())
            }

            /// Computes the arccosine of `self`, in radians, rounded to this format.
            pub fn acos(self) -> Self {
                Self(self.0.acos())
            }

            /// Computes the arctangent of `self`, in radians, rounded to this format.
            pub fn atan(self) -> Self {
                Self(self.0.atan())
            }

            /// Computes the four-quadrant arctangent of `self` and `other`, in radians.
            ///
            /// This is equivalent to `f32::atan2(self.to_f32(), other.to_f32())`,
            /// rounded to this format.
            pub fn atan2(self, other: Self) -> Self {
                Self(self.0.atan2(other.0))
            }

            /// Computes the hyperbolic sine of `self`, rounded to this format.
            pub fn sinh(self) -> Self {
                Self(self.0.sinh())
            }

            /// Computes the hyperbolic cosine of `self`, rounded to this format.
            pub fn cosh(self) -> Self {
                Self(self.0.cosh())
            }

            /// Computes the hyperbolic tangent of `self`, rounded to this format.
            pub fn tanh(self) -> Self {
                Self(self.0.tanh())
            }

            /// Returns the minimum of `self` and `other`, ignoring NaN when possible.
            ///
            /// If exactly one argument is NaN, the other argument is returned.
            pub fn min(self, other: Self) -> Self {
                Self(self.0.min(other.0))
            }

            /// Returns the maximum of `self` and `other`, ignoring NaN when possible.
            ///
            /// If exactly one argument is NaN, the other argument is returned.
            pub fn max(self, other: Self) -> Self {
                Self(self.0.max(other.0))
            }

            /// Restricts `self` to the interval `[min, max]`.
            ///
            /// # Panics
            ///
            /// Panics if `min` or `max` is `NaN`, or if `min > max`.
            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self(self.0.clamp(min.0, max.0))
            }

            /// Returns a total ordering over all bit patterns in this format.
            ///
            /// The ordering distinguishes signed zeros and orders NaN values consistently.
            pub fn total_cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.0.total_cmp(&other.0)
            }

            /// Alternate serde adapter for serializing as an `f32`.
            ///
            /// # Errors
            ///
            /// Returns an error if the serializer cannot serialize an `f32`.
            #[cfg(feature = "serde")]
            pub fn serialize_as_f32<S: ::serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                serializer.serialize_f32(self.to_f32())
            }

            /// Alternate serde adapter for serializing as a string.
            ///
            /// # Errors
            ///
            /// Returns an error if the serializer cannot collect this value as a string.
            #[cfg(feature = "serde")]
            pub fn serialize_as_string<S: ::serde::Serializer>(
                &self,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                serializer.collect_str(self)
            }
        }

        #[cfg(feature = "bytemuck")]
        unsafe impl bytemuck::Zeroable for $type {}

        #[cfg(feature = "bytemuck")]
        unsafe impl bytemuck::Pod for $type {}

        #[cfg(feature = "rkyv")]
        unsafe impl rkyv::Portable for $type {}

        #[cfg(feature = "rkyv")]
        unsafe impl rkyv::traits::NoUndef for $type {}

        #[cfg(feature = "rkyv")]
        impl rkyv::Archive for $type {
            type Archived = Self;
            type Resolver = ();

            fn resolve(&self, _: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                out.write(*self);
            }
        }

        #[cfg(feature = "rkyv")]
        impl<S> rkyv::Serialize<S> for $type
        where
            S: rkyv::rancor::Fallible + ?Sized,
        {
            fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
                Ok(())
            }
        }

        #[cfg(feature = "rkyv")]
        impl<D> rkyv::Deserialize<$type, D> for $type
        where
            D: rkyv::rancor::Fallible + ?Sized,
        {
            fn deserialize(&self, _: &mut D) -> Result<$type, D::Error> {
                Ok(*self)
            }
        }

        impl Default for $type {
            fn default() -> Self {
                Self::ZERO
            }
        }

        impl core::fmt::Debug for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }

        impl core::fmt::LowerExp for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::LowerExp::fmt(&self.0, f)
            }
        }

        impl core::fmt::UpperExp for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::UpperExp::fmt(&self.0, f)
            }
        }

        impl core::str::FromStr for $type {
            type Err = <f32 as core::str::FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <MicroFloat<$format> as core::str::FromStr>::from_str(s).map(Self)
            }
        }

        impl PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl core::ops::Neg for $type {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl core::ops::Add for $type {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl core::ops::AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl core::ops::Sub for $type {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl core::ops::SubAssign for $type {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl core::ops::Mul for $type {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.0 * rhs.0)
            }
        }

        impl core::ops::MulAssign for $type {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
            }
        }

        impl core::ops::Div for $type {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self(self.0 / rhs.0)
            }
        }

        impl core::ops::DivAssign for $type {
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0;
            }
        }

        impl core::ops::Rem for $type {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self(self.0 % rhs.0)
            }
        }

        impl core::ops::RemAssign for $type {
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0;
            }
        }

        impl core::iter::Sum for $type {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, core::ops::Add::add)
            }
        }

        impl<'a> core::iter::Sum<&'a Self> for $type {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.copied().sum()
            }
        }

        impl core::iter::Product for $type {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, core::ops::Mul::mul)
            }
        }

        impl<'a> core::iter::Product<&'a Self> for $type {
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.copied().product()
            }
        }
    };
}
