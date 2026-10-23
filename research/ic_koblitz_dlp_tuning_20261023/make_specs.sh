#!/usr/bin/env bash
# Thin orchestration: writes the ecbench.spec/v1 files of PROTOCOL.md with jq.
#
#   make_specs.sh train           -> specs/train-n{37,41,43,47}.json, specs/poscontrol-n41.json
#   make_specs.sh holdout SEL.json -> specs/holdout-n{37,41,43,47}.json from the frozen selection
#
# Curves: Koblitz K_0 at n=37 and n=41, K_1 at n=43 and n=47 (the curves of
# research/ic_frontier_campaign_20261015/phase1). Baseline columns Kb per n
# are that campaign's frozen values (spec-K*.json, arm ic-compact).
set -euo pipefail
cd "$(dirname "$0")"
mkdir -p specs
declare -A A=([37]=0 [41]=0 [43]=1 [47]=1)
declare -A KB=([37]=8 [41]=128 [43]=16 [47]=64)
declare -A TRAIN_T=([37]=48 [41]=32 [43]=32 [47]=24)
HOLD_T=96
TRAIN_TARGET_SEED=20261023001; TRAIN_ALG_SEED=20261023002
HOLD_TARGET_SEED=20261023101;  HOLD_ALG_SEED=20261023102
POS_TARGET_SEED=20261023201;   POS_ALG_SEED=20261023202
# LP grid: K x c with K*c <= 640 (PROTOCOL.md section 4).
KS="1 2 3 4 6 8 12 16"; CS="1 2 5 10 20 40 80 160 320"

ic_arm() { # name role factor_base oracle
  jq -nc --arg n "$1" --arg r "$2" --arg fb "$3" --arg o "$4" \
   '{name:$n,role:$r,method:{id:"ic.pipeline",params:{factor_base:$fb,oracle:$o,max_trials:"50000000"}}}'; }
base_arm() { ic_arm "$1" "$2" "compact-orbit-scan:columns=$3,raw_x_cap=1000000" "mitm-frobenius-counted:m=2"; }
dlp_arm() { ic_arm "$1" "$2" "compact-orbit-scan:columns=$3,raw_x_cap=1000000" "mitm-frobenius:m=3,large_primes=2,lp_domain_factor=$4"; }
rho_arm() { jq -nc --arg n "$1" --arg r "$2" --arg m "$3" '{name:$n,role:$r,method:{id:$m}}'; }
spec() { # label a n targets tseed aseed arms-json out
  jq -n --arg label "$1" --argjson a "$2" --argjson n "$3" --argjson nt "$4" --argjson ts "$5" \
        --argjson as "$6" --argjson arms "$7" \
   '{schema:"ecbench.spec/v1",label:$label,
     description:"Koblitz double-large-prime tuning against the compact-orbit m=2 baseline; research/ic_koblitz_dlp_tuning_20261023/PROTOCOL.md. Counts only (L0); public synthetic targets; no security claim.",
     workloads:{targets_per_curve:$nt,target_seed:$ts,target_kind:"public",curves:[{kind:"koblitz",a:$a,n:$n}]},
     arms:$arms,
     measurement:{rounds:1,warmup:0,order:"alternate",seed:$as,isolation_required:"L0",timeout_seconds:3600}}' > "$8"; }

case "${1:-}" in
train)
  for n in 37 41 43 47; do
    kb=${KB[$n]}; arms='[]'
    arms=$(jq -c --argjson x "$(rho_arm rho-frob reference rho.signed_frobenius)" '.+[$x]' <<<"$arms")
    for k in $((kb/4)) $((kb/2)) $kb $((kb*2)) $((kb*4)); do
      arms=$(jq -c --argjson x "$(base_arm base-k$k baseline $k)" '.+[$x]' <<<"$arms"); done
    for k in $KS; do for c in $CS; do
      [ $((k*c)) -le 640 ] || continue
      arms=$(jq -c --argjson x "$(dlp_arm dlp-k$k-c$c candidate $k $c)" '.+[$x]' <<<"$arms"); done; done
    spec "DLP tuning TRAIN Koblitz n=$n" ${A[$n]} $n ${TRAIN_T[$n]} $TRAIN_TARGET_SEED $TRAIN_ALG_SEED "$arms" specs/train-n$n.json
  done
  # Positive control: at fixed base (10 columns, n=41) the double-LP oracle against the same
  # oracle with large_primes=0; the integration note measured 23.8x there.
  arms=$(jq -nc --argjson a "$(rho_arm rho-frob reference rho.signed_frobenius)" \
    --argjson b "$(ic_arm plain-m3-k10 baseline compact-orbit-scan:columns=10,raw_x_cap=1000000 mitm-frobenius:m=3)" \
    --argjson c "$(dlp_arm dlp-k10-c10 candidate 10 10)" '[$a,$b,$c]')
  spec "DLP tuning POSITIVE CONTROL Koblitz n=41" 0 41 8 $POS_TARGET_SEED $POS_ALG_SEED "$arms" specs/poscontrol-n41.json
  ;;
holdout)
  sel=${2:?selection json}
  for n in 37 41 43 47; do
    kb=${KB[$n]}
    dk=$(jq -r --arg n $n '.[$n].dlp.k' "$sel"); dc=$(jq -r --arg n $n '.[$n].dlp.c' "$sel")
    rk=$(jq -r --arg n $n '.[$n].base_retuned.k' "$sel")
    arms=$(jq -nc --argjson a "$(rho_arm rho-frob reference rho.signed_frobenius)" \
      --argjson s "$(rho_arm rho-strong reference rho.signed_frobenius_strong)" \
      --argjson b "$(base_arm base-frozen baseline $kb)" \
      --argjson d "$(dlp_arm dlp-selected candidate $dk $dc)" \
      --argjson x "$(base_arm base-frozen-aa control $kb)" \
      --argjson y "$(dlp_arm dlp-selected-aa control $dk $dc)" '[$a,$s,$b,$d,$x,$y]')
    if [ "$rk" != "$kb" ]; then
      arms=$(jq -c --argjson x "$(base_arm base-retuned baseline $rk)" '.+[$x]' <<<"$arms"); fi
    spec "DLP tuning HOLDOUT Koblitz n=$n" ${A[$n]} $n $HOLD_T $HOLD_TARGET_SEED $HOLD_ALG_SEED "$arms" specs/holdout-n$n.json
  done
  ;;
*) echo "usage: $0 train | holdout SELECTION.json" >&2; exit 2;;
esac
