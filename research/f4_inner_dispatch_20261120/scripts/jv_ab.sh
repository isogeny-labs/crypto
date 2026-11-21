#!/usr/bin/env bash
# jv_cover dlp A/B across F4 dispatch arms. Usage: jv_ab.sh BIN ROUNDS OUTDIR
BIN=$1; ROUNDS=$2; OUT=$3; mkdir -p "$OUT"; : > "$OUT/runs.jsonl"
NAMES=(default par_old no_sym)
envs_of() { case $1 in default) echo "";; par_old) echo "F4_FP_PAR_OLD=1";; no_sym) echo "F4_FP_PAR_SYM_SEEDS=1000000000";; esac; }
for r in $(seq 1 "$ROUNDS"); do
  for k in 0 1 2; do
    a=${NAMES[$(( (k + r) % 3 ))]}
    f="$OUT/r${r}_${a}.json"
    env -u F4_FP_MEM $(envs_of $a) RAYON_NUM_THREADS=4 "$BIN" --exp dlp --sizes 53 --seeds 1 --rho-runs 0 --json "$f" > /dev/null 2>&1
    jq -c --arg a "$a" --argjson r "$r" '.[0] | {arm:$a, round:$r, wall_ms, residuals, decompositions, relations, solved, correct, oracle_muls, total_muls, s}' "$f" >> "$OUT/runs.jsonl"
  done
done
