#!/usr/bin/env bash
# usage: verify_replay.sh <same-seed replay dir> <seed-offset replay dir>  -> writes VERIFICATION.json
# Compares every results/*.tsv of the same-seed quick replay with this run column by column (wall_s dropped: contention-affected),
# and quotes the mechanical verdict rows of the seed-offset replay.
cd "$(dirname "$0")"
SAME="$1"; SEED="$2"
strip() { awk -F'\t' 'NR==1{for(i=1;i<=NF;i++) if($i=="wall_s") w=i} {o=""; for(i=1;i<=NF;i++) if(i!=w) o=o (o==""?"":"\t") $i; print o}' "$1"; }
{
echo '{"same_seed_replay":['
first=1
for f in "$SAME"/results/*.tsv; do
  b=$(basename "$f")
  [ -f "results/$b" ] || continue
  if diff -q <(strip "results/$b") <(strip "$f") >/dev/null; then r=true; else r=false; fi
  [ $first = 1 ] || echo ','
  first=0
  printf '{"file":"%s","identical_excluding_wall_time":%s}' "$b" "$r"
done
echo '],'
echo '"samples_identical":['
first=1
for f in "$SAME"/results/samples/*.txt; do
  b=$(basename "$f")
  [ -f "results/samples/$b" ] || continue
  if cmp -s "results/samples/$b" "$f"; then r=true; else r=false; fi
  [ $first = 1 ] || echo ','
  first=0
  printf '{"file":"%s","identical":%s}' "$b" "$r"
done
echo '],'
echo '"seed_offset_replay":{"L2SEED_XOR":777,"cells":"quick subset (k<=19 binary and prime arms, controls, rho k<=23, full solves k<=19)","verdict_rows":['
first=1
grep -E '^\| F[0-9] ' "$SEED/results/auto_tables.md" | while IFS= read -r line; do
  [ $first = 1 ] || echo ','
  first=0
  esc=$(printf '%s' "$line" | sed 's/\\/\\\\/g; s/"/\\"/g')
  printf '"%s"' "$esc"
done
echo ']},'
echo '"gp_cross_check":['
first=1
while IFS= read -r line; do
  [ $first = 1 ] || echo ','
  first=0
  esc=$(printf '%s' "$line" | sed 's/\\/\\\\/g; s/"/\\"/g')
  printf '"%s"' "$esc"
done < results/gp_verify.txt
echo ']}'
} > VERIFICATION.json
