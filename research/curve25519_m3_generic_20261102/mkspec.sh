#!/bin/bash
# usage: mkspec.sh train|holdout k [selected_arm_name]   (emits spec JSON on stdout)
# Specs are generated with jq (no research computation).
set -e
cd /home/user/crypto
phase=$1; k=$2; sel=${3:-}
B=research/curve25519_ic_baseline_20261102/specs/c25519-k$k.json
case $k in 20) Na=32;; 24) Na=81;; 28) Na=203;; esac
fb(){ echo "prime-projected-abscissa:size=$1,scan_cap=$((16*$1))"; }
arm(){ jq -n --arg n "$1" --arg r "$2" --arg fb "$3" --arg o "$4" \
  '{name:$n,role:$r,method:{id:"ic.pipeline",params:{factor_base:$fb,oracle:$o}}}'; }
rho(){ jq -n --arg n "$1" --arg r "$2" '{name:$n,role:$r,method:{id:"rho.negation"}}'; }
arms=()
if [ $phase = train ]; then
  arms+=("$(rho rho-neg reference)")
  arms+=("$(arm inc baseline "$(fb $Na)" mitm:negation_folded=1)")
  for f in 0.25 0.35 0.5 0.7; do
    N=$(awk -v n=$Na -v f=$f 'BEGIN{printf "%d", n*f+0.5}')
    arms+=("$(arm mitm3-n$N candidate "$(fb $N)" mitm:m=3,negation_folded=1)")
    arms+=("$(arm ksum3-n$N candidate "$(fb $N)" ksum:m=3)")
  done
  seed=$((2026111000+k)); tg=16; rounds=1
else
  # sel is e.g. mitm3-n16 or ksum3-n16
  N=${sel##*-n}; case $sel in mitm3*) o=mitm:m=3,negation_folded=1;; ksum3*) o=ksum:m=3;; esac
  arms+=("$(rho rho-neg reference)")
  arms+=("$(arm inc baseline "$(fb $Na)" mitm:negation_folded=1)")
  arms+=("$(arm sel candidate "$(fb $N)" "$o")")
  arms+=("$(arm inc-aa control "$(fb $Na)" mitm:negation_folded=1)")
  arms+=("$(rho rho-neg-aa control)")
  seed=$((2026111100+k)); tg=32; rounds=2
fi
printf '%s\n' "${arms[@]}" | jq -s --slurpfile b $B --arg ph $phase --argjson k $k --argjson seed $seed --argjson tg $tg --argjson rounds $rounds '
 {arms:., description:("research/curve25519_m3_generic_20261102/PROTOCOL.md, "+$ph+", k = "+($k|tostring)+". Public synthetic curve and targets; no security claim."),
  label:("Curve25519-family m=3 generic, "+$ph+", k = "+($k|tostring)),
  measurement:{isolation_required:"L0",order:($ph|if .=="train" then "alternate_measured" else "alternate" end),rounds:$rounds,seed:($seed+7),timeout_seconds:1800,warmup:1},
  schema:"ecbench.spec/v1",
  workloads:{curves:[$b[0].workloads.curves[0]],target_kind:"public",target_seed:$seed,targets_per_curve:$tg}}'
