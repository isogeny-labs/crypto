#!/bin/sh
set -eu

first=/home/ec2-user/rho/results/b9a2ed25901949fa8a888bf97117c1ed
plan=/home/ec2-user/rho/worker/worker-fullwidth-plan.json

while systemctl is-active --quiet rho-worker-freshq.service; do
    sleep 5
done

python3 - "$first/summary.json" <<'PY'
import json
import sys
summary = json.load(open(sys.argv[1]))
assert summary['status'] == 'completed'
assert summary['completed_arms'] == 768
assert summary['completed_pairs'] == 384
assert summary['artifacts_stable'] is True
PY

printf '%s  %s\n' \
    4a5e38e2f1ed29bce319b34f7bc521cd91b82c231ad6c2e2bad7b933bda3d5a4 \
    "$plan" | sha256sum --check --status

exec /usr/bin/taskset -c 0 /usr/bin/python3 \
    /home/ec2-user/rho/worker/run_tier1_worker_batch.py \
    --manifest "$plan" --queue-root /home/ec2-user/rho
