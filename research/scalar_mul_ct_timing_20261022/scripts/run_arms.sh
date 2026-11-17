#!/bin/sh
# Thin orchestration for PROTOCOL.md (no computation here).
#
#   run_arms.sh <raw_dir> <baseline_bin> <candidate_bin>
#
# 5 rounds; arms rotated A B A2 / B A2 A / A2 A B; A and A2 are the baseline
# binary (the A/A control), B the candidate.  Every process runs under
# tools/isolated_bench.py on CPU 3; its isolation record goes to
# <raw_dir>/isolation.jsonl and the driver's JSON line to
# <raw_dir>/timing.jsonl.  A label already recorded is skipped, so an
# interrupted run resumes where it stopped.
set -eu
raw=$1; base=$2; cand=$3
here=$(cd "$(dirname "$0")" && pwd)
repo=$(cd "$here/../../.." && pwd)
iso="$raw/isolation.jsonl"
out="$raw/timing.jsonl"
mkdir -p "$raw"

run() {
  label=$1; shift
  if [ -f "$out" ] && grep -q "\"label\":\"$label\"" "$out"; then
    return 0
  fi
  tries=0
  while :; do
    if python3 "$repo/tools/isolated_bench.py" run --wait --cpus 3 --out "$iso" \
        --label "$label" -- "$@" > "$raw/.last" 2> "$raw/.err"; then
      cat "$raw/.last" >> "$out"
      return 0
    fi
    if grep -q "machine is busy" "$raw/.err" && [ "$tries" -lt 60 ]; then
      tries=$((tries + 1)); sleep 15; continue
    fi
    cat "$raw/.err" >&2
    return 1
  done
}

arms_for_round() {
  case $(( ($1 - 1) % 3 )) in
    0) echo "A B A2" ;; 1) echo "B A2 A" ;; *) echo "A2 A B" ;;
  esac
}

r=1
while [ "$r" -le 5 ]; do
  for arm in $(arms_for_round "$r"); do
    case $arm in B) bin=$cand ;; *) bin=$base ;; esac
    label="$arm-r$r"
    run "$label" "$bin" "$label" 3
  done
  r=$((r + 1))
done
