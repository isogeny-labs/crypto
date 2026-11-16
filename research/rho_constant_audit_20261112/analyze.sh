#!/usr/bin/env bash
# Fixed native analysis. Counts only; all exits retained, no outcome-dependent reruns.
set -u
binary=$(realpath "$1")
session=$(realpath "$2")
output=$3
mkdir "$output" || exit 2
output=$(realpath "$output")
failures=0
run() {
  local label=$1
  shift
  "$@" > "$output/$label.stdout" 2> "$output/$label.stderr"
  local rc=$?
  printf '%s\n' "$rc" > "$output/$label.exit"
  if [ "$rc" -ne 0 ]; then failures=$((failures+1)); fi
}
run verify "$binary" verify --dir "$session" --replay-all --replay-seed 20261112013 --out "$output/audit.json"
run table "$binary" table --dir "$session" --reference rho-neg
run aa "$binary" compare --dir "$session" --a rho-neg --b rho-neg-aa --resamples 4000 --seed 20261112014 --json
run plain "$binary" compare --dir "$session" --a rho-neg --b rho-plain --resamples 4000 --seed 20261112014 --json
run band "$binary" band --dir "$session" --weights 0,0.1,1 --json "$output/band.json"
printf '%s\n' "$failures" > "$output/failures.txt"
test "$failures" -eq 0
