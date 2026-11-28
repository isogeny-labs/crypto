#!/usr/bin/env bash
# m = 2 resgcd at large t (gcd degree 2^t): classical / ntt / half / half+ntt, same targets.
# usage: sweep_m2_high_t.sh BIN OUT.jsonl
set -u
BIN=$1; OUT=$2; SEED=20261400
for t in 12 13; do
  a=""; for s in $(seq 0 20); do n=$("$BIN" bench --bits 16 --family generic --seed $SEED --repeats 1 --targets walk --factor-base "prime-isogeny:t=$t,seed=$s,aux=ordinary" --oracle subtract --max-trials 1 --rho-runs 0 --json 2>/dev/null | jq '.rows[0].factor_base.abscissae // empty'); [ -n "$n" ] && { a=$s; break; }; done
  [ -z "$a" ] && { echo "{\"m\":2,\"t\":$t,\"error\":\"no aux seed builds\"}" >> "$OUT"; continue; }
  for arm in "classical:" "ntt:,mul=ntt" "half:,gcd=half" "half-ntt:,gcd=half,mul=ntt"; do
    label=${arm%%:*}; extra=${arm#*:}
    t0=$(date +%s.%N)
    res=$(RAYON_NUM_THREADS=1 timeout 1800 "$BIN" bench --bits 16 --family generic --seed $SEED --repeats 1 --targets walk \
      --factor-base "prime-isogeny:t=$t,seed=$a,aux=ordinary" --oracle "pkm-semaev:m=2,engine=resgcd$extra" --max-trials 30 --rho-runs 0 --json 2>/dev/null); ec=$?
    t1=$(date +%s.%N)
    jq -cn --arg label "m2-resgcd-$label" --argjson t $t --argjson ec $ec --arg wall "$(echo "$t1 - $t0" | bc)" --arg sha "$(sha256sum "$BIN" | cut -d' ' -f1)" --argjson r "${res:-null}" '
      ($r.rows[0].decomposition // {}) as $d | {label:$label, m:2, t:$t, exit_code:$ec, wall_s:($wall|tonumber), binary_sha256:$sha,
      abscissae:$r.rows[0].factor_base.abscissae, solver:$d.solver.name, solver_ops:$d.solver.ops, solver_calls:$d.solver.calls,
      trials:$d.targets_tried, relations:$d.relations_found, native:$d.cost.native}' >> "$OUT"
  done
done
