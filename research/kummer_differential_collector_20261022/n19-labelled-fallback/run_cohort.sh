#!/usr/bin/env bash
set -uo pipefail
cd /var/tmp/crypto-pr2304-full-b5ca || exit 70
base=research/kummer_differential_collector_20261022/n19-labelled-fallback
mkdir -p "$base/results/paired"
exec 9>"$base/results/cohort.lock"
flock -n 9 || exit 75
started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
sha256sum \
  /dev/shm/n19-target/release/paired_labelled \
  research/kummer_differential_collector_20261022/src/main_labelled.rs \
  research/kummer_differential_collector_20261022/src/bin/paired_labelled.rs \
  "$base/PROTOCOL.md" >"$base/results/inputs.sha256"
env -u KUMMER_TARGET_CAP -u KUMMER_START_INDEX -u KUMMER_VALIDATE_TRANSITIONS \
  /dev/shm/n19-target/release/paired_labelled >"$base/results/cohort-run.log" 2>&1
status=$?
finished=$(date -u +%Y-%m-%dT%H:%M:%SZ)
printf '{"schema":"n19-cohort-receipt/v1","started_utc":"%s","finished_utc":"%s","exit_code":%d,"unit":"n19-labelled-paired.service","validation_mode":false}\n' \
  "$started" "$finished" "$status" >"$base/results/cohort-receipt.json"
exit "$status"
