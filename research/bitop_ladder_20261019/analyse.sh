#!/usr/bin/env bash
# Phase B analysis of PROTOCOL.md, all by ecbench, over sessions/holdout-P*.
#
#   research/bitop_ladder_20261019/analyse.sh
#
# Writes analysis/: bound fits (bitop/v1) per arm over 16-40 and over the
# bitop/v1 ladder's 16-26 range, per-curve `ecbench compare` of every arm
# against rho-neg, `ecbench table`, the §5a band, `ecbench amortize`, and
# `ecbench work --counted` over every hold-out session.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
bin="$root/target/release/ecbench"
out="$here/analysis"
mkdir -p "$out/bounds" "$out/compare" "$out/amortize"
cd "$root"

dirs() { # sessions holding sizes $1..$2
  for b in $(seq "$1" "$2"); do printf -- '--dir %s ' "research/bitop_ladder_20261019/sessions/holdout-P$b"; done
}
rule="research/bitop_ladder_20261019/PROTOCOL.md Phase A: base per size from params.json (train panel, bitop/v1)"

fit() { # name arm lo hi [rule]
  local name=$1 arm=$2 lo=$3 hi=$4 extra=()
  [ "${5:-}" = rule ] && extra=(--param-rule "$rule")
  # shellcheck disable=SC2046
  "$bin" bound fit $(dirs "$lo" "$hi") --arm "$arm" --unit bitop/v1 "${extra[@]}" \
    --label "bitop ladder hold-out: $arm, $lo-$hi bits" \
    --out "$out/bounds/$name.json" >"$out/bounds/$name.log" 2>&1 \
    || echo "FIT FAILED $name (see $out/bounds/$name.log)"
}

for arm in rho-neg rho-neg-aa bsgs-neg kangaroo; do
  fit "$arm-16-40" "$arm" 16 40
  fit "$arm-16-26" "$arm" 16 26
done
fit ic-pair-rule-16-40 ic-pair-rule 16 40 rule
fit ic-pair-rule-16-26 ic-pair-rule 16 26 rule
fit ic-pair-rule-16-28 ic-pair-rule 16 28 rule
fit ic-pair-b32-16-28 ic-pair-b32 16 28
fit ic-pair-b32-16-26 ic-pair-b32 16 26
fit ic-rootfind-m3-rule-16-28 ic-rootfind-m3-rule 16 28 rule

for b in $(seq 16 40); do
  d="research/bitop_ladder_20261019/sessions/holdout-P$b"
  arms=(rho-neg-aa bsgs-neg kangaroo ic-pair-rule)
  [ "$b" -le 28 ] && arms+=(ic-pair-b32 ic-rootfind-m3-rule)
  for arm in "${arms[@]}"; do
    "$bin" compare --dir "$d" --a rho-neg --b "$arm" --json >"$out/compare/P$b-$arm.json" 2>/dev/null \
      || echo "COMPARE FAILED P$b $arm"
  done
done

# shellcheck disable=SC2046
"$bin" table $(dirs 16 40) >"$out/table.txt"
# shellcheck disable=SC2046
"$bin" band $(dirs 16 40) --ranks --json "$out/band.json" >"$out/band.txt"
# shellcheck disable=SC2046
"$bin" amortize $(dirs 16 40) --arm ic-pair-rule --reference rho-neg >"$out/amortize/ic-pair-rule.txt"
# shellcheck disable=SC2046
"$bin" work --counted $(for b in $(seq 16 40); do echo "research/bitop_ladder_20261019/sessions/holdout-P$b"; done) \
  >"$out/work.jsonl"
echo "analysis written to $out"
