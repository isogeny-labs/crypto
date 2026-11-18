# Prime31 RR9 public-residual-only B8 Macaulay experiment — source-specific result

**Date:** 2026-11-17 UTC. **Producer status:** [pcc:unverified]. **GitHub PR:** #4548 stacked on #4547. **Conclusion:** degree-5 Macaulay elimination is **inconclusive** on the three preregistered public residuals, not a relation extractor, not a complete ECDLP solver and not a measured advantage.

## Original instance and reproduction
- Original prime-order, cofactor-one curve: p=2147483647, a=847514846, b=2087444724, n=2147446831, generator (625037715,2100627251).
- Reused original Rust library `research/prime31_ic_20261115/native/lib.rs` from the immutable merged base `1df714645aa683cf975bc76fcacb9842cb4780fd`. The new example is `research/prime31_ic_20261115/native/examples/prime31_rr9_public_macaulay.rs`.
- Register-before-code: `RR9_PUBLIC_MACAULAY_PROTOCOL_20261117.md` at `6ca7ab2d`; positive normal-form amendment at `6bcde39` before the initial CI produced any numeric result. Final corrected example source commit `493d5ea7a97e1e1e1182b446c8a6c3a61798efd9`.
- [Exact-source pinned Rust 1.100.0 Actions #44084070161](https://github.com/isogeny-labs/crypto/actions/runs/44084070161): **SUCCESS**, unchanged original native Rust library **40/40** and new public Macaulay example **1/1**; raw producer artifact [#14328996551](https://github.com/isogeny-labs/crypto/actions/runs/44084070161/artifacts/14328996551) (GitHub ZIP digest SHA-256 `36d587f81ea7a91b6f28d4620bd19918857c07ca807f0a84fbca442d829088e9`). JSON `prime31-rr9-public-macaulay.json`, 2862 uncompressed bytes.
- Preserve failed predecessor CI [#44083153755](https://github.com/isogeny-labs/crypto/actions/runs/44083153755) and later prior-source failures: all 40 library tests passed, but Rust E0308 because fixture `Point = Option<(u64,u64)>` had been unwrapped to a tuple on return. Fixed at `493d5ea7` by rewrapping `Some(...)`, without changing degree, algebraic equations, fixed public tests or selection gates.
- No paid compute, no source-local cargo toolchain available in the connector runtime, no signed independent PCC verification. The CI and raw JSON are **producer evidence** only.

## Exact tested mathematical contract
Let `R=(r,s)` be a finite public point, `H_B(X)=prod_{x in B8}(X-x)` with the first eight rational x coordinates [0,1,2,3,5,6,8,9], and unknown `A=sum_{i=0}^3 a_i X^i`, `V=X^3+sum_{i=0}^2 v_i X^i`. In the signed RR9 chart `F=(X-r)A+(Y+s)V`, compute
```
M(X) = (X^2+rX+r^2+a) V(X)^2 - (X-r) A(X)^2 - 2s A(X)V(X).
```
For the **squarefree full B8** stratum `M=H_B` exactly, yielding eight degree-2 equations in seven unknowns from the coefficients 0..7. Exceptional `x(R)=x_i` and infinity are explicitly refused. This is *not* the complete B31/all-multiplicity relation ideal.

The native candidate uses **only public R, x-coordinate factor-base H_B and original curve metadata**. It creates a truncated Macaulay matrix with all monomials of degree <=5 (792 columns) and all eight quadratic equations multiplied by monomials of degree <=3 (960 input rows). Exact modular elimination can (1) construct and independently replay a `1=sum c_{ij}m_j q_i` unit certificate, establishing the candidate ideal has no solution, or (2) reduce the 1 and seven coordinate normal forms, returning coefficients only if all reduce to constants and the eight original equations hold. If neither, the result is **inconclusive**.

The independent *test-only* oracle enumerates 256 possible signs for the fixed eight genuine factor-base points, builds a planted positive and two negatives outside the full signed-sum set using existing original-curve Rust arithmetic; it never supplies any signed mask, scalar or support to the algebraic candidate. Fixed fixtures and exact matrix cap were frozen before numeric data.

## Recorded results (same producer CI run)

| Public residual R | Classification by independent original curve test oracle | Columns | Rank | Quotient degrees of freedom in truncated span | Fp multiplications (elimination) | Elimination wall | Unit certificate? | Unique coefficient extraction? |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- |
| (1911579686, 548820096) | Planted positive | 792 | 736 | 56 | 22,557,136 | 37.414 ms | No | No |
| (625037715, 2100627251) | Outside all 256 B8 sign supports | 792 | 736 | 56 | 22,557,136 | 35.174 ms | No | No |
| (1923055775, 307731458) | Outside all 256 B8 sign supports | 792 | 736 | 56 | 22,557,136 | 35.081 ms | No | No |

Each run recorded 960 generated candidate multiples, 64,714 elimination reductions, 736 field inversions, and a dense-pivot storage upper metric of 4,663,296 bytes. These are **actual candidate phase** counters and elapsed times, *not* full relation-finding cost. The source independently checks eight symbolic coefficient quadrics against the direct factor polynomial at eight deterministic seven-coefficient vectors, and rejects nonfinite/exceptional residuals. The fact all three residuals have the same degree-5 rank does **not** show their ideals are equal or imply anything at a higher degree. The 56-dimensional uneliminated span is an observation at the fixed truncation only.

**Falsifier outcome:** No false positive unit certificate and no fabricated positive coefficients. However, **both true negatives are also not refuted**, and the planted positive is not recovered: degree 5 provides no useful decision or extraction on this panel. There is no unknown-target DLP recovered and no non-generic computational advantage.

## Cost frontier and why the negative matters
Historical, **not remeasured** single-run #4532 on the same 1024 target cohort: shared-precomputed generic BSGS took 170.113ms including setup; mixed-base IC took 2221.474ms [pcc:unverified]. If 61 basis relations plus 1024 target relations must fit in the 170.113ms budget and **every other cost is free and every attempt succeeds**, the necessary ceiling is only `170.113 / (61+1024) = 0.1568ms` per success. A **single** degree-5 Macaulay elimination here takes 35–37ms, over 220× that optimistic ceiling. This is only a rough **different-run necessary-cost comparison**, not a same-host full-ECDLP speed ratio; do not infer a solver performance comparison. Neither completeness nor relation yield is demonstrated.

## Prior-search audit and constrained next experiments
Read #4466 Hermite norm; #4539 public seven-coefficient chart; #4541 radical; #4545 signed B8 fibres; #4546 exceptional residual; #4547 seven-known-anchor generic-equivalence/interpolation obstruction; #4373 capped F4, #4228/#4349 quotient/rootfinding; #4513 mixed-base complete 1024Q IC, #4532 cached BSGS. Autoresearcher #3301 (41-lead heatmap) and #3305 (28 Semaev-alternative directions) constrain new novelty claims. Published comparator: Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51 (2018), 168–182, DOI 10.1016/j.ffa.2018.01.009; Kudo et al., *Acceleration of index calculus for solving ECDLP over prime fields and its limitation* (hybrid Gröbner/symmetry prior art). The research-KB MCP and CLI were unavailable in this connector runtime; this is a bounded fallback literature and repository audit, not an exhaustive novelty search.

**Priority queue with new, separately frozen protocols:**
1. Explain the **degree-5 rank 736 and 56-dimensional residual space** by monomial syzygies, compare with an independent exact row rank and with generic eight quadrics. If a common target-independent nullspace can be quotient-reduced, demonstrate algebraic correctness before any performance claim.
2. Prospectively test **degree 6, then 7 only if separately authorized/preregistered**, using the actual repo's Rust F4/sparse kernel, not dense Gaussian by default: at degree 6 there are `C(13,6)=1716` columns and `8*C(11,4)=2640` rows; at degree 7, 3432 columns and 6336 rows. Freeze maximum RAM/CPU/multiplications, exact positive and negative outputs, and preserve failed or inconclusive strata. No post-data degree escalation in this study.
3. For a positive fibre, recover its chart coefficients from public R and H_B without signs, independently orient every leaf and verify the original curve sum. On B31 require *unknown target* log recovery, repeated-root handling, rank-complete factor-base setup and individually charged failures. A truncated unit test alone is not an extraction mechanism.
4. Compare the **complete** Rust method against the same-cohort cached BSGS, matched optimized negation-map rho, and strongest published/repo IC on one pinned runner, with memory, setup and unsuccessful trials charged. Claim performance only with independently replayed evidence and signed PCC standing.

**Disposition:** B8 degree-5 Macaulay coefficient extraction is **negative/inconclusive**, preserved to avoid repeating unchanged underpowered experiments. The public-only predicate construction and 40+1 Rust CI tests are complete and committed; non-generic complete prime-field ECDLP solver cost advantage remains an open goal.
