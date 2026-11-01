#!/usr/bin/env python3
"""Bind the common-2+2 residual head theorem to unit-weight Rust formulas."""

from __future__ import annotations

import ast
import hashlib
import json
import re
import sys
from pathlib import Path

from check_coefficient_slice import evaluate, mul, fail
from check_common22_residual import SOURCE_NAMES
from verify import rust_function_fragment
from check_coefficient_slice import extract_rust


ROOT = Path(__file__).resolve().parent
RUST = ROOT / "../src/bin/projective_law_gate.rs"
LEAN = ROOT / "Common22SupportHead.lean"
AFFINE = ("q1", "r1", "s1", "t1", "q2", "r2", "s2", "t2")
WEIGHTS = ("z1", "w1", "z2", "w2")


def block(source: str, name: str, end_marker: str) -> str:
    begin = source.index(f"def {name} ")
    return source[begin:source.index(end_marker, begin)]


def lean_let(source: str, name: str) -> str:
    match = re.search(r"(?m)^  let " + re.escape(name) + r" := (.*)$", source)
    if match is None:
        fail(f"missing Lean let {name}")
    next_line = source.find("\n  let ", match.end())
    tuple_line = source.find("\n  (", match.end())
    limits = [position for position in (next_line, tuple_line) if position >= 0]
    end = min(limits) if limits else len(source)
    return " ".join((match.group(1) + source[match.end():end]).split()).replace("^", "**")


def polynomial(expression: str, environment: dict[str, dict], *, rust: bool) -> dict:
    return evaluate(ast.parse(expression, mode="eval").body, environment, rust=rust,
                    variables=AFFINE + WEIGHTS, rename={})


def terms(value: dict) -> list[dict]:
    return [
        {"variables": list(monomial), "coefficient": value[monomial]}
        for monomial in sorted(value)
    ]


def compare(rust_text: str, lean_text: str) -> list[dict]:
    fragment = rust_function_fragment(rust_text.encode(), "add22_weighted_common").decode()
    source_env = {name: {(): 1} for name in WEIGHTS}
    for name in SOURCE_NAMES:
        source_env[name] = polynomial(extract_rust(fragment, name), source_env, rust=True)

    data = block(lean_text, "coefficientData", "/-- The x^6")
    lean_env: dict[str, dict] = {}
    for name in ("dq", "dr", "ds", "dt", "B", "A", "C", "Q", "Rbar"):
        lean_env[name] = polynomial(lean_let(data, name), lean_env, rust=False)
    source_names = {"B": "bcoef", "A": "acoef", "C": "ccoef", "Q": "q"}
    rows = []
    for name in ("B", "A", "C", "Q"):
        actual = source_env[source_names[name]]
        expected = lean_env[name]
        if actual != expected:
            fail(f"support-head source mismatch: {name}")
        rows.append({"obligation": name, "equal": True, "monomials": len(terms(actual)), "terms": terms(actual)})
    if source_env["r"] != mul(source_env["c2"], lean_env["Rbar"]):
        fail("support-head source mismatch: Rbar")
    rows.append({"obligation": "Rbar_via_r_factor", "equal": True,
                 "monomials": len(terms(lean_env["Rbar"])), "terms": terms(lean_env["Rbar"])})

    head = block(lean_text, "headCoefficients", "/-- Exact high-degree")
    for name in ("n3", "n2", "n1", "e6", "e5", "e4"):
        lean_env[name] = polynomial(lean_let(head, name), lean_env, rust=False)
    expected_heads = {"e6": {}, "e5": mul(lean_env["B"], lean_env["B"]), "e4": {}}
    for name in ("e6", "e5", "e4"):
        actual = lean_env[name]
        if actual != expected_heads[name]:
            fail(f"support-head polynomial mismatch: {name}")
        rows.append({"obligation": name, "equal": True, "monomials": len(terms(actual)),
                     "terms": terms(actual)})
    return rows


def main() -> None:
    rust_text = RUST.read_text()
    lean_text = LEAN.read_text()
    rows = compare(rust_text, lean_text)
    old = "let q = f.sub(f.add(d, ac), bw2);"
    if old not in rust_text:
        fail("Rust Q mutation target missing")
    try:
        compare(rust_text.replace(old, "let q = f.add(f.add(d, ac), bw2);", 1), lean_text)
    except ValueError as error:
        if str(error) != "support-head source mismatch: Q":
            raise
    else:
        fail("mutated Rust Q was accepted")
    old_lean = "let Q := dq * C ^ 2 + 2 * A * C - B ^ 2"
    if old_lean not in lean_text:
        fail("Lean Q mutation target missing")
    try:
        compare(rust_text, lean_text.replace(old_lean,
                "let Q := dq * C ^ 2 + 2 * A * C + B ^ 2", 1))
    except ValueError as error:
        if str(error) != "support-head source mismatch: Q":
            raise
    else:
        fail("mutated Lean Q was accepted")
    certificate = {
        "schema": "projective-law-common22-support-head/v1",
        "scope": "unit-weight common 2+2 residual x^6,x^5,x^4 coefficients only",
        "rust_source_sha256": hashlib.sha256(RUST.read_bytes()).hexdigest(),
        "lean_source_sha256": hashlib.sha256(LEAN.read_bytes()).hexdigest(),
        "rows": rows,
        "negative_checks": ["Rust Q sign mutation rejected", "Lean Q sign mutation rejected"],
    }
    encoded = json.dumps(certificate, indent=2) + "\n"
    output = ROOT / "common22-support-head.json"
    if sys.argv[1:] == ["--check"]:
        if output.read_text() != encoded:
            fail("common22-support-head certificate is stale or altered")
    elif not sys.argv[1:]:
        output.write_text(encoded)
    else:
        fail("usage: check_common22_support_head.py [--check]")
    print(f"{certificate['schema']}: {len(rows)} identities checked; two mutations rejected")


if __name__ == "__main__":
    main()
