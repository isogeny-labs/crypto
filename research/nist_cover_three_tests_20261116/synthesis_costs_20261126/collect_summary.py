#!/usr/bin/env python3
"""Collect terminal JSON/file-custody receipts; no field/group computation."""
import argparse
import json
from pathlib import Path
from check_evidence import check


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("attempts", nargs="+", type=Path)
    args = parser.parse_args()
    attempts = []
    for path in args.attempts:
        receipt = check(path, False)
        attempts.append({"attempt": path.name, "path": str(path), **receipt})
    summary = {
        "schema": "crypto.p192.fixed-synthesis-cost/v1",
        "scope": "construction from given recorded torsion/module/kernel inputs",
        "historical_input_generation_cost": "unknown; not part of these intervals",
        "timing_boundary": "Original construction chain including its builds and checks; initial static copy/snapshot and final file hash check excluded.",
        "attempt_count": len(attempts),
        "complete_chain_count": sum(a["complete_chain"] for a in attempts),
        "artifact_match_count": max(sum(bool(c.get("complete_json_equal"))
                                        for c in a["artifact_comparisons"]) for a in attempts),
        "executed_wrapper_seconds": round(sum(a["wrapper_cost"]["wall_seconds"] for a in attempts), 6),
        "nested_build_seconds": round(sum(a["nested_call_totals_not_additive_to_wrapper"]["build"]["wall_seconds"] for a in attempts), 6),
        "nested_native_seconds": round(sum(a["nested_call_totals_not_additive_to_wrapper"]["native"]["wall_seconds"] for a in attempts), 6),
        "attempts": attempts,
        "standing": "producer-unverified diagnostics; not a benchmark comparison",
    }
    print(json.dumps(summary, indent=2, sort_keys=True))
