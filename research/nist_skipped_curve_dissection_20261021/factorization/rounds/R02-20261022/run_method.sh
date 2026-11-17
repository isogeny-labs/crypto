#!/usr/bin/env bash
# Thin native GMP-ECM orchestration. Reported factors are unverified hints.
set -euo pipefail
method=$1 number=$2 out=$3 b1=$4 curves=$5 b2=$6 start=$7 limit=$8
[[ $method == ecm || $method == pm1 || $method == pp1 ]]
for n in "$number" "$b1" "$curves" "$b2" "$start" "$limit"; do
  [[ $n =~ ^[0-9]+$ ]] || exit 2
done
((curves > 0 && limit > 0))
mkdir -p "$out"
[[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]] || {
  echo 'Choose a fresh attempt directory' >&2; exit 1;
}
out=$(cd -- "$out" && pwd)
date -u +%FT%TZ > "$out/started.txt"
printf '%s\n' "$number" > "$out/input.txt"
git rev-parse HEAD > "$out/code-commit.txt"
sha256sum "${BASH_SOURCE[0]}" > "$out/wrapper-sha256.txt"
bin=${ECM_BIN:-ecm}
"$bin" -printconfig > "$out/version.txt" 2>&1
argv=("$bin" -one -c "$curves" -maxmem 1024 -save "$out/residues.txt")
[[ $method == ecm ]] || argv+=("-$method")
((start == 0)) || argv+=(-x0 "$start")
if [[ -n ${ECM_SIGMA:-} ]]; then
  [[ $method == ecm && $ECM_SIGMA =~ ^[0-9]+:[0-9]+$ ]]
  argv+=(-sigma "$ECM_SIGMA")
fi
argv+=("$b1")
((b2 == 0)) || argv+=("$b2")
printf '%q ' timeout --kill-after=10s "$limit" "${argv[@]}" > "$out/command.txt"
printf '\n' >> "$out/command.txt"
set +e
timeout --kill-after=10s "$limit" "${argv[@]}" < "$out/input.txt" > "$out/stdout.txt" 2> "$out/stderr.txt"
status=$?
set -e
date -u +%FT%TZ > "$out/finished.txt"
awk '/Factor found in step/ {print $NF}' "$out/stdout.txt" > "$out/reported-factors.txt"
awk -v original="$number" '"x" $0 != "x" original && "x" $0 != "x1"' \
  "$out/reported-factors.txt" > "$out/candidate-factors.txt"
used=$(awk '/^Using B1=/ {n++} END {print n+0}' "$out/stdout.txt")
stage2=$(awk '/^Step 2 took / {n++} END {print n+0}' "$out/stdout.txt")
jq -n --arg method "$method" --arg n "$number" --argjson b1 "$b1" \
  --argjson b2 "$b2" --argjson start "$start" --argjson requested "$curves" \
  --argjson used "$used" --argjson stage2 "$stage2" --argjson status "$status" --argjson limit "$limit" \
  '{schema:"nist-factor-search-attempt/v2",method:$method,input:$n,b1:$b1,
    b2_requested:(if $b2==0 then null else $b2 end),start:(if $start==0 then null else $start end),
    requested_runs:$requested,attempted_runs:$used,completed_stage2:$stage2,
    limit_seconds:$limit,exit_status:$status,timed_out:($status==124),factor_labels:"unverified"}' > "$out/receipt.json"
if [[ $status != 124 ]] && ((status & 1)); then exit 1; fi
echo "$method recorded: $used attempted, $stage2 completed stage 2, exit $status"
