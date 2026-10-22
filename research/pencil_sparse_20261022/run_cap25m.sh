#!/usr/bin/env bash
set -u
root=research/pencil_sparse_20261022
stem="$root/results/p186379-peel-cap25m-t0"
date -u +%Y-%m-%dT%H:%M:%SZ > "$stem.started"
(
  ulimit -v 8388608
  timeout 3600s env PENCIL_MAX_TARGETS=1 PENCIL_NODE_ORDER=identity \
    PENCIL_SOLVER=peel PENCIL_MATRIX_CAP=25000000 \
    "$root/target/release/pencil-sparse" --cohort 32 186379
) > "$stem.jsonl" 2> "$stem.stderr"
status=$?
printf '%s\n' "$status" > "$stem.exit"
date -u +%Y-%m-%dT%H:%M:%SZ > "$stem.ended"
python3 - "$status" "$stem" <<'PY'
import json
import sys
from pathlib import Path

exit_code = int(sys.argv[1])
stem = Path(sys.argv[2])
rows = []
for line in stem.with_suffix(".jsonl").read_text().splitlines():
    try:
        rows.append(json.loads(line))
    except json.JSONDecodeError:
        break
target = rows[1] if len(rows) > 1 else None
status = ("complete" if exit_code == 0 and target is not None else
          "timeout" if exit_code == 124 else "process_failure")
record = {
    "schema": "pencil-sparse-process/v1",
    "bits": 32,
    "p": 186379,
    "target_index": 0,
    "status": status,
    "exit_code": exit_code,
    "started_utc": stem.with_suffix(".started").read_text().strip(),
    "ended_utc": stem.with_suffix(".ended").read_text().strip(),
    "raw_rows": len(rows),
    "candidate_verified": target["verified"] if target else None,
    "failure": target["failure"] if target else status,
    "rho_verified": target["rho"]["verified"] if target else None,
}
stem.with_suffix(".status.json").write_text(json.dumps(record, indent=2) + "\n")
PY
exit "$status"
