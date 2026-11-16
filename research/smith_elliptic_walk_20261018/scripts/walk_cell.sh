#!/bin/sh
# One Q2 cell. Usage: walk_cell.sh BIN OUTDIR K:ARM:WALKS
BIN="$1"; OUT="$2"; k=$(echo "$3" | cut -d: -f1); arm=$(echo "$3" | cut -d: -f2); n=$(echo "$3" | cut -d: -f3)
/usr/bin/time -l timeout 43200 nice -n 10 "$BIN" walk --bits "$k" --arm "$arm" --walks "$n" --seed 20261018 \
  --max-nodes 20000000 --max-secs 3300 --cell-secs 40000 --out "$OUT" > "$OUT/log_walk_${arm}_k$k.txt" 2>&1
echo "exit=$?" >> "$OUT/log_walk_${arm}_k$k.txt"
