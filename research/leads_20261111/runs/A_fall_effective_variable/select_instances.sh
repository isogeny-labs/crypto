#!/bin/bash
# Scan seeds 1,2,... per cell; record the solution count (emin count) of each instance line.
# Usage: select_instances.sh TYPE N K MAXSEED >> out/instances.jsonl
set -e
HERE=$(cd "$(dirname "$0")" && pwd); L=$HERE/../../../lfd_lower_bound_20261111
T=$1; NN=$2; K=$3; MAX=$4
for s in $(seq 1 "$MAX"); do
  if [ "$T" = R ]; then line=$(python3 "$L/emit_instance.py" "$NN" "$s" "$K"); else line=$(python3 "$L/emit_instance_T.py" "$NN" "$s" "$K"); fi
  c=$(echo "$line" | "$L/emin" count | tr -dc '0-9\n' | head -1)
  echo "{\"type\": \"$T\", \"n\": $NN, \"k\": $K, \"seed\": $s, \"sols\": $c, \"line\": \"$line\"}"
done
