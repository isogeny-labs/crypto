#!/usr/bin/env bash
# Experiment A of PROTOCOL_F4_DEGREE32_16BIT_SLOPE_RESGCD_20261125.md: per-call solver cost against tower height t.
# usage: sweep_t_engines.sh BIN OUT.jsonl
set -u
BIN=$1; OUT=$2; SEED=20261400
run() { # label t fb oracle max_trials
  local t0 t1 res ec
  t0=$(date +%s.%N)
  res=$(RAYON_NUM_THREADS=1 timeout 900 "$BIN" bench --bits 16 --family generic --seed $SEED --repeats 1 --targets walk \
        --factor-base "$3" --oracle "$4" --max-trials "$5" --rho-runs 0 --json --solver-budget-seconds 900 2>/dev/null); ec=$?
  t1=$(date +%s.%N)
  jq -cn --arg label "$1" --argjson t "$2" --arg fb "$3" --arg or "$4" --argjson ec $ec --arg wall "$(echo "$t1 - $t0" | bc)" \
     --arg sha "$(sha256sum "$BIN" | cut -d' ' -f1)" --argjson r "${res:-null}" '
    ($r.rows[0].decomposition // {}) as $d |
    {label:$label, t:$t, factor_base:$fb, oracle:$or, exit_code:$ec, wall_s:($wall|tonumber), binary_sha256:$sha,
     instance:$r.instance, skipped:$r.configurations_skipped, abscissae:$r.rows[0].factor_base.abscissae,
     solver:$d.solver.name, op_unit:$d.solver.op_unit, solver_ops:$d.solver.ops, solver_calls:$d.solver.calls,
     solver_wall_ns:$d.solver.wall_ns, trials:$d.targets_tried, relations:$d.relations_found,
     native:$d.cost.native}' >> "$OUT"
}
first_aux() { # t -> first aux seed in 0..20 whose base builds
  for a in $(seq 0 20); do
    n=$("$BIN" bench --bits 16 --family generic --seed $SEED --repeats 1 --targets walk --factor-base "prime-isogeny:t=$1,seed=$a,aux=ordinary" \
        --oracle subtract --max-trials 1 --rho-runs 0 --json 2>/dev/null | jq '.rows[0].factor_base.abscissae // empty')
    [ -n "$n" ] && { echo $a; return; }
  done
}
for t in 5 6 7 8 9 10 11 12; do
  a=$(first_aux $t); [ -z "$a" ] && { echo "{\"t\":$t,\"error\":\"no aux seed builds\"}" >> "$OUT"; continue; }
  C="prime-isogeny:t=$t,seed=$a,aux=ordinary"; I="prime-abscissa:below=$((1<<t))"
  run cand-resgcd $t "$C" "pkm-semaev:m=2,engine=resgcd" 200
  run cand-scan   $t "$C" "pkm-semaev:m=2,engine=scan" 200
  run cand-grid   $t "$C" "pkm-semaev:m=2,engine=exhaustive" 200
  run int-resgcd  $t "$I" "pkm-semaev:m=2,engine=resgcd,bits=$t,lo=0" 200
  if [ $t -le 7 ]; then
    mt=200; [ $t -eq 7 ] && mt=20
    run cand-f4 $t "$C" "pkm-semaev:m=2,engine=f4,degree=8,mode=chain" $mt
    [ $t -le 6 ] && run int-f4 $t "$I" "pkm-semaev:m=2,engine=f4,degree=8,bits=$t,lo=0,mode=chain" 50
  fi
done
