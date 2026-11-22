#!/usr/bin/env bash
# jv_cover ccov screen of the F4 cascade/FL threshold. Usage: fl_screen.sh BIN ROUNDS OUTDIR
BIN=$1; ROUNDS=$2; OUT=$3; mkdir -p "$OUT"; : > "$OUT/runs.jsonl"
NAMES=(default fl_always fl_128 fl_512 fl_off)
envs_of() { case $1 in default) echo "";; fl_always) echo "F4_FP_FL_MIN_COLS=0";; fl_128) echo "F4_FP_FL_MIN_COLS=128";; fl_512) echo "F4_FP_FL_MIN_COLS=512";; fl_off) echo "F4_FP_FL=0";; esac; }
for r in $(seq 1 "$ROUNDS"); do
  for p in 53 101; do
    for k in 0 1 2 3 4; do
      a=${NAMES[$(( (k + r) % 5 ))]}
      f="$OUT/r${r}_p${p}_${a}.json"
      env -u F4_FP_MEM $(envs_of $a) RAYON_NUM_THREADS=1 "$BIN" --exp ccov --sizes $p --seeds 1 --residuals 300 --constructed 8 --oracle-base 0 --json "$f" > /dev/null 2>&1
      jq -c --arg a "$a" --argjson r "$r" --argjson p "$p" '.[0] | {arm:$a, round:$r, p:$p, f4_muls:.f4_muls.mean, f4_ms:.f4_ms.mean, lin_muls:.lin_muls.mean, c_cov:.c_cov.mean, decomposable:.random_decomposable, planted:.planted_found, rows:.max_rows.mean, cols:.max_cols.mean, wall_ms}' "$f" >> "$OUT/runs.jsonl"
    done
  done
done
