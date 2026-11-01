#!/bin/sh
set -eu

root=/home/ec2-user/rho
run=17f520f4a4f94a3ca8f0d49829e4ab8b
result="$root/results/$run"

if systemctl is-active --quiet rho-worker-production.service; then
    echo 'timed production worker service is still active' >&2
    exit 2
fi

python3 - "$result/summary.json" <<'PY'
import json
import sys

with open(sys.argv[1]) as handle:
    row = json.load(handle)
assert row['status'] == 'completed', row
assert row['completed_arms'] == 768, row
assert row['completed_pairs'] == 384, row
assert row['artifacts_stable'] is True, row
PY

python3 "$root/worker/analyze_production.py" \
    --result-dir "$result" \
    --output "$root/worker/production-audit.json"
tar -C "$root/results" -czf "$root/worker/production-raw.tar.gz" "$run"
journalctl -u rho-worker-production.service --no-pager --output=short-iso \
    > "$root/worker/production-service-journal.txt"
sha256sum "$root/worker/production-audit.json" \
    "$root/worker/production-raw.tar.gz" \
    "$root/worker/production-service-journal.txt" \
    "$root/worker/worker-production-plan.json" \
    "$root/worker/analyze_production.py" \
    "$root/worker/source.tar.gz" \
    "$root/worker/target/release/examples/rho_worker_batch_benchmark"
