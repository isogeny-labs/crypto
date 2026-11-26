#!/usr/bin/env bash
# Thin build-result/test orchestration only. This does not launch a campaign.
set -euo pipefail
if [[ $# != 2 ]]; then
  echo 'usage: qualify-development.sh ABSOLUTE_CARGO_JSONL ABSOLUTE_NEW_OUTPUT_DIRECTORY' >&2
  exit 64
fi
build_log=$(realpath "$1")
output=$2
[[ "$output" == /* && ! -e "$output" ]] || { echo 'output must be an absent absolute directory' >&2; exit 64; }
jq -e -s 'last | .reason == "build-finished" and .success == true' "$build_log" >/dev/null
mkdir -p "$output/bin" "$output/logs"
jq -r 'select(.reason=="compiler-artifact" and .profile.test==true and .executable!=null) | [.target.name,.executable] | @tsv' "$build_log" | sort -u > "$output/build-executables.tsv"
[[ -s "$output/build-executables.tsv" ]] || { echo 'no test executables found' >&2; exit 1; }
while IFS=$'\t' read -r name executable; do
  [[ "$name" =~ ^[a-zA-Z0-9_-]+$ ]] || exit 64
  install -m 0555 "$executable" "$output/bin/$name"
done < "$output/build-executables.tsv"
(cd "$output" && sha256sum bin/*) > "$output/pre-call.sha256"
chmod 0444 "$output/pre-call.sha256"
chmod 0555 "$output/bin"
date -u +%Y-%m-%dT%H:%M:%SZ > "$output/started-utc.txt"
: > "$output/results.jsonl"
overall=0
while IFS=$'\t' read -r name executable; do
  if "$output/bin/$name" --test-threads=1 > "$output/logs/$name.txt" 2>&1; then code=0; else code=$?; overall=1; fi
  summary=$(rg '^test result:' "$output/logs/$name.txt" | tail -1 || true)
  counts=$(sed -nE 's/^test result: .* ([0-9]+) passed; ([0-9]+) failed; ([0-9]+) ignored;.*/\1 \2 \3/p' <<< "$summary")
  read -r passed failed ignored <<< "$counts"
  passed=${passed:-0}; failed=${failed:-0}; ignored=${ignored:-0}
  jq -n -c --arg test "$name" --arg command "$output/bin/$name --test-threads=1" \
    --arg log "logs/$name.txt" --arg summary "$summary" --argjson exit_code "$code" \
    --argjson passed "$passed" --argjson failed "$failed" --argjson ignored "$ignored" \
    '{test:$test,command:$command,log:$log,summary:$summary,exit_code:$exit_code,passed:$passed,failed:$failed,ignored:$ignored}' >> "$output/results.jsonl"
  printf '%s: %s\n' "$name" "$summary"
done < "$output/build-executables.tsv"
date -u +%Y-%m-%dT%H:%M:%SZ > "$output/finished-utc.txt"
(cd "$output" && sha256sum bin/*) > "$output/post-call.sha256"
cmp "$output/pre-call.sha256" "$output/post-call.sha256"
jq -s --argjson exit_code "$overall" '{schema:"prime31-isogeny-choice.development-tests/v1",scope:"fixed arithmetic and parser contracts; no benchmark or campaign",tests_passed:(map(.passed)|add),tests_failed:(map(.failed)|add),tests_ignored:(map(.ignored)|add),test_executables:length,exit_code:$exit_code,pre_post_binary_hashes_match:true,results:.}' "$output/results.jsonl" > "$output/summary.json"
exit "$overall"
