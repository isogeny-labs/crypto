"""Count observed completion states from raw records; no cryptographic work."""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
RESULTS = ROOT / "results"
validation = json.loads((RESULTS / "validation.json").read_text())
solved = set()
failed = []
arms = []
for path in sorted(RESULTS.glob("p*-*.jsonl")):
    rows = [json.loads(line) for line in path.read_text().splitlines()]
    assert rows[0]["schema"] == "pencil-cohort-setup/v1"
    counts = {"raw_path": str(path.relative_to(ROOT)), "bits": rows[0]["bits"],
              "p": rows[0]["p"], "target_rows": len(rows) - 1,
              "verified": 0, "failed": 0}
    for row in rows[1:]:
        if row["verified"] is True:
            assert row["rho"]["verified"] is True
            solved.add(row["workload_id"])
            counts["verified"] += 1
        else:
            failed.append({"raw_path": str(path.relative_to(ROOT)),
                           "workload_id": row["workload_id"],
                           "failure": row["failure"],
                           "candidate_ratio": None})
            counts["failed"] += 1
    arms.append(counts)

status = {
    "schema": "pencil-sparse-cohort-status/v1",
    "original_cohort_targets": 1260,
    "source_verified_distinct_targets": len(solved),
    "distinct_targets_still_open": 1260 - len(solved),
    "overall_status": "partial" if len(solved) < 1260 else "complete",
    "validation_status": validation["status"],
    "pending_replay_paths": validation["missing_replays"],
    "new_study_arms": arms,
    "failed_target_records": failed,
    "process_outcomes": [json.loads(path.read_text()) for path in
                         sorted(RESULTS.glob("p*-*.status.json"))],
    "note": "A repeated arm on an existing workload is not a new distinct target; "
            "a stopped run has no candidate cost ratio.",
}
(RESULTS / "status.json").write_text(json.dumps(status, indent=2, sort_keys=True) + "\n")
