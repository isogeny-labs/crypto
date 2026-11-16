# Prime31: compare against the index-calculus frontier, not BSGS (2026-11-15)

**Scope:** Rust-only source-bound benchmark admission and next scientific experiment; **[pcc:unverified]**. Existing prime31 curve: p=2147483647, a=847514846, b=2087444724, prime order n=2147446831, cofactor 1. This is not a claim of a cheaper complete non-generic solver.

## Correction to benchmark methodology

BSGS and rho are necessary generic security baselines for **whole cold ECDLP** results, but they are **not** the proper denominator for an *algebraic index-calculus oracle* improvement. The first head-to-head must compare comparable algebraic stages on exactly the same polynomial ideal, naturally sampled public targets, same selected factor base, and same hardware.

Frontier arms:

1. Petit–Kosters–Messeng 2016: compositional low-degree rational-map factor base / prime-field algebraic index calculus (PKC 2016, https://doi.org/10.1007/978-3-662-49387-8_1).
2. Amadori–Pintore–Sala 2018: one-factor-base-relation prime-field index calculus, which avoids the conventional large linear-algebra phase (FFA 51, https://doi.org/10.1016/j.ffa.2018.01.009). The single-relation algorithm has a *different full solver cost model*, so compare complete cold solves as well as algebraic query work.
3. Existing Rust dense F4 and tower F4, #4275; compare with external msolve/Singular on byte-identical input before claiming fastest Gröbner solving.
4. Certified quotient/subresultant original-ideal and root-worker implementations, #4228/#4349. **Limitation:** these development reports have no completed wide-prime31 target-dependent m8 benchmark; do not transfer small-prime numbers.
5. Existing prime31 factorized S4 (#4188), as a same-input lower-arity engineering reference, not an m8 baseline for absolute speed claims.

The 46.2-second S3 root-index full scalar solver is **generic pair-MITM equivalent** and a correctness/negative control, not a novel IC frontier candidate. Its 23.6 ms BSGS comparator remains archived, but should not be used as the *primary progress metric* while optimizing polynomial relation finding.

## Actual evidence and blockers at the inherited head

- The Rust bounded 16-natural-public-target m8 F4 panel was **16/16 undetermined**, all oversize at degree 6 under a 15,000 reduced-block cap; zero completed root extractions. The reported 1.52–2.14 ms internal F4 calls are not full query costs or valid timing wins against an algebraic oracle that actually finishes.
- Rust m4 factorized resultant on B4/B8/B16 produced one constructed-positive relation and zero for a fixed public control, but no measured natural-positive rank yield or complete unknown scalar recovery. B31 four-summand natural-target support is too sparse for the target goal.
- The candidate Rust certified quotient / actual-root worker in #4228/#4349 is still limited by its current wide-field contract; the stacked branch also has unresolved PCC merge-history dependencies. Do not copy source to bypass the gate.
- Existing whole-call gamma/delta feasibility gate from #4077 remains an optimistic **necessary** condition, not a result or empirically fitted exponent.

## Executable audit

`native/examples/prime31_ic_frontier_audit.rs` is a **Rust-only** parser for the checked-in frozen JSON evidence. It checks the prime31 fixture, all 16 actual m8 F4 verdicts, censorship classification, and paired B4/B8/B16 m4 controls, then emits a machine-readable frontier priority and admission protocol. It rejects false conclusions that a censored target was proved relation-negative. It is **not** an F4 invocation, new solver, new measured speed, or PCC-signed independent replay.

Reproduce in a complete checkout:

    cargo test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_ic_frontier_audit
    cargo run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_ic_frontier_audit

No local Rust compiler/full checkout was available in the connector session; checked-in tool is source-only until CI independently executes it.

## Next genuine non-generic experiment — do not substitute a generic sum table

**N1 fixed-ideal equivalence:** on the same frozen B31 Dickson factor base, construct the exact 2-leaf S3 and 3-leaf S4 balanced m8 polynomial systems using the existing Rust F4/tower code. Try certified fixed-pair quotient reduction in the #4228/4349 actual root path. Require verified original-ideal dependence certificates and equal complete rational solution sets on feasible B4/B8 controls, including identity, repeated x, vertical and multiplicity cases. Do not use prior generic signed-pair-sum tables to propose roots.

**N2 first actual natural roots:** run all 16 frozen natural targets, plus a separately frozen holdout, through the old and proposed solver with equal caps. Report each target's completed/undetermined/positive status, complete original-ideal field roots, signed y-orientation full-target replay, projective dedup and incremental independent rank. The query field operations, wall/CPU time, peak RSS, failure/censor and preparation must all be retained. Never claim an unfinished oracle slower/faster by comparing only internal F4 steps.

**N3 proper IC-frontier cost comparison:** compare complete-output cost-per-usable-relation and incremental-rank rates against PKM-style and APS-style Rust implementations where available, and same-ideal dense F4/tower/certified quotient with matched external CAS. A literature-only APS arm is marked **unimplemented**, not assigned invented timings.

**N4 complete scalar solve:** only after algebraic m8 naturally yields enough independent rows, integrate with existing Rust modular LA for B+1 unknowns and verify 12 held-out unknown logs. Charge setup and every failed call. Then compare full cold costs with optimized Rust rho **separately**; run on additional ordinary prime-order curves before asserting a structural advantage.

## Lead decisions

**Advance experimentally:** fixed S3 certified quotient plus target-dependent S4 elimination with actual roots, then source-equivalent dense/tower F4. **Keep conditional:** y-oriented slope / injective-y basis if it reduces complete algebraic query cost, not only sign lifting. **Deprioritize:** generic S3 pair root indexing (classified generic-equivalent), m4-only B31 yield ceiling, mere permutation orbit counting without a quotient solver, chained transfer without scalar-blind complete map. The outstanding scientific gate remains both algebraic relation yield and independent row rank, not raw candidate counts.

Research-KB, Conductor and authorized EKS environment were not available in this connector-only session; GitHub #4188/#4077/#4228/#4349/#4275 primary source and the two published articles served as limited prior-work review. No exhaustive priority/novelty claim.
