#!/usr/bin/env -S uv run --script
#
# /// script
# requires-python = ">=3.12"
# dependencies = ["ml-dtypes==0.5.4"]
# ///
"""Generate Rust fixtures from ml-dtypes.

The checked-in output lets `cargo test` validate compatibility without a
runtime Python dependency.
"""

from __future__ import annotations

import math
from pathlib import Path

import ml_dtypes
import numpy as np


FORMATS = [
    ("f8e3m4", "float8_e3m4"),
    ("f8e4m3", "float8_e4m3"),
    ("f8e4m3b11fnuz", "float8_e4m3b11fnuz"),
    ("f8e4m3fn", "float8_e4m3fn"),
    ("f8e4m3fnuz", "float8_e4m3fnuz"),
    ("f8e5m2", "float8_e5m2"),
    ("f8e5m2fnuz", "float8_e5m2fnuz"),
    ("f8e8m0fnu", "float8_e8m0fnu"),
    ("f4e2m1fn", "float4_e2m1fn"),
    ("f6e2m3fn", "float6_e2m3fn"),
    ("f6e3m2fn", "float6_e3m2fn"),
]


CONVERSION_VALUES = [
    float("nan"),
    float("inf"),
    float("-inf"),
    -0.0,
    0.0,
    -1.0e-8,
    1.0e-8,
    -1.0e-5,
    1.0e-5,
    -(2.0**-20),
    2.0**-20,
    -(2.0**-15),
    2.0**-15,
    -(2.0**-10),
    2.0**-10,
    -1.0,
    1.0,
    -0.03125,
    0.03125,
    -0.5,
    0.5,
    -1.5,
    1.5,
    -15.75,
    15.75,
    -16.0,
    16.0,
    -240.0,
    240.0,
    -248.0,
    248.0,
    -448.0,
    448.0,
    -480.0,
    480.0,
    -57344.0,
    57344.0,
    -1.0e30,
    1.0e30,
    2.0**-128,
    2.0**-127,
    2.0**127,
    3.5e38,
]


ARITHMETIC_VALUES = [
    -8.0,
    -1.5,
    -1.0,
    -0.5,
    -0.0,
    0.0,
    0.5,
    1.0,
    1.5,
    8.0,
    float("inf"),
    float("-inf"),
    float("nan"),
]

METHOD_VALUES = [
    -8.0,
    -3.0,
    -2.0,
    -1.5,
    -1.0,
    -0.75,
    -0.5,
    -0.25,
    -0.0,
    0.0,
    0.25,
    0.5,
    0.75,
    1.0,
    1.5,
    2.0,
    3.0,
    8.0,
    float("inf"),
    float("-inf"),
    float("nan"),
]

UNARY_METHODS = [
    ("neg", "np.negative(value)"),
    ("abs", "np.absolute(value)"),
    ("sign", "np.sign(value)"),
    ("floor", "np.floor(value)"),
    ("ceil", "np.ceil(value)"),
    ("trunc", "np.trunc(value)"),
    ("round_ties_even", "np.rint(value)"),
    ("recip", "np.reciprocal(value)"),
    ("sqrt", "np.sqrt(value)"),
    ("exp", "np.exp(value)"),
    ("exp2", "np.exp2(value)"),
    ("exp_m1", "np.expm1(value)"),
    ("ln", "np.log(value)"),
    ("ln_1p", "np.log1p(value)"),
    ("log2", "np.log2(value)"),
    ("log10", "np.log10(value)"),
    ("cbrt", "np.cbrt(value)"),
    ("sin", "np.sin(value)"),
    ("cos", "np.cos(value)"),
    ("tan", "np.tan(value)"),
    ("asin", "np.arcsin(value)"),
    ("acos", "np.arccos(value)"),
    ("atan", "np.arctan(value)"),
    ("sinh", "np.sinh(value)"),
    ("cosh", "np.cosh(value)"),
    ("tanh", "np.tanh(value)"),
]

BINARY_METHODS = [
    ("copysign", "np.copysign(left, right)"),
    ("min", "np.fmin(left, right)"),
    ("max", "np.fmax(left, right)"),
    ("powf", "np.power(left, right)"),
    ("hypot", "np.hypot(left, right)"),
    ("atan2", "np.arctan2(left, right)"),
]

COMPARISONS = [
    ("lt", "np.less(left, right)"),
    ("le", "np.less_equal(left, right)"),
    ("eq", "np.equal(left, right)"),
    ("ne", "np.not_equal(left, right)"),
    ("ge", "np.greater_equal(left, right)"),
    ("gt", "np.greater(left, right)"),
]


def bits_of(value: float, dtype: type) -> int:
    array = np.array([value], dtype=dtype)
    return int(array.view(np.uint8)[0])


def f32_bits_from_raw(raw: int, dtype: type) -> int:
    array = np.array([raw], dtype=np.uint8).view(dtype)
    value = np.array(array, dtype=np.float32).view(np.uint32)[0]
    return int(value)


def arith_bits(lhs: float, rhs: float, dtype: type, op: str) -> int:
    left = np.array([lhs], dtype=dtype)
    right = np.array([rhs], dtype=dtype)
    with np.errstate(all="ignore"):
        if op == "add":
            result = left + right
        elif op == "sub":
            result = left - right
        elif op == "mul":
            result = left * right
        elif op == "div":
            result = left / right
        elif op == "rem":
            result = left % right
        else:
            raise ValueError(op)
    return int(result.astype(dtype).view(np.uint8)[0])


def unary_bits(input_value: float, dtype: type, expression: str) -> int:
    value = np.array([input_value], dtype=dtype)
    with np.errstate(all="ignore"):
        result = eval(expression, {"np": np}, {"value": value})
    return int(result.astype(dtype).view(np.uint8)[0])


def binary_bits(lhs: float, rhs: float, dtype: type, expression: str) -> int:
    left = np.array([lhs], dtype=dtype)
    right = np.array([rhs], dtype=dtype)
    with np.errstate(all="ignore"):
        result = eval(expression, {"np": np}, {"left": left, "right": right})
    return int(result.astype(dtype).view(np.uint8)[0])


def comparison_result(lhs: float, rhs: float, dtype: type, expression: str) -> str:
    left = np.array([lhs], dtype=dtype)
    right = np.array([rhs], dtype=dtype)
    with np.errstate(all="ignore"):
        result = eval(expression, {"np": np}, {"left": left, "right": right})
    return "true" if bool(result[0]) else "false"


def finite_extreme_bits(dtype: type, fn) -> int:
    raw = np.arange(256, dtype=np.uint8)
    values = raw.view(dtype)
    f32_values = np.array(values, dtype=np.float32)
    finite = np.isfinite(values)
    finite_raw = raw[finite]
    finite_f32 = f32_values[finite]
    return int(finite_raw[fn(finite_f32)])


def rust_float(value: float) -> str:
    with np.errstate(over="ignore"):
        value = np.float32(value).item()
    if math.isnan(value):
        return "f32::NAN"
    if math.isinf(value):
        return "f32::INFINITY" if value > 0 else "f32::NEG_INFINITY"
    if value == 0.0 and math.copysign(1.0, value) < 0:
        return "-0.0"
    return repr(value)


def write_array(name: str, ty: str, values: list[str], indent: str = "") -> str:
    lines = [f"{indent}pub const {name}: [{ty}; {len(values)}] = ["]
    for i in range(0, len(values), 8):
        lines.append(indent + "    " + ", ".join(values[i : i + 8]) + ",")
    lines.append(indent + "];")
    return "\n".join(lines)


def main() -> None:
    fixtures_dir = Path(__file__).resolve().parents[1] / "tests" / "fixtures"
    out = fixtures_dir / "generated.rs"
    format_dir = fixtures_dir / "generated"
    format_dir.mkdir(exist_ok=True)
    lines = [
        "// @generated by scripts/generate_fixtures.py",
        "#[allow(clippy::excessive_precision)]",
        "",
        "pub struct FormatFixture {",
        "    pub rust_type: &'static str,",
        "    pub nan_bits: u8,",
        "    pub infinity_bits: u8,",
        "    pub neg_infinity_bits: u8,",
        "    pub neg_zero_bits: u8,",
        "    pub min_bits: u8,",
        "    pub max_bits: u8,",
        "    pub is_nan: &'static [bool; 256],",
        "    pub is_infinite: &'static [bool; 256],",
        "    pub is_finite: &'static [bool; 256],",
        "    pub is_sign_negative: &'static [bool; 256],",
        "    pub decode_f32_bits: &'static [u32; 256],",
        "    pub conversions: &'static [(f32, u8)],",
        "    pub arithmetic: &'static [(f32, f32, u8, u8, u8, u8, u8)],",
        "    pub unary_methods: &'static [UnaryMethodsFixture],",
        "    pub binary_methods: &'static [BinaryMethodsFixture],",
        "    pub comparisons: &'static [ComparisonFixture],",
        "}",
        "",
        "#[derive(Clone, Copy)]",
        "pub struct UnaryMethodsFixture {",
        "    pub input: f32,",
        *[f"    pub {name}: u8," for name, _ in UNARY_METHODS],
        "}",
        "",
        "#[derive(Clone, Copy)]",
        "pub struct BinaryMethodsFixture {",
        "    pub lhs: f32,",
        "    pub rhs: f32,",
        *[f"    pub {name}: u8," for name, _ in BINARY_METHODS],
        "}",
        "",
        "#[derive(Clone, Copy)]",
        "pub struct ComparisonFixture {",
        "    pub lhs: f32,",
        "    pub rhs: f32,",
        *[f"    pub {name}: bool," for name, _ in COMPARISONS],
        "}",
        "",
    ]

    fixture_names = []
    format_files = []
    for rust_name, ml_name in FORMATS:
        dtype = getattr(ml_dtypes, ml_name)
        mod = ml_name.upper().replace("_", "")
        format_file = f"{ml_name}.rs"
        format_files.append(format_dir / format_file)
        decode = [f"0x{f32_bits_from_raw(raw, dtype):08x}" for raw in range(256)]
        values = np.arange(256, dtype=np.uint8).view(dtype)
        is_nan = ["true" if value else "false" for value in np.isnan(values)]
        is_infinite = ["true" if value else "false" for value in np.isinf(values)]
        is_finite = ["true" if value else "false" for value in np.isfinite(values)]
        is_sign_negative = ["true" if value else "false" for value in np.signbit(values)]
        conversions = [
            f"({rust_float(value)}, 0x{bits_of(value, dtype):02x})"
            for value in CONVERSION_VALUES
        ]
        arithmetic = []
        for lhs in ARITHMETIC_VALUES:
            for rhs in ARITHMETIC_VALUES:
                arithmetic.append(
                    "({}, {}, 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x}, 0x{:02x})".format(
                        rust_float(lhs),
                        rust_float(rhs),
                        arith_bits(lhs, rhs, dtype, "add"),
                        arith_bits(lhs, rhs, dtype, "sub"),
                        arith_bits(lhs, rhs, dtype, "mul"),
                        arith_bits(lhs, rhs, dtype, "div"),
                        arith_bits(lhs, rhs, dtype, "rem"),
                    )
                )
        unary_methods = []
        for input_value in METHOD_VALUES:
            fields = [
                f"{name}: 0x{unary_bits(input_value, dtype, expression):02x}"
                for name, expression in UNARY_METHODS
            ]
            unary_methods.append(
                "UnaryMethodsFixture {{ input: {}, {} }}".format(
                    rust_float(input_value), ", ".join(fields)
                )
            )
        binary_methods = []
        for lhs in METHOD_VALUES:
            for rhs in METHOD_VALUES:
                fields = [
                    f"{name}: 0x{binary_bits(lhs, rhs, dtype, expression):02x}"
                    for name, expression in BINARY_METHODS
                ]
                binary_methods.append(
                    "BinaryMethodsFixture {{ lhs: {}, rhs: {}, {} }}".format(
                        rust_float(lhs), rust_float(rhs), ", ".join(fields)
                    )
                )
        comparisons = []
        for lhs in METHOD_VALUES:
            for rhs in METHOD_VALUES:
                fields = [
                    f"{name}: {comparison_result(lhs, rhs, dtype, expression)}"
                    for name, expression in COMPARISONS
                ]
                comparisons.append(
                    "ComparisonFixture {{ lhs: {}, rhs: {}, {} }}".format(
                        rust_float(lhs), rust_float(rhs), ", ".join(fields)
                    )
                )
        fixture_name = f"{mod}_FIXTURE"
        fixture_names.append(fixture_name)
        format_lines = [
            "// @generated by scripts/generate_fixtures.py",
            "#[allow(clippy::excessive_precision)]",
            "",
            "use super::{BinaryMethodsFixture, ComparisonFixture, FormatFixture, UnaryMethodsFixture};",
            "",
            write_array("IS_NAN", "bool", is_nan),
            "",
            write_array("IS_INFINITE", "bool", is_infinite),
            "",
            write_array("IS_FINITE", "bool", is_finite),
            "",
            write_array("IS_SIGN_NEGATIVE", "bool", is_sign_negative),
            "",
            write_array("DECODE_F32_BITS", "u32", decode),
            "",
            write_array("CONVERSIONS", "(f32, u8)", conversions),
            "",
            write_array(
                "ARITHMETIC", "(f32, f32, u8, u8, u8, u8, u8)", arithmetic
            ),
            "",
            write_array("UNARY_METHODS", "UnaryMethodsFixture", unary_methods),
            "",
            write_array("BINARY_METHODS", "BinaryMethodsFixture", binary_methods),
            "",
            write_array("COMPARISONS", "ComparisonFixture", comparisons),
            "",
            "pub const FIXTURE: FormatFixture = FormatFixture {",
            f'    rust_type: "{rust_name}",',
            f"    nan_bits: 0x{bits_of(float('nan'), dtype):02x},",
            f"    infinity_bits: 0x{bits_of(float('inf'), dtype):02x},",
            f"    neg_infinity_bits: 0x{bits_of(float('-inf'), dtype):02x},",
            f"    neg_zero_bits: 0x{bits_of(-0.0, dtype):02x},",
            f"    min_bits: 0x{finite_extreme_bits(dtype, np.argmin):02x},",
            f"    max_bits: 0x{finite_extreme_bits(dtype, np.argmax):02x},",
            "    is_nan: &IS_NAN,",
            "    is_infinite: &IS_INFINITE,",
            "    is_finite: &IS_FINITE,",
            "    is_sign_negative: &IS_SIGN_NEGATIVE,",
            "    decode_f32_bits: &DECODE_F32_BITS,",
            "    conversions: &CONVERSIONS,",
            "    arithmetic: &ARITHMETIC,",
            "    unary_methods: &UNARY_METHODS,",
            "    binary_methods: &BINARY_METHODS,",
            "    comparisons: &COMPARISONS,",
            "};",
            "",
        ]
        (format_dir / format_file).write_text("\n".join(format_lines))
        lines.extend(
            [
                f'#[path = "generated/{format_file}"]',
                f"mod {mod};",
                f"pub use {mod}::FIXTURE as {fixture_name};",
                "",
            ]
        )

    lines.append(f"pub const ALL_FIXTURES: [&FormatFixture; {len(fixture_names)}] = [")
    lines.extend(f"    &{name}," for name in fixture_names)
    lines.append("];")
    lines.append("")
    for stale_file in format_dir.glob("*.rs"):
        if stale_file not in format_files:
            stale_file.unlink()
    out.write_text("\n".join(lines))


if __name__ == "__main__":
    main()
