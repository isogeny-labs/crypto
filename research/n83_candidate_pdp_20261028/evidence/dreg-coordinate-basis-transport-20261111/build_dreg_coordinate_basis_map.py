#!/usr/bin/env python3
"""Build a source-bound map of known N83 bases and conditional chain-Dreg values.

This is a theorem-application/coverage artifact, not a new solver measurement.
It never upgrades a candidate case to exact Dreg without a matching
chain-exact certificate.
"""

from __future__ import annotations

import hashlib
import json
from collections import defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
STUDY = ROOT / "research/n83_candidate_pdp_20261028"
VARIANTS = STUDY / "evidence/n83-factorbase-variants"
CHAIN_SUMMARY = STUDY / "evidence/chain-exact/verified-summary.json"
OUT = Path(__file__).resolve().parent / "coordinate-basis-dreg-map.json"
R38_ATTEMPT = VARIANTS / "r38-attempt-01/top-form-20261105T1840Z-attempt-09"
R38_REPLAY = R38_ATTEMPT / "replay-summary.json"
R38_INDEX = R38_ATTEMPT / "S3_PERSISTENCE_INDEX.json"
R38_RECONCILIATION = R38_ATTEMPT / "ATTEMPT-09-RECONCILIATION.json"
R38_READBACK = R38_ATTEMPT / "RESULTS-S3-READBACK.json"
N = 83
ELL = 27
FAMILIES = ("koblitz", "binary", "prime")
TARGETS = ("natural", "planted")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_json(path: Path):
    return json.loads(path.read_text())


def candidate_inputs():
    records = defaultdict(list)
    for path in sorted(VARIANTS.rglob("inputs.json")):
        try:
            payload = load_json(path)
        except (OSError, json.JSONDecodeError):
            continue
        for row in payload if isinstance(payload, list) else [payload]:
            if not isinstance(row, dict) or not row.get("candidate_id"):
                continue
            if not isinstance(row.get("coordinate_bit_positions"), list):
                continue
            key = (row["candidate_id"], row.get("family"), row.get("m"), row.get("target_kind"))
            records[key].append((path, row))
    return records


def candidate_manifests():
    records = defaultdict(list)
    paths = [STUDY / "data" / family / "manifest.json" for family in FAMILIES]
    paths += sorted(VARIANTS.rglob("manifest.json"))
    for path in paths:
        if not path.is_file():
            continue
        try:
            row = load_json(path)
        except (OSError, json.JSONDecodeError):
            continue
        if not isinstance(row, dict) or not row.get("candidate_id"):
            continue
        if row.get("family") not in FAMILIES or not isinstance(row.get("coordinate_bit_positions"), list):
            continue
        key = (row["candidate_id"], row["family"])
        records[key].append((path, row))
    return records


def canonical_input(entries):
    signatures = {
        (
            row.get("ell"),
            tuple(row.get("coordinate_bit_positions", [])),
            row.get("factor_base_manifest_sha256"),
            row.get("source_sha256"),
            row.get("matched_original_inputs_sha256"),
        )
        for _, row in entries
    }
    if len(signatures) != 1:
        raise ValueError(f"conflicting input metadata records: {signatures}")
    return sorted(entries, key=lambda item: str(item[0]))[0]


def canonical_manifest(entries):
    signatures = {
        (
            tuple(row.get("coordinate_bit_positions", [])),
            row.get("ell"),
            row.get("point_count"),
            row.get("candidate_x_count"),
        )
        for _, row in entries
    }
    if len(signatures) != 1:
        raise ValueError(f"conflicting factor-base manifest records: {signatures}")
    return sorted(entries, key=lambda item: str(item[0]))[0]


def main() -> None:
    inputs = candidate_inputs()
    manifests = candidate_manifests()
    chain = load_json(CHAIN_SUMMARY)
    if not chain.get("all_verified") or not chain.get("replay_passed"):
        raise ValueError("chain-exact reference summary is not fully verified/replayed")

    exact_rows = chain["rows"]
    exact_by_key = {row["key"]: row for row in exact_rows}
    for row in exact_rows:
        m = int(row["key"].split("-m", 1)[1].split("-", 1)[0])
        expected = N * (m - 2) + ELL + 1
        if not row.get("verified") or row.get("dreg_exact") != expected:
            raise ValueError(f"chain-exact reference does not match formula: {row['key']}")

    top_replay = load_json(R38_REPLAY)
    top_index = load_json(R38_INDEX)
    top_reconciliation = load_json(R38_RECONCILIATION)
    top_readback = load_json(R38_READBACK)
    if len(top_replay.get("rows", [])) != 48 or not top_replay.get("all_verified"):
        raise ValueError("spike38 top-form replay is not complete and verified")
    if not top_reconciliation.get("fresh_replay_all_verified"):
        raise ValueError("spike38 top-form reconciliation lacks fresh replay verification")
    if not top_reconciliation.get("s3_indexed_objects_byte_identical"):
        raise ValueError("spike38 top-form S3 object readback did not match")
    if not top_readback.get("all_byte_identical") or top_readback.get("failures"):
        raise ValueError("spike38 top-form byte readback is not clean")
    if top_readback.get("objects_checked") != top_reconciliation.get("s3_indexed_object_count"):
        raise ValueError("spike38 top-form readback object count mismatch")
    top_index_by_path = {row["path"]: row for row in top_index["files"]}
    top_rows = {}
    for row in top_replay["rows"]:
        key = (row["family"], int(row["m"]), row["target_kind"])
        if key in top_rows:
            raise ValueError(f"duplicate spike38 top-form result: {key}")
        cert_rel = f"top-certificates/{row['certificate']}"
        cert_entry = top_index_by_path.get(cert_rel)
        cert_path = R38_ATTEMPT / cert_rel
        if not cert_entry or not cert_path.is_file() or sha256(cert_path) != cert_entry.get("sha256"):
            raise ValueError(f"spike38 top-form certificate is not bound by its index: {key}")
        top_rows[key] = (row, cert_entry)

    bases = []
    cases = []
    for bit in range(27, 40):
        candidate = f"spike{bit}"
        expected_positions = list(range(26)) + [bit]
        for family in FAMILIES:
            ikey = (candidate, family)
            manifest_entries = manifests.get(ikey, [])
            input_keys = [key for key in inputs if key[:2] == ikey]
            all_records = [row for _, row in manifest_entries]
            all_records += [row for key in input_keys for _, row in inputs[key]]
            positions = {tuple(r.get("coordinate_bit_positions", [])) for r in all_records}
            ell_values = {r.get("ell") for r in all_records}
            if positions and positions != {tuple(expected_positions)}:
                raise ValueError(f"unexpected selector positions for {candidate}/{family}: {positions}")
            if ell_values and ell_values != {ELL}:
                raise ValueError(f"unexpected ell for {candidate}/{family}: {ell_values}")

            manifest_entry = canonical_manifest(manifest_entries) if manifest_entries else None
            input_manifest_hashes = sorted({
                row.get("factor_base_manifest_sha256")
                for key in input_keys for _, row in inputs[key]
                if row.get("factor_base_manifest_sha256")
            })
            base = {
                "candidate_id": candidate,
                "coordinate_bit": bit,
                "family": family,
                "n": N,
                "ell": ELL,
                "coordinate_bit_positions": expected_positions,
                "selector_rank_proved_by_distinct_polynomial_basis_vectors": len(set(expected_positions)) == ELL,
                "local_point_manifest_present": bool(manifest_entry),
                "point_manifest": None,
                "input_metadata_manifest_sha256_values": input_manifest_hashes,
                "case_input_count_m2_m9_per_target": len(input_keys),
                "case_input_sources": [],
            }
            if manifest_entry:
                path, row = manifest_entry
                base["point_manifest"] = {
                    "path": path.relative_to(ROOT).as_posix(),
                    "sha256": sha256(path),
                    "point_count": row.get("point_count"),
                    "candidate_x_count": row.get("candidate_x_count"),
                    "compressed_bytes": row.get("compressed_bytes"),
                }
            for key in sorted(input_keys, key=lambda k: (int(k[2]), str(k[3]))):
                path, row = canonical_input(inputs[key])
                case_dir = path.parent.parent
                output_path = case_dir / "output" / f"{row.get('id')}.json"
                output_row = None
                if output_path.is_file():
                    out_data = load_json(output_path)
                    if isinstance(out_data, list):
                        output_row = next((x for x in out_data if x.get("input", {}).get("id") == row.get("id")), None)
                    elif isinstance(out_data, dict):
                        output_row = out_data
                meta = {
                    "m": int(row["m"]),
                    "target_kind": row["target_kind"],
                    "input_metadata_path": path.relative_to(ROOT).as_posix(),
                    "input_metadata_file_sha256": sha256(path),
                    "source_sha256": row.get("source_sha256"),
                    "matched_original_inputs_sha256": row.get("matched_original_inputs_sha256"),
                    "input_equations": row.get("input_equations"),
                    "n_vars": row.get("n_vars"),
                    "saved_first_fall_degree": output_row.get("first_fall_degree") if output_row else None,
                    "saved_last_fall_degree_field": output_row.get("last_fall_degree") if output_row else None,
                    "saved_output_path": output_path.relative_to(ROOT).as_posix() if output_path.is_file() else None,
                    "saved_output_sha256": sha256(output_path) if output_path.is_file() else None,
                }
                base["case_input_sources"].append(meta)
            bases.append(base)

            for m in range(2, 10):
                for target in TARGETS:
                    observed = next((
                        row for key, row in inputs.items()
                        if key[0] == candidate and key[1] == family and key[2] == m and key[3] == target
                    ), None)
                    theorem_value = N * (m - 2) + ELL + 1 if family in ("koblitz", "binary") and m >= 3 else None
                    saved_top_form = None
                    if candidate == "spike38" and (family, m, target) in top_rows:
                        top_row, cert_entry = top_rows[(family, m, target)]
                        saved_top_form = {
                            "result_status": top_row["status"],
                            "dreg_lower_bound": top_row.get("dreg_lower_bound"),
                            "dreg_upper_bound": top_row.get("dreg_upper_bound"),
                            "source_sha256": top_row.get("source_sha256"),
                            "factor_base_manifest_sha256": top_row.get("factor_base_manifest_sha256"),
                            "certificate_path": (R38_ATTEMPT / f"top-certificates/{top_row['certificate']}").relative_to(ROOT).as_posix(),
                            "certificate_sha256": cert_entry["sha256"],
                            "replay_summary_sha256": sha256(R38_REPLAY),
                            "s3_object_readback_verified": True,
                            "lower_bound_matches_conditional_formula": (
                                theorem_value is not None
                                and top_row.get("dreg_lower_bound") == theorem_value
                            ),
                        }
                    cases.append({
                        "candidate_id": candidate,
                        "coordinate_bit": bit,
                        "family": family,
                        "m": m,
                        "target_kind": target,
                        "coordinate_bit_positions": expected_positions,
                        "factor_base_manifest_present": bool(manifest_entry),
                        "input_metadata_present": observed is not None,
                        "theorem_value_if_chain_hypotheses_hold": theorem_value,
                        "dreg_exact_for_this_candidate": None,
                        "candidate_source_shape_status": "not_verified_by_chain_exact_parser",
                        "saved_top_form_result": saved_top_form,
                        "status": (
                            "conditional_theorem_projection_not_measurement"
                            if theorem_value is not None
                            else "outside_chain_theorem_scope"
                        ),
                    })

    result = {
        "schema": "n83.coordinate-basis-dreg-map/v1",
        "classification": "derived theorem-application and source-coverage map; no new PDP solver run",
        "theorem": {
            "formula": "dreg = n*(m-2) + ell + 1",
            "n": N,
            "ell": ELL,
            "covered_m": [3, 4, 5, 6, 7, 8, 9],
            "scope": "original individual highest-form ideal including Boolean squares; Koblitz/binary chain source shape; same ell independent F2 selector weights and full-rank intermediate blocks",
            "nonderived_claims": ["FFD", "LFD", "affine solving degree", "prime-family transfer", "end-to-end attack cost"],
            "formalization_and_review": "full algebraic Lean formalization and independent scientific review remain pending",
        },
        "reference_certificate": {
            "summary_path": CHAIN_SUMMARY.relative_to(ROOT).as_posix(),
            "summary_sha256": sha256(CHAIN_SUMMARY),
            "all_verified": chain["all_verified"],
            "replay_passed": chain["replay_passed"],
            "exact_row_count": len(exact_rows),
            "exact_row_keys": sorted(exact_by_key),
            "exact_dreg_values_by_m": {
                str(m): N * (m - 2) + ELL + 1 for m in range(3, 10)
            },
        },
        "saved_top_form_reference": {
            "attempt_path": R38_ATTEMPT.relative_to(ROOT).as_posix(),
            "attempt_reconciliation_sha256": sha256(R38_RECONCILIATION),
            "replay_summary_sha256": sha256(R38_REPLAY),
            "s3_persistence_index_sha256": sha256(R38_INDEX),
            "s3_readback_receipt_sha256": sha256(R38_READBACK),
            "verified_case_count": len(top_rows),
            "fresh_replay_all_verified": top_reconciliation["fresh_replay_all_verified"],
            "s3_indexed_objects_byte_identical": top_reconciliation["s3_indexed_objects_byte_identical"],
            "s3_objects_checked": top_readback["objects_checked"],
            "s3_bytes_checked": top_readback["total_bytes_checked"],
            "claim_scope": "Dreg intervals or positive-dimensional original-top ideals; not exact candidate Dreg, FFD, LFD, affine solving degree, or attack cost",
        },
        "candidate_base_count": len(bases),
        "candidate_case_count": len(cases),
        "bases": bases,
        "cases": cases,
    }
    OUT.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(f"wrote {OUT.relative_to(ROOT)} bases={len(bases)} cases={len(cases)} exact_reference_rows={len(exact_rows)}")


if __name__ == "__main__":
    main()
