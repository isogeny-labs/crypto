#!/usr/bin/env python3
"""Validate every preregistered native thinning arm and its independent audit."""
from __future__ import annotations

import difflib
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
NEW_METADATA = {"retained_l2_orbits", "line0_len", "full_base_size",
                "full_folded_columns", "chosen_l2_full_columns", "selected_full_ids"}


def path(name: str) -> Path:
    return ROOT / name


def read(name: str) -> dict:
    return json.loads(path(name).read_text())


def digest(name: str) -> str:
    return hashlib.sha256(path(name).read_bytes()).hexdigest()


def strip(value, metadata: bool = False):
    if isinstance(value, dict):
        return {key: strip(item, metadata) for key, item in value.items()
                if not key.endswith("_ms_l0") and (not metadata or key not in NEW_METADATA)}
    if isinstance(value, list):
        return [strip(item, metadata) for item in value]
    return value


def diff(a, b, left: str, right: str) -> str:
    first = json.dumps(a, sort_keys=True, indent=2).splitlines(keepends=True)
    second = json.dumps(b, sort_keys=True, indent=2).splitlines(keepends=True)
    return "".join(difflib.unified_diff(first, second, fromfile=left, tofile=right))


if __name__ == "__main__":
    ideal = read("K1_ORBIT_THINNING_v1.json")
    assert ideal["status"] == "pass" and len(ideal["runs"]) == 21
    ideal_by_key = {(r["p"], r["seed"], r["orbit_count"]): r for r in ideal["runs"]}
    rows = []
    for p, maximum in ((73, 4), (97, 3)):
        for seed in range(3):
            baseline_name = f"quartic_twist_k1_scale_p{p}_seed{seed}_v1.json"
            baseline = read(baseline_name)
            for j in range(1, maximum + 1):
                stem = f"quartic_twist_k1_thin_p{p}_seed{seed}_j{j}"
                raw_name, aa_name = stem + "_v1.json", stem + "_aa_v1.json"
                audit_name, audit_aa_name = stem + "_audit_v1.json", stem + "_audit_aa_v1.json"
                raw, aa, audit = read(raw_name), read(aa_name), read(audit_name)
                assert path(stem + "_v1.exit").read_text().strip() == "0"
                assert path(stem + "_aa_v1.exit").read_text().strip() == "0"
                assert path(stem + "_audit_v1.exit").read_text().strip() == "0"
                assert path(stem + "_audit_aa_v1.exit").read_text().strip() == "0"
                aa_diff = diff(strip(raw), strip(aa), raw_name, aa_name)
                path(stem + "_aa_v1.diff").write_text(aa_diff)
                assert aa_diff == "", f"A/A mismatch {stem}"
                assert path(audit_name).read_bytes() == path(audit_aa_name).read_bytes()
                assert (raw["p"], raw["seed"], raw["retained_l2_orbits"]) == (p, seed, j)
                assert audit["subset_coordinates_verified"]
                assert audit["status"] == raw["status"]
                assert audit["processed_queries"] == len(raw["trace"])
                assert audit["anchored_rank"] == raw["anchored_rank"]
                assert audit["recovered"] == raw["recovered"]
                assert audit["ic_counted_fp_muls"] == raw["counted_ic_fp_muls"]
                assert audit["ic_counted_group_ops"] == raw["counted_ic_group_ops"]
                assert audit["rho_counted_fp_muls"] == raw["rho"]["fp_muls"]
                assert raw["counted_ic_fp_muls"] == sum(raw[key] for key in (
                    "model_fp_muls", "twist_geometry_fp_muls", "eigen_fp_muls",
                    "base_fp_muls", "map_fp_muls", "s4_precompute_fp_muls",
                    "stream_external_fp_muls", "stream_internal_fp_muls",
                    "verify_fp_muls"))
                assert raw["counted_ic_group_ops"] == sum(raw[key] for key in (
                    "eigen_group_ops", "base_group_ops", "map_group_ops",
                    "stream_group_ops", "verify_group_ops"))
                assert raw["rho"] == baseline["rho"]
                assert audit["base_logs_verified"] == raw["base_size"]
                assert audit["expected_signed_relations"] - audit["polynomial_signed_relations"] == audit["missing_signed_relations"]
                assert sum(len(q["patterns"][0]["signed_relations"]) for q in raw["trace"]) == audit["polynomial_signed_relations"]
                expected = ideal_by_key[(p, seed, j)]
                assert raw["base_size"] == expected["physical_base"]
                assert raw["folded_columns"] == expected["folded_columns"]
                assert raw["selected_full_ids"][:raw["line0_len"]] == list(range(raw["line0_len"]))
                assert raw["chosen_l2_full_columns"] == expected["chosen_l2_full_columns"]
                assert sum(q["signed_relations"] for q in expected["trace"][:len(raw["trace"])]) == audit["expected_signed_relations"]
                systems = sum(q["patterns"][0]["systems"] for q in raw["trace"])
                logs = raw["physical_factor_logs"]
                assert len(logs) == raw["base_size"]
                exact_attempts = sum(
                    (a + 271 * b) % raw["n"] != (sign * logs[index]) % raw["n"]
                    for q in raw["trace"] for a, b in [(q["a"], q["b"])]
                    for index in range(raw["line0_len"], raw["base_size"])
                    for sign in (-1, 1))
                assert systems == exact_attempts
                if j == maximum:
                    parity = diff(strip(baseline), strip(raw, metadata=True),
                                  baseline_name, raw_name)
                    path(stem + "_full_parity.diff").write_text(parity)
                    assert parity == "", f"full-base source parity mismatch {stem}"
                rows.append({"p": p, "seed": seed, "retained_l2_orbits": j,
                             "status": raw["status"], "processed_queries": len(raw["trace"]),
                             "rank": raw["anchored_rank"],
                             "exact_prefix_relations": audit["expected_signed_relations"],
                             "polynomial_prefix_relations": audit["polynomial_signed_relations"],
                             "system_failures": audit["system_failures"],
                             "attempted_s4_systems": systems,
                             "ic_counted_fp_muls": raw["counted_ic_fp_muls"],
                             "ic_counted_group_ops": raw["counted_ic_group_ops"],
                             "full_base_ic_counted_fp_muls": baseline["counted_ic_fp_muls"],
                             "full_base_over_candidate_fp_ratio":
                             baseline["counted_ic_fp_muls"] / raw["counted_ic_fp_muls"],
                             "rho_counted_fp_muls": raw["rho"]["fp_muls"],
                             "raw_sha256": digest(raw_name), "audit_sha256": digest(audit_name)})
    mutation = read("quartic_twist_k1_thin_mutation_control_v1.json")
    assert mutation["status"] == "pass" and mutation["auditor_exit_code"] != 0
    out = {"status": "pass", "method": "native polynomial IC plus independent point-keyed/BSGS audit",
           "workloads": len(rows), "rows": rows,
           "exact_relation_set_runs": sum(r["exact_prefix_relations"] == r["polynomial_prefix_relations"]
                                          and r["system_failures"] == 0 for r in rows),
           "full_rank_recoveries": sum(r["rank"] == (4 if r["p"] == 97 else 5) +
                                       r["retained_l2_orbits"] - 1 for r in rows),
           "full_base_parity_checks": 6, "mutation_rejected": True}
    path("K1_THIN_NATIVE_VALIDATION.json").write_text(json.dumps(out, sort_keys=True, indent=2) + "\n")
    print(json.dumps({key: value for key, value in out.items() if key != "rows"}, sort_keys=True))
