# Full-width Curve25519 pair collector candidate

The native candidate constructs a bounded public factor base, searches signed two-summand decompositions, verifies every discovered relation, inserts its exact coefficients modulo the Curve25519 subgroup order and verifies any recovered target scalar. Factor-base lifts are generated from successive public x coordinates and cleared by the cofactor; the generator is the first base point and its logarithm1 is the only pre-admitted logarithm. No target secret or factor-base logarithm vector is accepted. [pcc:unverified]

The collector probes Q first, then rejection-samples full-width a and nonzero b using separately named SHA256 domains and probes aG+bQ. Hash-table keys include complete points. Repeated summands and negative coefficients are combined modulo the exact order. Caps bound base size to128, x scans to1000000 and relation trials to1000000; each relation trial performs at most2N lookups. Factor-base exhaustion is an explicit partial result. [pcc:unverified]

## Checks

Standalone native attempt3 passed5 tests,0 failures: full-width scalar recovery r-2, repeated signed summands, deterministic bounded exhaustion, invalid input/cap rejection, bounded reproducible subgroup factor-base construction, and stage-observation equivalence. The successful recovery fixture intentionally uses Q=±2G and base={G}; it is a deterministic correctness control, not a distributional experiment. Attempt1's three-test source and evidence remain under attempt1/; attempt2's four-test source and evidence remain under attempt2/. [pcc:unverified]

The standalone harness compiles the repository's actual curve, Fp256, exact sampler and matrix source. It uses an extracted GroupOps/CountedGroup declaration and an unused Fp64 primality shim. The candidate is not yet registered in ecbench; these checks do not exercise runner dispatch. [pcc:unverified]

## Accounting

Group ledgers retain base construction, setup validation, search, relation admission and recovery separately. Matrix scalar-field API counts, SHA256 draw counts, full-point lookups and square-root call counts are retained separately. These are component counters; they do not price allocations, hash-map internals, coordinate conversions, canonical-group construction or all validation work. In particular a square-root call is not a count of its internal arithmetic. Integration must retain those unpriced costs and the complete elapsed interval. [pcc:unverified]

## Replay

Run from the repository root with an existing locked Cargo target directory containing the dependency rlibs:

```sh
python3 research/curve25519_autolab_parity_20261105/next-integration/pair-collector/replay.py --target /tmp/curve25519-autolab-target --out /tmp/curve25519-pair-replay-fresh
```

The output directory must be new. The script checks pinned source hashes before compiling and records dependency hashes, compiler identity, commands, build and test exit codes. It implements no research algorithm in Python. [pcc:unverified]

The collector exposes Setup, Query, Decomposition, AdmitAndEliminate, Recovery and Finished stage transitions. Admission combines relation verification and matrix insertion; callers must preserve that combined timing scope rather than invent separate timings. The no-op observer and observed path return the same recovery and counters. [pcc:unverified]
