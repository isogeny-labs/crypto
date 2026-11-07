#!/usr/bin/env python3
"""Check Rust common-2+2 output expressions against the Lean residual theorem."""

from __future__ import annotations

import ast
import csv
import hashlib
import io
import json
import re
import sys
from pathlib import Path

from check_coefficient_slice import evaluate, extract_rust, fail
from verify import rust_function_fragment


ROOT = Path(__file__).resolve().parent
RUST = ROOT / "../src/bin/projective_law_gate.rs"
LEAN = ROOT / "Common22Residual.lean"
VARIABLES = ("q1", "r1", "s1", "t1", "z1", "w1", "q2", "r2", "s2", "t2", "z2", "w2")
SOURCE_NAMES = (
    "z1_2", "z2_2", "z1_3", "z2_3", "z1_4", "z2_4", "z1_5", "z2_5",
    "q1c", "q2c", "r1c", "r2c", "s1c", "s2c", "t1c", "t2c",
    "dq", "dr", "ds", "dt", "bcoef", "acoef", "ccoef",
    "c2", "c3", "c4", "c5", "ac", "bw", "bw2", "d", "q", "r",
    "qdelta", "rdelta", "s", "t",
)


def polynomial(expression: str, environment: dict[str, dict], *, rust: bool) -> dict:
    expression = expression.replace("^", "**") if not rust else expression
    return evaluate(ast.parse(expression, mode="eval").body, environment, rust=rust,
                    variables=VARIABLES, rename={})


def lean_body(source: str, name: str, end_marker: str) -> tuple[list[tuple[str, str]], str]:
    begin = source.index(f"def {name} ")
    end = source.index(end_marker, begin)
    lines = [line.strip() for line in source[begin:end].splitlines() if line.strip()]
    lets = []
    for line in lines:
        match = re.fullmatch(r"let ([A-Za-z_][A-Za-z0-9_]*) := (.+)", line)
        if match:
            lets.append((match.group(1), match.group(2)))
    if [name for name, _ in lets] != ["r", "qdelta", "rdelta"]:
        fail(f"unexpected Lean local bindings in {name}")
    return lets, lines[-1]


def canonical_terms(value: dict) -> list[dict]:
    return [
        {"variables": list(monomial), "coefficient": value[monomial]}
        for monomial in sorted(value)
    ]


def compare(rust_text: str, lean_text: str) -> list[dict]:
    fragment = rust_function_fragment(rust_text.encode(), "add22_weighted_common").decode()
    return_pattern = (
        r"Some\(ResultPoint::DegreeTwo\(\[\s*q,\s*r,\s*s,\s*t,\s*"
        r"f\.mul\(f\.mul\(z1, z2\), ccoef\),\s*bw,\s*\]\)\)"
    )
    if re.search(return_pattern, fragment) is None:
        fail("common-branch return tuple no longer matches q,r,s,t,z1*z2*C,bw")
    environment: dict[str, dict] = {}
    for name in SOURCE_NAMES:
        environment[name] = polynomial(extract_rust(fragment, name), environment, rust=True)

    B, A, C = (environment[name] for name in ("bcoef", "acoef", "ccoef"))
    lean_environment = {
        "q1": environment["q1c"], "r1": environment["r1c"],
        "s1": environment["s1c"], "t1": environment["t1c"],
        "B": B, "A": A, "C": C, "Q": environment["q"],
    }
    rbar = polynomial(
        "acoef*(dq*ccoef+acoef) - (s1c+s2c)*bcoef*ccoef + (q1c+q2c)*bw2",
        environment, rust=False,
    )
    lean_environment["Rbar"] = rbar
    expected_r = polynomial("c2 * Rbar", {**environment, "Rbar": rbar}, rust=False)
    checks = [("r_has_C_squared_factor", environment["r"], expected_r)]

    for lean_name, source_name, end_marker in (
        ("outputS", "s", "def outputT "),
        ("outputT", "t", "\n/--"),
    ):
        locals_, body = lean_body(lean_text, lean_name, end_marker)
        local_environment = lean_environment.copy()
        for name, expression in locals_:
            local_environment[name] = polynomial(expression, local_environment, rust=False)
            if name in ("qdelta", "rdelta") and lean_name == "outputS":
                checks.append((f"{name}_matches_source", environment[name], local_environment[name]))
        checks.append((f"{source_name}_matches_lean", environment[source_name],
                       polynomial(body, local_environment, rust=False)))

    rows = []
    for label, actual, expected in checks:
        if actual != expected:
            fail(f"common-branch output mismatch: {label}")
        terms = canonical_terms(actual)
        rows.append({
            "obligation": label,
            "equal": True,
            "monomials": len(terms),
            "polynomial_sha256": hashlib.sha256(json.dumps(terms, separators=(",", ":")).encode()).hexdigest(),
            "terms": terms,
        })
    return rows


def main() -> None:
    rust_text = RUST.read_text()
    lean_text = LEAN.read_text()
    rows = compare(rust_text, lean_text)
    mutated_rust = rust_text.replace("let s = f.sub(\n", "let s = f.add(\n", 1)
    if mutated_rust == rust_text:
        fail("Rust output mutation target missing")
    try:
        compare(mutated_rust, lean_text)
    except ValueError as error:
        if str(error) != "common-branch output mismatch: s_matches_lean":
            raise
    else:
        fail("mutated Rust output was accepted")
    mutated_lean = lean_text.replace(
        "rdelta + (A * C - Q) * qdelta - s1 * B * C ^ 3",
        "rdelta + (A * C - Q) * qdelta + s1 * B * C ^ 3",
        1,
    )
    if mutated_lean == lean_text:
        fail("Lean output mutation target missing")
    try:
        compare(rust_text, mutated_lean)
    except ValueError as error:
        if str(error) != "common-branch output mismatch: s_matches_lean":
            raise
    else:
        fail("mutated Lean output was accepted")
    buffer = io.StringIO(newline="")
    writer = csv.writer(buffer, lineterminator="\n")
    writer.writerow(("obligation", "coefficient", "variables"))
    term_count = 0
    for row in rows:
        for term in row["terms"]:
            writer.writerow((row["obligation"], term["coefficient"], "*".join(term["variables"])))
            term_count += 1
    term_bytes = buffer.getvalue().encode()
    certificate = {
        "schema": "projective-law-common22-residual/v1",
        "scope": "common 2+2 returned-coordinate congruence only; no full divisor-class or exceptional-branch theorem",
        "rust_source_sha256": hashlib.sha256(RUST.read_bytes()).hexdigest(),
        "lean_source_sha256": hashlib.sha256(LEAN.read_bytes()).hexdigest(),
        "rows": [{key: value for key, value in row.items() if key != "terms"} for row in rows],
        "raw_terms": {
            "path": "common22-residual-terms.csv",
            "sha256": hashlib.sha256(term_bytes).hexdigest(),
            "bytes": len(term_bytes),
            "rows": term_count,
        },
        "return_tuple": "q,r,s,t,(z1*z2)*ccoef,bw",
        "negative_checks": ["Rust output-s subtraction mutation rejected", "Lean outputS subtraction mutation rejected"],
    }
    encoded = json.dumps(certificate, indent=2) + "\n"
    output = ROOT / "common22-residual.json"
    raw_output = ROOT / certificate["raw_terms"]["path"]
    if sys.argv[1:] == ["--check"]:
        if output.read_text() != encoded:
            fail("common22-residual certificate is stale or altered")
        manifest = json.loads((ROOT / ".artifacts.json").read_text())
        registered = manifest["files"][raw_output.name]
        if registered != {"sha256": certificate["raw_terms"]["sha256"],
                          "bytes": certificate["raw_terms"]["bytes"]}:
            fail("common22-residual artifact manifest is stale or altered")
        if raw_output.exists() and raw_output.read_bytes() != term_bytes:
            fail("common22-residual raw terms are stale or altered")
    elif not sys.argv[1:]:
        output.write_text(encoded)
        raw_output.write_bytes(term_bytes)
    else:
        fail("usage: check_common22_residual.py [--check]")
    print(f"{certificate['schema']}: {len(rows)} output obligations matched; two mutations rejected")


if __name__ == "__main__":
    main()
