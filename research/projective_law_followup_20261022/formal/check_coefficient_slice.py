#!/usr/bin/env python3
"""Compare a fixed Rust/Lean 2+2 coefficient slice as integer polynomials.

This deliberately small parser rejects syntax outside field add/sub/mul/sq and
the seven declared Lean lets. It is not a Rust interpreter or a Jacobian proof.
"""

from __future__ import annotations

import ast
import hashlib
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

from verify import rust_function_fragment


ROOT = Path(__file__).resolve().parent
RUST = ROOT / "../src/bin/projective_law_gate.rs"
LEAN = ROOT / "RuntimeSemantics.lean"
RUST_NAMES = ("dq", "dr", "ds", "dt", "bcoef", "acoef", "ccoef")
LEAN_NAMES = ("dq", "dr", "ds", "dt", "B", "A", "C")
RUST_TO_LEAN = dict(zip(RUST_NAMES, LEAN_NAMES))
VARIABLES = ("q1", "r1", "s1", "t1", "q2", "r2", "s2", "t2")
SUBSCRIPT = str.maketrans("₁₂", "12")
SOURCE_RENAME = {f"{v[0]}{v[1]}c": v for v in VARIABLES}


def fail(message: str) -> None:
    raise ValueError(message)


def add(left: dict, right: dict, sign: int = 1) -> dict:
    out = defaultdict(int, left)
    for monomial, coefficient in right.items():
        out[monomial] += sign * coefficient
    return {m: c for m, c in out.items() if c}


def mul(left: dict, right: dict) -> dict:
    out = defaultdict(int)
    for a, ca in left.items():
        for b, cb in right.items():
            out[tuple(sorted(a + b))] += ca * cb
    return {m: c for m, c in out.items() if c}


def evaluate(
    node: ast.AST,
    environment: dict[str, dict],
    *,
    rust: bool,
    variables: tuple[str, ...] = VARIABLES,
    rename: dict[str, str] = SOURCE_RENAME,
) -> dict:
    if not rust and isinstance(node, ast.Attribute):
        if isinstance(node.value, ast.Name) and node.value.id == "i" and node.attr in variables:
            return environment.get(node.attr, {(node.attr,): 1})
        fail(f"unsupported Lean projection: {ast.dump(node)}")
    if isinstance(node, ast.Name):
        name = rename.get(node.id, node.id) if rust else node.id
        if name in environment:
            return environment[name]
        if name in variables:
            return {(name,): 1}
        fail(f"unknown variable: {name}")
    if isinstance(node, ast.Constant) and type(node.value) is int:
        return {(): node.value} if node.value else {}
    if isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.USub):
        return mul({(): -1}, evaluate(node.operand, environment, rust=rust, variables=variables, rename=rename))
    if rust and isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute):
        if not isinstance(node.func.value, ast.Name) or node.func.value.id != "f":
            fail("unsupported Rust receiver")
        operation = node.func.attr
        if node.keywords or operation not in {"add", "sub", "mul", "sq"}:
            fail(f"unsupported Rust field call: {operation}")
        required = 1 if operation == "sq" else 2
        if len(node.args) != required:
            fail(f"wrong arity: {operation}")
        values = [evaluate(arg, environment, rust=True, variables=variables, rename=rename) for arg in node.args]
        if operation == "add":
            return add(*values)
        if operation == "sub":
            return add(values[0], values[1], -1)
        return mul(values[0], values[0] if operation == "sq" else values[1])
    if not rust and isinstance(node, ast.BinOp):
        left = evaluate(node.left, environment, rust=False, variables=variables, rename=rename)
        if isinstance(node.op, ast.Pow):
            if not isinstance(node.right, ast.Constant) or type(node.right.value) is not int or node.right.value != 2:
                fail("only square powers are supported")
            return mul(left, left)
        right = evaluate(node.right, environment, rust=False, variables=variables, rename=rename)
        if isinstance(node.op, ast.Add):
            return add(left, right)
        if isinstance(node.op, ast.Sub):
            return add(left, right, -1)
        if isinstance(node.op, ast.Mult):
            return mul(left, right)
    fail(f"unsupported {'Rust' if rust else 'Lean'} expression: {ast.dump(node)}")


def extract_rust(fragment: str, name: str) -> str:
    matches = re.findall(r"(?m)^    let " + re.escape(name) + r" = (.*?);$", fragment, re.S)
    # A non-greedy multiline match may begin at the named declaration; the
    # ending semicolon is unique for this straight-line slice.
    if len(matches) != 1:
        fail(f"expected one Rust let {name}, got {len(matches)}")
    return matches[0].strip()


def extract_lean(source: str, name: str) -> str:
    match = re.findall(r"(?m)^  let " + re.escape(name) + r" := (.+)$", source)
    if len(match) != 1:
        fail(f"expected one Lean let {name}, got {len(match)}")
    return match[0].strip().translate(SUBSCRIPT).replace("^", "**")


def compare(rust_text: str, lean_text: str) -> list[dict]:
    fragment = rust_function_fragment(rust_text.encode(), "add22_weighted_common").decode()
    rust_env: dict[str, dict] = {}
    lean_env: dict[str, dict] = {}
    rows = []
    for rust_name, lean_name in RUST_TO_LEAN.items():
        rust_expr = extract_rust(fragment, rust_name)
        lean_expr = extract_lean(lean_text, lean_name)
        rust_poly = evaluate(ast.parse(rust_expr, mode="eval").body, rust_env, rust=True)
        lean_poly = evaluate(ast.parse(lean_expr, mode="eval").body, lean_env, rust=False)
        if rust_poly != lean_poly:
            fail(f"polynomial mismatch: {rust_name}/{lean_name}")
        rust_env[rust_name] = rust_poly
        lean_env[lean_name] = lean_poly
        rows.append({
            "rust": rust_name,
            "lean": lean_name,
            "monomials": len(rust_poly),
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
    mutated = rust_text.replace(
        "let bcoef = f.sub(f.mul(dq, f.sub(f.mul(q2c, r1c), f.mul(q1c, r2c))), f.sq(dr));",
        "let bcoef = f.add(f.mul(dq, f.sub(f.mul(q2c, r1c), f.mul(q1c, r2c))), f.sq(dr));",
        1,
    )
    if mutated == rust_text:
        fail("mutation target missing")
    try:
        compare(mutated, lean_text)
    except ValueError as error:
        if str(error) != "polynomial mismatch: bcoef/B":
            raise
    else:
        fail("changed Rust subtraction was accepted")
    lean_mutated = lean_text.replace(
        "let B := dq * (i.q₂ * i.r₁ - i.q₁ * i.r₂) - dr ^ 2",
        "let B := dq * (i.q₂ * i.r₁ - i.q₁ * i.r₂) + dr ^ 2",
        1,
    )
    if lean_mutated == lean_text:
        fail("Lean mutation target missing")
    try:
        compare(rust_text, lean_mutated)
    except ValueError as error:
        if str(error) != "polynomial mismatch: bcoef/B":
            raise
    else:
        fail("changed Lean subtraction was accepted")
    certificate = {
        "schema": "projective-law-coefficient-slice/v1",
        "scope": "common 2+2 coefficient slice only; no full Rust semantics or Jacobian group-law proof",
        "rust_source_sha256": hashlib.sha256(RUST.read_bytes()).hexdigest(),
        "lean_source_sha256": hashlib.sha256(LEAN.read_bytes()).hexdigest(),
        "rows": rows,
        "negative_checks": [
            "Rust bcoef subtraction-to-addition rejected",
            "Lean B subtraction-to-addition rejected",
        ],
    }
    encoded = json.dumps(certificate, indent=2) + "\n"
    if sys.argv[1:] == ["--check"]:
        if (ROOT / "coefficient-slice.json").read_text() != encoded:
            fail("coefficient-slice certificate is stale or altered")
    elif not sys.argv[1:]:
        (ROOT / "coefficient-slice.json").write_text(encoded)
    else:
        fail("usage: check_coefficient_slice.py [--check]")
    print(f"{certificate['schema']}: {len(rows)} matched; both mutations rejected")


if __name__ == "__main__":
    main()
