#!/usr/bin/env bash
# Read-only metadata call, once; no wall benchmark or new solver execution.
set -u
cd /workspace/ic-local-runs/successor-review-20261124-01 || exit 65
(set -o noclobber; : > started) || exit 65
sha256sum -c runtime.sha256 > prehash.log 2>&1 || exit 66
timeout 120 prlimit --as=8589934592 -- ./worker \
  /workspace/ic-runpod-runs/runpod-unit-deferred-20261117-51/returned/task/run/public.json \
  /workspace/ic-runpod-runs/runpod-unit-deferred-20261117-51/returned/task/run/pair-unit-five.raw.json \
  /workspace/crypto-action-local-20261123/research/extension_ic_graph_20261115/template-cost-20261124/results/analysis/derived.json \
  /workspace/ic-local-runs/template-cost-20261124-01/output/summary.json \
  result.json > worker.stdout 2> worker.stderr
status=$?
printf '%s\n' "$status" > native.status
sha256sum -c runtime.sha256 > posthash.log 2>&1
hash_status=$?
printf '%s\n' "$hash_status" > posthash.status
if [ "$status" -ne 0 ]; then exit "$status"; fi
exit "$hash_status"
