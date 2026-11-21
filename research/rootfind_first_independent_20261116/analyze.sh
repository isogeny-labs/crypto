#!/usr/bin/env bash
set -euo pipefail

study_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
stdout_sha="$(shasum -a 256 "$study_dir/run.stdout" | cut -d ' ' -f 1)"
stderr_sha="$(shasum -a 256 "$study_dir/run.stderr" | cut -d ' ' -f 1)"
meta_sha="$(shasum -a 256 "$study_dir/run.meta" | cut -d ' ' -f 1)"
test "$(rg -c '^FIRST_INDEPENDENT_JSON=' "$study_dir/run.stdout")" = 3
rg -q '^test result: ok\. 1 passed; 0 failed' "$study_dir/run.stdout"
rg -q '^exit_code=0$' "$study_dir/run.meta"
sed -n 's/^FIRST_INDEPENDENT_JSON=//p' "$study_dir/run.stdout" |
  jq -s --arg stdout_sha "$stdout_sha" --arg stderr_sha "$stderr_sha" \
    --arg meta_sha "$meta_sha" -f "$study_dir/analyze.jq" > "$study_dir/RESULTS.json"
jq -e '.candidate_passes_all_cells == false and (.cells | length) == 3' "$study_dir/RESULTS.json" >/dev/null
