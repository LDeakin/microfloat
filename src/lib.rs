#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # 8-bit and sub-byte floating point types for Rust
//!
//! This crate implements microfloat types for Rust, including common 8-bit
//! formats and sub-byte 4-bit and 6-bit formats. Microfloats are a subset of
//! [`minifloat`](https://en.wikipedia.org/wiki/Minifloat) formats.
//!
//! 8-bit floating point representations:
//! - [`f8e3m4`] - signed E3M4, bias 3, IEEE-like NaN/Inf.
//! - [`f8e4m3`] - signed E4M3, bias 7, IEEE-like NaN/Inf.
//! - [`f8e4m3b11fnuz`] - signed E4M3, bias 11, finite-only, unsigned zero.
//! - [`f8e4m3fn`] - signed E4M3, bias 7, finite-only, signed outer NaNs.
//! - [`f8e4m3fnuz`] - signed E4M3, bias 8, finite-only, unsigned zero.
//! - [`f8e5m2`] - signed E5M2, bias 15, IEEE-like NaN/Inf.
//! - [`f8e5m2fnuz`] - signed E5M2, bias 16, finite-only, unsigned zero.
//! - [`f8e8m0fnu`] - unsigned E8M0 scale, bias 127, no zero, single NaN.
//!
//! Microscaling (MX) sub-byte floating point representations:
//! - [`f4e2m1fn`] - signed 4-bit E2M1, bias 1, finite-only, saturating.
//! - [`f6e2m3fn`] - signed 6-bit E2M3, bias 1, finite-only, saturating.
//! - [`f6e3m2fn`] - signed 6-bit E3M2, bias 3, finite-only, saturating.
//!
//! In type suffixes,
//! - `f` means finite-only with no infinities,
//! - `n` means the format has a special NaN encoding,
//! - `uz` means unsigned zero with no distinct negative zero encoding, and
//! - `u` means unsigned.
//!
//! This crate is modeled to be compatible with the microfloat types in the
//! [`ml-dtypes`](https://pypi.org/project/ml-dtypes/) Python package.
//! For broader minifloat types such as `f16` and `bf16`, use the
//! [`half`](https://crates.io/crates/half) crate; `microfloat` is heavily inspired by
//! `half`.
//!
//! ## Usage
//!
//! The float types attempt to match existing Rust floating point type functionality where
//! possible, and provide conversion operations, classification, formatting, parsing,
//! arithmetic operations, and common math operations. Calculations are performed through
//! `f32` and rounded back to the target format.
//!
//! ```
//! use microfloat::f8e4m3;
//!
//! let x = f8e4m3::from_f32(1.5);
//! let y = f8e4m3::from_f32(2.0);
//! let z = x + y;
//!
//! assert_eq!(z.to_f32(), 3.5);
//! ```
//!
//! This crate provides [`no_std`](https://rust-embedded.github.io/book/intro/no-std.html)
//! support.
//!
//! *Requires Rust 1.85 or greater.*
//!
//! ## Optional Features
//!
//! - **`serde`** - Implement `Serialize` and `Deserialize` traits for the float
//!   types. This adds a dependency on the [`serde`](https://crates.io/crates/serde)
//!   crate.
//!
//! - **`num-traits`** - Enable `ToPrimitive`, `FromPrimitive`, `Num`, `NumCast`,
//!   `FloatCore`, `Signed`, `Bounded`, `Zero`, and `One` trait implementations from
//!   the [`num-traits`](https://crates.io/crates/num-traits) crate.
//!
//! - **`bytemuck`** - Enable `Zeroable` and `Pod` trait implementations from the
//!   [`bytemuck`](https://crates.io/crates/bytemuck) crate.
//!
//! - **`rand_distr`** - Enable sampling from distributions like `StandardUniform`
//!   and `StandardNormal` from the [`rand_distr`](https://crates.io/crates/rand_distr)
//!   crate.
//!
//! - **`rkyv`** - Enable zero-copy serialization support with the
//!   [`rkyv`](https://crates.io/crates/rkyv) crate.
//!
//! ## Testing
//!
//! Compatibility with `ml-dtypes` is tested by generated fixtures in `tests/fixtures/`.
//! These fixtures validate conversions, classifications, arithmetic, and math methods.
//!
//! ## Related Crates
//!
//! ### `float8`
//!
//! The [`float8`](https://crates.io/crates/float8) crate provides `F8E4M3` and `F8E5M2` types that are not fully OCP compliant.
//! They use NVIDIA's `__NV_SATFINITE` saturation mode ([`cuda_fp8.hpp`](https://gitlab.com/nvidia/headers/cuda-individual/cudart/-/raw/main/cuda_fp8.hpp)).
//! In this mode `INFINITY` constants are `FP8_MAXNORM` overflow sentinels rather than true infinities.
//! In contrast, microfloat uses `__NV_NOSAT` semantics (IEEE NaN/Inf on overflow).

mod bits;
mod format;
#[macro_use]
mod formats;
mod micro;
#[cfg(feature = "num-traits")]
mod num_traits_impl;
mod ops;
#[cfg(feature = "rand_distr")]
mod rand_distr;
#[cfg(feature = "serde")]
#[macro_use]
mod serde;

use crate::format::{Format, NanEncoding, Overflow, SignMode, ZeroMode};
use crate::micro::MicroFloat;

define_format!(
    /// Signed 8-bit E3M4 floating point type with bias 3 and IEEE-like NaN/Inf.
    pub struct f8e3m4, Float8E3M4Format {
        storage: 8,
        exponent: 3,
        mantissa: 4,
        digits: 5,
        bias: 3,
        sign: Signed,
        zero: Signed,
        nan: NanEncoding::Ieee,
        overflow: Infinity,
        bits: {
            neg_zero: 0x80,
            one: 0x30,
            neg_one: 0xb0,
            min_positive_subnormal: 0x01,
            max_subnormal: 0x0f,
            min_positive: 0x10,
            min: 0xef,
            max: 0x6f,
            epsilon: 0x31,
            infinity: 0x70,
            neg_infinity: 0xf0,
            nan: 0x78
        }
    }
);

define_format!(
    /// Signed 8-bit E4M3 floating point type with bias 7 and IEEE-like NaN/Inf.
    pub struct f8e4m3, Float8E4M3Format {
        storage: 8,
        exponent: 4,
        mantissa: 3,
        digits: 4,
        bias: 7,
        sign: Signed,
        zero: Signed,
        nan: NanEncoding::Ieee,
        overflow: Infinity,
        bits: {
            neg_zero: 0x80,
            one: 0x38,
            neg_one: 0xb8,
            min_positive_subnormal: 0x01,
            max_subnormal: 0x07,
            min_positive: 0x08,
            min: 0xf7,
            max: 0x77,
            epsilon: 0x39,
            infinity: 0x78,
            neg_infinity: 0xf8,
            nan: 0x7c
        }
    }
);

define_format!(
    /// Signed 8-bit E4M3 finite-only type with bias 11, unsigned zero, and a single NaN.
    pub struct f8e4m3b11fnuz, Float8E4M3B11FnuzFormat {
        storage: 8,
        exponent: 4,
        mantissa: 3,
        digits: 4,
        bias: 11,
        sign: Signed,
        zero: Unsigned,
        nan: NanEncoding::Single(0x80),
        overflow: Nan,
        bits: {
            neg_zero: 0x00,
            one: 0x58,
            neg_one: 0xd8,
            min_positive_subnormal: 0x01,
            max_subnormal: 0x07,
            min_positive: 0x08,
            min: 0xff,
            max: 0x7f,
            epsilon: 0x59,
            infinity: 0x80,
            neg_infinity: 0x80,
            nan: 0x80
        }
    }
);

define_format!(
    /// Signed 8-bit E4M3 finite-only type with bias 7 and signed outer NaNs.
    pub struct f8e4m3fn, Float8E4M3FnFormat {
        storage: 8,
        exponent: 4,
        mantissa: 3,
        digits: 4,
        bias: 7,
        sign: Signed,
        zero: Signed,
        nan: NanEncoding::Outer,
        overflow: Nan,
        bits: {
            neg_zero: 0x80,
            one: 0x38,
            neg_one: 0xb8,
            min_positive_subnormal: 0x01,
            max_subnormal: 0x07,
            min_positive: 0x08,
            min: 0xfe,
            max: 0x7e,
            epsilon: 0x39,
            infinity: 0x7f,
            neg_infinity: 0xff,
            nan: 0x7f
        }
    }
);

define_format!(
    /// Signed 8-bit E4M3 finite-only type with bias 8, unsigned zero, and a single NaN.
    pub struct f8e4m3fnuz, Float8E4M3FnuzFormat {
        storage: 8,
        exponent: 4,
        mantissa: 3,
        digits: 4,
        bias: 8,
        sign: Signed,
        zero: Unsigned,
        nan: NanEncoding::Single(0x80),
        overflow: Nan,
        bits: {
            neg_zero: 0x00,
            one: 0x40,
            neg_one: 0xc0,
            min_positive_subnormal: 0x01,
            max_subnormal: 0x07,
            min_positive: 0x08,
            min: 0xff,
            max: 0x7f,
            epsilon: 0x41,
            infinity: 0x80,
            neg_infinity: 0x80,
            nan: 0x80
        }
    }
);

define_format!(
    /// Signed 8-bit E5M2 floating point type with bias 15 and IEEE-like NaN/Inf.
    pub struct f8e5m2, Float8E5M2Format {
        storage: 8,
        exponent: 5,
        mantissa: 2,
        digits: 3,
        bias: 15,
        sign: Signed,
        zero: Signed,
        nan: NanEncoding::Ieee,
        overflow: Infinity,
        bits: {
            neg_zero: 0x80,
            one: 0x3c,
            neg_one: 0xbc,
            min_positive_subnormal: 0x01,
            max_subnormal: 0x03,
            min_positive: 0x04,
            min: 0xfb,
            max: 0x7b,
            epsilon: 0x3d,
            infinity: 0x7c,
            neg_infinity: 0xfc,
            nan: 0x7e
        }
    }
);

define_format!(
    /// Signed 8-bit E5M2 finite-only type with bias 16, unsigned zero, and a single NaN.
    pub struct f8e5m2fnuz, Float8E5M2FnuzFormat {
        storage: 8,
        exponent: 5,
        mantissa: 2,
        digits: 3,
        bias: 16,
        sign: Signed,
        zero: Unsigned,
        nan: NanEncoding::Single(0x80),
        overflow: Nan,
        bits: {
            neg_zero: 0x00,
            one: 0x40,
            neg_one: 0xc0,
            min_positive_subnormal: 0x01,
            max_subnormal: 0x03,
            min_positive: 0x04,
            min: 0xff,
            max: 0x7f,
            epsilon: 0x41,
            infinity: 0x80,
            neg_infinity: 0x80,
            nan: 0x80
        }
    }
);

define_format!(
    /// Unsigned 8-bit E8M0 MX scale format with bias 127, no zero, and a single NaN.
    pub struct f8e8m0fnu, Float8E8M0FnuFormat {
        storage: 8,
        exponent: 8,
        mantissa: 0,
        digits: 1,
        bias: 127,
        sign: Unsigned,
        zero: None,
        nan: NanEncoding::Single(0xff),
        overflow: Nan,
        bits: {
            neg_zero: 0xff,
            one: 0x7f,
            neg_one: 0xff,
            min_positive_subnormal: 0x00,
            max_subnormal: 0x00,
            min_positive: 0x00,
            min: 0x00,
            max: 0xfe,
            epsilon: 0x7f,
            infinity: 0xff,
            neg_infinity: 0xff,
            nan: 0xff
        }
    }
);

define_format!(
    /// Signed 4-bit E2M1 MX finite-only type with bias 1, stored in a byte.
    pub struct f4e2m1fn, Float4E2M1FnFormat {
        storage: 4,
        exponent: 2,
        mantissa: 1,
        digits: 2,
        bias: 1,
        sign: Signed,
        zero: Signed,
        nan: NanEncoding::None,
        overflow: Saturate,
        bits: {
            neg_zero: 0x08,
            one: 0x02,
            neg_one: 0x0a,
            min_positive_subnormal: 0x01,
            max_subnormal: 0x01,
            min_positive: 0x02,
            min: 0x0f,
            max: 0x07,
            epsilon: 0x03,
            infinity: 0x07,
            neg_infinity: 0x0f,
            nan: 0x08
        }
    }
);

define_format!(
    /// Signed 6-bit E2M3 MX finite-only type with bias 1, stored in a byte.
    pub struct f6e2m3fn, Float6E2M3FnFormat {
        storage: 6,
        exponent: 2,
        mantissa: 3,
        digits: 4,
        bias: 1,
        sign: Signed,
        zero: Signed,
        nan: NanEncoding::None,
        overflow: Saturate,
        bits: {
            neg_zero: 0x20,
            one: 0x08,
            neg_one: 0x28,
            min_positive_subnormal: 0x01,
            max_subnormal: 0x07,
            min_positive: 0x08,
            min: 0x3f,
            max: 0x1f,
            epsilon: 0x09,
            infinity: 0x1f,
            neg_infinity: 0x3f,
            nan: 0x20
        }
    }
);

define_format!(
    /// Signed 6-bit E3M2 MX finite-only type with bias 3, stored in a byte.
    pub struct f6e3m2fn, Float6E3M2FnFormat {
        storage: 6,
        exponent: 3,
        mantissa: 2,
        digits: 3,
        bias: 3,
        sign: Signed,
        zero: Signed,
        nan: NanEncoding::None,
        overflow: Saturate,
        bits: {
            neg_zero: 0x20,
            one: 0x0c,
            neg_one: 0x2c,
            min_positive_subnormal: 0x01,
            max_subnormal: 0x03,
            min_positive: 0x04,
            min: 0x3f,
            max: 0x1f,
            epsilon: 0x0d,
            infinity: 0x1f,
            neg_infinity: 0x3f,
            nan: 0x20
        }
    }
);

#[cfg(feature = "serde")]
serde_impls!(
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
