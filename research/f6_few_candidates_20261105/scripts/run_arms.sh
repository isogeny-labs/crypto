#!/bin/sh
# F6-IC few-candidate closure, one binary, arms by environment only:
#   on   no switch set (closure on, KIC_F6_FEW_CANDIDATES default 64)
#   off  KIC_F6_FEW_CANDIDATES=0 (the gate as before)
# Cells as in research/inherited_f4_eager_runs_20261016; engines f4 (control:
# no gate, must be unchanged) and f6p (F6-IC with the pair index); 3 reps.
#   PROBE  path to f6_ic_probe    OUT  output directory    ROUNDS (default 5)
PROBE=${PROBE:?set PROBE}; OUT=${OUT:?set OUT}; ROUNDS=${ROUNDS:-5}
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
run_arm() {
  arm=$1; cell=$2; round=$3
  file="$OUT/raw/r${round}-${cell}-${arm}.jsonl"
  echo "{\"uptime\":\"$(uptime | sed 's/.*load average[s]*: //')\",\"utc\":\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",\"arm\":\"$arm\",\"cell\":\"$cell\",\"round\":$round}" > "$file"
  case $arm in
    on)  env -u KIC_F6_FEW_CANDIDATES "$PROBE" $(cell_flags "$cell") --engines f4,f6p --reps 3 >> "$file" 2>/dev/null ;;
    off) KIC_F6_FEW_CANDIDATES=0 "$PROBE" $(cell_flags "$cell") --engines f4,f6p --reps 3 >> "$file" 2>/dev/null ;;
  esac
  echo "exit=$?" > "$OUT/raw/r${round}-${cell}-${arm}.status"
}
round=1
while [ "$round" -le "$ROUNDS" ]; do
  for cell in K1_17 K1_23 K0_23 K0_31 K0_19 K1_19 K1_17d7 K1_29 K0_37; do
    if [ $((round % 2)) = 1 ]; then order="on off"; else order="off on"; fi
    for arm in $order; do run_arm "$arm" "$cell" "$round"; done
  done
  round=$((round + 1))
done
echo done
