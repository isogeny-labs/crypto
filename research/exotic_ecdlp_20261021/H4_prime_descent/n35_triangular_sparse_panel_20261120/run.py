#!/usr/bin/env python3
"""Five distinct local same-point IC/rho pairs for the combined n35 candidate."""
import hashlib
import importlib.util
import json
from pathlib import Path
import platform
import resource
import signal
import statistics
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
FIXTURES = json.loads((HERE / "fixtures.json").read_text())


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


assert sha(HERE / "run.py") == SPEC["runner_sha256"]
for name, expected in SPEC["source_hashes"].items():
    assert sha(ROOT / name) == expected
assert sha(HERE / "fixtures.json") == SPEC["fixtures_sha256"]
FULL_PATH = ROOT / SPEC["full_runner"]
module_spec = importlib.util.spec_from_file_location("n35_panel_full", FULL_PATH)
full = importlib.util.module_from_spec(module_spec)
module_spec.loader.exec_module(full)
SOURCE_RESULT = json.loads((ROOT / SPEC["full_result"]).read_text())
CANDIDATE = json.loads((ROOT / SPEC["candidate"]).read_text())
assert CANDIDATE["candidate_id"] == FIXTURES["candidate_id"] == SOURCE_RESULT["candidate_id"]
assert len(FIXTURES["targets"]) == 5


def save(out):
    (HERE / "result.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")


def timeout(*_):
    raise TimeoutError("frozen per-arm deadline")


def calibration(G):
    start = time.perf_counter_ns()
    P = G.curve()(0)
    for _ in range(SPEC["calibration_additions"]):
        P += G
    elapsed = time.perf_counter_ns() - start
    assert P == SPEC["calibration_additions"] * G
    return elapsed


def main():
    result = {"schema": "n35-triangular-sparse-local-panel-result-v1", "status": "running",
              "protocol_sha256": sha(HERE / "protocol.json"),
              "fixtures_sha256": SPEC["fixtures_sha256"],
              "candidate_id": CANDIDATE["candidate_id"],
              "host": {"node": platform.node(), "system": platform.system(),
                       "machine": platform.machine(), "tier": "ordinary_exploratory"},
              "rows": []}
    save(result)
    start = time.perf_counter_ns()
    t = time.perf_counter_ns()
    E, elt = full.build_context()
    context_ns = time.perf_counter_ns() - t
    t = time.perf_counter_ns()
    base, reps, encoding, G = full.build_base(E, elt)
    base_ns = time.perf_counter_ns() - t
    t = time.perf_counter_ns()
    index = full.build_index(base)
    index_ns = time.perf_counter_ns() - t
    assert len(index) == SPEC["distinct_pair_sums"]
    t = time.perf_counter_ns()
    logs = SOURCE_RESULT["factor_logs"]
    assert full.digest(logs) == SOURCE_RESULT["factor_logs_sha256"]
    assert len(logs) == len(reps) == 2048
    assert all(logs[i] * G == reps[i] for i in range(len(reps)))
    logs_replay_ns = time.perf_counter_ns() - t
    result["shared_setup"] = {"context_ns": context_ns, "base_ns": base_ns,
                              "index_ns": index_ns, "logs_replay_ns": logs_replay_ns,
                              "total_ns": time.perf_counter_ns() - start,
                              "factor_logs_sha256": SOURCE_RESULT["factor_logs_sha256"],
                              "distinct_pair_sums": len(index)}
    assert resource.getrusage(resource.RUSAGE_SELF).ru_maxrss < SPEC["rss_cap_bytes"]
    save(result)
    signal.signal(signal.SIGALRM, timeout)
    for item in FIXTURES["targets"]:
        Q = E(elt(item["point"][0]), elt(item["point"][1]))
        assert item["fixture_scalar"] * G == Q
        workload = {"curve_id": FIXTURES["curve_id"], "subgroup_order": full.R,
                    "G": full.point_record(G), "target": item["point"],
                    "target_count": 1, "target_law": "sha256_nonzero_scalar_times_G",
                    "target_label": item["tag"], "precomputation_state": "ready"}
        workload_id = "W" + full.digest(workload)[:12]
        row = {"index": item["index"], "target": item["point"],
               "fixture_scalar": item["fixture_scalar"], "tag": item["tag"],
               "workload_id": workload_id,
               "run_id": CANDIDATE["candidate_id"] + workload_id + "R1",
               "order": item["order"], "calibration_before_ns": calibration(G),
               "arms": {}, "status": "running"}
        result["rows"].append(row)
        save(result)
        for arm in item["order"]:
            signal.alarm(SPEC["ic_cap_seconds"] if arm == "IC" else SPEC["rho_cap_seconds"])
            try:
                data = full.ic_online(base, reps, encoding, G, index, logs, Q) if arm == "IC" else full.rho_online(G, Q)
                data["scalar_replay"] = data.get("scalar") == item["fixture_scalar"] and data["scalar"] * G == Q
                if not data["scalar_replay"]:
                    data["status"] = "invalid_scalar"
            except TimeoutError as exc:
                data = {"status": "censored_timeout", "error": str(exc)}
            except Exception as exc:
                data = {"status": "error", "error": repr(exc)}
            finally:
                signal.alarm(0)
            row["arms"][arm] = data
            save(result)
        row["calibration_after_ns"] = calibration(G)
        row["calibration_ratio"] = max(row["calibration_before_ns"], row["calibration_after_ns"]) / min(row["calibration_before_ns"], row["calibration_after_ns"])
        row["peak_rss_bytes"] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        row["resource_cap_exceeded"] = row["peak_rss_bytes"] >= SPEC["rss_cap_bytes"]
        row["status"] = "verified_pair" if not row["resource_cap_exceeded"] and all(row["arms"].get(arm, {}).get("status") == "verified" and row["arms"][arm].get("scalar_replay") for arm in ("IC", "rho")) else "incomplete_pair"
        save(result)
    result["status"] = "complete" if all(row["status"] == "verified_pair" for row in result["rows"]) else "incomplete"
    result["median_ic_online_ns"] = statistics.median(row["arms"]["IC"]["wall_ns"] for row in result["rows"] if row["status"] == "verified_pair") if any(row["status"] == "verified_pair" for row in result["rows"]) else None
    result["median_rho_online_ns"] = statistics.median(row["arms"]["rho"]["wall_ns"] for row in result["rows"] if row["status"] == "verified_pair") if any(row["status"] == "verified_pair" for row in result["rows"]) else None
    result["peak_rss_bytes"] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    save(result)
    print(json.dumps({k: v for k, v in result.items() if k != "rows"}, sort_keys=True), flush=True)
    for row in result["rows"]:
        print(json.dumps({"index": row["index"], "status": row["status"],
                          "ic_ns": row["arms"].get("IC", {}).get("wall_ns"),
                          "rho_ns": row["arms"].get("rho", {}).get("wall_ns")}, sort_keys=True), flush=True)


if __name__ == "__main__":
    main()
