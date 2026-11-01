#!/usr/bin/env bash
# Frozen producer for one prospective prime panel. Every planned attempt gets
# its own raw, stdout, stderr, and status file; failures are never replaced.
set -euo pipefail

if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "usage: $0 P [OUTPUT_ROOT]" >&2
  exit 2
fi

P=$1
case "$P" in
  1009|1511|1823) ;;
  *) echo "P must be 1009, 1511, or 1823" >&2; exit 2 ;;
esac

STUDY=research/jv_sieve_validation_20261101
ROOT=${2:-$STUDY/runs}
PANEL=$ROOT/p$P
BIN=./target/release/examples/jv_cover_ic_vs_rho_bitops
ANALYSE=./target/release/examples/jv_validation_analyse
PROTOCOL=$STUDY/protocol.json
JOBS=${JV_VALIDATION_JOBS:-4}
SEEDS=(33 34 35 36 37 38 39 40)

mkdir -p "$PANEL/raw" "$PANEL/status" "$PANEL/stdout" "$PANEL/stderr"
if [[ ! -x "$BIN" || ! -x "$ANALYSE" ]]; then
  echo "build the frozen examples before running the panel" >&2
  exit 2
fi

{
  for seed in "${SEEDS[@]}"; do
    printf 'ic_s%s\tic\t%s\t0\n' "$seed" "$seed"
    printf 'rho_R3_run0_s%s\tr3\t%s\t0\n' "$seed" "$seed"
    printf 'rho_R0_run0_s%s\tr0\t%s\t0\n' "$seed" "$seed"
    printf 'rho_R3_run1_s%s\tr3\t%s\t1\n' "$seed" "$seed"
  done
  printf 'ic_aa_s33\tic_aa\t33\t0\n'
} > "$PANEL/plan.tsv"

jq -n \
  --arg schema 'jv_sieve_validation_execution/v1' \
  --argjson p "$P" \
  --arg source_commit "$(git rev-parse HEAD)" \
  --arg protocol_sha256 "$(jq -r '.freeze.protocol_sha256' "$PROTOCOL")" \
  --arg producer_sha256 "$(sha256sum "$BIN" | awk '{print $1}')" \
  --arg analysis_sha256 "$(sha256sum "$ANALYSE" | awk '{print $1}')" \
  --arg kernel "$(uname -srvmo)" \
  --arg cpu "$(awk -F: '/model name/{sub(/^ /,"",$2); print $2; exit}' /proc/cpuinfo)" \
  --arg memory_bytes "$(awk '/MemTotal/{print $2 * 1024}' /proc/meminfo)" \
  --argjson jobs "$JOBS" \
  '{schema:$schema,p:$p,source_commit:$source_commit,git_dirty:false,
    protocol_sha256:$protocol_sha256,producer_sha256:$producer_sha256,
    analysis_sha256:$analysis_sha256,kernel:$kernel,cpu:$cpu,
    memory_bytes:$memory_bytes,jobs:$jobs,primary_clock:"counted",
    wall_time:"diagnostic_only",seeds:[33,34,35,36,37,38,39,40]}' \
  > "$PANEL/execution.json"

export P PANEL BIN
run_one() {
  local id=$1 kind=$2 seed=$3 run=$4
  local raw="$PANEL/raw/$id.jsonl"
  local stdout="$PANEL/stdout/$id.log"
  local stderr="$PANEL/stderr/$id.log"
  local status="$PANEL/status/$id.json"
  local start end rc valid raw_sha command
  start=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  case "$kind" in
    ic|ic_aa)
      command="$BIN --arm ic --p $P --seeds $seed --moves 8 --jobs 1 --json $raw"
      ;;
    r3)
      command="$BIN --arm rho --p $P --seeds $seed --moves 8 --rho R3 --runs $run --jobs 1 --json $raw"
      ;;
    r0)
      command="$BIN --arm rho --p $P --seeds $seed --moves 8 --rho R0 --runs $run --jobs 1 --json $raw"
      ;;
    *) echo "unknown kind $kind" >&2; return 2 ;;
  esac
  set +e
  JV_SIEVE_ROUND_THREADS=4 F4_FP_THREAD_OPS=1 bash -c "$command" >"$stdout" 2>"$stderr"
  rc=$?
  set -e
  valid=false
  if [[ -s "$raw" ]] && [[ $(wc -l < "$raw") -eq 1 ]] && jq -e . "$raw" >/dev/null 2>&1; then
    valid=true
  elif [[ $rc -eq 0 ]]; then
    rc=90
  fi
  raw_sha=null
  if [[ -f "$raw" ]]; then
    raw_sha=$(sha256sum "$raw" | awk '{print $1}')
  fi
  end=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  jq -n \
    --arg schema 'jv_sieve_validation_attempt/v1' \
    --arg id "$id" --arg kind "$kind" --argjson p "$P" \
    --argjson seed "$seed" --argjson run "$run" --arg command "$command" \
    --arg started_at "$start" --arg ended_at "$end" --argjson exit_code "$rc" \
    --argjson output_valid "$valid" --arg raw "$raw" --arg raw_sha256 "$raw_sha" \
    --arg stdout "$stdout" --arg stderr "$stderr" \
    '{schema:$schema,id:$id,kind:$kind,p:$p,seed:$seed,run:$run,command:$command,
      started_at:$started_at,ended_at:$ended_at,exit_code:$exit_code,
      output_valid:$output_valid,raw:$raw,
      raw_sha256:(if $raw_sha256 == "null" then null else $raw_sha256 end),
      stdout:$stdout,stderr:$stderr}' > "$status"
  return 0
}
export -f run_one

xargs -P "$JOBS" -n 4 bash -c 'run_one "$@"' _ < "$PANEL/plan.tsv"
jq -s 'sort_by(.id)' "$PANEL"/status/*.json > "$PANEL/attempts.json"
"$ANALYSE" --panel "$PANEL" --protocol "$PROTOCOL" --out "$PANEL/analysis.json" \
  > "$PANEL/analysis.stdout"
sha256sum "$PANEL"/execution.json "$PANEL"/plan.tsv "$PANEL"/attempts.json \
  "$PANEL"/analysis.json "$PANEL"/raw/*.jsonl > "$PANEL/SHA256SUMS"

