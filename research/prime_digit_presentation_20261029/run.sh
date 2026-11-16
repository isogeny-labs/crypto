#!/bin/sh
# PROTOCOL.md section 3, thin orchestration only (every number is native `digit_fall`).
# Usage: ./run.sh OUT_DIR
#   OUT_DIR must not exist: records go to OUT_DIR/{A..E}.jsonl, never to the committed results/.
#   The committed results/ were produced by registered/run.sh.v0 (panel A) and registered/run.sh.v1
#   (panels B-E); this launcher has the same cells, draw counts, cmax values and seeds.
# Build: rustc -O digit_fall.rs -o digit_fall   (DIGIT_FALL=path selects the binary)
set -eu
cd "$(dirname "$0")"
[ $# -eq 1 ] || { echo "usage: $0 OUT_DIR (a directory that does not exist yet)" >&2; exit 2; }
OUT=$1
[ ! -e "$OUT" ] || { echo "refusing to overwrite existing $OUT" >&2; exit 2; }
B=${DIGIT_FALL:-./digit_fall}
T=${THREADS:-4}
mkdir -p "$OUT"
run() { # panel m s arm draws cmax [extra]
  seed=$((20261029 + $2 * 100000 + $3 * 1000 + $(printf '%s' "$4" | cksum | cut -d' ' -f1) % 997))
  "$B" digit --m "$2" --s "$3" --arm "$4" --draws "$5" --seed "$seed" --cmax "$6" --threads "$T" ${7:-} >> "$OUT/$1.jsonl"
}
for p in A B C D E; do : > "$OUT/$p.jsonl"; done
for s in 2 3 4 5; do run A 2 $s U 12 14; done; run A 2 6 U 8 14; run A 2 7 U 4 14
for s in 2 3 4 5 6; do run B 2 $s P 8 14; done; run B 2 7 P 4 14
for s in 2 3 4 5 6; do for a in Usupp Ublock; do run C 2 $s $a 8 14; done; done
for a in Usupp Ublock; do run C 2 7 $a 4 14; done
run D 3 4 U 4 16
run D 3 4 U 4 16 "--first-fall-only true"
run D 3 5 U 4 16 "--first-fall-only true"
for p in 7 11 13 17 19 23 29 31; do
  "$B" plain --p $p --draws 4 --seed $((20261029 + p)) --cmax $((2 * p + 2)) --threads "$T" >> "$OUT/E.jsonl"
done
# last, because it is the slowest cell (n = 16)
run B 2 8 P 2 14
