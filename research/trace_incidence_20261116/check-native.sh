#!/usr/bin/env bash
# Native development checks only. No new timing result is extracted here.
set -euo pipefail
study=$(cd -- "$(dirname -- "$0")" && pwd)
build_dir=$(mktemp -d)
trap 'rm -rf -- "$build_dir"' EXIT
g++ -O3 -std=c++17 -Wall -Wextra -Werror \
  "$study/historical/repo_controls/pair_control.cpp" -o "$build_dir/pair_control"
checked=0
for fixture in "$study"/historical/prime_fixtures/*.json; do
  if [[ "$fixture" == *.validation.json ]]; then continue; fi
  if ! jq -e '.schema == "prime-pools/v1"' "$fixture" >/dev/null; then continue; fi
  validation=${fixture%.json}.validation.json
  expected=$(jq -r 'if .signed_zero_sum_exists then "relation" else "none" end' "$validation")
  jq -r '[.curve.p,.curve.a,.curve.b,.curve.r,.curve.P[],.curve.Q[],
    (.pools[]|length),(.pools[][]|.x,.y,.alpha,.beta)]|.[]' "$fixture" \
    | "$build_dir/pair_control" > "$build_dir/answer.json"
  jq -e --arg expected "$expected" '.status == $expected and
    (if .status == "relation" then .recovery.recovered_scalar_verified else true end)' \
    "$build_dir/answer.json" >/dev/null
  checked=$((checked+1))
done
[[ "$checked" -eq 90 ]]
printf 'Native pair correctness fixtures checked: %s\n' "$checked"
