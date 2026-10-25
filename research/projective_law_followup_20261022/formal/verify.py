#!/usr/bin/env python3
"""Fail-closed integrity check for the N6 formal-boundary packet."""

from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent


def digest(relative: str) -> str:
    return hashlib.sha256((ROOT / relative).read_bytes()).hexdigest()


def main() -> None:
    bindings = json.loads((ROOT / "source-bindings.json").read_text())
    assert bindings["schema"] == "projective-law-formal-source-bindings/v1"
    assert digest(bindings["runtime"]["path"]) == bindings["runtime"]["sha256"]
    assert digest(bindings["written_audit"]["path"]) == bindings["written_audit"]["sha256"]
    assert len(bindings["native_certificates"]) == 8
    runtime_text = (ROOT / bindings["runtime"]["path"]).read_text()
    native_schemas = {row["schema"] for row in bindings["native_certificates"]}
    for row in bindings["native_certificates"]:
        assert digest(row["source_path"]) == row["source_sha256"], row["schema"]
        assert digest(row["result_path"]) == row["result_sha256"], row["schema"]
        result = json.loads((ROOT / row["result_path"]).read_text())
        assert result["schema"] == row["schema"]
        assert result["status"] == "pass"

    coverage = json.loads((ROOT / "branch-coverage.json").read_text())
    assert coverage["schema"] == "projective-law-formal-branch-coverage/v1"
    assert coverage["runtime_source_sha256"] == bindings["runtime"]["sha256"]
    assert len(coverage["rows"]) == 17
    assert len(coverage["unreachable_or_unresolved_labels"]) == 5
    for row in coverage["rows"]:
        assert f'"{row["runtime_label"]}"' in runtime_text, row["runtime_label"]
        assert row["certificate_schema"] == "definitional-inverse/v1" or row["certificate_schema"] in native_schemas
    for row in coverage["unreachable_or_unresolved_labels"]:
        assert f'"{row["runtime_label"]}"' in runtime_text, row["runtime_label"]

    status = json.loads((ROOT / "STATUS.json").read_text())
    assert status["schema"] == "projective-law-formal-boundary/v1"
    assert status["status"] == "partial_mechanization"
    assert status["build"]["exit_status"] == 0
    assert status["axiom_policy"]["unexpected"] == []
    assert status["report"]["schema"] == "projective-law-formal-report/v1"
    for key in ("markdown", "pdf", "graph_svg", "graph_png", "graph_data"):
        assert (ROOT / status["report"][key]).is_file(), key
        assert (ROOT / status["report"][key]).stat().st_size > 0, key
    assert (ROOT / status["report"]["pdf"]).read_bytes().startswith(b"%PDF-")
    report_text = (ROOT / status["report"]["markdown"]).read_text()
    assert "projective-law-formal-report/v1" in report_text
    assert "does not establish Rust-to-Lean semantic correspondence" in report_text
    for relative in (status["report"]["markdown"], status["report"]["graph_svg"]):
        assert all(line == line.rstrip() for line in (ROOT / relative).read_text().splitlines()), relative
    with (ROOT / status["report"]["graph_data"]).open(newline="") as handle:
        graph_rows = list(csv.DictReader(handle))
    assert len(graph_rows) == 8
    assert sum(int(row["count"]) for row in graph_rows if row["class"] == "mechanized") == 8
    assert sum(int(row["count"]) for row in graph_rows if row["class"] == "unresolved") == 4
    theorem_text = (ROOT / "ProjectiveLawBoundary.lean").read_text()
    assert "let u₁ := X ^ 2 + q₁ * X + r₁" in theorem_text
    assert "N - B * v₁ = -(C * X + A) * u₁" in theorem_text
    assert "(hu₁ : u₁ = 0)" not in theorem_text
    build_log = (ROOT / status["build"]["log"]).read_text()
    assert "Build completed successfully (740 jobs)." in build_log
    axiom_log = (ROOT / status["axiom_policy"]["log"]).read_text()
    assert axiom_log.count("AXIOM_AUDIT|") == 8
    assert "Forbidden proof dependency" not in axiom_log
    print("projective-law-formal-boundary/v1: pass")


if __name__ == "__main__":
    main()
