#!/usr/bin/env python3
"""Fail-closed integrity and negative checks for the N9 formal-boundary packet."""

from __future__ import annotations

import copy
import csv
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import NoReturn


ROOT = Path(__file__).resolve().parent
RUNTIME_SCHEMA = "projective-law-runtime-fragment-bindings/v1"
RUNTIME_SOURCE = "../src/bin/projective_law_gate.rs"
BINDING_KIND = "manual_abstraction_hash_bound"
SEMANTIC_STATUS = "not_established"
EXPECTED_RUNTIME_BINDINGS = {
    "add12_weighted_complete": frozenset(
        {
            "ProjectiveLawRuntime.classify12",
            "ProjectiveLawRuntime.classify12_ne_mismatch_of_valid",
        }
    ),
    "add12_weighted_generic": frozenset(
        {
            "ProjectiveLawRuntime.generic12Runtime",
            "ProjectiveLawRuntime.generic12Runtime_cleared",
        }
    ),
    "add22_weighted_common": frozenset(
        {
            "ProjectiveLawRuntime.generic22Runtime",
            "ProjectiveLawRuntime.generic22Runtime_mod_first",
            "ProjectiveLawRuntime.generic22Runtime_mod_second",
        }
    ),
    "add22_weighted": frozenset(
        {"ProjectiveLawBoundary.runtime_certificate_mapping_total"}
    ),
}
EXPECTED_PROVED = frozenset(
    {
        "ProjectiveLawBoundary.equal_or_opposite_of_sq_eq_sq",
        "ProjectiveLawBoundary.same_u_zero_slope_delta_is_inverse",
        "ProjectiveLawBoundary.shared_zero_q_delta_unreachable",
        "ProjectiveLawBoundary.generic12_output_v_clearing",
        "ProjectiveLawBoundary.generic22_interpolates_first",
        "ProjectiveLawBoundary.generic22_interpolates_second",
        "ProjectiveLawBoundary.runtime_certificate_mapping_total",
        "ProjectiveLawBoundary.formula_to_divisor_class",
        "ProjectiveLawRuntime.classify12_ne_mismatch_of_valid",
        "ProjectiveLawRuntime.generic12Runtime_cleared",
        "ProjectiveLawRuntime.generic22Runtime_mod_first",
        "ProjectiveLawRuntime.generic22Runtime_mod_second",
        "ProjectiveLawRuntime.residual_represents_input",
    }
)
EXPECTED_ROW_KEYS = frozenset(
    {
        "rust_function",
        "source_path",
        "fragment_sha256",
        "fragment_bytes",
        "lean_identifiers",
        "binding_kind",
        "semantic_equivalence",
    }
)


def fail(message: str) -> NoReturn:
    raise RuntimeError(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def digest(relative: str) -> str:
    return hashlib.sha256((ROOT / relative).read_bytes()).hexdigest()


def rust_function_fragment(source: bytes, name: str) -> bytes:
    """Return one exact brace-balanced Rust function, including both braces."""
    matches = list(
        re.finditer(
            rb"(?m)^(?:pub\(crate\) )?fn " + re.escape(name.encode()) + rb"\b",
            source,
        )
    )
    require(len(matches) == 1, f"expected one Rust declaration for {name}, got {len(matches)}")
    match = matches[0]
    opening = source.find(b"{", match.start())
    require(opening >= 0, f"missing opening brace for {name}")
    depth = 0
    mode = "code"
    index = opening
    while index < len(source):
        byte = source[index : index + 1]
        pair = source[index : index + 2]
        if mode == "code":
            if pair == b"//":
                mode = "line"
                index += 2
                continue
            if pair == b"/*":
                mode = "block"
                index += 2
                continue
            if byte == b'"':
                mode = "string"
                index += 1
                continue
            if byte == b"'":
                mode = "char"
                index += 1
                continue
            if byte == b"{":
                depth += 1
            elif byte == b"}":
                depth -= 1
                if depth == 0:
                    return source[match.start() : index + 1]
        elif mode == "line":
            if byte in (b"\n", b"\r"):
                mode = "code"
        elif mode == "block":
            if pair == b"*/":
                mode = "code"
                index += 2
                continue
        else:
            if byte == b"\\":
                index += 2
                continue
            if (mode == "string" and byte == b'"') or (mode == "char" and byte == b"'"):
                mode = "code"
        index += 1
    fail(f"unterminated Rust function: {name}")


def lean_code_without_comments_or_strings(source: str) -> str:
    """Preserve line boundaries while removing nested comments and strings."""
    output: list[str] = []
    index = 0
    block_depth = 0
    mode = "code"
    while index < len(source):
        pair = source[index : index + 2]
        char = source[index]
        if block_depth:
            if pair == "/-":
                block_depth += 1
                output.extend("  ")
                index += 2
                continue
            if pair == "-/":
                block_depth -= 1
                output.extend("  ")
                index += 2
                continue
            output.append("\n" if char == "\n" else " ")
            index += 1
            continue
        if mode == "line":
            output.append("\n" if char == "\n" else " ")
            if char == "\n":
                mode = "code"
            index += 1
            continue
        if mode == "string":
            if char == "\\" and index + 1 < len(source):
                output.extend("  ")
                index += 2
                continue
            output.append("\n" if char == "\n" else " ")
            if char == '"':
                mode = "code"
            index += 1
            continue
        if pair == "/-":
            block_depth = 1
            output.extend("  ")
            index += 2
        elif pair == "--":
            mode = "line"
            output.extend("  ")
            index += 2
        elif char == '"':
            mode = "string"
            output.append(" ")
            index += 1
        else:
            output.append(char)
            index += 1
    require(block_depth == 0 and mode != "string", "unterminated Lean comment or string")
    return "".join(output)


def lean_declarations(source: str) -> set[str]:
    """Extract fully-qualified top-level declarations from comment-free Lean."""
    code = lean_code_without_comments_or_strings(source)
    namespaces: list[str] = []
    declarations: set[str] = set()
    declaration = re.compile(
        r"^\s*(?:(?:private|protected|noncomputable)\s+)*(?:def|theorem|structure|inductive)\s+([A-Za-z_][A-Za-z0-9_']*)\b"
    )
    for line in code.splitlines():
        namespace_match = re.match(r"^\s*namespace\s+([A-Za-z_][A-Za-z0-9_.']*)\s*$", line)
        if namespace_match:
            namespaces.extend(namespace_match.group(1).split("."))
            continue
        end_match = re.match(r"^\s*end\s+([A-Za-z_][A-Za-z0-9_.']*)\s*$", line)
        if end_match and namespaces and end_match.group(1) == namespaces[-1]:
            namespaces.pop()
            continue
        declaration_match = declaration.match(line)
        if declaration_match:
            name = ".".join([*namespaces, declaration_match.group(1)])
            require(name not in declarations, f"duplicate Lean declaration parsed: {name}")
            declarations.add(name)
    return declarations


def validate_runtime_bindings(
    document: dict, runtime_bytes: bytes, declarations: set[str]
) -> None:
    require(
        set(document) == {"schema", "extraction", "rows", "scope_limit"},
        "unexpected runtime-bindings top-level fields",
    )
    require(document["schema"] == RUNTIME_SCHEMA, "runtime-bindings schema mismatch")
    rows = document["rows"]
    require(isinstance(rows, list), "runtime-bindings rows must be a list")
    require(len(rows) == len(EXPECTED_RUNTIME_BINDINGS), "runtime binding row count mismatch")
    names = [row.get("rust_function") for row in rows]
    require(len(names) == len(set(names)), "duplicate Rust function binding")
    require(set(names) == set(EXPECTED_RUNTIME_BINDINGS), "Rust function binding set mismatch")
    all_identifiers: list[str] = []
    for row in rows:
        require(set(row) == EXPECTED_ROW_KEYS, f"unexpected fields for {row.get('rust_function')}")
        name = row["rust_function"]
        identifiers = row["lean_identifiers"]
        require(isinstance(identifiers, list), f"Lean identifiers must be a list for {name}")
        require(len(identifiers) == len(set(identifiers)), f"duplicate Lean identifier for {name}")
        require(frozenset(identifiers) == EXPECTED_RUNTIME_BINDINGS[name], f"Lean identifier set mismatch for {name}")
        all_identifiers.extend(identifiers)
        require(row["source_path"] == RUNTIME_SOURCE, f"runtime source mismatch for {name}")
        fragment = rust_function_fragment(runtime_bytes, name)
        require(len(fragment) == row["fragment_bytes"], f"fragment length mismatch for {name}")
        require(
            hashlib.sha256(fragment).hexdigest() == row["fragment_sha256"],
            f"fragment digest mismatch for {name}",
        )
        require(row["binding_kind"] == BINDING_KIND, f"binding kind mismatch for {name}")
        require(row["semantic_equivalence"] == SEMANTIC_STATUS, f"semantic status mismatch for {name}")
        for identifier in identifiers:
            require(identifier in declarations, f"missing Lean declaration: {identifier}")
    require(len(all_identifiers) == len(set(all_identifiers)), "Lean identifier reused across runtime bindings")


def expect_rejected(label: str, operation) -> None:
    try:
        operation()
    except (RuntimeError, KeyError, TypeError, ValueError):
        return
    fail(f"negative verifier self-test was accepted: {label}")


def run_negative_self_tests(
    valid_document: dict, runtime_bytes: bytes, declarations: set[str]
) -> tuple[str, ...]:
    duplicate = copy.deepcopy(valid_document)
    duplicate["rows"][1] = copy.deepcopy(duplicate["rows"][0])
    expect_rejected(
        "duplicate Rust function",
        lambda: validate_runtime_bindings(duplicate, runtime_bytes, declarations),
    )

    substitution = copy.deepcopy(valid_document)
    row = substitution["rows"][0]
    replacement = rust_function_fragment(runtime_bytes, "add11")
    row["rust_function"] = "add11"
    row["fragment_bytes"] = len(replacement)
    row["fragment_sha256"] = hashlib.sha256(replacement).hexdigest()
    expect_rejected(
        "hash-valid unexpected Rust function substitution",
        lambda: validate_runtime_bindings(substitution, runtime_bytes, declarations),
    )

    identifier_substitution = copy.deepcopy(valid_document)
    identifier_substitution["rows"][0]["lean_identifiers"][0] = (
        "ProjectiveLawRuntime.residual_represents_input"
    )
    expect_rejected(
        "declared but unexpected Lean identifier substitution",
        lambda: validate_runtime_bindings(identifier_substitution, runtime_bytes, declarations),
    )

    fake = "namespace ProjectiveLawRuntime\n-- theorem classify12 : True := by trivial\n/- def generic12Runtime := 0 -/\nend ProjectiveLawRuntime\n"
    require(not lean_declarations(fake), "comment-only Lean names parsed as declarations")
    return (
        "duplicate_rust_function",
        "hash_valid_rust_function_substitution",
        "declared_lean_identifier_substitution",
        "comment_only_lean_name",
    )


def main() -> None:
    bindings = json.loads((ROOT / "source-bindings.json").read_text())
    require(bindings["schema"] == "projective-law-formal-source-bindings/v1", "source schema mismatch")
    require(bindings["runtime"]["path"] == RUNTIME_SOURCE, "unexpected runtime source path")
    require(digest(bindings["runtime"]["path"]) == bindings["runtime"]["sha256"], "runtime source digest mismatch")
    require(digest(bindings["written_audit"]["path"]) == bindings["written_audit"]["sha256"], "written audit digest mismatch")
    require(len(bindings["native_certificates"]) == 8, "native certificate count mismatch")
    runtime_text = (ROOT / RUNTIME_SOURCE).read_text()
    runtime_bytes = (ROOT / RUNTIME_SOURCE).read_bytes()
    native_schemas = {row["schema"] for row in bindings["native_certificates"]}
    require(len(native_schemas) == 8, "duplicate native certificate schema")
    for row in bindings["native_certificates"]:
        require(digest(row["source_path"]) == row["source_sha256"], f"certificate source digest mismatch: {row['schema']}")
        require(digest(row["result_path"]) == row["result_sha256"], f"certificate result digest mismatch: {row['schema']}")
        result = json.loads((ROOT / row["result_path"]).read_text())
        require(result["schema"] == row["schema"], f"certificate schema mismatch: {row['schema']}")
        require(result["status"] == "pass", f"certificate did not pass: {row['schema']}")

    lean_sources = {
        name: (ROOT / name).read_text()
        for name in ("ProjectiveLawBoundary.lean", "RuntimeSemantics.lean")
    }
    declarations: set[str] = set()
    for name, source in lean_sources.items():
        parsed = lean_declarations(source)
        require(declarations.isdisjoint(parsed), f"duplicate declarations across modules: {name}")
        declarations.update(parsed)
    runtime_bindings = json.loads((ROOT / "runtime-bindings.json").read_text())
    validate_runtime_bindings(runtime_bindings, runtime_bytes, declarations)
    negative_checks = run_negative_self_tests(runtime_bindings, runtime_bytes, declarations)
    subprocess.run(
        [sys.executable, str(ROOT / "check_coefficient_slice.py"), "--check"],
        check=True,
        capture_output=True,
        text=True,
    )

    forbidden_proof_token = re.compile(r"(?m)\b(?:sorry|admit)\b|^\s*axiom\b")
    for name, source in lean_sources.items():
        require(forbidden_proof_token.search(source) is None, f"forbidden proof token in {name}")

    coverage = json.loads((ROOT / "branch-coverage.json").read_text())
    require(coverage["schema"] == "projective-law-formal-branch-coverage/v1", "coverage schema mismatch")
    require(coverage["runtime_source_sha256"] == bindings["runtime"]["sha256"], "coverage runtime digest mismatch")
    require(coverage["binding_kind"] == "manual_label_inventory_plus_function_fragment_hashes", "coverage binding kind mismatch")
    require(coverage["runtime_fragment_binding"] == "runtime-bindings.json", "coverage binding path mismatch")
    require(len(coverage["rows"]) == 17, "reachable coverage row count mismatch")
    require(len(coverage["unreachable_or_unresolved_labels"]) == 5, "unresolved coverage row count mismatch")
    for row in coverage["rows"]:
        require(f'"{row["runtime_label"]}"' in runtime_text, f"missing runtime label: {row['runtime_label']}")
        require(row["certificate_schema"] == "definitional-inverse/v1" or row["certificate_schema"] in native_schemas, f"unknown certificate schema: {row['certificate_schema']}")
    for row in coverage["unreachable_or_unresolved_labels"]:
        require(f'"{row["runtime_label"]}"' in runtime_text, f"missing unresolved label: {row['runtime_label']}")

    status = json.loads((ROOT / "STATUS.json").read_text())
    require(status["schema"] == "projective-law-formal-boundary/v1", "status schema mismatch")
    require(status["status"] == "partial_mechanization_with_executable_abstractions", "status value mismatch")
    require(status["build"]["exit_status"] == 0, "recorded build failed")
    require(status["axiom_policy"]["unexpected"] == [], "unexpected axioms recorded")
    require(status["report"]["schema"] == "projective-law-formal-report/v1", "report schema mismatch")
    status_binding = status["runtime_fragment_bindings"]
    require(
        set(status_binding) == {"schema", "path", "function_count", "binding_kind", "semantic_equivalence"},
        "unexpected STATUS runtime binding fields",
    )
    require(status_binding["schema"] == runtime_bindings["schema"] == RUNTIME_SCHEMA, "STATUS binding schema mismatch")
    require(status_binding["path"] == "runtime-bindings.json", "STATUS binding path mismatch")
    require(status_binding["function_count"] == len(runtime_bindings["rows"]) == len(EXPECTED_RUNTIME_BINDINGS), "STATUS binding count mismatch")
    require(status_binding["binding_kind"] == BINDING_KIND, "STATUS binding kind mismatch")
    require(status_binding["semantic_equivalence"] == SEMANTIC_STATUS, "STATUS semantic status mismatch")
    proved = status["proved"]
    require(len(proved) == len(set(proved)), "duplicate STATUS proved theorem")
    require(frozenset(proved) == EXPECTED_PROVED, "STATUS proved theorem set mismatch")
    require(EXPECTED_PROVED <= declarations, "STATUS proved name is not a parsed Lean declaration")

    for key in ("markdown", "pdf", "graph_svg", "graph_png", "graph_data"):
        path = ROOT / status["report"][key]
        require(path.is_file() and path.stat().st_size > 0, f"missing report artifact: {key}")
    require((ROOT / status["report"]["pdf"]).read_bytes().startswith(b"%PDF-"), "invalid report PDF")
    report_text = (ROOT / status["report"]["markdown"]).read_text()
    require("projective-law-formal-report/v1" in report_text, "report schema marker missing")
    require("does not establish Rust-to-Lean semantic equivalence" in report_text, "semantic limit missing from report")
    require("manual abstraction" in report_text, "manual-abstraction scope missing from report")
    for relative in (status["report"]["markdown"], status["report"]["graph_svg"]):
        require(all(line == line.rstrip() for line in (ROOT / relative).read_text().splitlines()), f"trailing whitespace: {relative}")
    with (ROOT / status["report"]["graph_data"]).open(newline="") as handle:
        graph_rows = list(csv.DictReader(handle))
    require(len(graph_rows) == 8, "graph row count mismatch")
    require(sum(int(row["count"]) for row in graph_rows if row["class"] == "mechanized") == len(EXPECTED_PROVED), "graph mechanized count mismatch")
    require(sum(int(row["count"]) for row in graph_rows if row["class"] == "unresolved") == 4, "graph unresolved count mismatch")

    theorem_text = lean_sources["ProjectiveLawBoundary.lean"]
    require("let u₁ := X ^ 2 + q₁ * X + r₁" in theorem_text, "first support polynomial missing")
    require("N - B * v₁ = -(C * X + A) * u₁" in theorem_text, "first interpolation identity missing")
    require("(hu₁ : u₁ = 0)" not in theorem_text, "vacuous interpolation hypothesis returned")
    build_log = (ROOT / status["build"]["log"]).read_text()
    require("Build completed successfully (742 jobs)." in build_log, "successful build receipt missing")
    axiom_log = (ROOT / status["axiom_policy"]["log"]).read_text()
    audited = re.findall(r"AXIOM_AUDIT\|([^|]+)\|", axiom_log)
    require(len(audited) == len(set(audited)), "duplicate axiom-audit theorem")
    require(frozenset(audited) == EXPECTED_PROVED, "axiom-audit theorem set mismatch")
    require("Forbidden proof dependency" not in axiom_log, "forbidden proof dependency recorded")
    print(f"projective-law-formal-boundary/v1: pass; negative checks: {','.join(negative_checks)}")


if __name__ == "__main__":
    main()
