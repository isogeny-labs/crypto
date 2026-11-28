#!/usr/bin/env bash
# This supervisor stays outside each guarded child's process group.
set -u
set -o pipefail
ROOT=$1
DRIVER=$2
AUDITOR=$3
CHECKER=$4
SOURCE=$(cd "$(dirname "$0")" && pwd)
export ECDLP_PHASE_ROOT="$ROOT"
STAGE="$SOURCE/run-stage-v2.sh"
set -o noclobber
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$ROOT/main-supervisor-started.txt" || exit 2
bash "$STAGE" 08-main-run 1500 "$DRIVER" run "$ROOT/main" "$ROOT/main-config.json" 10
DRIVER_CODE=$?
printf '%s\n' "$DRIVER_CODE" > "$ROOT/main-driver-exit.txt"
bash "$STAGE" 09-main-finalize 120 bash "$SOURCE/finalize-main-v2.sh" "$ROOT" "$DRIVER"
ANALYSIS_CODE=$?
printf '%s\n' "$ANALYSIS_CODE" > "$ROOT/main-analysis-exit.txt"
AUDIT_CODE=125
if [ "$DRIVER_CODE" -eq 0 ] && [ "$ANALYSIS_CODE" -eq 0 ]; then
  bash "$STAGE" 10-main-point-audit 120 "$AUDITOR" "$ROOT/main" "$CHECKER"
  AUDIT_CODE=$?
fi
printf '{"driver_exit":%s,"analysis_exit":%s,"point_audit_exit":%s}\n' "$DRIVER_CODE" "$ANALYSIS_CODE" "$AUDIT_CODE" > "$ROOT/main-disposition.json"
sha256sum -c "$ROOT/main-execution-before.sha256" > "$ROOT/main-post-call-check.log"
HASH_CODE=$?
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$ROOT/main-supervisor-finished.txt"
if [ "$DRIVER_CODE" -ne 0 ]; then exit "$DRIVER_CODE"; fi
if [ "$ANALYSIS_CODE" -ne 0 ]; then exit "$ANALYSIS_CODE"; fi
if [ "$AUDIT_CODE" -ne 0 ]; then exit "$AUDIT_CODE"; fi
exit "$HASH_CODE"
