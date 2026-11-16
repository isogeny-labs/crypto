#!/usr/bin/env python3
"""Source-derived polynomial comparison for common 2+2 cross scaling."""

from __future__ import annotations

import ast
import hashlib
import json
import sys
from pathlib import Path

from check_coefficient_slice import RUST_TO_LEAN, evaluate, extract_lean, extract_rust, fail
from verify import rust_function_fragment


ROOT = Path(__file__).resolve().parent
RUST = ROOT / "../src/bin/projective_law_gate.rs"
LEAN = ROOT / "CrossScale22.lean"
LEAN_COEFFICIENTS = ROOT / "RuntimeSemantics.lean"
NAMES = (
    "z1_2", "z2_2", "z1_3", "z2_3", "z1_4", "z2_4", "z1_5", "z2_5",
    "q1c", "q2c", "r1c", "r2c", "s1c", "s2c", "t1c", "t2c",
)
INPUTS = ("q1", "r1", "s1", "t1", "z1", "w1", "q2", "r2", "s2", "t2", "z2", "w2")
OUTPUTS = NAMES[8:]
EXPECTED_WEIGHTS = ((2, 0), (2, 0), (4, 0), (4, 0), (3, 1), (3, 1), (5, 1), (5, 1))


def compare(rust_text: str, lean_text: str) -> list[dict]:
    fragment = rust_function_fragment(rust_text.encode(), "add22_weighted_common").decode()
    rust_env: dict[str, dict] = {}
    lean_env: dict[str, dict] = {}
    rows = []
    for name in NAMES:
        rust_expr = extract_rust(fragment, name)
        lean_expr = extract_lean(lean_text, name)
        rust_poly = evaluate(ast.parse(rust_expr, mode="eval").body, rust_env, rust=True, variables=INPUTS, rename={})
        lean_poly = evaluate(ast.parse(lean_expr, mode="eval").body, lean_env, rust=False, variables=INPUTS, rename={})
        if rust_poly != lean_poly:
            fail(f"cross-scale polynomial mismatch: {name}")
        rust_env[name] = rust_poly
        lean_env[name] = lean_poly
        z_power, w_power = EXPECTED_WEIGHTS[OUTPUTS.index(name)] if name in OUTPUTS else (None, None)
        rows.append({
            "rust": name,
            "lean": name,
            "kind": "cross_input" if name in OUTPUTS else "power_intermediate",
            "z_power": z_power,
            "w_power": w_power,
            "equal": True,
            "terms": [
                {"variables": list(monomial), "coefficient": rust_poly[monomial]}
                for monomial in sorted(rust_poly)
            ],
        })
    coefficient_source = LEAN_COEFFICIENTS.read_text()
    lean_coefficient_env = {name[:-1]: lean_env[name] for name in OUTPUTS}
    for rust_name, lean_name in RUST_TO_LEAN.items():
        rust_expr = extract_rust(fragment, rust_name)
        lean_expr = extract_lean(coefficient_source, lean_name)
        rust_poly = evaluate(ast.parse(rust_expr, mode="eval").body, rust_env, rust=True, variables=INPUTS, rename={})
        lean_poly = evaluate(ast.parse(lean_expr, mode="eval").body, lean_coefficient_env, rust=False, variables=INPUTS, rename={})
        if rust_poly != lean_poly:
            fail(f"composed polynomial mismatch: {rust_name}/{lean_name}")
        rust_env[rust_name] = rust_poly
        lean_coefficient_env[lean_name] = lean_poly
        rows.append({
            "rust": rust_name,
            "lean": lean_name,
            "kind": "composed_coefficient",
            "z_power": None,
            "w_power": None,
            "equal": True,
            "terms": [
                {"variables": list(monomial), "coefficient": rust_poly[monomial]}
                for monomial in sorted(rust_poly)
            ],
        })
    return rows


def main() -> None:
    rust_text = RUST.read_text()
    lean_text = LEAN.read_text()
    rows = compare(rust_text, lean_text)
    rust_old = "let z2_3 = f.mul(z2_2, z2);"
    rust_new = "let z2_3 = f.add(z2_2, z2);"
    if rust_old not in rust_text:
        fail("Rust mutation target missing")
    try:
        compare(rust_text.replace(rust_old, rust_new, 1), lean_text)
    except ValueError as error:
        if str(error) != "cross-scale polynomial mismatch: z2_3":
            raise
    else:
        fail("mutated Rust cross scaling was accepted")
    lean_old = "let z2_3 := z2_2 * i.z2"
    lean_new = "let z2_3 := z2_2 + i.z2"
    if lean_old not in lean_text:
        fail("Lean mutation target missing")
    try:
        compare(rust_text, lean_text.replace(lean_old, lean_new, 1))
    except ValueError as error:
        if str(error) != "cross-scale polynomial mismatch: z2_3":
            raise
    else:
        fail("mutated Lean cross scaling was accepted")
    coefficient_old = "let bcoef = f.sub(f.mul(dq, f.sub(f.mul(q2c, r1c), f.mul(q1c, r2c))), f.sq(dr));"
    coefficient_new = "let bcoef = f.add(f.mul(dq, f.sub(f.mul(q2c, r1c), f.mul(q1c, r2c))), f.sq(dr));"
    if coefficient_old not in rust_text:
        fail("composed-coefficient mutation target missing")
    try:
        compare(rust_text.replace(coefficient_old, coefficient_new, 1), lean_text)
    except ValueError as error:
        if str(error) != "composed polynomial mismatch: bcoef/B":
            raise
    else:
        fail("mutated composed coefficient was accepted")

    certificate = {
        "schema": "projective-law-cross-scale22/v1",
        "scope": "eight power intermediates, eight cross-scaled inputs, seven composed coefficients; no returned Jacobian-class proof",
        "rust_source_sha256": hashlib.sha256(RUST.read_bytes()).hexdigest(),
        "lean_source_sha256": hashlib.sha256(LEAN.read_bytes()).hexdigest(),
        "lean_coefficient_source_sha256": hashlib.sha256(LEAN_COEFFICIENTS.read_bytes()).hexdigest(),
        "rows": rows,
        "negative_checks": [
            "Rust z2_3 mul-to-add rejected",
            "Lean z2_3 mul-to-add rejected",
            "Rust composed B sub-to-add rejected",
        ],
    }
    encoded = json.dumps(certificate, indent=2) + "\n"
    output = ROOT / "cross-scale22.json"
    if sys.argv[1:] == ["--check"]:
        if output.read_text() != encoded:
            fail("cross-scale22 certificate is stale or altered")
    elif not sys.argv[1:]:
        output.write_text(encoded)
    else:
        fail("usage: check_cross_scale22.py [--check]")
    print(f"{certificate['schema']}: {len(rows)} expressions matched; three mutations rejected")


if __name__ == "__main__":
    main()
