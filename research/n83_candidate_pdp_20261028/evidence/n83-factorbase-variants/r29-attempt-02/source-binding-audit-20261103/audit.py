#!/usr/bin/env python3
"""Verify that saved spike29 PDP sources match frozen V2 and fresh replay records."""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path
from typing import Any


PROTOCOL_ID = "N83-FACTOR-BASE-PDP-FFD-LFD-20261031-V2"
PROTOCOL_FREEZE_SHA256 = "50cd6f8213789de2dfd54a3b33a067ed87cc0def761b7c98769be3e19aee47a9"
SOURCE_COMMIT = "52665c7a6e8308f86201e10c9305afa346fd134b"
CANDIDATE_ID = "spike29"
PDP_BINARY_SHA256 = "b8e00f73512977828497c4bb30f0ae08d319350688a90b15310aa4143308a3ca"
FAMILIES = ("koblitz", "binary", "prime")
MS = tuple(range(2, 10))
TARGETS = ("natural", "planted")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def load(path: Path) -> Any:
    with path.open(encoding="utf-8") as stream:
        return json.load(stream)


def verify_manifest(manifest: Path, root: Path) -> int:
    checked = 0
    for line_number, line in enumerate(manifest.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip():
            continue
        parts = line.split(maxsplit=1)
        if len(parts) != 2 or len(parts[0]) != 64:
            raise ValueError(f"malformed SHA-256 row: {manifest}:{line_number}")
        expected, relative = parts
        relative = relative.lstrip("*")
        path = (root / relative).resolve()
        if root.resolve() not in path.parents:
            raise ValueError(f"manifest path escapes its root: {manifest}:{line_number}")
        actual = sha256(path)
        if actual != expected:
            raise ValueError(f"SHA-256 mismatch: {path}")
        checked += 1
    return checked


def find_repo_root() -> Path:
    here = Path(__file__).resolve().parent
    for candidate in (here, *here.parents):
        if (candidate / "AGENTS.md").is_file() and (candidate / ".git").exists():
            return candidate
    raise RuntimeError("could not locate repository root")


def build_audit() -> dict[str, Any]:
    repo = find_repo_root()
    base = repo / "research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r29-attempt-02"
    attempt_dir = base / "pdp-inputs/attempt-02"
    structure_dir = attempt_dir / "candidate-structure"
    factor_base_output_root = base / "factor-base-output"
    factor_base_dir = factor_base_output_root / "data"
    protocol_path = repo / "research/n83_candidate_pdp_20261028/protocol-pdp-factorbase-v2.json"
    attempt_path = attempt_dir / "attempt.json"
    inputs_path = structure_dir / "inputs.json"
    factor_index_path = structure_dir / "factor-base-index.json"
    top_form_dir = base / "top-form/attempt-01"
    replay_path = top_form_dir / "replay-summary.json"
    top_run_path = top_form_dir / "run.json"

    protocol = load(protocol_path)
    attempt = load(attempt_path)
    inputs = load(inputs_path)
    factor_index = load(factor_index_path)
    replay = load(replay_path)
    top_run = load(top_run_path)

    assert protocol["protocol_id"] == PROTOCOL_ID
    assert protocol["freeze"]["status"] == "frozen"
    assert protocol["freeze"]["protocol_sha256"] == PROTOCOL_FREEZE_SHA256
    assert protocol["freeze"]["commit"] == SOURCE_COMMIT
    assert attempt["candidate_id"] == CANDIDATE_ID
    assert attempt["exit_code"] == 0
    assert attempt["membership_points_all_present"] is True
    assert attempt["rows"] == attempt["systems"] == len(inputs) == 48
    assert attempt["input_anchor_sha256"] == attempt["required_input_anchor_sha256"]
    assert attempt["pdp_binary_sha256"] == PDP_BINARY_SHA256
    assert sha256(inputs_path) == attempt["input_index_sha256"]
    assert sha256(factor_index_path) == attempt["factor_base_index_sha256"]
    assert factor_index["candidate_id"] == CANDIDATE_ID
    assert factor_index["candidate_x_count_per_curve"] == 2**27
    assert factor_index["complete"] is True
    assert factor_index["ell"] == 27

    positions = factor_index["coordinate_bit_positions"]
    family_rows = {row["family"]: row for row in factor_index["families"]}
    assert set(family_rows) == set(FAMILIES)
    for family in FAMILIES:
        family_row = family_rows[family]
        assert family_row["same_base_for_m"] == list(MS)
        manifest_path = factor_base_dir / family_row["manifest"]
        assert sha256(manifest_path) == family_row["manifest_sha256"]

    protocol_arms = {arm["id"]: arm for arm in protocol["arms"]}
    for family in FAMILIES:
        arm = protocol_arms[f"spike29-{family}"]
        parameters = arm["parameters"]
        assert parameters["candidate_id"] == CANDIDATE_ID
        assert parameters["candidate_x_count"] == 2**27
        assert parameters["coordinate_bit"] == 29
        assert parameters["coordinate_positions"] == positions
        assert parameters["family"] == family
        assert parameters["m"] == list(MS)
        assert parameters["target_kinds"] == ["planted", "natural"]
        assert arm["instrument"]["commit"] == SOURCE_COMMIT
        assert arm["instrument"]["binary_sha256"]["pdp"] == PDP_BINARY_SHA256

    expected_cross = {(family, m, target) for family in FAMILIES for m in MS for target in TARGETS}
    observed_cross = {(row["family"], row["m"], row["target_kind"]) for row in inputs}
    assert observed_cross == expected_cross
    assert len({row["id"] for row in inputs}) == 48
    assert all(row["candidate_id"] == CANDIDATE_ID for row in inputs)
    assert all(row["coordinate_bit_positions"] == positions for row in inputs)
    assert all(row["factor_base_manifest_sha256"] == family_rows[row["family"]]["manifest_sha256"] for row in inputs)
    assert all(all(point["present"] is True for point in row["factor_base_membership"]) for row in inputs)

    assert replay["all_verified"] is True
    replay_rows = {row["id"]: row for row in replay["rows"]}
    assert len(replay_rows) == 48
    assert top_run["candidate_id"] == CANDIDATE_ID
    assert top_run["pdp_input_index_sha256"] == attempt["input_index_sha256"]
    assert top_run["replay_all_verified"] is True
    assert top_run["replay_rows"] == 48
    bound_rows = []
    for row in sorted(inputs, key=lambda item: item["id"]):
        replay_row = replay_rows[row["id"]]
        assert replay_row["source_sha256"] == row["source_sha256"]
        assert replay_row["family"] == row["family"]
        assert replay_row["m"] == row["m"]
        assert replay_row["target_kind"] == row["target_kind"]
        assert replay_row["factor_base_manifest_sha256"] == row["factor_base_manifest_sha256"]
        bound_rows.append({
            "id": row["id"],
            "family": row["family"],
            "m": row["m"],
            "target_kind": row["target_kind"],
            "source_sha256": row["source_sha256"],
            "factor_base_manifest_sha256": row["factor_base_manifest_sha256"],
            "top_form_replay_status": replay_row["status"],
        })

    manifests = {
        "factor_base_output": verify_manifest(factor_base_output_root / "manifest.sha256", factor_base_output_root),
        "pdp_attempt": verify_manifest(attempt_dir / "manifest.sha256", attempt_dir),
        "candidate_structure": verify_manifest(structure_dir / "manifest.sha256", structure_dir),
        "top_form_attempt": verify_manifest(top_form_dir / "manifest.sha256", top_form_dir),
    }
    return {
        "schema": "n83.r29-pdp-source-binding-audit/v1",
        "scope": "Source binding and custody only; no FFD, LFD, affine degree-of-regularity, solving-degree, or new IC measurement is produced.",
        "protocol": {
            "id": PROTOCOL_ID,
            "frozen_protocol_sha256": PROTOCOL_FREEZE_SHA256,
            "source_commit": SOURCE_COMMIT,
            "arms_verified": [f"spike29-{family}" for family in FAMILIES],
        },
        "factor_base": {
            "candidate_id": CANDIDATE_ID,
            "candidate_x_count_per_curve": factor_index["candidate_x_count_per_curve"],
            "complete": factor_index["complete"],
            "coordinate_bit_positions": positions,
            "same_base_for_m": list(MS),
            "families": [
                {
                    "family": family,
                    "manifest": family_rows[family]["manifest"],
                    "manifest_sha256": family_rows[family]["manifest_sha256"],
                    "point_count": family_rows[family]["point_count"],
                    "same_base_for_m": family_rows[family]["same_base_for_m"],
                }
                for family in FAMILIES
            ],
            "factor_base_index_sha256": sha256(factor_index_path),
        },
        "pdp_inputs": {
            "rows": len(inputs),
            "cross": {"families": list(FAMILIES), "m": list(MS), "target_kinds": list(TARGETS)},
            "input_index_sha256": sha256(inputs_path),
            "input_anchor_sha256": attempt["input_anchor_sha256"],
            "pdp_binary_sha256": attempt["pdp_binary_sha256"],
            "membership_points_all_present": attempt["membership_points_all_present"],
        },
        "top_form_replay_binding": {
            "rows": len(bound_rows),
            "all_verified": replay["all_verified"],
            "pdp_input_index_sha256": top_run["pdp_input_index_sha256"],
            "source_hashes_match": True,
            "cases": bound_rows,
        },
        "sha256_manifests_verified": manifests,
        "source_files": {
            "protocol": str(protocol_path.relative_to(repo)),
            "attempt": str(attempt_path.relative_to(repo)),
            "inputs": str(inputs_path.relative_to(repo)),
            "factor_base_index": str(factor_index_path.relative_to(repo)),
            "top_form_replay_summary": str(replay_path.relative_to(repo)),
            "top_form_run": str(top_run_path.relative_to(repo)),
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="verify that audit.json matches current sources")
    args = parser.parse_args()
    output_path = Path(__file__).resolve().parent / "audit.json"
    try:
        audit = build_audit()
    except (AssertionError, KeyError, OSError, ValueError, RuntimeError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        return 1
    rendered = json.dumps(audit, indent=2, sort_keys=True) + "\n"
    if args.check:
        if not output_path.is_file() or output_path.read_text(encoding="utf-8") != rendered:
            print("FAIL: audit.json is missing or differs from verified source files", file=sys.stderr)
            return 1
        print("PASS: frozen V2 binding, 48 source/replay links, factor-base manifests, and SHA-256 manifests")
    else:
        output_path.write_text(rendered, encoding="utf-8")
        print(f"WROTE: {output_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
