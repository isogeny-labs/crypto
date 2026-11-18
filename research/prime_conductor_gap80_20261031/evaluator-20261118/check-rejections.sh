#!/bin/sh
# Thin native-test orchestration. Preserve every failed input and raw response.
set -eu
if [ "$#" -ne 2 ]; then echo "usage: $0 ABSOLUTE_BINARY NEW_OUTPUT_DIRECTORY" >&2; exit 2; fi
binary=$1
output=$2
study=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
mkdir "$output" # Refuse to replace prior receipts.
reject() {
    label=$1
    reason=$2
    input=$3
    shift 3
    set +e
    "$binary" "$@" < "$input" > "$output/$label.stdout" 2> "$output/$label.stderr"
    result=$?
    set -e
    test "$result" -eq 2
    test ! -s "$output/$label.stdout"
    rg -F -- "$reason" "$output/$label.stderr" > /dev/null
    printf '{"case":"%s","exit_code":%s,"stdout_empty":true,"expected_reason_observed":true}\n' "$label" "$result" >> "$output/rejections.jsonl"
}
printf '71 5 5 19 4 35 1\n0 0\n' > "$output/off-curve.input"
printf '71 5 5 17 4 35 0\n' > "$output/wrong-kernel.input"
printf '71 5 5 15 4 35 0\n' > "$output/composite-degree.input"
printf '71 5 5 19 4 35 0\nextra\n' > "$output/trailing-input.input"
printf '71 5 5 19 4 106 0\n' > "$output/noncanonical-kernel.input"
printf '71 5 5 19 4 35 65\n' > "$output/point-cap.input"
reject off-curve 'source membership' "$output/off-curve.input" --evaluate 4 1
reject wrong-kernel 'kernel witness' "$output/wrong-kernel.input" --evaluate 4 1
reject composite-degree 'nonprime characteristic/degree' "$output/composite-degree.input" --evaluate 4 1
reject trailing-input 'extra input after declared points' "$output/trailing-input.input" --evaluate 4 1
reject noncanonical-kernel 'noncanonical kernel coordinates' "$output/noncanonical-kernel.input" --evaluate 4 1
reject point-cap 'at most 64 point inputs' "$output/point-cap.input" --evaluate 4 1
reject window-cap 'window exceeds declared limit' "$study/inputs/control10009.txt" --evaluate 1 17
reject target80-cap 'window exceeds declared limit' "$study/inputs/target80.txt" --evaluate 1024 64
