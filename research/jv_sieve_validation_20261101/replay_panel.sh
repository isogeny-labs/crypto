#!/usr/bin/env bash
# Independent clean-job replay of one IC and one corrected R3 row.
set -euo pipefail

if [[ $# -ne 3 ]]; then
  echo "usage: $0 P ORIGINAL_PANEL REPLAY_ROOT" >&2
  exit 2
fi
P=$1
ORIGINAL=$2
ROOT=$3
BIN=./target/release/examples/jv_cover_ic_vs_rho_bitops
OUT=$ROOT/p$P
mkdir -p "$OUT/raw" "$OUT/stdout" "$OUT/stderr"

replay() {
  local id=$1 kind=$2
  local raw="$OUT/raw/$id.jsonl"
  case "$kind" in
    ic)
      JV_SIEVE_ROUND_THREADS=4 F4_FP_THREAD_OPS=1 "$BIN" --arm ic --p "$P" \
        --seeds 33 --moves 8 --jobs 1 --json "$raw" \
        >"$OUT/stdout/$id.log" 2>"$OUT/stderr/$id.log"
      ;;
    r3)
      "$BIN" --arm rho --p "$P" --seeds 33 --moves 8 --rho R3 --runs 0 \
        --jobs 1 --json "$raw" >"$OUT/stdout/$id.log" 2>"$OUT/stderr/$id.log"
      ;;
  esac
}

replay ic_s33 ic
replay rho_R3_run0_s33 r3

runs='[]'
all_pass=true
for id in ic_s33 rho_R3_run0_s33; do
  expected="$ORIGINAL/raw/$id.jsonl"
  actual="$OUT/raw/$id.jsonl"
  jq -S 'del(.wall_ms)' "$expected" > "$OUT/$id.expected.normalized.json"
  jq -S 'del(.wall_ms)' "$actual" > "$OUT/$id.actual.normalized.json"
  pass=false
  if cmp -s "$OUT/$id.expected.normalized.json" "$OUT/$id.actual.normalized.json"; then
    pass=true
  else
    all_pass=false
  fi
  scalar_verified=$(jq -e \
    '(.verified_on_challenge == true) and ((.arm == "ic" and .solved == true and .correct == true) or (.arm == "rho" and .correct == true and .capped == false))' \
    "$actual" >/dev/null && echo true || echo false)
  if [[ $scalar_verified != true ]]; then all_pass=false; fi
  row=$(jq -n --arg id "$id" --arg expected "$expected" --arg actual "$actual" \
    --arg expected_sha256 "$(sha256sum "$expected" | awk '{print $1}')" \
    --arg actual_sha256 "$(sha256sum "$actual" | awk '{print $1}')" \
    --arg normalized_sha256 "$(sha256sum "$OUT/$id.actual.normalized.json" | awk '{print $1}')" \
    --argjson counts_identical "$pass" --argjson scalar_verified "$scalar_verified" \
    '{id:$id,expected:$expected,actual:$actual,expected_sha256:$expected_sha256,
      actual_sha256:$actual_sha256,normalized_sha256:$normalized_sha256,
      counts_identical:$counts_identical,scalar_verified:$scalar_verified}')
  runs=$(jq --argjson row "$row" '. + [$row]' <<< "$runs")
done

jq -n --arg schema 'jv_sieve_validation_replay/v1' --argjson p "$P" \
  --arg status "$([[ $all_pass == true ]] && echo verified || echo fail)" \
  --arg source_commit "$(git rev-parse HEAD)" \
  --arg producer_sha256 "$(sha256sum "$BIN" | awk '{print $1}')" \
  --arg kernel "$(uname -srvmo)" --argjson runs "$runs" \
  '{schema:$schema,status:$status,independent:true,
    independence_basis:"separate clean workflow job after producer artifacts were sealed",
    p:$p,source_commit:$source_commit,producer_sha256:$producer_sha256,kernel:$kernel,
    runs:$runs,scalars_verified:([$runs[].scalar_verified] | all),
    counts_identical:([$runs[].counts_identical] | all)}' > "$OUT/replay-certificate.json"

[[ $all_pass == true ]]
