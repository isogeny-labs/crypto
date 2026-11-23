#!/usr/bin/env bash
# Run once only after publication/pin checks and qualification; no concurrent work.
set -euo pipefail
packed_study=$(cd -- "$(dirname -- "$0")" && pwd)
packed_repo=$(cd -- "$packed_study/../.." && pwd)
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
cd -- "$packed_repo"
python3 tools/isolated_bench.py reserve --cpus 0 \
  --out /workspace/fusion-packed-monitor-200.jsonl --period 0.1 \
  --max-other-cpu 0.1 --label relation-packed-200 -- \
  taskset -c 0 /workspace/relation-native-qualifiers/fusion-200/packed-compare \
  /workspace/fusion-packed-200 \
  /workspace/relation-native-qualifiers/fusion-85/fused-native \
  /workspace/relation-native-qualifiers/fusion-96/fused-native \
  /workspace/relation-native-qualifiers/fusion-200/fused-native \
  /workspace/fusion-qualification-66
