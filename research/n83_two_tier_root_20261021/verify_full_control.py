"""Independent group replay for one N83 K4405 split-index local control."""

import argparse
import hashlib
import json
import math
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO))
from research.n83_highk_online_distribution_20261020 import arms, generate_fresh  # noqa: E402

GROUP = generate_fresh.group
PHASE_KEYS = ("target_query_ms", "target_pdp_ms",
              "target_relation_check_ms", "target_descent_ms",
              "target_recovery_check_ms")


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def replay(root):
    input_record = json.loads((root / "input.json").read_text())
    process_file = root / "process_result.json"
    if not process_file.exists():
        return {"status": "RUN_PENDING"}
    process = json.loads(process_file.read_text())
    if process["status"] != "PASS_PROCESS" or process["exit_code"] != 0:
        guard_path = root / "n83_split_k4405_disk_guard_result.json"
        if guard_path.exists():
            guard = json.loads(guard_path.read_text())
            guard_script = root / "n83_disk_guard.py"
            if (guard_script.exists() and
                    sha256(guard_script) ==
                    "a625fb2eab1e2aa3b77f89b54768df03fac35b2cb3a2a3f5c934fce94eb772b7" and
                    guard.get("schema") == "n83.local-disk-guard/v1" and
                    guard.get("status") == "DISK_GUARD_STOP" and
                    guard.get("min_free_bytes") == 8 * 1024**3 and
                    isinstance(guard.get("observed_free_bytes"), int) and
                    guard["observed_free_bytes"] < guard["min_free_bytes"] and
                    process["status"] == "FAILED_PROCESS" and
                    process["exit_code"] == -15):
                return {"status": "BOUNDED_UNKNOWN",
                        "reason": "DISK_GUARD_STOP", "process": process,
                        "disk_guard": guard}
        status = ("BOUNDED_UNKNOWN" if process["status"].startswith("TIMEOUT_")
                  or process["status"] == "RSS_CAP" else "PRODUCER_FAILURE")
        return {"status": status, "process": process}
    try:
        if process["sampled_peak_rss_bytes"] > input_record["rss_cap_bytes"]:
            raise ValueError("sampled RSS exceeded frozen cap")
        committed_source = subprocess.check_output(
            ["git", "show", input_record["source_head"] +
             ":examples/koblitz_orbit_dlp_fast.rs"], cwd=REPO)
        if hashlib.sha256(committed_source).hexdigest() != input_record["source_sha256"]:
            raise ValueError("source_sha256 differs from frozen source commit")
        for name, path in (("binary_sha256", Path(input_record["command"][0])),
                           ("runner_sha256", root / "runner.py"),
                           ("target_sha256", root / "target.jsonl")):
            if sha256(path) != input_record[name]:
                raise ValueError(f"{name} differs from frozen input")
        summary = json.loads((root / "stdout.jsonl").read_text())
        rows = [json.loads(line) for line in
                (root / "targets_result.jsonl").read_text().splitlines() if line]
        variant = input_record["variant"]
        if (summary.get("n") != 83 or summary.get("a") != 1 or
                summary.get("orbit_columns") != 4405 or
                summary.get("factor_base_points") != 166 * 4405 or
                summary.get("regular_states") != 805269240 or
                summary.get("rank") != 4405 or
                summary.get("targets") != 1 or summary.get("targets_solved") != 1 or
                summary.get("targets_failed") != 0 or len(rows) != 1 or
                summary.get("indexed5_split_at") != 4300 or
                summary.get("shared_bloom_enabled") is not (variant == "shared") or
                summary.get("root_table_slot_bytes") != 5 or
                summary.get("root_state_slot_bytes") != 11 or
                summary.get("primary_table_allocated_bytes") != 5 * (1 << 31) or
                summary.get("overflow_table_allocated_bytes") != 5 * (1 << 27) or
                summary.get("root_table_allocated_bytes") !=
                5 * ((1 << 31) + (1 << 27)) or
                summary.get("root_bloom_bytes") !=
                ((1 << 31) + (0 if variant == "shared" else (1 << 27))) or
                summary.get("root_bloom_profile") != "wide3"):
            raise ValueError("curve, base, rank, split layout or target count differs")
        arm = next(arm for arm in arms.ARMS if arm[0] == "k4405_wide3")
        candidate_id = arm[3]
        manifest_path = ("research/n83_root4_frontier_current_20261018/"
                         "highk_20261019/identities/k4405_wide3_identity/"
                         f"candidate_{candidate_id}.json")
        manifest = json.loads(subprocess.check_output(
            ["git", "show", input_record["source_head"] + ":" + manifest_path],
            cwd=REPO))
        stripped = {key: value for key, value in manifest.items()
                    if key != "candidate_id"}
        digest = hashlib.sha256(json.dumps(stripped, sort_keys=True,
                                          separators=(",", ":"),
                                          ensure_ascii=False).encode()).hexdigest()[:12]
        if (manifest["candidate_id"] != candidate_id or
                not candidate_id.endswith("h" + digest) or
                manifest["factor_base"]["actual_usable_points"] != 166 * 4405):
            raise ValueError("frozen K4405 candidate identity differs")
        base_file = root / "base_dump.jsonl"
        if (summary["base_hash"] !=
                manifest["factor_base"]["source_base_hash_blake3"] or
                sha256(base_file) !=
                manifest["factor_base"]["ordered_base_dump_sha256"]):
            raise ValueError("factor base differs from frozen K4405 candidate")
        base = GROUP.replay_base(json.loads(base_file.read_text()), summary)
        target = GROUP.point_from_json(json.loads((root / "target.jsonl").read_text()))
        scalar = input_record["expected_scalar"]
        row = rows[0]
        indices = row["point_indices"]
        if (len(base) != 166 * 4405 or len(indices) != 4 or
                any(not isinstance(i, int) or i < 0 or i >= len(base)
                    for i in indices) or
                int(row["recovered_scalar"]) != scalar or
                row.get("group_verified") is not True or
                GROUP.multiply(GROUP.G, scalar) != target):
            raise ValueError("target scalar or relation indices differ")
        summed = None
        for index in indices:
            summed = GROUP.add(summed, base[index])
        if summed != target:
            raise ValueError("independent four-point relation sum differs")
        phases = {key: float(row[key]) for key in PHASE_KEYS}
        online = float(row["target_ms"])
        if (not math.isfinite(online) or online <= 0 or
                any(not math.isfinite(value) or value < 0
                    for value in phases.values()) or
                abs(math.fsum(phases.values()) - online) > 1):
            raise ValueError("online phase accounting differs")
        return {
            "status": "PASS_INDEPENDENT_REPLAY",
            "variant": variant,
            "base_hash": summary["base_hash"],
            "base_dump_sha256": sha256(base_file),
            "recovered_scalar": scalar,
            "point_indices": indices,
            "online_ms": online,
            "online_phases_ms": phases,
            "cold_process_ms": summary["timing_ms"]["process_total"],
            "root_table_allocated_bytes": summary["root_table_allocated_bytes"],
            "root_bloom_bytes": summary["root_bloom_bytes"],
            "sampled_peak_rss_bytes": process["sampled_peak_rss_bytes"],
            "binary_sha256": input_record["binary_sha256"],
            "source_sha256": input_record["source_sha256"],
        }
    except Exception as error:
        return {"status": "VERIFICATION_FAILED", "error": str(error)}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    args = parser.parse_args()
    root = args.root.resolve(strict=True)
    result = replay(root)
    (root / "verification.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(result, sort_keys=True))
    raise SystemExit(0 if result["status"] == "PASS_INDEPENDENT_REPLAY" else 1)


if __name__ == "__main__":
    main()
