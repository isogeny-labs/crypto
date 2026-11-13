# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (round 2)

## Goal
User (2026-11-13): "More improvements" after PR #4029 (round 1, merged c5678a98ae) on the IC CUDA kernels.

## Lead / hypothesis
1. FES fes_walk_block<K>: low K derivatives and cross terms in registers (compile-time indices);
   expect fewer instructions per point than fes_walk_gray. Same values, same order.
2. macaulay MAC_UPDATE_2D=1: lanes x columns, warps x rows, no cursor. Expect fewer instructions per update.
3. rootfind launch knobs (RF_BLOCK, RF_MIN_BLOCKS), never measured; ncu if the pod allows counters.

## Spec
research/ic_gpu_measure_20261112/job_round2.sh on one A100 (same class as round 1); trees: origin/main
c5678a98ae vs this branch. Record benchmarks/records/<stamp>-ic-gpu-round2.json.

## State
2026-11-13 01:20Z: round 2 done, pod deleted, record 20261113T0057Z-ic-gpu-round2 closed, PR open.

## Runs in flight
Pods named icgpu-*; delete with research/ic_gpu_measure_20261112/rp_gpu.sh delete NAME.

## Stop and success conditions
Identity failure on any arm = that arm is excluded. Speed counts only outside the A/A spread.
Cap: ~2 GPU-hours, $5. S3 checkpoint: unavailable (AWS keys rejected); same waiver as round 1.

## Next steps
Track the PR. Then: wire the FES worker into a relation phase for an end-to-end number; rootfind profile on a host with GPU counters.
