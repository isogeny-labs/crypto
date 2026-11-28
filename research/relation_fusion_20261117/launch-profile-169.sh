#!/usr/bin/env bash
# Prospective169; run only after source/pins publication and no active builds.
set -euo pipefail
profile_study=$(cd -- "$(dirname -- "$0")" && pwd)
profile_repo=$(cd -- "$profile_study/../.." && pwd)
profile_tools=/workspace/fusion-profile-tools-85/root
export PATH="$profile_tools/usr/bin:$PATH"
export VALGRIND_LIB="$profile_tools/usr/libexec/valgrind"
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
cd -- "$profile_repo"
python3 tools/isolated_bench.py reserve --cpus 0 \
  --out /workspace/fusion-profile-monitor-169.jsonl --period 0.1 \
  --max-other-cpu 0.1 --label relation-cost-profile-169 -- \
  taskset -c 0 /workspace/relation-native-qualifiers/fusion-169/profile-matrix \
  /workspace/fusion-profile-matrix-169 \
  /workspace/relation-native-qualifiers/fusion-85/fused-native \
  /workspace/relation-native-qualifiers/fusion-96/fused-native \
  /workspace/fusion-qualification-66 "$profile_repo/tools/profile.sh" \
  "$profile_study/PROFILE_PINS_169.json"
