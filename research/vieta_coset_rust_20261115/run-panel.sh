#!/usr/bin/env bash
# Thin orchestration only. Native code computes results and analyses.
set -uo pipefail
[[ $# == 3 ]] || { echo 'Usage: run-panel.sh preflight|all OUTPUT_DIR QUALIFIER' >&2; exit 2; }
mode=$1 output=$(realpath "$2") qualifier=$(realpath "$3")
[[ "$mode" == preflight || "$mode" == all ]] || exit 2
[[ ! -e "$output/driver.jsonl" ]] || { echo 'Refusing to replace an existing run' >&2; exit 2; }
ulimit -v 8388608
ulimit -f 1048576
export VIETA_ISOLATED_RUN=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
deadline=$((SECONDS + 3600))
run_config() {
  local panel=$1 curve=$2 m=$3 rounds=$4 extra=${5:-}
  local remaining=$((deadline - SECONDS)) stem="$panel-$curve-m$m" code
  if (( remaining <= 0 )); then
    jq -cn --arg panel "$panel" --arg curve "$curve" --argjson m "$m" '{panel:$panel,curve:$curve,m:$m,status:"not_attempted_deadline"}' >> "$output/driver.jsonl"
    return
  fi
  local args=(bench "$curve" "$m" "$rounds" 120)
  [[ -z "$extra" ]] || args+=("$extra")
  jq -cn --arg panel "$panel" --arg curve "$curve" --argjson m "$m" --arg started "$(date -u +%FT%TZ)" '{panel:$panel,curve:$curve,m:$m,status:"started",started_at:$started}' >> "$output/driver.jsonl"
  VIETA_PANEL="$panel" timeout --signal=TERM --kill-after=5s "$remaining" "$qualifier" "${args[@]}" > "$output/$stem.jsonl" 2> "$output/$stem.stderr"
  code=$?
  jq -cn --arg panel "$panel" --arg curve "$curve" --argjson m "$m" --argjson code "$code" --arg finished "$(date -u +%FT%TZ)" '{panel:$panel,curve:$curve,m:$m,status:"finished",exit_code:$code,finished_at:$finished}' >> "$output/driver.jsonl"
}
if [[ "$mode" == preflight ]]; then
  run_config preflight word31 32 1
else
  for curve in word17 word31 word62 p224; do
    for m in 32 128 512 2048 8192; do run_config two "$curve" "$m" 10; done
  done
  for curve in word17 word31 word62 p224; do
    for m in 16 64 128; do run_config three "$curve" "$m" 10 three; done
  done
fi
failed=$(jq -s '[.[] | select(.status=="not_attempted_deadline" or (.status=="finished" and .exit_code!=0))] | length' "$output/driver.jsonl")
(( failed == 0 ))
