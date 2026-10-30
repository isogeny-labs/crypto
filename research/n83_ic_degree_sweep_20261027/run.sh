#!/bin/bash
set -euo pipefail
cd /work
mkdir -p results
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
uname -a > results/host.txt
lscpu >> results/host.txt
Singular --version > results/singular-version.txt 2>&1
sha256sum /opt/sweep > results/binary.sha256
cp /opt/source-revision.txt /opt/protocol.json results/
upload() { aws s3 sync /work/results "$ISOLAB_RESULTS_URI" --only-show-errors; }
trap 'upload' EXIT
/opt/sweep selfcheck /work/results/controls > results/controls.jsonl
/opt/sweep generate /work/results/inputs
upload
pids=()
for shard in 0 1 2 3; do
  (ulimit -v 3145728; /opt/sweep run /work/results/inputs 40 "$shard" 4 > "results/shard-$shard.jsonl") &
  pids+=("$!")
done
status=0
for pid in "${pids[@]}"; do wait "$pid" || status=1; done
/opt/sweep summary /work/results/inputs
(cd results && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
upload
exit "$status"
