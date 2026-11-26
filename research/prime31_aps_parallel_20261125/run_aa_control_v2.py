#!/usr/bin/env python3
"""Same-host cold A/A control on the archived development Q, before V2 admission."""

import argparse
import hashlib
import json
import os
import platform
import subprocess
import sys
from pathlib import Path

from run_heldout_v2 import ROOT, invoke, load_inputs, preflight, sha256


def development_targets(manifest):
    path = ROOT / "public_targets.json"
    raw = path.read_bytes() if path.exists() else subprocess.check_output(
        ["git", "show", "HEAD:research/prime31_aps_parallel_20261125/public_targets.json"],
        cwd=ROOT)
    digest = hashlib.sha256(raw).hexdigest()
    if digest != manifest["development_targets_sha256"]:
        raise RuntimeError("archived development target hash mismatch")
    targets = json.loads(raw)
    if len(targets) != 48:
        raise RuntimeError("archived development target count mismatch")
    return targets, digest


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--admission", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--execute", action="store_true")
    args = parser.parse_args()
    manifest, heldout_public = load_inputs()
    admission = json.loads(args.admission.read_bytes())
    if admission.get("admitted") is not False:
        raise RuntimeError("A/A admission must remain false")
    binary, cpus, memory = preflight(admission, manifest, require_aa=False)
    development, development_digest = development_targets(manifest)
    if development[0] in heldout_public:
        raise RuntimeError("development control target missing or in heldout panel")
    q = development[0]
    streams = {(r["case"], r["replicate"]): r for r in manifest["streams"]}
    schedule = []
    for replicate in manifest["replicate_ids"]:
        stream = streams[(0, replicate)]
        order = stream["arm_order"]
        schedule.extend((replicate, arm, "A", stream) for arm in order)
        schedule.extend((replicate, arm, "B", stream) for arm in reversed(order))
    assert len(schedule) == 30
    if not args.execute:
        print(json.dumps({"control": "preflight_passed", "calls": len(schedule),
                          "development_Q": q,
                          "development_targets_sha256": development_digest,
                          "binary_sha256": admission["binary_sha256"],
                          "cpus": cpus}, indent=2))
        return
    if args.output.exists():
        raise RuntimeError("A/A control output exists; preserve it and choose a new path")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("x") as handle:
        for ordinal, (replicate, arm, run, stream) in enumerate(schedule):
            result = invoke(binary, cpus, memory, 30, q, stream, arm)
            result.update({"control": "development_Q0_AA", "ordinal": ordinal,
                           "replicate": replicate, "run": run,
                           "development_targets_sha256": development_digest,
                           "admission_sha256": sha256(args.admission),
                           "binary_sha256": admission["binary_sha256"],
                           "host": platform.node(), "physical_cpu_ids": cpus,
                           "memory_limit_bytes": memory})
            handle.write(json.dumps(result, sort_keys=True) + "\n")
            handle.flush()
            os.fsync(handle.fileno())
            print(f"A/A {ordinal + 1}/{len(schedule)} {replicate=} {arm=} {run=}",
                  file=sys.stderr, flush=True)


if __name__ == "__main__":
    main()
