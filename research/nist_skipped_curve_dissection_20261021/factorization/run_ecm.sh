#!/usr/bin/env bash
# Native ECM search; factor/primality labels remain unverified until GP replay.
set -euo pipefail
number=$1
out=$2
b1=${3:-100000}
curves=${4:-100}
[[ $number =~ ^[0-9]+$ && $b1 =~ ^[0-9]+$ && $curves =~ ^[0-9]+$ ]]
mkdir -p "$out"
[[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]] || { echo 'Choose a fresh attempt directory' >&2; exit 1; }
printf '%s\n' "$number" > "$out/input.txt"
date -u +%FT%TZ > "$out/started.txt"
argv=("${ECM_BIN:-ecm}" -one -c "$curves" -maxmem 1024 -save "$out/residues.txt" "$b1")
printf '%q ' "${argv[@]}" > "$out/command.txt"
printf '\n' >> "$out/command.txt"
"${ECM_BIN:-ecm}" -printconfig > "$out/version.txt" 2>&1
set +e
timeout --kill-after=10s 600 "${argv[@]}" < "$out/input.txt" > "$out/stdout.txt" 2> "$out/stderr.txt"
status=$?
set -e
date -u +%FT%TZ > "$out/finished.txt"
awk '/Factor found in step/ {print $NF}' "$out/stdout.txt" > "$out/candidate-factors.txt"
count=$(awk '/^Step 2 took / {n++} END {print n+0}' "$out/stdout.txt")
jq -n --arg n "$number" --argjson b1 "$b1" --argjson curves "$curves" --argjson completed "$count" --argjson status "$status" \
  '{schema:"nist-factor-ecm-attempt/v1",input:$n,b1:$b1,requested_curves:$curves,completed_stage2:$completed,exit_status:$status,
    factor_labels:"unverified",timed_out:($status==124)}' > "$out/receipt.json"
if [[ $status != 124 ]] && ((status & 1)); then exit 1; fi
echo "ECM attempt recorded: stage 2 completed $count times; exit $status"
