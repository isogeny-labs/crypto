# The device pair-table build with batched inversions (round 6; protocol, frozen before the run)

## Question
Round 5 (`research/ic_gpu_rootfind_la_20261115`) left the GPU `m = 3` arm's set-up at 0.12 s at
|F_x| = 4096, and the daemon's timing lines put the pairs-mode session creation at a median 0.112 s
(16.8M keys), linear in the number of pairs. The key kernel computed every cell with two field
inversions (one per sign, a Fermat exponentiation each). `rf_pair_run` inverts a run of 16 partners of
a row together (prefix products, one inversion) and writes both signs from one inverse; the keys are
the same bit for bit (host test against `rf_pair_x`, every cell). How much of the set-up does it remove?

## Measurements
1. `rootfind_serve --bench-pairs` on the pod: both kernels (`k_pair_keys_cell`, the old one, and
   `k_pair_keys`) in one process on one device, n = 1024, 2048, 4096, random base over the 40-bit
   field; device time of each (second of two runs) and the count of cells whose keys differ.
2. `specs/smoke.json` and `specs/rf3setup.json`: the round-5 curves (38/40 bits), |F_x| 2048 and 4096,
   3 targets, 2 rounds: `rootfind-subtract:m=3` and the GPU arm (`probe_build=pairs`, daemon, cuda),
   both with `linalg=incremental-gauss-echelon`; the GPU arm's A/A copy at 4096; `rho.negation` and A/A.
   The daemon runs with `RF_SERVE_TIMING=1`, which now also prints the pair build's stages (keys,
   dedupe, compact, probe insert, copy back; each stage synchronised).

## Predictions
1. `--bench-pairs`: 0 mismatching cells at every n, and the batched kernel at least 5× faster than the
   per-cell kernel at n = 4096.
2. Identical trials, relations, logarithm, S and every phase's charge across the IC arms.
3. The GPU arm's median oracle set-up at |F_x| = 4096 at most half of round 5's 0.12 s (cross-session:
   indicative only; the in-process bench is the controlled comparison).

## Reading
L0 on a shared pod; walls descriptive. The `ecbench` binary's sha256 is recorded at dispatch (lesson
L-c830b9a6fdb2). S3: the same waiver as earlier rounds.
