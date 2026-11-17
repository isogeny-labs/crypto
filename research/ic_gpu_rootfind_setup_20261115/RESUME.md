# Resume: crypto @ claude/index-calculus-gpu-optimization-otouqv (round 6: device pair-table build with batched inversions)

## Goal
User (2026-11-15): "keep working on open leads". After round 5 the GPU m = 3 set-up at |F_x| = 4096
(0.12 s; session creation 0.112 s) is one of the three equal bounds. Cut it.

## Lead / hypothesis
k_pair_keys did two Fermat inversions per cell; rf_pair_run (gpu/rootfind/rootfind.cuh) inverts runs
of 16 partners together, same keys bit for bit (host test, every cell; mutation caught). Expected:
the key kernel >= 5x faster at n = 4096; set-up roughly halved or better.

## Spec
PROTOCOL.md; specs/smoke.json, specs/rf3setup.json; job.sh (bench-pairs first: ABORT on any mismatch);
run_pod.sh under benchrec run (records the ecbench sha256 at dispatch).

## State
2026-11-15: code, host tests and protocol committed; pod not yet launched.

## Runs in flight
None yet (pod icgpu-rsu; cleanup: research/ic_gpu_measure_20261112/rp_gpu.sh delete icgpu-rsu).

## Stop and success conditions
Any key mismatch in bench-pairs aborts; any S/relation/logarithm mismatch voids the comparison.
Cap ~1 GPU-hour, $3. S3: same waiver as earlier rounds.
