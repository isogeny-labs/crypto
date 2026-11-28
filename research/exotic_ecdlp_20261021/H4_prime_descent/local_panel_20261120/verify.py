#!/usr/bin/env python3
"""Replay five frozen targets, witnesses, phases, and paired run rows."""

from sage.all import *
import hashlib
import itertools
import json
from pathlib import Path
import statistics

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
FIX = json.loads((HERE / "fixtures.json").read_text())
RESULT_PATH = HERE / "result.json"
RESULT = json.loads(RESULT_PATH.read_text())
C = json.loads((ROOT / SPEC["candidate"]).read_text())
PREP = json.loads((ROOT / SPEC["precomputation_receipt"]).read_text())
assert RESULT["status"] == "complete"

def file_hash(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def digest(obj):
    return hashlib.sha256(json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()).hexdigest()

for path_key, hash_key in (("candidate", "candidate_sha256"),
                           ("precomputation_receipt", "precomputation_receipt_sha256"),
                           ("candidate_runner", "candidate_runner_sha256")):
    assert file_hash(ROOT / SPEC[path_key]) == SPEC[hash_key]
assert file_hash(HERE / "fixtures.json") == SPEC["fixtures_sha256"]
assert file_hash(HERE / "make_fixtures.py") == SPEC["fixture_generator_sha256"]
assert file_hash(HERE / "run.py") == SPEC["panel_runner_sha256"]
assert file_hash(HERE / "protocol.json") == RESULT["protocol_sha256"]
assert len(FIX["targets"]) == len(RESULT["pairs"]) == 5
assert FIX["candidate_id"] == C["candidate_id"] == RESULT["candidate_id"]

field = C["record"]["field"]
n, poly = field["n"], field["defining_polynomial_int"]
F2 = GF(2)
PR = PolynomialRing(F2, "z")
z = PR.gen()
mod = sum(F2((poly >> i) & 1) * z**i for i in range(n + 1))
K = GF(1 << n, "z", modulus=mod)
def elt(v):
    return K([(v >> i) & 1 for i in range(n)])
coeff = C["record"]["curve"]["coefficients"]
E = EllipticCurve(K, [1, elt(coeff["a2"]), 0, 0, elt(coeff["a6"])])
def point(pair):
    return E(elt(pair[0]), elt(pair[1]))
def to_int(e):
    return sum(int(c) << i for i, c in enumerate(e.polynomial().list()))
def record(P):
    return [to_int(P[0]), to_int(P[1])]
G = point(C["record"]["curve"]["G"])
r = C["record"]["curve"]["r"]
assert r == SPEC["subgroup_order"] and r * G == E(0)
base = [point(pair) for pair in C["record"]["factor_base"]["encoded_points"]]
reps = sorted({min((P, -P), key=record) for P in base}, key=record)
assert len(base) == 250 and len(reps) == len(PREP["factor_logs"]) == 125
col_of = {P: i for i, P in enumerate(reps)}
logs = PREP["factor_logs"]
assert all(logs[i] * G == reps[i] for i in range(len(reps)))
known = {entry["known_scalar"] for entry in PREP["relation_trace"]}
known.add(PREP["target_fixture_scalar"])
scalars = [entry["fixture_scalar"] for entry in FIX["targets"]]
assert len(set(scalars)) == 5 and not set(scalars) & known

def row_for(indices):
    row = [0] * len(reps)
    for i in indices:
        P = base[i]
        rep = min((P, -P), key=record)
        row[col_of[rep]] = (row[col_of[rep]] + (1 if P == rep else -1)) % r
    return row

phase_names = ("setup", "isogeny", "factor_base", "precompute", "queries", "pdp",
               "relation_check", "matrix_build", "relation_la", "target_descent",
               "recovery_check")
online_names = ("target_query", "target_pdp", "target_relation_check",
                "target_descent", "target_recovery_check")
runs = []
ratios = []
for i, (fixture, pair) in enumerate(zip(FIX["targets"], RESULT["pairs"])):
    assert pair["index"] == fixture["index"] == i
    assert pair["order"] == SPEC["arm_order"][i].split(",")
    assert pair["workload_id"] == fixture["workload_id"]
    assert pair["target"] == fixture["target"]
    assert pair["target_point_sha256"] == fixture["target_point_sha256"]
    label = fixture["target_label"]
    assert int.from_bytes(hashlib.sha256(label.encode()).digest(), "big") % (r - 1) + 1 == fixture["fixture_scalar"]
    assert fixture["workload_id"] == "W" + digest(fixture["workload_record"])[:12]
    Q = point(fixture["target"])
    assert fixture["fixture_scalar"] * G == Q
    ic, rho = pair["arms"]["IC"], pair["arms"]["rho"]
    assert ic["status"] == rho["status"] == "verified"
    assert ic["scalar"] == rho["scalar"] == fixture["fixture_scalar"]
    assert ic["scalar"] * G == rho["scalar"] * G == Q
    indices = ic["witness_indices"]
    assert [record(base[j]) for j in indices] == ic["witness"]
    assert sum((base[j] for j in indices), E(0)) == Q
    row = row_for(indices)
    assert digest(row) == ic["row_sha256"]
    assert sum(v * logs[j] for j, v in enumerate(row)) % r == ic["scalar"]
    assert set(ic["phase_wall_ns"]) == set(online_names)
    assert sum(ic["phase_wall_ns"].values()) == ic["wall_ns"]
    assert 0 < ic["wall_ns"] <= ic["outer_wall_ns"]
    assert rho["wall_ns"] > 0
    a, b = pair["calibration_pre_ns"], pair["calibration_post_ns"]
    assert a > 0 and b > 0
    assert abs(pair["calibration_ratio"] - max(a, b) / min(a, b)) < 1e-12
    assert pair["calibration_pass"] == (max(a, b) / min(a, b) <= 1.25)
    assert pair["scalar_certificate_pass"]
    ratios.append(rho["wall_ns"] / ic["wall_ns"])
    run_id = C["candidate_id"] + fixture["workload_id"] + "R1"
    run = {"schema_version": 2, "kind": "full_dlp", "status": "complete",
           "proposal_id": None, "candidate_id": C["candidate_id"],
           "run_id": run_id, "workload_id": fixture["workload_id"],
           "pair_block_id": fixture["workload_id"],
           "source_curve_ref": "research/exotic_ecdlp_20261021/H4_prime_descent/n23_subgroup_pair_20261120",
           "profile_id": "h4_n23_cofactor36_m3_pair_index", "isogeny_route_ref": "none",
           "subgroup_order": str(r),
           "provenance": {"workload_fixture_sha256": digest(fixture["workload_record"]),
                          "source_sha256": SPEC["candidate_runner_sha256"],
                          "host_id": RESULT["host"]["node"],
                          "resource_envelope_id": "one_local_sage_process_one_cpu_8GiB_observed_240s_panel_cap",
                          "calibration_id": "pre_post_512_additions_ratio_le_1.25",
                          "shared_precomputation_sha256": SPEC["precomputation_receipt_sha256"],
                          "panel_runner_sha256": SPEC["panel_runner_sha256"],
                          "rho_source_sha256": SPEC["sage_rho_source_sha256"]},
           "counts": {"ordinary_queries": 0, "solved_queries": 0,
                      "verified_decompositions": 0, "verified_relations": 0,
                      "novel_rows": 0, "effective_columns": len(reps),
                      "final_rank": PREP["final_rank"],
                      "pdp_attempts": 0, "pdp_verified": 0,
                      "pdp_proved_unsat": 0, "pdp_timeout": 0,
                      "pdp_budget": 0, "pdp_error": 0, "pdp_lift_rejected": 0},
           "phase_operations": {name: None for name in phase_names},
           "phase_wall_ns": {name: None for name in phase_names},
           "online_phase_wall_ns": ic["phase_wall_ns"],
           "online_wall_ns": ic["wall_ns"],
           "rho_online_wall_ns": rho["wall_ns"], "rho_verified": True,
           "target_count": 1, "target_point_sha256": fixture["target_point_sha256"],
           "precomputation_ready": True,
           "total_operations": None, "rho_operations": None,
           "verified_scalar": True,
           "scalar_certificate_ref": f"result.json#pairs/{i}",
           "peak_rss_bytes": pair["peak_rss_so_far_bytes"],
           "wall_ns": ic["wall_ns"] + rho["wall_ns"]}
    runs.append(run)

(HERE / "runs.jsonl").write_text("".join(json.dumps(run, sort_keys=True) + "\n" for run in runs))
bootstrap = sorted(statistics.median(sample) for sample in itertools.product(ratios, repeat=5))
validation = {"schema": "h4-n23-five-target-local-panel-validation-v1", "status": "passed",
              "result_sha256": file_hash(RESULT_PATH), "fixture_sha256": SPEC["fixtures_sha256"],
              "candidate_id": C["candidate_id"], "verified_pairs": len(runs),
              "calibration_passes": sum(pair["calibration_pass"] for pair in RESULT["pairs"]),
              "ratios": ratios, "median_ratio": statistics.median(ratios),
              "bootstrap_median_ratio_95pct": [bootstrap[int(0.025 * (len(bootstrap) - 1))],
                                                bootstrap[int(0.975 * (len(bootstrap) - 1))]],
              "host_tier": "ordinary_exploratory"}
(HERE / "validation.json").write_text(json.dumps(validation, indent=2, sort_keys=True) + "\n")
print(json.dumps(validation, indent=2, sort_keys=True))
