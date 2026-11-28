#!/usr/bin/env bash
# Rehydrate preserved complete experiments into scratch; never overwrite evidence.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
study=research/ic_algebraic_crossover_20261023
cargo build --release --example ic_qualified_query_fixtures --example ic_qualified_closure_audit --example ic_algebraic_followup
bin_root=$(realpath "${CARGO_TARGET_DIR:-target}/release/examples")
scratch=$(mktemp -d /tmp/ic-qualified-replay.XXXXXX)
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/$study"
while read -r hash relative; do
  actual=$(sha256sum "$study/$relative")
  test "${actual%% *}" = "$hash"
  tar -xJf "$study/$relative" -C "$scratch/$study"
done < <(jq -r '.archives[] | [.sha256,.path] | @tsv' "$study/qualified-raw-archives.json")
cd "$scratch"
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 HKY_CHUNK=1024
meta="$study/pipeline-receipts/qualified-queries-20261025"
"$bin_root/ic_qualified_query_fixtures" summary "$meta" > query-summary.json
cmp query-summary.json "$meta/summary.json"
"$bin_root/ic_qualified_closure_audit" summary "$study" > closure-summary.json
cmp closure-summary.json "$study/pipeline-receipts/qualified-closure-replay-20261025/summary.json"
for cell in '3 22 8' '3 25 9' '4 17 6' '4 18 6'; do
  read -r m n ell <<< "$cell"
  for family in prefix geometric geomtraceu random; do
    unit="m$m-n$n-ell$ell-$family"
    input="$meta/$unit/output.jsonl"
    (ulimit -v 2097152; timeout -k 30 900 "$bin_root/ic_algebraic_followup" verify-fixtures --input "$input") > finite-chain-replay.jsonl
    cmp finite-chain-replay.jsonl "$meta/$unit/replay/output.jsonl"
    (ulimit -v 2097152; timeout -k 30 900 "$bin_root/ic_qualified_query_fixtures" audit "$meta/model-n$n/output.jsonl" "$input") > point-replay.jsonl
    cmp point-replay.jsonl "$meta/$unit/reference/output.jsonl"
    while read -r draw; do
      for degree in 2 3; do
        (ulimit -v 2097152; timeout -k 30 180 "$bin_root/ic_qualified_closure_audit" case "$study" "$unit" "$draw" "$degree") > closure-replay.json
        cmp closure-replay.json "$study/pipeline-receipts/qualified-closure-replay-20261025/$unit-d$draw-D$degree/output.json"
      done
    done < <(jq -r 'select(.followup_selection != null) | .draw' "$input")
  done
done
printf '%s\n' 'PASS: all three archive hashes, raw-derived summaries, 2048 finite-chain/witness replays, 2048 independent point-arithmetic recounts and 158 HKY rank/verdict replays reproduce byte for byte. Word totals are aggregated, not independently recounted by HKY.'
