#!/bin/sh
# Fixed-input corruption controls; arithmetic validation remains in compare.cpp.
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
out=${COMPARE_OUT:-"$here/records"}
test_stage=$(mktemp -d)
trap 'rm -rf "$test_stage"' EXIT HUP INT TERM
for test in seed coefficient point order; do
  case "$test" in
    seed) column=11; replacement=0x0 ;;
    coefficient) column=7; replacement=0x0 ;;
    point) column=8; replacement=0x1 ;;
    order) column=10; replacement=0x11 ;;
  esac
  awk -v column="$column" -v replacement="$replacement" \
    'BEGIN {FS=OFS="\t"} /^#/ {print;next} !done {$column=replacement;done=1} {print}' \
    "$here/inputs.tsv" > "$test_stage/input.tsv"
  if "$out/compare" "$test_stage/input.tsv" 0 > "$test_stage/stdout" 2> "$test_stage/stderr"; then
    printf '%s\n' "FAIL: $test corruption accepted"; exit 1
  fi
  if test -s "$test_stage/stdout"; then
    printf '%s\n' "FAIL: $test corruption produced a result"; exit 1
  fi
  printf '%s\n' "PASS: $test corruption rejected"
  tail -n 1 "$test_stage/stderr"
done
