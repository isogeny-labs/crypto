# Preregistered 64-bit packed open-addressed prime31 APS x-index matched gate

2026-11-16. Frozen **before implementation**, standing [pcc:unverified].

## Changed mechanism and historical check

The matched 24-Q Rust compact `HashMap<u64,usize>` factorized-APS index experiment (protocol, raw receipts and report in this same directory) passed its development gate and reduced complete cold cost by 15.316% relative to the old per-key-Vec map; matched cold BSGS was still 1.835% less costly. The label/source generator already guarantees globally distinct rational x coordinates. A **64-bit packed linear-probing open-addressed table** may remove per-probe SipHash overhead and heap-heavy std HashMap structure while preserving **exact** public input/output and the original signed relation-first-hit search. This is only a static array indexing optimization of the **same published APS 2018 finite S3 pair search**; it is generic-MITM-equivalent and **not** new non-generic index calculus.

Historical audit: repository `research/pdp_preregistrations_20261007/PROTOCOL-E1-packed-root-index.md` already proposed packed/sorted indexes for **binary-field Koblitz** tables; `benchmarks/20261029-ic-packed-filter/VALIDATION.json` has separate prime-field packed-filter evidence and negation rho references, on different workloads. Neither is a new prime31 S3 x→single-index algorithm. Reuse the approach, do not assert novelty. Also reviewed prior #4391 source and session logs, rejected APS partition retunes (#4383 and post-Jac), censored m8 F4 (#4373), certified quotient #4228/#4349, crypto-autoresearcher SA-02/SA-14b/SA-15b, APS2018 DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009), PKM2016 DOI [10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1). KB/Conductor unavailable through connector-only runtime.

## Exact source design

- `FlatXIndex` is a power-of-two `Vec<u64>` with at least **two slots per third-partition source point** (max 40,000 list points), empty sentinel 0, encoding `((x+1)<<16)|(k+1)` where canonical x is below p=2^31−1 and k below 65535. Any duplicate key returns error.
- Use a fixed public multiplicative mix of x to choose initial bucket, then linear probing with wraparound. `get(x)` either returns unique exact index or none, and must cap probe count at table length. This must not use caller-known k, y sign, polynomial root hints or group sums; group additions are post-root verification only. Check packed representation constraints and **exhaustively round-trip all third-partition keys** plus random misses on a natural source before benchmarking.
- Preserve the existing `HashMap<u64,usize>` compact candidate as the frozen control, not the old Vec map. Put packed mode behind an optional flag and retain exact signed-root candidate enumeration and Rust replay; every old/new/repeat attempt, selected signed relation coefficients and algebraic pair query must match exactly.

## Prospective benchmark

- Same ordinary prime-order curve over p=2147483647, E:y²=x³+847514846x+2087444724, n=2147446831, cofactor 1, G=(625037715,2100627251), APS partitions [384,384,16384], at most six attempts.
- **24 new public Q** from caller-only SplitMix64 seed `0x4658_5441_424c_4531`, never previously used in these runs; nonzero, distinct k, k used only for final caller verification. Four independent paid cold arms per Q, rotated order: (0) current compact `HashMap<u64,usize>` APS, (1) packed open-addressed APS, (2) packed A/A repeat, (3) current batched-Jacobian BSGS with new ~46k baby table every Q.
- Each arm builds all public walks/index/table from zero and independently recovers full unknown k and verifies [k]G=Q. Record exact per-Q/raw JSON, misses/rounds, source ns, query ns, root count, table size/collisions/probes where possible, memory per entry and full cold duration; no dropped failures.
- **Engineering gate:** all **96/96** complete answers verified, identical old/new signed relations and round/s3 counters, packed total ≤0.90× old HashMap compact total, ≥18/24 per-Q packed wins against old, packed A/A repeat drift ≤5%. A BSGS matched complete win is a separate *reported outcome* not presumed; beat ratio 1 or loss needs exact statement, not a selected per-case claim. No PCC or broad external advantage without independent validation.
- Source-specific locked Rust 1.100 CI via existing `.github/workflows/prime31-s4-split-20261116.yml`, no paid EKS. Fail/error/censored cases remain in evidence; repair and rerun rather than inventing results.

## Independent scientific work not substituted by this test

Target-conditioned original-ideal-certified 3+3+2 symmetric eight-leaf S9 rational-root extraction using #4228/#4349 and B4/B8 given-f validator, then B31 natural relation rank and full scalar recovery, not enumerating `C(38,8)=48,903,492` f candidates. This packed table is **not** that solver.
