# Frozen protocol: PRIME31 RR9 B8 signed-support algebra versus generic 4+4 MITM

**Registered:** 2026-11-17, before code or measurements. **Status:** proposed, [pcc:unverified].
**Repository task:** PR #4575. Scope is a *falsification/control study*, not complete index calculus.
No paid compute; use existing native Rust PRIME31 arithmetic and free GitHub Actions CI.

## Instance
Actual ordinary cofactor-one curve from `prime31_algebraic_ic::PRIME31`: p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251).
B8: first eight rational abscissas [0,1,2,3,5,6,8,9], chosen via existing square-root routine. Each leaf is ±P_i. The signed eight-leaf sum set contains at most 256 points.

## Distinct complete predicates
1. **Generic point-law control:** split the eight coordinates into ordered first and last groups of four. Precompute 16 right signed half sums into `HashMap<Point,mask>`, 16 left sums; for a public `R`, query `R - left` for each left key. Return one replayable eight-leaf witness iff any match. Group arithmetic is reused from `native/lib.rs`. No private scalar or algebraic chart coefficients may enter candidate queries.
2. **Existing B8 algebraic chart control:** call existing merged public RR9 barycentric extractor from #4556 for its exact preregistered three fixtures only, and compare **predicate truth** for those fixtures. This oracle enumerates 256 masks and excludes exceptional `x(R)=x_i` and non-monic chart strata. Do not conflate all signed-sum membership with monic RR9 chart existence on arbitrary planted supports.
3. **Batch:** deterministically enumerate all 256 signed masks, replay each sum using original curve arithmetic and verify a generic witness. Also construct 1,024 distinct nonzero random scalar multiples `kG` with SplitMix64 seed `0x4238_4d49_544d_3236`. Scalar generation is test-only; query gets only Q. Verify each generic positive by independent original-curve group sum. Track hit counts and misses, not complete ECDLP logs.

## Timing and accounting
- Pin Rust 1.100.0 and original native arithmetic to immutable commit `1df714645aa683cf975bc76fcacb9842cb4780fd`; overlay only proposed example source, retain a runnable JSON artifact.
- Record precomputation setup wall time and native group-addition counters; online wall for the fixed three residuals and separate 1,024-Q batch; report 16 right entries/16 left entries and per-query max 16 complement probes, 256 full mask controls, and group-operation counters.
- Avoid meaningful speed-ratio claims across different-host runs; same-run B8 compare has only single calls and cannot estimate B31 complete-solver performance.
- Fail CI on arithmetic mismatch, a missed planted witness, a false positive on the two known off-support negatives, fixture chart mismatch, duplicated scalars, or JSON schema invalidity. Preserve failures and source-specific CI artifact. No extra parameter tuning after results.

## Prior work and novelty guard
Internal: #4556/#4558 public B8 sign-enumerative chart, #4545 signed fibres, #4547 seven-anchor generic equivalence, #4548/#4549 Macaulay D5/D6 nulls, #4550 D9 theorem, #4513/#4532 fully charged 1024-Q frontier, existing `prime31_s9_b31_half_sum_oracle.rs` (B31 4+4 generic oracle). This B8 experiment is a **matched comparator for #4556**, not a new algorithm or literature priority.
External: Petit–Kosters–Messeng PKC 2016 DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala FFA 2018 DOI 10.1016/j.ffa.2018.01.009; Shoup EUROCRYPT 1997 DOI 10.1007/3-540-69053-0_18. Generic signed MITM is longstanding prior art. Research-KB MCP absent in current runtime and wrapper unavailable (no checked-out repo); GitHub/primary-publication audit is bounded, not exhaustive.

## Success and decision
Result succeeds when fixed B8 memberships match known charts, every all-mask planted support is covered, 1024 blind Q are checked with correct witness replays, setup/online operation budgets are available, and exact-source CI passes. An outcome that the generic comparator is faster should retire **B8 sign enumeration as a candidate speedup**. Next non-generic gate: public-R seven-equation sparse quotient or a new B31 factor-base-conditioned extractor with no signed-support enumeration, then end-to-end unknown-log B31 cost including all failures/precompute/memory against cached BSGS and strong IC baselines.