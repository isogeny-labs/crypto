#!/usr/bin/env bash
set -u
cd /workspace/ic-local-runs/forced-divisor-h6-fiber-20261124-01 || exit 65
(set -o noclobber; : > launched) || exit 65
sha256sum -c runtime.sha256 > prehash.log 2>&1 || exit 66
RAYON_NUM_THREADS=1 F4_FP_THREAD_OPS=1 JV_LIN=fast \
 ./isolated-bench run --cpus 0 --out conditions.jsonl --settle 2 \
 --max-other-cpu 0.10 --max-psi 5.0 --label forced-divisor-h6-fiber-20261124-01 -- \
 prlimit --as=8589934592 -- ./worker run seed.json geometry.json output \
 > worker.stdout 2> worker.stderr
status=$?
printf '%s\n' "$status" > native.status
sha256sum -c runtime.sha256 > posthash.log 2>&1
hash_status=$?
printf '%s\n' "$hash_status" > posthash.status
if [ "$status" -ne 0 ]; then exit "$status"; fi
exit "$hash_status"
