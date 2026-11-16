#!/bin/bash
set -euo pipefail
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
cd /results
Singular --version > singular-version.txt 2>&1
/opt/fixed-sweep selfcheck /results/controls > controls.jsonl
/opt/fixed-sweep generate /results/inputs /bases
pids=()
for shard in 0 1 2 3; do
 (ulimit -v 3145728; /opt/fixed-sweep run /results/inputs 40 "$shard" 4 > "shard-$shard.jsonl") &
 pids+=("$!")
done
status=0
for pid in "${pids[@]}"; do wait "$pid" || status=1; done
/opt/fixed-sweep summary /results/inputs
exit "$status"
