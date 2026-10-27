#!/usr/bin/env bash
# PROTOCOL v2 / v2.1 analysis (§1, §4 and amendment v2.1). Every number in
# the report comes from these volcano-stats and ecbench band runs.
# usage: bash analyse.sh [VOLCANO_STATS] [ECBENCH]   (run from the repository root)
set -euo pipefail
D=research/isogeny-volcano-cost-v2_20261017
vs=${1:-./target/release/volcano-stats}
eb=${2:-./target/release/ecbench}
cd "$(git rev-parse --show-toplevel)"
mkdir -p $D/analysis $D/band

# The main session of l2-s1002 was interrupted at 191 runs and re-run whole
# as spec-r2 (§3); the analysis uses the complete one.
main_of() { if [ -d $D/sessions/$1/spec-r2 ]; then echo spec-r2; else echo spec; fi; }
records() { # class... -> comma-separated records of every arm of those classes
  local out=()
  for c in "$@"; do
    for s in $(main_of $c) spec-aa spec-bsgs spec-icctl; do
      f=$D/sessions/$c/$s/records.jsonl
      [ -f $f ] || f=$f.gz
      out+=("$f")
    done
  done
  (IFS=,; echo "${out[*]}")
}
curves() { local out=(); for c in "$@"; do out+=("$D/classes/$c/curves.jsonl"); done; (IFS=,; echo "${out[*]}"); }
run() { # name class-list extra-args...
  local name=$1 cl=$2; shift 2
  # shellcheck disable=SC2086
  $vs --records "$(records $cl)" --curves "$(curves $cl)" --vertical stratified \
      --boot 2000 --perm 10000 --pair ic-mitm-plus1:ic-mitm "$@" \
      --out $D/analysis/$name.json 2> $D/analysis/$name.log
  echo "$name done"
}

V2_L2="l2-s1001 l2-s1002 l2-s1003 l2-s1004"
V2_L3="l3-s1001 l3-s1002 l3-s1003 l3-s1004"
V21_L2="$V2_L2 $(printf 'l2-s%d ' $(seq 1005 1014))"
V21_L3="$V2_L3 $(printf 'l3-s%d ' $(seq 1005 1014))"

# H1 (one-sided less), H2 and H3/H4 (two-sided), for v2 (8 classes) and v2.1 (28).
run v2-H1-l2 "$V2_L2" --alternative less &
run v2-H2-l3 "$V2_L3" &
run v2-H3H4-all "$V2_L2 $V2_L3" &
wait
run v2.1-H1-l2 "$V21_L2" --alternative less &
run v2.1-H2-l3 "$V21_L3" &
run v2.1-H3H4-all "$V21_L2 $V21_L3" &
wait

# Analysis positive controls (§4): H1 power, H3 power, single-curve power.
run v2.1-ctl-H1power-l3-plant-floor-0.95 "$V21_L3" --alternative less --plant-level 3:0.95 &
run v2.1-ctl-H3power-all-plant-floor-1.05 "$V21_L2 $V21_L3" --plant-level 3:1.05 &
run v2.1-ctl-curve-l2-s1001-v2-1.15 "l2-s1001" --plant-curve 2:1.15 &
wait
run v2-ctl-H1power-l3-plant-floor-0.95 "$V2_L3" --alternative less --plant-level 3:0.95 &
run v2-ctl-H3power-all-plant-floor-1.05 "$V2_L2 $V2_L3" --plant-level 3:1.05 &
wait

# Per-class contrasts (reported, not tested): every class, two-sided.
for c in $V21_L2 $V21_L3; do
  run class-$c "$c" --perm 2000 &
  while [ "$(jobs -rp | wc -l)" -ge 4 ]; do wait -n; done
done
wait

# §5a pricing band per class (main session: rho-neg and ic-mitm).
for c in $V21_L2 $V21_L3; do
  $eb band --dir $D/sessions/$c/$(main_of $c) --weights 0,0.1,1 --json $D/band/$c.json > /dev/null
done
echo "band done"
