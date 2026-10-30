#!/usr/bin/env bash
# New b-aware ordinary-query controls, separate from all frozen b=1 runs.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
meta="$study/pipeline-receipts/qualified-queries-20261025"
bin=/workspace/ic-frozen-20261025/ic_qualified_query_fixtures
replay=/workspace/ic-frozen-20261025/ic_algebraic_followup-generic-b
test ! -e "$meta"
mkdir -p "$meta"
git rev-parse HEAD > "$meta/source-commit.txt"
sha256sum "$bin" "$replay" "$study/BINARY_QUERY_LAW_PROTOCOL.md" > "$meta/hashes.txt"
date -u +%FT%TZ > "$meta/started.txt"
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
run_one() {
  local folder=$1
  shift
  mkdir -p "$folder"
  printf '%q ' "$@" > "$folder/command.txt"
  printf '\n' >> "$folder/command.txt"
  date -u +%FT%TZ > "$folder/started.txt"
  set +e
  (ulimit -v 2097152; /workspace/.tools/usr/bin/time -f 'wall_seconds=%e\nmax_rss_kib=%M\nexit=%x' -o "$folder/resources.txt" timeout -k 30 900 "$@") > "$folder/output.jsonl" 2> "$folder/stderr.txt"
  local rc=$?
  set -e
  printf '{"exit_code":%s,"address_space_bytes":2147483648,"timeout_seconds":900}\n' "$rc" > "$folder/status.json"
  date -u +%FT%TZ > "$folder/ended.txt"
  printf '%s exit=%s\n' "$folder" "$rc"
}
for cell in '3 22 8' '3 25 9' '4 17 6' '4 18 6'; do
  read -r m n ell <<< "$cell"
  model="$meta/model-n$n"
  run_one "$model" "$bin" model "$n"
  if ! jq -se 'any(.status == "selected")' "$model/output.jsonl" > /dev/null; then
    printf '%s\n' "No selected model; preserve failure and leave all four cell families ungenerated." > "$model/UNGENERATED.txt"
    continue
  fi
  for family in prefix geometric geomtraceu random; do
    unit="$meta/m$m-n$n-ell$ell-$family"
    run_one "$unit" "$bin" fixtures "$model/output.jsonl" "$m" "$ell" "$family"
    run_one "$unit/replay" "$replay" verify-fixtures --input "$unit/output.jsonl"
  done
done
date -u +%FT%TZ > "$meta/ended.txt"
printf '%s\n' 'Inspect every status, all planned/generated/missing cases, paired target and point-oracle controls; no automatic selection launch, publication or claim.' > "$meta/REVIEW_REQUIRED.txt"
