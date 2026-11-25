#!/usr/bin/env bash
# m = 3 companion to sweep_t_engines.sh: per-call cost of resgcd, grid and F4 (chain) against t.
# usage: sweep_t_engines_m3.sh BIN OUT.jsonl
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
    {label:$label, m:3, t:$t, factor_base:$fb, oracle:$or, exit_code:$ec, wall_s:($wall|tonumber), binary_sha256:$sha,
     instance:$r.instance, skipped:$r.configurations_skipped, abscissae:$r.rows[0].factor_base.abscissae,
     solver:$d.solver.name, solver_ops:$d.solver.ops, solver_calls:$d.solver.calls,
     trials:$d.targets_tried, relations:$d.relations_found, native:$d.cost.native}' >> "$OUT"
}
first_aux() {
  for a in $(seq 0 20); do
    n=$("$BIN" bench --bits 16 --family generic --seed $SEED --repeats 1 --targets walk --factor-base "prime-isogeny:t=$1,seed=$a,aux=ordinary" \
        --oracle subtract --max-trials 1 --rho-runs 0 --json 2>/dev/null | jq '.rows[0].factor_base.abscissae // empty')
    [ -n "$n" ] && { echo $a; return; }
  done
}
for t in 3 4 5 6 7 8; do
  a=$(first_aux $t); [ -z "$a" ] && { echo "{\"m\":3,\"t\":$t,\"error\":\"no aux seed builds\"}" >> "$OUT"; continue; }
  C="prime-isogeny:t=$t,seed=$a,aux=ordinary"
  run m3-resgcd $t "$C" "pkm-semaev:m=3,engine=resgcd" 50
  run m3-grid   $t "$C" "pkm-semaev:m=3,engine=exhaustive" 50
  [ $t -le 5 ] && run m3-f4-chain $t "$C" "pkm-semaev:m=3,engine=f4,degree=8,mode=chain" $([ $t -le 4 ] && echo 50 || echo 5)
done
