#!/usr/bin/env python3
"""Check the same-host A/A raw control and report paired timing noise."""

import argparse
import base64
import hashlib
import json
import math
import statistics
from collections import defaultdict
from pathlib import Path

from run_heldout_v2 import load_inputs, sha256
from run_aa_control_v2 import development_targets


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--admission", type=Path, required=True)
    parser.add_argument("--records", type=Path, required=True)
    args = parser.parse_args()
    manifest, heldout = load_inputs()
    admission = json.loads(args.admission.read_bytes())
    development, development_digest = development_targets(manifest)
    q = development[0]
    if q in heldout:
        raise ValueError("A/A control target belongs to heldout panel")
    streams = {(r["case"], r["replicate"]): r for r in manifest["streams"]}
    expected = []
    for replicate in manifest["replicate_ids"]:
        order = streams[(0, replicate)]["arm_order"]
        expected.extend((replicate, arm, "A") for arm in order)
        expected.extend((replicate, arm, "B") for arm in reversed(order))
    records = [json.loads(line) for line in args.records.open()]
    if len(records) != len(expected):
        raise ValueError("A/A record count mismatch")
    if len({r.get("host") for r in records}) != 1:
        raise ValueError("A/A records span multiple hosts")
    paired = defaultdict(dict)
    for ordinal, (record, (replicate, arm, run)) in enumerate(zip(records, expected)):
        if any(record.get(k) != v for k, v in (
                ("ordinal", ordinal), ("replicate", replicate),
                ("arm", arm), ("run", run), ("public_Q", q),
                ("binary_sha256", admission["binary_sha256"]),
                ("admission_sha256", sha256(args.admission)),
                ("development_targets_sha256", development_digest),
                ("physical_cpu_ids", admission["physical_cpu_ids"]),
                ("memory_limit_bytes", admission["memory_limit_bytes"]))):
            raise ValueError(f"A/A record {ordinal}: identity mismatch")
        if record["exit_code"] != 0 or record["timed_out"] or \
                record["input_error"] is not None:
            raise ValueError(f"A/A record {ordinal}: child failed")
        for name in ("stdout", "stderr"):
            raw = base64.b64decode(record[name + "_base64"], validate=True)
            if hashlib.sha256(raw).hexdigest() != record[name + "_sha256"]:
                raise ValueError(f"A/A record {ordinal}: {name} hash mismatch")
        report = record["native_report"]
        if json.loads(base64.b64decode(record["stdout_base64"])) != report or \
                report["complete_target_scalar_recovered"] is not True or \
                report["known_answer_in_solver"] is not False:
            raise ValueError(f"A/A record {ordinal}: native answer invalid")
        stream = streams[(0, replicate)]
        request = {"public_Q": q}
        if arm == "queued_aps":
            request["aps_source_seed_hex"] = stream["aps_source_seed_hex"]
            solution = report["solution"]["log"]
            if report["queue"]["errors"] or \
                    report["queue"]["dequeued_cells"] != 24:
                raise ValueError(f"A/A record {ordinal}: queue incomplete")
        else:
            request["rho_walk_seed_hex"] = stream["rho_walk_seed_hex"]
            request["rho_lane_restart_seed_hex"] = stream[
                "rho_lane_restart_seed_hex"]
            solution = report["log"]
        if record["input"] != request:
            raise ValueError(f"A/A record {ordinal}: input stream mismatch")
        expected_seed = stream["aps_source_seed_hex"] if arm == "queued_aps" \
            else stream["rho_walk_seed_hex"]
        if report["seed"] != expected_seed or report["public_Q"] != q:
            raise ValueError(f"A/A record {ordinal}: native seed or target mismatch")
        paired[(replicate, arm)][run] = (record, solution)
    noise = {}
    for arm in manifest["arms"]:
        ratios = {"external_cold_wall_ns": [], "child_cpu_ns": []}
        for replicate in manifest["replicate_ids"]:
            pair = paired[(replicate, arm)]
            if set(pair) != {"A", "B"} or pair["A"][1] != pair["B"][1]:
                raise ValueError(f"A/A {arm}/{replicate}: answer mismatch")
            a, b = pair["A"][0], pair["B"][0]
            for metric in ratios:
                ratios[metric].append(b[metric] / a[metric])
        noise[arm] = {metric: {"B_over_A_ratios": values,
                    "geometric_mean": math.exp(statistics.fmean(
                        math.log(r) for r in values)),
                    "max_absolute_log_ratio": max(abs(math.log(r))
                                                  for r in values)}
                    for metric, values in ratios.items()}
    print(json.dumps({"control": "same_host_AA_checked",
                      "records_sha256": sha256(args.records),
                      "calls": len(records), "noise": noise},
                     indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
