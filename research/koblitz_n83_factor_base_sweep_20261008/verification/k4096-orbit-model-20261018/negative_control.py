#!/usr/bin/env python3
"""Copy retained K4096 receipts and reject a coherently rehashed false model count."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import shutil

import audit


def write(path: Path, value: dict) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkout", type=Path, required=True)
    parser.add_argument("--evidence", type=Path, required=True)
    parser.add_argument("--object", type=Path, required=True)
    parser.add_argument("--scratch", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    copied = args.scratch / "retained-copy"
    forged = args.scratch / "forged-model-count"
    shutil.copytree(args.evidence, copied)
    positive = audit.audit(args.checkout, copied, args.object)
    shutil.copytree(copied, forged)
    worker_path = forged / "capacity-02/worker.json"
    outer_path = forged / "capacity-02/outer.json"
    worker, outer = audit.read(worker_path), audit.read(outer_path)
    worker["sat_clauses"] += 1
    write(worker_path, worker)
    outer["worker_sha256"] = audit.digest(worker_path)
    write(outer_path, outer)
    try:
        audit.audit(args.checkout, forged, args.object)
    except ValueError as exc:
        reason = str(exc)
        audit.require("corrected model receipt differs" in reason,
                      f"negative rejected for wrong reason: {reason}")
    else:
        raise AssertionError("forged model count was accepted")
    result = {
        "schema": "n83.k4096-orbit-model-negative-control/v1",
        "status": "PASS_FORGED_MODEL_REJECTED",
        "retained_copy_status": positive["status"],
        "mutated_field": "capacity-02/worker.json:sat_clauses",
        "original_sat_clauses": 2021197,
        "forged_sat_clauses": worker["sat_clauses"],
        "outer_worker_sha256_recomputed": outer["worker_sha256"],
        "rejection": reason,
        "selected_best_total_runtime": None,
    }
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
