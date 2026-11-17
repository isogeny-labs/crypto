#!/usr/bin/env python3
"""Validate frozen k=1 panel invariants and A/A evidence without solving ECDLP."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def read(name: str) -> dict:
    return json.loads((ROOT / name).read_text())


def digest(name: str) -> str:
    return hashlib.sha256((ROOT / name).read_bytes()).hexdigest()


def strip_time(value):
    if isinstance(value, dict):
        return {key: strip_time(item) for key, item in value.items()
                if not key.endswith("_ms_l0")}
    if isinstance(value, list):
        return [strip_time(item) for item in value]
    return value


if __name__ == "__main__":
    frozen = [json.loads(line) for line in (ROOT / "quartic_twist_mixed_v1.ndjson").read_text().splitlines()]
    rows = []
    for p in (73, 97):
        for seed in range(3):
            stem = f"quartic_twist_k1_scale_p{p}_seed{seed}"
            rec_name = stem + "_v1.json"
            aa_name = stem + "_aa_v1.json"
            audit_name = stem + "_audit_v1.json"
            audit_aa_name = stem + "_audit_aa_v1.json"
            rec, aa, audit = read(rec_name), read(aa_name), read(audit_name)
            assert strip_time(rec) == strip_time(aa)
            assert (ROOT / (stem + "_aa_v1.diff")).stat().st_size == 0
            assert (ROOT / audit_name).read_bytes() == (ROOT / audit_aa_name).read_bytes()
            assert rec["status"] == audit["status"]
            assert rec["recovered"] == audit["recovered"] == 271
            assert rec["anchored_rank"] == audit["anchored_rank"] == rec["folded_columns"]
            assert len(rec["trace"]) == audit["processed_queries"]
            assert sum(len(q["patterns"][0]["signed_relations"]) for q in rec["trace"]) == audit["polynomial_signed_relations"]
            cstem = f"quartic_twist_k1_census_p{p}_seed{seed}"
            cen_name = cstem + "_v1.json"
            cen_aa_name = cstem + "_aa_v1.json"
            caudit_name = cstem + "_audit_v1.json"
            caudit_aa_name = cstem + "_audit_aa_v1.json"
            cen, caudit = read(cen_name), read(caudit_name)
            assert (ROOT / cen_name).read_bytes() == (ROOT / cen_aa_name).read_bytes()
            assert (ROOT / caudit_name).read_bytes() == (ROOT / caudit_aa_name).read_bytes()
            assert caudit["status"] == cen["status"] == "pass"
            assert caudit["per_query_signed_set_digests_verified"] == cen["queries"]
            assert caudit["factor_logs_verified"] == cen["physical_base"]
            assert caudit["independent_target_log"] == 271
            assert len(cen["trace"]) == cen["queries"]
            assert sum(q["k1_signed_relations"] for q in cen["trace"]) == cen["total_k1_signed_relations"]
            assert sum(q["k1_signed_relations"] == 0 for q in cen["trace"]) == cen["zero_k1_queries"]
            assert [q["query_index"] for q in cen["trace"]] == list(range(cen["queries"]))
            assert all(len(q["signed_set_sha256"]) == 64 for q in cen["trace"])
            assert sum(q["k1_signed_relations"] for q in cen["trace"][:len(rec["trace"])]) == audit["expected_signed_relations"]
            assert audit["expected_signed_relations"] - audit["polynomial_signed_relations"] == audit["missing_signed_relations"]
            prior = next(v for v in frozen if v["p"] == p and v["seed"] == seed
                         and v["line_powers"] == [0, 2])
            assert cen["queries"] == len(prior["query_trace"])
            for q, earlier in zip(cen["trace"], prior["query_trace"]):
                assert (q["a"], q["b"], q["recorded_all_pattern_hits"]) == tuple(earlier)
                assert q["k1_signed_relations"] <= q["recorded_all_pattern_hits"]
            rows.append({"p": p, "seed": seed, "status": audit["status"],
                         "processed_queries": len(rec["trace"]),
                         "expected_prefix_relations": audit["expected_signed_relations"],
                         "polynomial_prefix_relations": audit["polynomial_signed_relations"],
                         "full_stream_queries": cen["queries"],
                         "full_stream_relations": cen["total_k1_signed_relations"],
                         "digest_checks": caudit["per_query_signed_set_digests_verified"],
                         "raw_sha256": digest(rec_name), "audit_sha256": digest(audit_name),
                         "census_sha256": digest(cen_name),
                         "census_audit_sha256": digest(caudit_name)})
    orbit = read("quartic_twist_k1_orbit_row_audit_v1.json")
    assert orbit["status"] == "pass" and orbit["selected_relations"] == 42
    assert orbit["rank_one_row_checks"] == 42 * 24
    assert orbit["factor_action_closure_checks"] == (96 * 3 + 72 * 3) * 24
    mutation = read("quartic_twist_k1_census_mutation_control_v1.json")
    assert mutation["status"] == "pass" and mutation["auditor_exit_code"] != 0
    assert (ROOT / "quartic_twist_k1_scale_p37_seed0_parity.diff").stat().st_size == 0
    out = {"status": "pass", "workloads": len(rows), "rows": rows,
           "complete_stream_digest_checks": sum(x["digest_checks"] for x in rows),
           "rank_one_row_checks": orbit["rank_one_row_checks"],
           "mutation_rejected": True,
           "generalized_p37_non_timing_parity": True}
    (ROOT / "K1_PANEL_VALIDATION.json").write_text(json.dumps(out, sort_keys=True, indent=2) + "\n")
    print(json.dumps({key: value for key, value in out.items() if key != "rows"}, sort_keys=True))
