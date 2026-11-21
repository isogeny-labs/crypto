#!/usr/bin/env python3
"""Independent group, matrix, identity, and timing audit of the n23 run."""

from sage.all import *
from collections import Counter
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
RESULT_PATH = HERE / "result.json"
RESULT = json.loads(RESULT_PATH.read_text())
CANDIDATE = json.loads((HERE / "candidate.json").read_text())
WORKLOAD = json.loads((HERE / "workload.json").read_text())
assert RESULT["status"] == "complete"

def canonical(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()

def digest(obj):
    return hashlib.sha256(canonical(obj)).hexdigest()

def to_int(e):
    return sum(int(c) << i for i, c in enumerate(e.polynomial().list()))

def point_record(P):
    return [to_int(P[0]), to_int(P[1])]

assert hashlib.sha256((HERE / "run.py").read_bytes()).hexdigest() == SPEC["runner_sha256"]
assert hashlib.sha256((ROOT / SPEC["source_instance_result"]).read_bytes()).hexdigest() == SPEC["source_instance_sha256"]
assert hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest() == RESULT["protocol_sha256"]
record = CANDIDATE["record"]
candidate_id = ("IC1N23Ch4randfb250PDP3mitmRCsampleLAgaussTDdirectISO0h"
                + digest(record)[:12])
assert candidate_id == CANDIDATE["candidate_id"] == RESULT["candidate_id"]
curve = dict(record["curve"])
curve_id = curve.pop("curve_id")
assert curve_id == RESULT["curve_id"] == "EC1N23Ch4randh" + digest({"field": record["field"], "curve": curve})[:12]
assert record["implementation"]["source_sha256"]["this_runner"] == SPEC["runner_sha256"]
assert record["isogeny"] == "none"
assert WORKLOAD["workload_id"] == RESULT["workload_id"] == "W" + digest(WORKLOAD["record"])[:12]
assert RESULT["run_id"] == candidate_id + WORKLOAD["workload_id"] + "R1"

n = 23
poly = record["field"]["defining_polynomial_int"]
F2 = GF(2)
PR = PolynomialRing(F2, "z")
z = PR.gen()
mod = sum(F2((poly >> i) & 1) * z**i for i in range(n + 1))
K = GF(1 << n, "z", modulus=mod)
def elt(v):
    return K([(v >> i) & 1 for i in range(n)])
coeff = record["curve"]["coefficients"]
E = EllipticCurve(K, [1, elt(coeff["a2"]), 0, 0, elt(coeff["a6"])])
r = record["curve"]["r"]
assert r == SPEC["subgroup_order"]
assert int(E.cardinality()) == record["curve"]["curve_order"]
def point(pair):
    return E(elt(pair[0]), elt(pair[1]))
G = point(record["curve"]["G"])
assert r * G == E(0)
base = [point(pair) for pair in record["factor_base"]["encoded_points"]]
assert len(base) == RESULT["base_points"] == 250
assert digest([point_record(P) for P in base]) == record["factor_base"]["enumerated_set_digest"]
assert all(r * P == E(0) for P in base)
reps = sorted({min((P, -P), key=point_record) for P in base}, key=point_record)
assert len(reps) == RESULT["folded_columns"] == 125
col_of = {P: i for i, P in enumerate(reps)}

def row_for(indices):
    row = [0] * len(reps)
    for i in indices:
        P = base[i]
        rep = min((P, -P), key=point_record)
        row[col_of[rep]] = (row[col_of[rep]] + (1 if P == rep else -1)) % r
    return row

anchor = [0] * len(reps)
rep = min((G, -G), key=point_record)
anchor[col_of[rep]] = 1 if G == rep else -1
rows, rhs = [anchor], [1]
rank = 1
assert len(RESULT["relation_trace"]) == RESULT["relation_attempts"]
for entry in RESULT["relation_trace"]:
    assert entry["known_scalar"] != RESULT["target_fixture_scalar"]
    assert entry["rank_before"] == rank
    assert entry["status"] == "verified"
    indices = entry["witness_indices"]
    assert len(indices) == 3 and all(0 <= i < len(base) for i in indices)
    target = entry["known_scalar"] * G
    assert sum((base[i] for i in indices), E(0)) == target
    row = row_for(indices)
    assert digest(row) == entry["row_sha256"]
    rows.append(row)
    rhs.append(entry["known_scalar"])
    rank += entry["rank_gain"]
    assert entry["rank_gain"] in (0, 1) and entry["rank_after"] == rank
assert rank == RESULT["final_rank"] == len(reps)
assert len(rows) == RESULT["matrix_rows"]
assert digest(list(zip(rows, rhs))) == RESULT["relation_matrix_sha256"]
F = GF(r)
M = matrix(F, rows)
b = vector(F, rhs)
assert M.rank() == len(reps)
logs = [int(x) for x in M.solve_right(b)]
assert logs == RESULT["factor_logs"]
assert digest(logs) == RESULT["factor_logs_sha256"]
assert all(logs[i] * G == reps[i] for i in range(len(reps)))

target = point(RESULT["target"])
assert point_record(target) == WORKLOAD["record"]["target"]
assert digest(point_record(target)) == RESULT["target_point_sha256"]
assert RESULT["ic"]["status"] == RESULT["rho"]["status"] == "verified"
indices = RESULT["ic"]["witness_indices"]
assert [point_record(base[i]) for i in indices] == RESULT["ic"]["witness"]
assert sum((base[i] for i in indices), E(0)) == target
target_row = row_for(indices)
assert digest(target_row) == RESULT["ic"]["row_sha256"]
recovered = sum(v * logs[i] for i, v in enumerate(target_row)) % r
assert recovered == RESULT["verified_scalar"] == RESULT["ic"]["scalar"]
assert recovered == RESULT["rho_verified_scalar"] == RESULT["rho"]["scalar"]
assert recovered == RESULT["target_fixture_scalar"]
assert recovered * G == target
assert sum(RESULT["ic"]["phase_wall_ns"].values()) == RESULT["online_wall_ns"]
assert sum(RESULT["cold_phase_wall_ns"].values()) == RESULT["cold_wall_ns"]
assert RESULT["rho_online_wall_ns"] == RESULT["rho"]["wall_ns"]
assert RESULT["relation_status_counts"] == {"verified": RESULT["relation_attempts"]}
assert RESULT["novel_rows"] == len(reps) - 1

phase_names = ("setup", "isogeny", "factor_base", "precompute", "queries", "pdp",
               "relation_check", "matrix_build", "relation_la", "target_descent",
               "recovery_check")
online_names = ("target_query", "target_pdp", "target_relation_check",
                "target_descent", "target_recovery_check")
assert set(RESULT["cold_phase_wall_ns"]) == set(phase_names)
assert set(RESULT["ic"]["phase_wall_ns"]) == set(online_names)
run = {"schema_version": 2, "kind": "full_dlp", "status": "complete",
       "proposal_id": None, "candidate_id": candidate_id,
       "run_id": RESULT["run_id"], "workload_id": RESULT["workload_id"],
       "pair_block_id": RESULT["workload_id"],
       "source_curve_ref": "research/exotic_ecdlp_20261021/H4_prime_descent/n23_subgroup_pair_20261120",
       "profile_id": "h4_n23_cofactor36_m3_pair_index", "isogeny_route_ref": "none",
       "subgroup_order": str(r),
       "provenance": {"workload_fixture_sha256": digest(WORKLOAD["record"]),
                      "source_sha256": SPEC["runner_sha256"],
                      "host_id": RESULT["host"]["node"],
                      "resource_envelope_id": "one_local_sage_process_one_cpu_8GiB_observed_180s_cap",
                      "calibration_id": "ordinary_local_unisolated_no_AA_calibration",
                      "relation_precompute_seed": 61023,
                      "target_descent_seed": 73023,
                      "rho_walk_seed": 92023},
       "counts": {"ordinary_queries": RESULT["relation_attempts"],
                  "solved_queries": RESULT["relation_attempts"],
                  "verified_decompositions": RESULT["relation_attempts"],
                  "verified_relations": RESULT["relation_attempts"],
                  "novel_rows": RESULT["novel_rows"],
                  "effective_columns": len(reps), "final_rank": rank,
                  "pdp_attempts": RESULT["relation_attempts"],
                  "pdp_verified": RESULT["relation_attempts"],
                  "pdp_proved_unsat": 0, "pdp_timeout": 0, "pdp_budget": 0,
                  "pdp_error": 0, "pdp_lift_rejected": 0},
       "phase_operations": {name: None for name in phase_names},
       "phase_wall_ns": RESULT["cold_phase_wall_ns"],
       "online_phase_wall_ns": RESULT["ic"]["phase_wall_ns"],
       "online_wall_ns": RESULT["online_wall_ns"],
       "rho_online_wall_ns": RESULT["rho_online_wall_ns"],
       "rho_verified": True, "target_count": 1,
       "target_point_sha256": RESULT["target_point_sha256"],
       "precomputation_ready": True,
       "total_operations": None, "rho_operations": None,
       "verified_scalar": True, "scalar_certificate_ref": "result.json#ic",
       "peak_rss_bytes": RESULT["peak_rss_bytes"],
       "wall_ns": RESULT["cold_wall_ns"] + RESULT["online_wall_ns"] + RESULT["rho_online_wall_ns"]}
(HERE / "run.jsonl").write_text(json.dumps(run, sort_keys=True) + "\n")
validation = {"schema": "h4-n23-full-pipeline-validation-v1", "status": "passed",
              "result_sha256": hashlib.sha256(RESULT_PATH.read_bytes()).hexdigest(),
              "candidate_id": candidate_id, "curve_id": curve_id,
              "workload_id": RESULT["workload_id"], "run_id": RESULT["run_id"],
              "relation_rows_verified": len(rows), "matrix_rank": M.rank(),
              "factor_logs_verified": len(logs), "target_scalar": recovered,
              "same_point_rho_scalar": RESULT["rho"]["scalar"],
              "cold_phase_sum_matches": True, "online_phase_sum_matches": True,
              "host_tier": "ordinary_exploratory"}
(HERE / "validation.json").write_text(json.dumps(validation, indent=2, sort_keys=True) + "\n")
print(json.dumps(validation, indent=2, sort_keys=True))
