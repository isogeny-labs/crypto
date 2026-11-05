#!/usr/bin/env bash
# Thin execution wrapper: frozen record, checkpoint gate and parent limits.
set -euo pipefail

if [ "$#" -lt 2 ]; then
  echo 'usage: run-stage.sh RUN_DIRECTORY {preflight|calibrate|evaluate|validate} NATIVE_ARGUMENTS...' >&2
  exit 2
fi
run_directory="$1"
stage="$2"
shift 2
case "$stage" in preflight|calibrate|evaluate|validate) ;; *) echo 'unknown stage' >&2; exit 2 ;; esac

task_root="$(git rev-parse --show-toplevel)"
cd "$task_root"
study=research/quotient_calibration_20261025/target-native
protocol="$study/protocol.json"
binary="$study/target/release/quotient-target-calibration"

jq -e '.freeze.status == "frozen" and .freeze.filed_before_affected_data == true' "$protocol" >/dev/null || {
  echo 'execution record is not frozen before affected data; no experiment launched' >&2
  exit 2
}
expected_binary="$(jq -r '.arms[] | select(.id == "candidate") | .instrument.binary_sha256' "$protocol")"
actual_binary="$(sha256sum "$binary" | cut -d' ' -f1)"
[ "$expected_binary" = "$actual_binary" ] || { echo 'native binary hash differs from frozen instrument' >&2; exit 2; }

scripts/checkpoint.sh gate
mkdir -p "$run_directory"
if [ ! -f "$run_directory/deadline-epoch.txt" ]; then
  ( set -o noclobber; date -u +%s | awk '{print $1+3600}' > "$run_directory/deadline-epoch.txt" )
fi
deadline="$(cat "$run_directory/deadline-epoch.txt")"
remaining=$((deadline - $(date -u +%s)))
if [ "$remaining" -le 0 ] || [ "$remaining" -gt 3600 ]; then
  echo 'parent 3600-second execution window exhausted or invalid' >&2
  exit 124
fi
for suffix in stdout.log stderr.log exit.txt started.txt; do
  [ ! -e "$run_directory/$stage.$suffix" ] || { echo 'stage receipt already exists; overwrite refused' >&2; exit 2; }
done
date -u +%Y-%m-%dT%H:%M:%SZ > "$run_directory/$stage.started.txt"
ulimit -v 2097152
if RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 \
  timeout --kill-after=5s "$remaining" /usr/bin/time -v "$binary" "$stage" "$@" \
  > "$run_directory/$stage.stdout.log" 2> "$run_directory/$stage.stderr.log"; then
  native_status=0
else
  native_status=$?
fi
printf '%s\n' "$native_status" > "$run_directory/$stage.exit.txt"
cat "$run_directory/$stage.stdout.log"
tail -25 "$run_directory/$stage.stderr.log" >&2
if ! scripts/checkpoint.sh push; then
  echo 'stage ended; its remote checkpoint failed and must be repaired before the next launch' >&2
  [ "$native_status" -ne 0 ] || native_status=2
fi
exit "$native_status"
