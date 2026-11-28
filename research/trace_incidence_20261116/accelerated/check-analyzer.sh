#!/usr/bin/env bash
# Schema/metadata mutation checks; the native analyzer does every comparison.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
out=$(mktemp -d "${TMPDIR:-/tmp}/trace-accelerated-analysis.XXXXXX")
printf '%s\n' "$out"
cp "$here/build/analyze" "$out/qualifier"
chmod 0555 "$out/qualifier"
subjects=("$here/analyze.cpp" "$here/../runpod/analyze.cpp" "$here/development/driver-checks.jsonl" "$here/check-analyzer.sh" "$out/qualifier")
sha256sum "${subjects[@]}" > "$out/build.sha256"
set -o noclobber
cp "$out/build.sha256" "$out/pre-call.sha256"
jq -c '.+{phase:"warmup",round:0,attempt:0,source_fixture:"b004_control.json",arm_label:.arm}' "$here/development/driver-checks.jsonl" > "$out/positive.jsonl"
"$out/qualifier" "$out/positive.jsonl" > "$out/positive-summary.json"
jq -e '.measured_rows==0 and .raw_rows==21 and .cross_mechanism_full_support_checks==11 and .cross_generator_rejection_accounting_checks==20' "$out/positive-summary.json" >/dev/null
jq -s -c '.[1].gcd_coefficients[0]+=1 | .[]' "$out/positive.jsonl" > "$out/bad-gcd.jsonl"
if "$out/qualifier" "$out/bad-gcd.jsonl" > "$out/bad-gcd.out" 2> "$out/bad-gcd.err"; then echo 'bad GCD accepted' >&2;exit 1;fi
jq -s -c '.[1].generation.draws+=1 | .[]' "$out/positive.jsonl" > "$out/bad-generation.jsonl"
if "$out/qualifier" "$out/bad-generation.jsonl" > "$out/bad-generation.out" 2> "$out/bad-generation.err"; then echo 'bad generation count accepted' >&2;exit 1;fi
sha256sum "${subjects[@]}" > "$out/post-call.sha256"
cmp "$out/pre-call.sha256" "$out/post-call.sha256"
printf '%s\n' '{"schema":"trace-accelerated-analyzer-checks/v1","status":"passed","timings_collected":false,"positive_rows":21,"negative_cases":2,"standing":"unverified"}' > "$out/checks.json"
cat "$out/checks.json"
