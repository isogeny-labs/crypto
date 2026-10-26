#!/bin/sh
# Q2 sampled walks (PROTOCOL.md "Measurements", Q2): frozen walk counts per p,
# per-walk caps 2e7 tested nodes / 3300 s wall (plus the amended step cap and
# trap rule in the module), 4 cells in parallel, largest p first.
# Usage: run_walks.sh BIN OUTDIR
BIN="$1"; OUT="$2"
mkdir -p "$OUT"; cd "$OUT" || exit 1
cells=""
for k in 20 18 16 15 14 13 12 11 10; do
  if [ $k -le 13 ]; then n=40; elif [ $k -le 16 ]; then n=24; elif [ $k -le 18 ]; then n=12; else n=6; fi
  for arm in A B; do cells="$cells $k:$arm:$n"; done
done
echo $cells | tr ' ' '\n' | xargs -P 4 -I{} sh -c '
  k=$(echo {} | cut -d: -f1); arm=$(echo {} | cut -d: -f2); n=$(echo {} | cut -d: -f3)
  /usr/bin/time -l timeout 43200 nice -n 10 "$0" walk --bits $k --arm $arm --walks $n --seed 20261018 \
     --max-nodes 20000000 --max-secs 3300 --cell-secs 40000 --out "$1" > "$1/log_walk_${arm}_k$k.txt" 2>&1
  echo "exit=$?" >> "$1/log_walk_${arm}_k$k.txt"' "$BIN" "$OUT"
echo ALLDONE
