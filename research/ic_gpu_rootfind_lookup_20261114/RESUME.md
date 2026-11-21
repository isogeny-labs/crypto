# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (rootfind lookup variants)

## Goal
User (2026-11-14): "Yes, try the rootfind lookup improvements" (research/ic_gpu_replay_20261114:
41-85% of rootfind's sampled cycles are in table probes).

## Lead / hypothesis
Packed hash slots (one load per probe step), paired eps lookups (overlapping latency), Eytzinger sorted
search (cache-friendlier top levels). Each opt-in flag; identical relation sets by construction and by
device selftest. Expect the hash path at |F|=65536 to gain most.

## Spec
research/ic_gpu_rootfind_lookup_20261114/job.sh on one A100: eight arms (M, M2 A/A, E, H, P, EP, HP,
EHP), selftests first, 5 rotated rounds of ./bench throughput at |F|=65536 and |F|=1024.

## State
2026-11-14 19:45Z: done; pod deleted; all three flags default on (1.12x at best configurations); record closed; PR open.

## Runs in flight
Pods icgpu-*; research/ic_gpu_measure_20261112/rp_gpu.sh delete NAME.

## Stop and success conditions
Any selftest failure excludes the arm. A gain counts only outside the M/M2 spread and at the best
configuration per workload (not just at some configuration). Cap ~1 GPU-hour, $3. S3: same waiver.

## Next steps
Track the PR.
