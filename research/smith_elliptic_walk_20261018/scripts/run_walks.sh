#!/bin/sh
# Q2 sampled walks (PROTOCOL.md "Measurements", Q2): frozen walk counts per p,
# per-walk caps 2e7 tested nodes / 3300 s wall (plus the amended step cap and
# trap rule in the module), 4 cells in parallel, largest p first.
# Usage: run_walks.sh BIN OUTDIR
BIN="$1"; OUT="$2"; HERE=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$OUT"
for k in 20 18 16 15 14 13 12 11 10; do
  if [ $k -le 13 ]; then n=40; elif [ $k -le 16 ]; then n=24; elif [ $k -le 18 ]; then n=12; else n=6; fi
  for arm in A B; do echo "$k:$arm:$n"; done
done | xargs -P 4 -n 1 sh "$HERE/walk_cell.sh" "$BIN" "$OUT"
echo ALLDONE
