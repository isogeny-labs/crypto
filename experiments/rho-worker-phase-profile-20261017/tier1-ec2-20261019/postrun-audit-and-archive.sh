#!/bin/sh
set -eu

root=/home/ec2-user/rho
first=b9a2ed25901949fa8a888bf97117c1ed
full=1ce77fcd6f2f47d28afc1534b3f33a18

if systemctl is-active --quiet rho-worker-freshq.service || \
   systemctl is-active --quiet rho-worker-fullwidth-pinned.service; then
    echo 'timed worker service is still active' >&2
    exit 2
fi

python3 - "$root/results/$first/summary.json" "$root/results/$full/summary.json" <<'PY'
import json
import sys
for name in sys.argv[1:]:
    row = json.load(open(name))
    assert row['status'] == 'completed', name
    assert row['completed_arms'] == 768, name
    assert row['completed_pairs'] == 384, name
    assert row['artifacts_stable'] is True, name
PY

python3 "$root/worker/analyze_tier1_worker_batch.py" \
    --result-dir "$root/results/$first" \
    --output "$root/worker/first-audit.json"
python3 "$root/worker/analyze_fullwidth.py" \
    --result-dir "$root/results/$full" \
    --output "$root/worker/fullwidth-audit.json"

tar -C "$root/results" -czf "$root/worker/first-raw.tar.gz" "$first"
tar -C "$root/results" -czf "$root/worker/fullwidth-raw.tar.gz" "$full"

journalctl -u rho-worker-freshq.service \
    -u rho-worker-fullwidth-pinned.service --no-pager --output=short-iso \
    > "$root/worker/service-journal.txt"

sha256sum "$root/worker/first-audit.json" \
    "$root/worker/fullwidth-audit.json" \
    "$root/worker/first-raw.tar.gz" \
    "$root/worker/fullwidth-raw.tar.gz" \
    "$root/worker/service-journal.txt" \
    "$root/worker/source.tar.gz" \
    "$root/worker/target/release/examples/rho_worker_batch_benchmark" \
    "$root/worker/tier1-freshq-plan.json" \
    "$root/worker/worker-fullwidth-plan.json" \
    "$root/worker/manifest.json"
