#!/bin/sh
# Parallel split of one target's tree (opt-in KIC_SOLVE_PARALLEL_DEPTH), inherited
# F4 engine (no node oracle), one binary, RAYON_NUM_THREADS=4, arms by environment:
#   d0  KIC_SOLVE_PARALLEL_DEPTH=0 (sequential, the default)
#   d2  KIC_SOLVE_PARALLEL_DEPTH=2 (four subtrees in parallel)
#   PROBE  OUT  ROUNDS (default 5)
PROBE=${PROBE:?}; OUT=${OUT:?}; ROUNDS=${ROUNDS:-5}
mkdir -p "$OUT/raw"
cell_flags() {
  case $1 in
    K1_17) echo "--a 1 --n 17 --dim 6 --targets 8" ;;
    K1_23) echo "--a 1 --n 23 --dim 7 --targets 4" ;;
    K0_23) echo "--a 0 --n 23 --dim 8 --targets 4" ;;
    K0_31) echo "--a 0 --n 31 --dim 9 --targets 1 --budget 2000" ;;
    K0_19) echo "--a 0 --n 19 --dim 7 --targets 4" ;;
    K1_19) echo "--a 1 --n 19 --dim 6 --targets 8" ;;
    K1_17d7) echo "--a 1 --n 17 --dim 7 --targets 8" ;;
    K1_29) echo "--a 1 --n 29 --dim 8 --targets 2" ;;
    K0_37) echo "--a 0 --n 37 --dim 9 --targets 1 --budget 2000" ;;
  esac
}
r=1
while [ "$r" -le "$ROUNDS" ]; do
  for cell in K1_17 K1_23 K0_23 K0_31 K0_19 K1_19 K1_17d7 K1_29 K0_37; do
    if [ $((r % 2)) = 1 ]; then order="d0 d2"; else order="d2 d0"; fi
    for arm in $order; do
      f="$OUT/raw/r$r-$cell-$arm.jsonl"
      echo "{\"utc\":\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",\"uptime\":\"$(uptime | sed 's/.*load average[s]*: //')\",\"arm\":\"$arm\",\"cell\":\"$cell\",\"round\":$r}" > "$f"
      KIC_SOLVE_PARALLEL_DEPTH=${arm#d} RAYON_NUM_THREADS=4 "$PROBE" $(cell_flags $cell) --engines f4 --reps 1 >> "$f" 2>/dev/null
      echo "exit=$?" > "${f%.jsonl}.status"
    done
  done
  r=$((r + 1))
done
echo done
