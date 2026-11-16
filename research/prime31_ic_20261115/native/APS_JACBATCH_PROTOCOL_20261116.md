# EXP-R31-APS-JACBATCH — preregistration, 2026-11-16

## Research question (before source changes / execution)

Does a strictly source-equivalent **Jacobian mixed-add walk with batched affine normalization** reduce the *fully cold*, single-target, published APS-style prime31 index-calculus solver cost, compared with the existing affine point-by-point additive walk with the same `[384,384,16384]` unequal partition and the same x-fibre S3 discriminant precompute?

This is a **group arithmetic optimization**, not a new non-generic index-calculus theorem. A fast internal implementation is not automatically faster than external APS/PKM/CAS or optimized rho. We must retain all negative results; report [pcc:unverified] until signed verification.

## Mathematical and implementation controls

- Same ordinary cofactor-one E/Fp: p=2147483647, y²=x³+847514846*x+2087444724, n=2147446831 prime. Reuse native Rust `prime31_algebraic_ic::Arithmetic`, the existing APS `Labelled`, S3 x-fibre algebraic root search, signed candidate checking and scalar replay.
- Use *identical* public 64-step table, source PRNG, partition sizes, labels a,b, source points and source order in both methods. No Q secret enters either method; the known scalar is caller-verifier-only. A batched coordinate normalizer may change timing but **must not change the lists** even after global-x collisions, identity/reseed, or cross-block transitions.
- Represent walk point as `(X:Y:Z)` with x=X/Z², y=Y/Z³. For each mixed addition with public affine table step, use usual complete case split (distinct, doubling and cancellation) and verify with the existing affine Arithmetic in unit tests. Normalize by Montgomery batch inversion of nonzero projective Z in bounded batches of at most 256 entries; group arithmetic in the independent checker never feeds the algebraic candidate search.
- This preserves the *same* S3 pair evaluations, roots, match indices, candidate signs and recovered k in every matched run. Repeated x, identity and failed bounded rounds are retained/charged. Charge 64-step setup, scalar-generated partition seeds, all mixed multiplications, batch normalization, failed rounds, root finding and complete replay to the wall.
- Bench precommitted 24 *fresh, distinct* public Q from seed `0x4a4143425f323431` using the same deterministic target panel for all arms; never reuse target known answer as a solver argument. Alternate execution order per Q: affine/batch/batch versus batch/batch/affine. Six full rounds maximum per target. Report all 72 complete arm outcomes and every failed attempt, even if underperforming. Same-source equivalence tests: [32,48,64] and [384,384,2048] with exact point/label comparisons, plus group equivalence on 100+ deterministic mixed-add and doubling/cancellation controls.
- **Provisional development gate** (not a research conclusion): 24/24 independently correct full logarithms in *all* three arms, exact same scalar, per-round relation-trial counts and found status, batch total cold cost <= 90% of matched affine cost, A/A aggregate drift <= 5%, faster on >=18/24 Q. A compiler/test failure counts as failure, not a zero-time speed gain.
- Frozen equal-source matched design means if the candidate wins, it is arithmetic *engineering* acceleration of APS-style index calculus, still mathematically equivalent to generic pair MITM. Separately compare optimized rho, published PKM/APS algebraic oracle and mature external CAS on identical inputs before any external frontier claim; independent PCC remains required.

## Previous leads and citations

- Existing complete Rust APS control & x-fibre precompute: #4391 and `S3_XFIBRE_MATCHED_REPORT_20261116.md`; previous asymmetric `[384,384,16384]` 24-Q matched confirmation; direct list S3 search is generic MITM-equivalent.
- Exhaustive B8 symmetric f/S9 **given-f** validator: `M8_SYMMETRIC_ROOT_RESULT_20261116.md`; target-dependent root selection remains open.
- Failed/censored algebraic approaches: #4373 m8 chained-S3 F4 memory caps; #4391 anchored m6 narrow support/negative F4 gate; #4383 unsuccessful T=1024 fresh confirmation.
- Existing Rust selected-root/certified quotient work: merged #4228 / open #4349; does not yet solve target-conditioned B31 S9. `src/cryptanalysis/symmetrized_semaev.rs` is prior implementation.
- Published primary sources: Amadori–Pintore–Sala, *Finite Fields Appl.* 51 (2018), 168–182, DOI 10.1016/j.ffa.2018.01.009; Petit–Kosters–Messeng, PKC 2016 DOI 10.1007/978-3-662-49387-8_1. Batched projective-to-affine conversion is classical Montgomery batch inversion, not our claimed invention.
- KB mandatory query unavailable: KB MCP tools and local repo wrapper are absent from this connector/container runtime. Bounded fallback: GitHub source, prior reports and publisher record. Conductor/EKS unavailable; no cloud expense.

## Success and failure recording

Commit the Rust code, tests, CI command, raw developer JSON and exact executed source SHA. On pass, report actual wall totals, per-target paired wins, A/A drift and counted operations; on fail, preserve the failure and stop claiming a speed ratio. Never promote a leaderboard/PCC row from this unsupervised producer run. Keep a separate non-generic research queue for genuinely target-conditioned S9 original-ideal root selection.
