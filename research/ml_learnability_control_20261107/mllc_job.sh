#!/usr/bin/env bash
# mllc full ladder (PROTOCOL.md). NOT for local use. Runs one process per size, 2 BLAS/OMP threads each.
# Usage: mllc_job.sh OUTDIR   (needs python3.13 with numpy==2.5.3 scipy scikit-learn==1.9.1)
set -euo pipefail
out=${1:?outdir}; mkdir -p "$out"; here=$(cd "$(dirname "$0")" && pwd); cd "$here"
export OMP_NUM_THREADS=2 OPENBLAS_NUM_THREADS=2 MKL_NUM_THREADS=2 PYTHONHASHSEED=0
python3 -I -c "import numpy,scipy,sklearn,sys;print(sys.version,numpy.__version__,scipy.__version__,sklearn.__version__)" | tee "$out/versions.txt"
sha256sum PROTOCOL.md mllc_ecgen.py mllc_harness.py | tee "$out/source.sha256"
for m in 12 16 20 24; do
  python3 -u mllc_harness.py --sizes $m --out "$out/ladder_m$m.json" > "$out/ladder_m$m.log" 2>&1 &
done
wait
sha256sum "$out"/ladder_m*.json > "$out/results.sha256"; cat "$out/results.sha256"
