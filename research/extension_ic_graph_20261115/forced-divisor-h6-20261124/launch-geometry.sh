#!/usr/bin/env bash
# Sole registered retained-point geometry call. No solver/timing benchmark.
set -u
cd /workspace/ic-local-runs/forced-divisor-h6-20261124-01 || exit 65
(set -o noclobber; : > geometry.started) || exit 65
sha256sum -c runtime.sha256 > geometry.prehash.log 2>&1 || exit 66
timeout 120 prlimit --as=8589934592 -- ./worker geometry public-fixture.json geometry.json \
  > geometry.stdout 2> geometry.stderr
status=$?
printf '%s\n' "$status" > geometry.native.status
sha256sum -c runtime.sha256 > geometry.posthash.log 2>&1
hash_status=$?
printf '%s\n' "$hash_status" > geometry.posthash.status
if [ "$status" -ne 0 ]; then exit "$status"; fi
exit "$hash_status"
