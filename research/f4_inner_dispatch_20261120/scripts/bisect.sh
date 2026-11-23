#!/usr/bin/env bash
# Knob bisection of the 4-thread small-system cost. Usage: bisect.sh BIN ROUNDS OUT.jsonl
BIN=$1; ROUNDS=$2; OUT=$3; : > "$OUT"
declare -A ARMS=(
  [default]=""
  [par_old]="F4_FP_PAR_OLD=1"
  [no_sym]="F4_FP_PAR_SYM_SEEDS=1000000000"
  [no_flwork]="F4_FP_PAR_FL_WORK=1000000000000"
  [no_inblock]="F4_FP_PAR_FL_INBLOCK=1000000000000"
  [rows]="F4_FP_PAR_FL_MODE=rows"
)
NAMES=(default par_old no_sym no_flwork no_inblock rows t1_default t1_par_old)
for r in $(seq 1 "$ROUNDS"); do
  # rotate the arm order each round
  for k in $(seq 0 $((${#NAMES[@]}-1))); do
    a=${NAMES[$(( (k + r) % ${#NAMES[@]} ))]}
    th=4; envs=${ARMS[$a]:-}
    case $a in t1_default) th=1; envs="";; t1_par_old) th=1; envs="F4_FP_PAR_OLD=1";; esac
    env -u F4_FP_MEM $envs RAYON_NUM_THREADS=$th "$BIN" 5 2>/dev/null | jq -c --arg a "$a" --argjson r "$r" \
      'select(.case) | {arm:$a, round:$r, case, f4_ms, basis_fp, field_ops}' >> "$OUT"
  done
done
