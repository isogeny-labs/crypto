#!/usr/bin/env bash
# Thin driver for PROTOCOL_F4_DEGREE32_16BIT_SLOPE_RESGCD_20261125.md (+ amendment).
# usage: run_16bit_rung.sh BIN SEED AUX OUT.jsonl ARM...   ARM in int-f4 cand-f4 cand-resgcd cand-scan int-resgcd int-subtract
set -u
BIN=$1; SEED=$2; AUX=$3; OUT=$4; shift 4
INT=prime-abscissa:below=32; CAND=prime-isogeny:t=5,seed=$AUX,aux=ordinary
COMMON=(bench --bits 16 --family generic --seed "$SEED" --repeats 16 --targets walk --max-trials 20000 --rho-runs 1 --json --solver-budget-seconds 5 --linalg incremental-gauss-full-rank-checked)
for arm in "$@"; do
  case $arm in
    int-f4)       FB=$INT;  OR="pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain";;
    cand-f4)      FB=$CAND; OR="pkm-semaev:m=2,engine=f4,degree=8,mode=chain";;
    cand-resgcd)  FB=$CAND; OR="pkm-semaev:m=2,engine=resgcd,degree=8,mode=chain";;
    cand-scan)    FB=$CAND; OR="pkm-semaev:m=2,engine=scan,degree=8,mode=chain";;
    int-resgcd)   FB=$INT;  OR="pkm-semaev:m=2,engine=resgcd,degree=8,bits=5,lo=0,mode=chain";;
    int-subtract) FB=$INT;  OR="subtract";;
    *) echo "bad arm $arm" >&2; exit 2;;
  esac
  t0=$(date +%s.%N)
  res=$(RAYON_NUM_THREADS=1 "$BIN" "${COMMON[@]}" --factor-base "$FB" --oracle "$OR" 2>/tmp/err_$$); ec=$?
  t1=$(date +%s.%N)
  jq -cn --arg arm "$arm" --argjson seed "$SEED" --arg fb "$FB" --arg or "$OR" --argjson ec $ec \
     --arg wall "$(echo "$t1 - $t0" | bc)" --arg sha "$(sha256sum "$BIN" | cut -d' ' -f1)" \
     --arg err "$(tail -c 2000 /tmp/err_$$)" --argjson result "${res:-null}" \
     '{id:("timed:"+$arm), arm:$arm, seed:$seed, factor_base:$fb, oracle:$or, exit_code:$ec, wall_s:($wall|tonumber), binary_sha256:$sha, stderr_tail:$err, result:$result}' >> "$OUT"
done
