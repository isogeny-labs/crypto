#!/usr/bin/env bash
# Fixed public correctness cases; no timing or performance comparison.
set -euo pipefail
if [ "$#" -ne 2 ]; then
  echo "usage: $0 PRODUCER_BINARY NEW_OUTPUT_DIRECTORY" >&2
  exit 64
fi
aps_producer=$1
aps_output=$2
if [ -e "$aps_output" ]; then
  echo "output already exists; preserve the original fixtures" >&2
  exit 1
fi
mkdir -p "$aps_output"
run_case() {
  local aps_case=$1
  shift
  "$aps_producer" solve --budget-ms 10000 --attempts 32 \
    --output "$aps_output/$aps_case.json" "$@"
}
run_case partitioned3 --arm partitioned3 --labels random
run_case unordered3 --arm unordered3 --first-informative
run_case line3 --arm line3 --first-informative
run_case expanded4 --arm expanded4
run_case split4 --arm split4 --labels random
run_case block-symmetric4 --arm block-symmetric4
run_case hybrid3 --arm hybrid3 --first-informative
run_case hybrid4 --arm hybrid4 --first-informative
run_case cover3 --arm cover3 --first-informative
run_case orbit3 --fixture toy103-j0 --subset-size 1 --labels random --arm orbit3
run_case orbit-line3 --fixture toy103-j0 --subset-size 1 --labels random --arm orbit-line3
run_case orbit-pruned-line3 --fixture toy103-j0 --subset-size 1 \
  --labels random --arm orbit-pruned-line3 --first-informative
run_case line3-toy211 --fixture toy211 --arm line3 --first-informative
run_case split4-toy211 --fixture toy211 --arm split4
