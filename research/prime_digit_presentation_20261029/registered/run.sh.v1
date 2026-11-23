#!/bin/sh
# PROTOCOL.md section 3, thin orchestration only (every number is native `digit_fall`).
# Build: rustc -O digit_fall.rs -o digit_fall   (the registered run used the binary in BINARY.sha256)
set -eu
cd "$(dirname "$0")"
B=${DIGIT_FALL:-./digit_fall}
T=${THREADS:-4}
mkdir -p results
run() { # panel m s arm draws cmax [extra]
  seed=$((20261029 + $2 * 100000 + $3 * 1000 + $(printf '%s' "$4" | cksum | cut -d' ' -f1) % 997))
  "$B" digit --m "$2" --s "$3" --arm "$4" --draws "$5" --seed "$seed" --cmax "$6" --threads "$T" ${7:-} >> "results/$1.jsonl"
}
# Panel A was produced by the registered binary (results/A.jsonl); PANELS=ABCDE reruns it too.
PANELS=${PANELS:-BCDE}
case $PANELS in *A*) : > results/A.jsonl;; esac
: > results/B.jsonl; : > results/C.jsonl; : > results/D.jsonl; : > results/E.jsonl
case $PANELS in *A*) for s in 2 3 4 5; do run A 2 $s U 12 14; done; run A 2 6 U 8 14; run A 2 7 U 4 14;; esac
for s in 2 3 4 5 6; do run B 2 $s P 8 14; done; run B 2 7 P 4 14
for s in 2 3 4 5 6; do for a in Usupp Ublock; do run C 2 $s $a 8 14; done; done
for a in Usupp Ublock; do run C 2 7 $a 4 14; done
run D 3 4 U 4 16
run D 3 4 U 4 16 "--first-fall-only true"
run D 3 5 U 4 16 "--first-fall-only true"
for p in 7 11 13 17 19 23 29 31; do
  "$B" plain --p $p --draws 4 --seed $((20261029 + p)) --cmax $((2 * p + 2)) --threads "$T" >> results/E.jsonl
done
# last, because it is the slowest cell (n = 16)
run B 2 8 P 2 14
