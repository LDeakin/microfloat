# 8-bit and sub-byte floating point types for Rust
[![Crates.io](https://img.shields.io/crates/v/microfloat.svg)](https://crates.io/crates/microfloat/) [![Documentation](https://docs.rs/microfloat/badge.svg)](https://docs.rs/microfloat/) ![Crates.io](https://img.shields.io/crates/l/microfloat)

This crate implements microfloat types for Rust, including common 8-bit
formats and sub-byte 4-bit and 6-bit formats. Microfloats are a subset of
[`minifloat`](https://en.wikipedia.org/wiki/Minifloat) formats.

8-bit floating point representations:
- [`f8e3m4`](https://docs.rs/microfloat/latest/microfloat/struct.f8e3m4.html) - signed E3M4, bias 3, IEEE-like NaN/Inf.
- [`f8e4m3`](https://docs.rs/microfloat/latest/microfloat/struct.f8e4m3.html) - signed E4M3, bias 7, IEEE-like NaN/Inf.
- [`f8e4m3b11fnuz`](https://docs.rs/microfloat/latest/microfloat/struct.f8e4m3b11fnuz.html) - signed E4M3, bias 11, finite-only, unsigned zero.
- [`f8e4m3fn`](https://docs.rs/microfloat/latest/microfloat/struct.f8e4m3fn.html) - signed E4M3, bias 7, finite-only, signed outer NaNs.
- [`f8e4m3fnuz`](https://docs.rs/microfloat/latest/microfloat/struct.f8e4m3fnuz.html) - signed E4M3, bias 8, finite-only, unsigned zero.
- [`f8e5m2`](https://docs.rs/microfloat/latest/microfloat/struct.f8e5m2.html) - signed E5M2, bias 15, IEEE-like NaN/Inf.
- [`f8e5m2fnuz`](https://docs.rs/microfloat/latest/microfloat/struct.f8e5m2fnuz.html) - signed E5M2, bias 16, finite-only, unsigned zero.
- [`f8e8m0fnu`](https://docs.rs/microfloat/latest/microfloat/struct.f8e8m0fnu.html) - unsigned E8M0 scale, bias 127, no zero, single NaN.

Microscaling (MX) sub-byte floating point representations: 
- [`f4e2m1fn`](https://docs.rs/microfloat/latest/microfloat/struct.f4e2m1fn.html) - signed 4-bit E2M1, bias 1, finite-only, saturating.
- [`f6e2m3fn`](https://docs.rs/microfloat/latest/microfloat/struct.f6e2m3fn.html) - signed 6-bit E2M3, bias 1, finite-only, saturating.
- [`f6e3m2fn`](https://docs.rs/microfloat/latest/microfloat/struct.f6e3m2fn.html) - signed 6-bit E3M2, bias 3, finite-only, saturating.

In type suffixes,
- `f` means finite-only with no infinities,
- `n` means the format has a special NaN encoding,
- `uz` means unsigned zero with no distinct negative zero encoding, and
- `u` means unsigned.

This crate is modeled to be compatible with the microfloat types in the [`ml-dtypes`](https://pypi.org/project/ml-dtypes/) Python package.
For broader minifloat types such as `f16` and `bf16`, use the [`half`](https://crates.io/crates/half) crate; `microfloat` is heavily inspired by `half`.

## Usage

The float types attempt to match existing Rust floating point type functionality where possible, and provide conversion operations, classification, formatting, parsing, arithmetic operations, and common math operations.
Calculations are performed through `f32` and rounded back to the target format.

```rust
use microfloat::f8e4m3;

let x = f8e4m3::from_f32(1.5);
let y = f8e4m3::from_f32(2.0);
let z = x + y;

assert_eq!(z.to_f32(), 3.5);
```

This crate provides [`no_std`](https://rust-embedded.github.io/book/intro/no-std.html) support.

*Requires Rust 1.85 or greater.*

See the [crate documentation](https://docs.rs/microfloat/) for more details.

## Related Crates

### `float8`

The [`float8`](https://crates.io/crates/float8) crate provides `F8E4M3` and `F8E5M2` types that are not fully OCP compliant.
They use NVIDIA's `__NV_SATFINITE` saturation mode ([`cuda_fp8.hpp`](https://gitlab.com/nvidia/headers/cuda-individual/cudart/-/raw/main/cuda_fp8.hpp)).
In this mode `INFINITY` constants are `FP8_MAXNORM` overflow sentinels rather than true infinities.
In contrast, microfloat uses `__NV_NOSAT` semantics (IEEE NaN/Inf on overflow).

### Optional Features

- **`serde`** - Implement `Serialize` and `Deserialize` traits for the float
  types. This adds a dependency on the [`serde`](https://crates.io/crates/serde)
  crate.

- **`num-traits`** - Enable `ToPrimitive`, `FromPrimitive`, `Num`, `NumCast`,
  `FloatCore`, `Signed`, `Bounded`, `Zero`, and `One` trait implementations from
  the [`num-traits`](https://crates.io/crates/num-traits) crate.

- **`bytemuck`** - Enable `Zeroable` and `Pod` trait implementations from the
  [`bytemuck`](https://crates.io/crates/bytemuck) crate.

- **`rand_distr`** - Enable sampling from distributions like `StandardUniform`
  and `StandardNormal` from the [`rand_distr`](https://crates.io/crates/rand_distr)
  crate.

- **`rkyv`** - Enable zero-copy serialization support with the
  [`rkyv`](https://crates.io/crates/rkyv) crate.

## Testing

Compatibility with `ml-dtypes` is tested by generated fixtures in [`tests/fixtures/`](tests/fixtures/).
These fixtures validate conversions, classifications, arithmetic, and math methods.

## License

All files in this library are dual-licensed and distributed under the terms of
either of:

- [MIT License](LICENCE-MIT)
  ([http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- [Apache License, Version 2.0](LICENCE-APACHE)
  ([http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
