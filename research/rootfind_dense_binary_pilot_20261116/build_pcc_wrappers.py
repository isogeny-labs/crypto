#!/usr/bin/env python3
"""Bind the local K13 diagnostic to unsigned PCC evidence wrappers.

These producer-authored wrappers make source arithmetic and input hashes
inspectable. They do not establish independent PCC verification.
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
STUDY = Path(__file__).resolve().parent
RAW = STUDY / "sessions/local-custody-rerun-202611160401/records.jsonl"
AUDIT = STUDY / "audits/custody-rerun/replay-all.json"
ANALYSIS = STUDY / "reports/custody-rerun/analysis.json"
OUT = STUDY / "reports/custody-rerun"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(path: Path, obj: dict) -> None:
    path.write_text(json.dumps(obj, indent=2, sort_keys=True) + "\n")


def main() -> None:
    records = [json.loads(line) for line in RAW.read_text().splitlines()]
    audit = json.loads(AUDIT.read_text())
    analysis = json.loads(ANALYSIS.read_text())
    measured = [row for row in records if not row["warmup"]]
    if len(records) != 200 or len(measured) != 160:
        raise ValueError("unexpected record count")
    if any(row["outcome"]["status"] != "verified" or not row["outcome"]["matches_target"] for row in records):
        raise ValueError("an answer failed the ecbench equality check")
    if not audit["ok"] or audit["verified_records"] != 200 or len(audit["replays"]) != 160:
        raise ValueError("audit count or status mismatch")
    if any(not replay["reproduced"] for replay in audit["replays"]):
        raise ValueError("deterministic replay mismatch")

    paired: dict[tuple[str, int], dict[str, int]] = {}
    for row in measured:
        if row["arm"] in ("batch-m3", "f6-ic"):
            key = (row["workload"]["workload_id"], row["round"])
            arm_times = paired.setdefault(key, {})
            if row["arm"] in arm_times:
                raise ValueError(f"duplicate arm in {key}")
            arm_times[row["arm"]] = row["time"]["process_wall_ns"]
    if len(paired) != 32 or any(set(times) != {"batch-m3", "f6-ic"} for times in paired.values()):
        raise ValueError("incomplete exact target-round pairs")
    batch_total = sum(times["batch-m3"] for times in paired.values())
    f6_total = sum(times["f6-ic"] for times in paired.values())
    ratio = batch_total / f6_total
    claimed = analysis["comparisons"]["batch-m3/f6-ic"]["sum_process_wall_ratio"]
    if abs(ratio - claimed) > 1e-14:
        raise ValueError("independent code path disagrees with analysis.json")

    replay = {
        "schema": "pcc.local-unsigned-replay-wrapper/v1",
        "status": "pass",
        "scalars_verified": True,
        "runs": [{"record_id": item["record_id"], "seq": item["seq"]} for item in audit["replays"]],
        "source_audit": {"path": AUDIT.relative_to(ROOT).as_posix(), "sha256": digest(AUDIT)},
        "note": "Producer-authored wrapper of ecbench audit; no independent signature.",
    }
    recompute = {
        "schema": "pcc.local-unsigned-recompute-wrapper/v1",
        "status": "pass",
        "raw_data": [
            {"path": RAW.relative_to(ROOT).as_posix(), "sha256": digest(RAW)},
            {"path": AUDIT.relative_to(ROOT).as_posix(), "sha256": digest(AUDIT)},
            {"path": ANALYSIS.relative_to(ROOT).as_posix(), "sha256": digest(ANALYSIS)},
        ],
        "headline": [{"name": "batch-m3/f6-ic summed complete-cold wall ratio", "claimed": claimed,
                      "recomputed": ratio, "rel_tol": 1e-12}],
        "pair_count": len(paired),
        "batch_total_process_wall_ns": batch_total,
        "f6_total_process_wall_ns": f6_total,
        "note": "Producer-authored arithmetic check; no independent signature.",
    }
    write(OUT / "pcc-replay-wrapper.json", replay)
    write(OUT / "pcc-recompute-wrapper.json", recompute)
    print(json.dumps({"pairs": len(paired), "ratio": ratio, "replays": len(audit["replays"])}))


if __name__ == "__main__":
    main()
