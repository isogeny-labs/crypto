#!/usr/bin/env bash
set -euo pipefail
# Run from the repository root only after source and frozen protocol are hosted.
cm_dir=research/cm-marked-scalar
cm_protocol=$cm_dir/protocol/cache-cold-v2/protocol.json
cm_run=$cm_dir/cold-v2/attempt-001
cm_build=/tmp/cm-marked-scalar-build
test ! -e "$cm_run"
test -s "$cm_dir/protocol/cache-cold-v2/freeze-publication.json"
cm_digest=$(jq -er '.freeze.protocol_sha256' "$cm_protocol")
for cm_pair in 'cache-eval arms[0].instrument.binary_sha256' 'cache-analysis execution.placement.analysis_binary_sha256' 'isolate execution.placement.isolation_binary_sha256'; do
  read -r cm_program cm_key <<< "$cm_pair"
  test "$(sha256sum "$cm_build/$cm_program" | cut -d' ' -f1)" = "$(jq -er ".$cm_key" "$cm_protocol")"
done
mkdir -p "$cm_run"
cp "$cm_dir/protocol/cache-cold-v2/launch.sh" "$cm_run/launch.sh"
for cm_panel in calibration heldout; do
  set +e
  (ulimit -v 2097152
   env OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1 CM_PROTOCOL_SHA="$cm_digest" \
     timeout 120 "$cm_build/isolate" 14,30 "$cm_run/$cm_panel-receipt.json" "$cm_run/$cm_panel-rows.jsonl" -- \
       "$cm_build/cache-eval" --bench "$cm_dir/fixtures-v2" "$cm_dir/residual-v1/plans.json" "$cm_protocol" "$cm_panel"
  ) > "$cm_run/$cm_panel-launch.stdout" 2> "$cm_run/$cm_panel-launch.stderr"
  cm_status=$?
  printf '%s\n' "$cm_status" > "$cm_run/$cm_panel.exit"
  set -e
  if test "$cm_status" -ne 0; then
    printf '%s\n' "Stopped after $cm_panel launch failure/refusal; no retry or dependent panel." > "$cm_run/stopping.txt"
    exit "$cm_status"
  fi
  set +e
  (ulimit -v 2097152
   env OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1 timeout 120 "$cm_build/cache-analysis" --check-panel "$cm_protocol" "$cm_panel" "$cm_run/$cm_panel-rows.jsonl" "$cm_run/$cm_panel-receipt.json"
  ) > "$cm_run/$cm_panel-check.json" 2> "$cm_run/$cm_panel-check.stderr"
  cm_status=$?
  printf '%s\n' "$cm_status" > "$cm_run/$cm_panel-check.exit"
  set -e
  if test "$cm_status" -ne 0; then
    printf '%s\n' "Stopped after $cm_panel native validation failed; no retry or dependent panel." > "$cm_run/stopping.txt"
    exit "$cm_status"
  fi
done
(ulimit -v 2097152
 env OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1 timeout 120 "$cm_build/cache-analysis" --analyze "$cm_protocol" "$cm_run/calibration-rows.jsonl" "$cm_run/calibration-receipt.json" "$cm_run/heldout-rows.jsonl" "$cm_run/heldout-receipt.json"
) > "$cm_run/analysis.json" 2> "$cm_run/analysis.stderr"
