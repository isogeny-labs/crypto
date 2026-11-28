# EXP-PRIME31-WRR9-20261118 — weighted-slot RR9 public resultant gate (frozen before source)

Date: 2026-11-18 UTC. Tracking PR #4728. **Mathematical and source-bound component gate only.** This is not a claimed recovered unknown logarithm, not a complete index-calculus solver, and not a measured end-to-end advantage. Existing pinned Rust crate: `research/prime31_ic_20261115/native`, immutable source revision `1df714645aa683cf975bc76fcacb9842cb4780fd`, Rust 1.100.0.

## Curve and deterministic fixture panel

The original ordinary prime-order cofactor-one curve is unchanged: `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`, `G=(625037715,2100627251)`. The original first 31 liftable rational x values are used, with canonical smaller nonzero y. Eight public weights are `w=(1,3,5,7,11,13,17,19)`; slot `j` consists of `[w_j]P_i` and negatives, `0<=i<31`. Test that all **248 x coordinates are pairwise different** before asserting the algebraic converse. If there is any overlap, do not invoke the converse; report the overlap and re-register a separate branch/solution.

Sixteen planted signed fixtures use `i_j=(3t+7j+j*j)%31`, `epsilon_j = +1 if ((t+3j)%4)<2 else -1`, for `t=0..15`, `j=0..7`. The independent test-only producer builds `R=sum epsilon_j[w_j]P_{i_j}`, rejects identity, and solves the 8-by-7 signed RR9 linear interpolation equations for the normalized chart `(A0..A3,V0..V2)` with `V3=1`. The candidate receives only public `R`, slot x lists, and chart coefficients; it must recover one unlabelled root per slot, the signed points, and replay the group relation without seeing fixture indices or signs.

## Exact algebraic statement and prospective controls

For `R=(r,s)!=O`, set `D_R=X²+rX+(r²+a)`, `A(X)=sum_0^3 a_k X^k`, `V(X)=X³+sum_0^2 v_k X^k`, and the monic degree-8 norm factor
`M=D_R V²-(X-r)A²-2s A V`. The signed RR9 function is `F=(X-r)A+(Y+s)V`, so `F(-R)=0` identically.

For slot polynomials `H_j(X)=prod_{i=0}^{30}(X-x([w_j]P_i))`, define the eight *factored resultant constraints*
`g_j=Res_X(H_j,M)=prod_{i=0}^{30}M(x([w_j]P_i))=0`.
Each `g_j` is a polynomial in seven coefficients of total degree at most 62, but the implementation evaluates it as a product of 31 quadratic polynomial evaluations, never expands the symbolic polynomial, and does not construct generic k-sum tables. **If the eight root sets are pairwise disjoint**, the eight zeros force eight distinct roots of the degree-8 `M`, one root from each slot: a necessary-and-sufficient *support criterion for a supplied RR9 chart*. The signed RR9 divisor certificate then gives a point from each signed slot whose sum equals R. This says nothing about efficiently *finding the chart*.

Required tests: 16/16 planted samples accepted and recovered; 16/16 `a0 -> a0+1` target-preserving chart mutations rejected (their zero-count vector may be inspected); additional changed-target negative if deterministic; original group replay on every extracted signed point; all 248 x values distinct; explicit overlap/Hall counterexample with slot 7 duplicated from slot 0, one root in shared slot 0, two roots from slot 1 and one from slots 2–6, for which all eight resultant constraints vanish but no distinct-root perfect matching exists. Assert the counterexample rather than silently extending the theorem.

## Costs, independence and hard scientific gate

Record setup scalar multiplications, per-fixture 248 degree-8 Horner evaluations, field multiplication counts, negative checks and elapsed times as **component diagnostics**. Reuse original native Rust `PRIME31` and `Arithmetic`. An independent Python BigInt implementation recalculates curve, weighted slots, R, chart, norm factor, zero roots, mutations, the Hall counterexample, and checks the JSON record. GitHub Actions compiles exact pinned native source and new example on Rust 1.100.0; no paid backend.

The next **uncompleted** gate is a solver for the **seven-unknown, eight factored resultant equations** on *unplanted public residuals*, with exact no-witness/timeout handling and without generic sums. Compare factored 7-variable F4 versus a slot-root auxiliary-variable system at B4/B8, then B31, connect to native rank31 and 1,024 blinded unknown-Q recovery. Price setup, 31-log training, failed queries, F4, linear algebra, extraction, recovery, memory and verification against the same-host mixed 4+4 index calculus, shared BSGS and optimized rho. Existing best recorded BSGS 1,024-Q run is 144.049 ms, producer-side and not a new measurement. If no chart extractor or fully charged advantage exists, do not claim solver success.

## Sources checked (KB unavailable)

Primary repository sources: #4539 (signed 7-coordinate RR9 norm chart), #4717 (shared-base low-power multiplicity strata), #4692 (unfinished RR9/F4 coefficient extractor), #4720 (signed pair projector), #4714 and #4722 (weighted M6 complete control and support ceiling); search of `RR9 weighted slots`, `slot-resolved`, `resultant weighted factor base` across the default crypto branch showed no exact prior slot-specific eight-resultant formulation (absence from index is not a novelty proof). External prior art: Semaev summation polynomials; Amadori–Pintore–Sala (FFA 51, 2018); Bochum et al., *Subexponential ECDLP* (2026). The norm/resultant algebra itself is classical. Research-KB MCP/wrapper not available in this runtime; signed independent PCC remains open.

**Registration rule:** This protocol is committed before the Rust, Python and CI implementations. Results outside these fixture classes are exploratory and must be labelled as such.
