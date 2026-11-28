# Prime31 run63 — charged affine-coordinate materialization after public shifts

**Date:** 2026-11-22. **Status:** code and Actions workflow committed before measurement, but this protocol was written immediately afterward; **not retrospectively called preregistered**. The objective is a feasibility discriminator, not a complete ECDLP claim.

## Target and lineage

Keep exactly the original cofactor-one curve over F_(2^31-1): `y²=x³+847514846x+2087444724`, `n=2147446831`, `G=(625037715,2100627251)`. Use original Rust `prime31_algebraic_ic::{Arithmetic,PRIME31}` for every correctness reference, no new solver or rootfinding engine.

- #4901 runs55–60: frozen 677-point hybrid two-summand support 916469; 88 free relation rows; conditional 1024-target rank model predicts 4699949 uniform shifts. Model is not a real extraction algorithm.
- #4901 run61–62: original Rust complete +G mixed Jacobian **21.56–21.87 ns/shift** at two Actions runners; benchmark did **not** produce the affine coordinates that an affine x/S3 oracle needs.
- #4391: the original APS Rust solver **already** has Mersenne field reduction, complete mixed Jacobian and Montgomery batch normalization; none of those primitives is new. The present experiment measures the materialization toll specifically against the run59 hybrid-query budget.
- #4692/#4812/#4822 and #4809/#4846 own the public-only non-generic RR9/F3/S3 extraction tests; do not duplicate or silently replace them with pair hash tables.
- Prior art: Amadori–Pintore–Sala (2018), DOI:10.1016/j.ffa.2018.01.009, already develops a one-relation prime-field method. No claimed novel ECDLP method here. Shared research KB unavailable in this tool runtime; repo prior runs, primary published paper metadata, and PR sources audited.

## Falsifiable execution

Existing Rust example `research/prime31_ic_20261115/native/examples/prime31_run63_normalized_probe.rs`. Original 1.100.0 Rust workflow `prime31-run63-normalized-probe.yml`.

On identical public sequences of 250000 complete fixed-G steps, 5 repetitions with rotated arm order, measure **within one run**:
1. run62-style exception-safe projective shift-only control, no per-step affine output;
2. the same shifts with **all x coordinates produced**, Montgomery prefix-product inversion in chunks of 256 (one Fp inversion per chunk);
3. the same shifts with **all (x,y) coordinates produced**, same normalization blocks.

For each arm, verify whole-chain endpoint against original `Arithmetic::scalar`. In untimed validation compare both coordinate-normalization arms with original independent affine `Arithmetic::add` for 40 starts x 192 consecutive steps, including O, ±G, n-wrap; no skipped identity event should be reported as a finite point. Emit all 15 raw times, medians and a deterministic digest. The benchmark does **not** perform hashing, Semaev filtering, polynomial arithmetic, relation rank or scalar recovery.

## Decision and caveats

Run59 model + historical BSGS 170.113ms (different runner) gives a cross-run sensitivity of ~36.195ns per uniform query for 1024 logs, **before** any other work. If normalized-x alone costs more on these Actions hosts, a pipeline that performs one such affine-x materialization for each modeled query cannot meet that historical sensitivity even with free rootfinding, but this **does not rule out** projective/homogeneous filtering, batched multi-query algorithms, altered query distributions, nonuniform useful rank, or any other IC mechanism. If it costs less, that is only a necessary gate. Compare complete costs only on matched host and public targets against optimized BSGS, rho and other Rust index-calculus.

**Next high-value experiment**: actual public-only blinded root extraction with reported positives *and negatives* and independent [k]G=Q checks, tested on 1,32,128,1024 natural Q. Do not count synthetic known-scalar planted witnesses as extraction. Check wall time/RSS/failed probes and all factor-base and linear-algebra setup.
