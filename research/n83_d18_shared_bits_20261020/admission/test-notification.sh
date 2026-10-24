#!/bin/bash
set -euo pipefail
source=$(realpath "$(dirname "$0")/wait-status.rs")
output=$1
test ! -e "$output"
mkdir -p "$output"
output=$(realpath "$output")
rustc --edition 2021 --test "$source" -o "$output/unit-tests"
"$output/unit-tests" > "$output/unit-tests.log" 2>&1
rustc --edition 2021 -O "$source" -o "$output/wait-status"
printf '%s\n' WAITING_FOR_VALIDATION_LOCK > "$output/status.txt"
"$output/wait-status" "$output/status.txt" 5 > "$output/event.log" 2>&1 &
child=$!
sleep 1
kill -0 "$child"
printf '%s\n' AWAITING_COMMITTED_HOST_CARD > "$output/status.txt"
wait "$child"
test "$(cat "$output/event.log")" = AWAITING_COMMITTED_HOST_CARD
"$output/wait-status" "$output/status.txt" 1 > "$output/already-ready.log" 2>&1
printf '%s\n' FAILED > "$output/status.txt"
if "$output/wait-status" "$output/status.txt" 1 > "$output/failure.log" 2>&1; then exit 1; fi
printf '%s\n' WAITING_FOR_VALIDATION_LOCK > "$output/status.txt"
if "$output/wait-status" "$output/status.txt" 1 > "$output/timeout.log" 2>&1; then exit 1; fi
printf '%s\n' PASS > "$output/result.txt"
