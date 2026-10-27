#!/usr/bin/env bash
# Before/after wall time of pipeline stages, interleaved (ABAB...), outputs
# checked identical:
#   plan:      ecbench plan --spec run/specs/spec.json (registry parsed per
#              lookup, before; parsed once, after)
#   construct: volcano-gap construct (affine kernel and plain-F_p Velu map,
#              before; Jacobian kernel, Montgomery threaded map, after),
#              pinned to CPU 2, and construct4 on CPUs 0-3 (the map's threads)
# usage: timing.sh OLD_ECBENCH NEW_ECBENCH OLD_VOLCANO_GAP NEW_VOLCANO_GAP ROUNDS [STAGES]
# STAGES: a subset of "plan construct construct4" (default: all three).
# One JSON line per run on stdout.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
study="$here/.."
old_eb=$1 new_eb=$2 old_vg=$3 new_vg=$4 rounds=$5 stages=${6:-plan construct construct4}
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
now() { date +%s%N; }
for r in $(seq 1 "$rounds"); do
  for side in before after; do
    if [ "$side" = before ]; then eb=$old_eb vg=$old_vg; else eb=$new_eb vg=$new_vg; fi
    if [[ " $stages " == *" plan "* ]]; then
    t0=$(now)
    taskset -c 2 "$eb" plan --spec "$study/run/specs/spec.json" > "$tmp/plan-$side.txt" 2>&1
    t1=$(now)
    jq -nc --arg stage plan --arg side "$side" --argjson round "$r" --argjson ns $((t1 - t0)) \
      --arg out "$(sha256sum < "$tmp/plan-$side.txt" | cut -d' ' -f1)" --arg bin "$(sha256sum < "$eb" | cut -d' ' -f1)" \
      '{stage:$stage, side:$side, round:$round, wall_ns:$ns, output_sha256:$out, binary_sha256:$bin}'
    fi
    for stage in construct construct4; do
    [[ " $stages " == *" $stage "* ]] || continue
    if [ "$stage" = construct ]; then cpus=2; else cpus=0-3; fi
    t0=$(now)
    taskset -c "$cpus" "$vg" construct --bits 52 --depth 6 --d0 11 --max-cofactor 16384 --seed 20261022 \
      --out "$tmp/pair-$side.json" > "$tmp/construct-$side.log" 2>&1
    t1=$(now)
    line=$(grep '^N-isogeny:' "$tmp/construct-$side.log")
    # The isogeny itself, without the timings and operation counts.
    math=$(jq -cS 'del(.command) | walk(if type == "object" then with_entries(select(.key | test("_s$|_s_per_point$|mults|seconds") | not)) else . end)' "$tmp/pair-$side.json" | sha256sum | cut -d' ' -f1)
    jq -nc --arg stage "$stage" --arg cpus "$cpus" --arg side "$side" --argjson round "$r" --argjson ns $((t1 - t0)) \
      --arg line "$line" --arg math "$math" --arg bin "$(sha256sum < "$vg" | cut -d' ' -f1)" \
      '{stage:$stage, cpus:$cpus, side:$side, round:$round, wall_ns:$ns, log:$line, isogeny_sha256:$math, binary_sha256:$bin,
        construct_s: ($line | capture("construct (?<s>[0-9.]+)s").s | tonumber),
        eval_s_per_point: ($line | capture("eval (?<s>[0-9.]+)s/point").s | tonumber)}'
    done
  done
done
