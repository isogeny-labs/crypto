#!/usr/bin/env python3
"""Five distinct frozen targets, alternating completed IC and Sage rho."""

from sage.all import *
from sage.groups.generic import discrete_log_rho
import hashlib
import importlib.util
import inspect
import json
from pathlib import Path
import platform
import resource
import signal
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())

def file_hash(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

for path_key, hash_key in (("candidate", "candidate_sha256"),
                           ("precomputation_receipt", "precomputation_receipt_sha256"),
                           ("candidate_runner", "candidate_runner_sha256")):
    assert file_hash(ROOT / SPEC[path_key]) == SPEC[hash_key]
assert file_hash(HERE / "fixtures.json") == SPEC["fixtures_sha256"]
assert file_hash(HERE / "make_fixtures.py") == SPEC["fixture_generator_sha256"]
assert hashlib.sha256(inspect.getsource(discrete_log_rho).encode()).hexdigest() == SPEC["sage_rho_source_sha256"]
C = json.loads((ROOT / SPEC["candidate"]).read_text())
PREP = json.loads((ROOT / SPEC["precomputation_receipt"]).read_text())
FIX = json.loads((HERE / "fixtures.json").read_text())
assert FIX["candidate_id"] == C["candidate_id"] == PREP["candidate_id"]
assert len(FIX["targets"]) == SPEC["target_count"]

module_spec = importlib.util.spec_from_file_location("h4_n29_candidate", ROOT / SPEC["candidate_runner"])
candidate = importlib.util.module_from_spec(module_spec)
module_spec.loader.exec_module(candidate)

def point_record(P):
    return candidate.point_record(P)

def checkpoint(result):
    (HERE / "result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")

def timeout(*_):
    raise TimeoutError("frozen per-arm or process wall cap")

def coord_hash(P):
    if P == E(0):
        return 0
    x, y = point_record(P)
    return ((x * 0x9e3779b97f4a7c15) ^ (y * 0xbf58476d1ce4e5b9)) & ((1 << 64) - 1)

def calibration(G):
    start = time.perf_counter_ns()
    P = G
    for _ in range(512):
        P += G
    assert P == 513 * G
    return time.perf_counter_ns() - start

signal.signal(signal.SIGALRM, timeout)
result = {"schema": "h4-n29-five-target-local-panel-result-v1", "status": "running",
          "protocol_sha256": file_hash(HERE / "protocol.json"),
          "fixture_sha256": SPEC["fixtures_sha256"],
          "candidate_id": C["candidate_id"],
          "host": {"node": platform.node(), "system": platform.system(),
                   "machine": platform.machine(), "tier": "ordinary_exploratory"},
          "pairs": []}
checkpoint(result)
process_deadline = time.monotonic() + SPEC["process_cap_seconds"]
signal.alarm(SPEC["process_cap_seconds"])
try:
    setup_start = time.perf_counter_ns()
    E, elt = candidate.build_context()
    base, reps, encoding, G = candidate.build_base(E, elt)
    index = candidate.build_index(base)
    logs = PREP["factor_logs"]
    assert len(logs) == len(reps) == 536
    assert all(logs[i] * G == reps[i] for i in range(len(reps)))
    assert [point_record(P) for P in base] == C["record"]["factor_base"]["encoded_points"]
    assert point_record(G) == FIX["generator"]
    assert len(index) == PREP["distinct_pair_sums"]
    result["shared_setup_ns"] = time.perf_counter_ns() - setup_start
    checkpoint(result)

    fixture_start = time.perf_counter_ns()
    targets = [E(elt(row["target"][0]), elt(row["target"][1])) for row in FIX["targets"]]
    assert all(point_record(P) == row["target"] for P, row in zip(targets, FIX["targets"]))
    result["fixture_rebuild_ns"] = time.perf_counter_ns() - fixture_start
    checkpoint(result)

    for i, (fixture, Q) in enumerate(zip(FIX["targets"], targets)):
        if time.monotonic() >= process_deadline:
            raise TimeoutError("frozen process wall cap")
        pair = {"index": i, "workload_id": fixture["workload_id"],
                "target": fixture["target"], "target_point_sha256": fixture["target_point_sha256"],
                "order": SPEC["arm_order"][i].split(","), "arms": {}}
        pair["calibration_pre_ns"] = calibration(G)
        for arm in pair["order"]:
            remaining = process_deadline - time.monotonic()
            if remaining <= 0:
                raise TimeoutError("frozen process wall cap")
            signal.alarm(min(SPEC["per_arm_cap_seconds"], max(1, int(remaining))))
            if arm == "IC":
                outer_start = time.perf_counter_ns()
                try:
                    receipt = candidate.ic_online(base, reps, encoding, G, index, logs, Q)
                    receipt["outer_wall_ns"] = time.perf_counter_ns() - outer_start
                except TimeoutError as exc:
                    receipt = {"status": "censored_timeout", "scalar": None,
                               "outer_wall_ns": time.perf_counter_ns() - outer_start,
                               "error": str(exc)}
                except Exception as exc:
                    receipt = {"status": "error", "scalar": None,
                               "outer_wall_ns": time.perf_counter_ns() - outer_start,
                               "error": repr(exc)}
            else:
                set_random_seed(94000 + i)
                start = time.perf_counter_ns()
                cpu_start = time.process_time_ns()
                try:
                    scalar = int(discrete_log_rho(Q, G, ord=SPEC["subgroup_order"],
                              operation="+", hash_function=coord_hash))
                    assert scalar * G == Q
                    receipt = {"status": "verified", "scalar": scalar,
                               "wall_ns": time.perf_counter_ns() - start,
                               "cpu_ns": time.process_time_ns() - cpu_start}
                except TimeoutError as exc:
                    receipt = {"status": "censored_timeout", "scalar": None,
                               "wall_ns": time.perf_counter_ns() - start,
                               "cpu_ns": time.process_time_ns() - cpu_start,
                               "error": str(exc)}
                except Exception as exc:
                    receipt = {"status": "error", "scalar": None,
                               "wall_ns": time.perf_counter_ns() - start,
                               "cpu_ns": time.process_time_ns() - cpu_start,
                               "error": repr(exc)}
            signal.alarm(0)
            pair["arms"][arm] = receipt
        pair["calibration_post_ns"] = calibration(G)
        a, b = pair["calibration_pre_ns"], pair["calibration_post_ns"]
        pair["calibration_ratio"] = max(a, b) / min(a, b)
        pair["calibration_pass"] = pair["calibration_ratio"] <= 1.25
        pair["scalar_certificate_pass"] = all(
            pair["arms"].get(arm, {}).get("status") == "verified"
            and pair["arms"][arm]["scalar"] == fixture["fixture_scalar"]
            and pair["arms"][arm]["scalar"] * G == Q for arm in ("IC", "rho"))
        pair["peak_rss_so_far_bytes"] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        result["pairs"].append(pair)
        checkpoint(result)
        if pair["peak_rss_so_far_bytes"] > 8 * 1024**3:
            result["status"] = "censored_memory_limit"
            break
    if result["status"] == "running":
        result["status"] = "complete" if len(result["pairs"]) == 5 and all(
            p["scalar_certificate_pass"] for p in result["pairs"]) else "incomplete"
except TimeoutError as exc:
    result.update(status="censored_process_timeout", error=str(exc))
finally:
    signal.alarm(0)
    result["peak_rss_bytes"] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    checkpoint(result)
print(json.dumps({k: v for k, v in result.items() if k != "pairs"}, indent=2, sort_keys=True))
print(json.dumps([{"index": p["index"], "order": p["order"],
                   "ic_ns": p["arms"].get("IC", {}).get("wall_ns"),
                   "rho_ns": p["arms"].get("rho", {}).get("wall_ns"),
                   "calibration_pass": p["calibration_pass"],
                   "scalar_certificate_pass": p["scalar_certificate_pass"]}
                  for p in result["pairs"]], indent=2))
