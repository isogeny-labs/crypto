# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (rootfind in a measured IC solve + Eytzinger prefetch)

## Goal
User (2026-11-14), "do these": (1) independent replay of the rootfind lookup measurement (validator:
research/ic_gpu_rootfind_lookup_20261114/VALIDATION_REVIEW.md, validated, scoped; corrections applied);
(2) hook rootfind into a measured index-calculus run; (3) prefetch the next level of the sorted search.

## Lead / hypothesis
(2) New ic_framework oracle `rootfind-subtract-gpu` (src/cryptanalysis/ic_framework/rootfind_gpu.rs):
rootfind-subtract (m = 2) with the sweep batched on gpu/rootfind via the persistent worker
rootfind_serve(_cuda); identical relations, logarithm and counted S by construction (48 local pipeline
runs and a 16-record ecbench smoke equal; mutation positive control fails). Expect wall time of the
relation phase to fall by a large factor at 28-32 bits; S unchanged (the device does not change S).
(3) RF_EYT_PREFETCH=d prefetches eyt[k<<d] during the descent; expect a few % on the sorted path at
|F| = 65536, nothing at |F| = 1024 (hash path).

## Spec
specs/main.json (frozen): prime_search 28/30/32 bits, 2 targets, 2 rounds; arms rho-neg (reference),
rf2-{cpu,gpu}-F{4096,16384}, rho-neg-aa. job.sh on one A100: Part 1 ecbench smoke + main, verify
--replay-all, table, compare; Part 2 six bench builds (D, D2 A/A, P3, P4, P5, P4L2), selftests first,
5 rotated rounds of ./bench throughput at |F| = 65536 and |F| = 1024.

## State
2026-11-14 23:45Z: done. Pod deleted. IC: 72/72 verified, replays reproduced, 24/24 pairs equal S; relation
phase 21-121x, solve 2.5-43.5x faster (REPORT.md). Prefetch: 1.2-1.9x slower, default off. Record closed.

## Runs in flight
None (icgpu-ric deleted 23:43Z).

## Stop and success conditions
Part 1: any cpu/gpu S or relation mismatch, or a failed replay, voids the IC comparison. A wall gain is
reported per workload with the CPU arm single-threaded (stated). Part 2: selftest failure excludes an
arm; a prefetch gain counts only outside the D/D2 spread at the best configuration per workload.
Cap ~1.5 GPU-hours, $4. S3: same waiver as the earlier rounds (session AWS keys rejected).

## Next steps
Track the PR. Open: m = 3 on the GPU; a validator check of REPORT.md.
