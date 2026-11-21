#!/usr/bin/env bash
# CONFIRM_PROTOCOL.md arms, rotated order. Usage: scripts_confirm.sh OLDBIN NEWBIN ROUNDS OUTDIR
OLD=$1; NEW=$2; ROUNDS=$3; OUT=$4; mkdir -p "$OUT"; : > "$OUT/runs.jsonl"
ARMS=(H0 H1 H1a F0 F1); TIMEFORMAT='%U %S'
for r in $(seq 1 "$ROUNDS"); do
  for k in 0 1 2 3 4; do
    a=${ARMS[$(( (k + r) % 5 ))]}
    case $a in H0) b=$OLD; f=(--stop hilbert:16);; H1|H1a) b=$NEW; f=(--stop hilbert:16 --trace);; F0) b=$OLD; f=();; F1) b=$NEW; f=();; esac
    j="$OUT/r${r}_${a}.json"
    cpu=$( { time env RAYON_NUM_THREADS=4 "$b" --exp dlp --sizes 53 --seeds 1 --rho-runs 0 "${f[@]}" --json "$j" > /dev/null 2>&1; } 2>&1 )
    jq -c --arg a "$a" --argjson r "$r" --arg cpu "$cpu" '.[0] | {arm:$a, round:$r, wall_ms, cpu_s: ($cpu|split(" ")|map(tonumber)|add), residuals, relations, oracle_muls, correct, s}' "$j" >> "$OUT/runs.jsonl"
  done
done
