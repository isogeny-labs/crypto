#!/usr/bin/env bash
set -euo pipefail

cd /home/ec2-user/rho-tier0
result=queue-r3/results/a72dedd301a4490ebf3a5b39b80d7318
while systemctl is-active --quiet rho-tier0-full-r5.service; do
    sleep 30
done

if [[ -f "$result/summary.json" ]]; then
    python3 analyze.py --result-dir "$result" \
        --strict-runner isolated_worker_runner.py --output audit-r5.json \
        > audit-r5-dispatch.json 2> audit-r5-stderr.txt || true
fi

files=(manifest-r5.json preflight-r5.json worker-plan-r3.json
       build-r3.log source-r3.tar.gz "$result")
for receipt in audit-r5.json audit-r5-dispatch.json audit-r5-stderr.txt; do
    if [[ -f "$receipt" ]]; then
        files+=("$receipt")
    fi
done
tar -czf full-r5-evidence.tar.gz "${files[@]}"
sha256sum full-r5-evidence.tar.gz > full-r5-evidence.sha256
