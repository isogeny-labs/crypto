#!/bin/bash
# Usage: select.sh TYPE N NUNSAT [MAXSEED]  -> appends first NUNSAT unsatisfiable instances (emin count == 0) to out/instances.jsonl
# Every scanned seed (sat or not) is logged to out/scan.jsonl.  k = (N+1)/2.
set -e
HERE=$(cd "$(dirname "$0")" && pwd); L=$HERE/../../../lfd_lower_bound_20261111
T=$1; NN=$2; WANT=$3; MAX=${4:-60}; K=$(( (NN+1)/2 )); got=0
for s in $(seq 1 "$MAX"); do
  if [ "$T" = R ]; then line=$(python3 "$L/emit_instance.py" "$NN" "$s" "$K"); else line=$(python3 "$L/emit_instance_T.py" "$NN" "$s" "$K"); fi
  c=$(echo "$line" | "$L/emin" count | tr -dc '0-9\n' | head -1)
  rec="{\"type\": \"$T\", \"n\": $NN, \"k\": $K, \"seed\": $s, \"sols\": $c, \"line\": \"$line\"}"
  echo "$rec" >> "$HERE/out/${SCANFILE:-scan.jsonl}"
  if [ "$c" = 0 ]; then echo "$rec" >> "$HERE/out/${INSTFILE:-instances.jsonl}"; got=$((got+1)); fi
  [ $got -ge "$WANT" ] && break
done
