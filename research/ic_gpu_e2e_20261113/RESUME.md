# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (FES GPU end to end)

## Goal
User (2026-11-13): hook the FES GPU worker into a real relation phase for the first end-to-end
index-calculus number.

## Lead / hypothesis
fes-f2-gpu (commit 0d28a6e605): fes-f2 with the search on the gpu/fes CUDA serve worker. Same
counted cost as fes-f2 by construction, so counted S cannot move; the expectation is a large drop in
relation-phase wall time at 24-30 unknowns, where CPU fes-f2 costs 11-700 ms per call and
inherited-f4 14-82 ms (research/inherited_f4_next_20261112/RESULTS_AB.md).

## Spec
research/ic_gpu_e2e_20261113/job_e2e.sh on one A100: `ic bench --sweep` for koblitz-n23 (inherited-f4,
fes-f2, fes-f2-gpu, 2 repeats), char2-n25 (same, 1 repeat), char2-n29 (inherited-f4, fes-f2-gpu).
ic binary rebuilt after the order fix (see commit after 343ac0bbc2); first n=23 run kept as raw/run1-unordered-koblitz-n23.json. Record benchmarks/records/<stamp>-ic-fes-gpu-e2e.json.

## State
2026-11-13 04:25Z: done. Pod deleted; record 20261113T0330Z-ic-fes-gpu-e2e closed; REPORT.md; PR open.

## Runs in flight
Pods icgpu-*; research/ic_gpu_measure_20261112/rp_gpu.sh delete NAME.

## Stop and success conditions
Every row must recover the planted log (verified) and fes-f2-gpu must equal fes-f2 in S and hit rate.
Wall time is a practicality note (AGENTS.md section 6). Cap: ~2 GPU-hours, $5. S3: same waiver as before.

## Next steps
Track the PR. Then: cut per-call overhead (binary protocol, batching).
