#!/bin/sh
# Writes the Stage T (train) and Stage K (coset control) specs of
# PROTOCOL.md from the values frozen there.  Thin generator (sh + jq);
# the committed JSON is the record, this script only shows how it was made.
# Usage: sh specs/generate.sh   (from the card directory)
set -eu
cd "$(dirname "$0")"

# Incumbent per size: research/ic_frontier_campaign_20261015/params.json
# (pair_size) and phase1/spec-P<b>.json (max_trials), recipe unchanged.
inc() { case $1 in 20) echo "32 1410236";; 24) echo "162 4243777";;
                   28) echo "407 26479085";; 32) echo "1024 167871873";; esac; }
# N grid: round(N_inc * 2^(k/2)), k = -2..2 (PROTOCOL.md §4.2).
grid() { awk -v n="$1" 'BEGIN{for(k=-2;k<=2;k++){printf "%d\n", int(n*2^(k/2)+0.5)}}'; }

ORACLES_MAIN="mitm2=mitm:negation_folded=1 batch2=mitm-prime-batch:m=2 mitm3=mitm:m=3,negation_folded=1 rfmitm3=rootfind-mitm:m=3 rfsub3=rootfind-subtract:m=3"
ORACLES_SMALL="rfsub2=rootfind-subtract:m=2 sub2=subtract rfpairs3=rootfind-pairs:m=3"

arm() { # name role factor_base oracle max_trials
  jq -cn --arg n "$1" --arg r "$2" --arg fb "$3" --arg o "$4" --arg t "$5" \
    '{name:$n, role:$r, method:{id:"ic.pipeline", params:{factor_base:$fb, oracle:$o, max_trials:$t}}}'
}

for b in 20 24 28 32; do
  set -- $(inc $b); ninc=$1; trials=$2
  {
    jq -cn '{name:"rho-neg", role:"reference", method:{id:"rho.negation"}}'
    for spec in $ORACLES_MAIN; do
      short=${spec%%=*}; o=${spec#*=}
      for n in $(grid $ninc); do
        role=candidate
        [ "$short" = mitm2 ] && [ "$n" = "$ninc" ] && role=baseline
        arm "$short-n$n" $role "prime-abscissa:size=$n" "$o" $trials
      done
    done
    if [ $b -le 24 ]; then
      for spec in $ORACLES_SMALL; do
        short=${spec%%=*}; o=${spec#*=}
        arm "$short-n$ninc" candidate "prime-abscissa:size=$ninc" "$o" $trials
      done
    fi
  } | jq -s --argjson b $b '{
      schema: "ecbench.spec/v1",
      label: ("XP-20261016-prime-base-oracle-tuning Stage T (train), \($b)-bit fresh prime-order curve"),
      description: "Train panel of the frozen PROTOCOL.md (section 4): every exact native prime decomposition oracle crossed with a five-point factor-base size grid around the frontier incumbent, low-x base, target-pinned linear algebra, the incumbent trial cap. Selection only; no claim. Public synthetic curve and targets.",
      workloads: {curves: [{kind: "prime_search", bits: $b, seed: 20261018101}],
                  targets_per_curve: 8, target_seed: 20261018111, target_kind: "public"},
      arms: .,
      measurement: {rounds: 1, warmup: 0, order: "alternate", seed: 20261018121,
                    isolation_required: "L0", timeout_seconds: 1800}}' > train-P$b.json
done

# Stage K: multiplicative-coset (Kummer) control against low-x at matched
# actual column counts, on the two prime-order curves of the registered PKM
# ladder (research/pkm_tower_ecbench_20261013/curves.jsonl, rows 2 and 3).
# The low-x size is the actual column count `ecbench fb` reported for
# prime-kummer:t on that curve before freezing (PROTOCOL.md §4.5):
# 22-bit t=7: 68, t=8: 129; 24-bit t=8: 119, t=9: 245.
kspec() { # bits p a b order gx gy "t:cols t:cols" trials
  {
    jq -cn '{name:"rho-neg", role:"reference", method:{id:"rho.negation"}}'
    for tc in $8; do
      t=${tc%%:*}; cols=${tc#*:}
      arm "kummer-t$t" candidate "prime-kummer:t=$t" "mitm:negation_folded=1" $9
      arm "lowx-c$cols" baseline "prime-abscissa:size=$cols" "mitm:negation_folded=1" $9
    done
  } | jq -s --argjson bits $1 --argjson p $2 --argjson a $3 --argjson bb $4 --argjson n $5 \
        --argjson gx $6 --argjson gy $7 '{
      schema: "ecbench.spec/v1",
      label: ("XP-20261016-prime-base-oracle-tuning Stage K (coset control), PKM-ladder \($bits)-bit prime-order curve"),
      description: "Secondary, non-decision control of PROTOCOL.md section 4.5: Kummer multiplicative-coset base against the low-x base at the same actual column count, incumbent oracle. Public synthetic curve and targets.",
      workloads: {curves: [{kind: "prime_explicit", p: $p, a: $a, b: $bb, group_order: $n, r: $n, gx: $gx, gy: $gy}],
                  targets_per_curve: 16, target_seed: 20261018311, target_kind: "public"},
      arms: .,
      measurement: {rounds: 2, warmup: 0, order: "alternate", seed: 20261018321,
                    isolation_required: "L0", timeout_seconds: 1800}}'
}
kspec 22 2387969 1427047 558169 2390917 1686673 2366352 "7:68 8:129" 4243777 > kummer-P22.json
kspec 24 10940417 10128206 867458 10937713 3240781 414610 "8:119 9:245" 4243777 > kummer-P24.json

# Stage H: holdout templates.  run.sh's `select` stage replaces the four
# @…@ strings by the train selection (PROTOCOL.md §4.4) and writes
# holdout-P<b>.json into the run directory; nothing else may change.
for b in 20 24 28 32; do
  set -- $(inc $b); ninc=$1; trials=$2
  {
    jq -cn '{name:"rho-neg", role:"reference", method:{id:"rho.negation"}}'
    arm inc baseline "prime-abscissa:size=$ninc" "mitm:negation_folded=1" $trials
    arm sel candidate "@SEL_FB@" "@SEL_ORACLE@" $trials
    arm size candidate "@SIZE_FB@" "mitm:negation_folded=1" $trials
    arm stride control "@STRIDE_FB@" "@SEL_ORACLE@" $trials
    arm inc-aa control "prime-abscissa:size=$ninc" "mitm:negation_folded=1" $trials
    jq -cn '{name:"rho-neg-aa", role:"control", method:{id:"rho.negation"}}'
  } | jq -s --argjson b $b '{
      schema: "ecbench.spec/v1",
      label: ("XP-20261016-prime-base-oracle-tuning Stage H (holdout), \($b)-bit fresh prime-order curve"),
      description: "Holdout panel of the frozen PROTOCOL.md (section 4.4): the frontier incumbent, the train-selected base size and exact oracle, the size-only change, a strided-base control at the selected size, matched rho and A/A arms, on a fresh curve and fresh public targets. Public synthetic curve and targets; no security claim.",
      workloads: {curves: [{kind: "prime_search", bits: $b, seed: 20261018202}],
                  targets_per_curve: 48, target_seed: 20261018211, target_kind: "public"},
      arms: .,
      measurement: {rounds: 2, warmup: 1, order: "alternate", seed: 20261018221,
                    isolation_required: "L0", timeout_seconds: 1800}}' > holdout-P$b.template.json
done
