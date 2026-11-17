# EXP-R31-ANCHOR2: exact two-anchor m6 prime31 F4 admission (2026-11-16)

**Status:** preregistered experimental plan, no new execution or IC speed claim. [pcc:unverified].

## Reviewed evidence and literature

- Existing Rust [#4391](https://github.com/isogeny-labs/crypto/pull/4391) APS one-relation solver achieved a complete, checked 24-target improvement using additive walks and p=2^31-1 field multiplication; explicit S3 pair scanning is generic meet-in-the-middle equivalent, not new non-generic index calculus.
- Existing [#4373](https://github.com/isogeny-labs/crypto/pull/4373), #4188 and `native/M8_NATURAL_F4_CI_OUTCOME_20261115.md`: full m8 tower F4 and S3 chains censored at 15k reducer cap, including natural masks. The bounded F4 result is **undetermined**, not no-relation.
- `native/ORIENTED_PAIR_S3_F4_20261115.md` and `native/S3_DISCRIMINANT_CLASSIFICATION_GATE_20261115.md` already examine orientation and the separable S3 discriminant, so this experiment will not repackage those ideas as new.
- [#4228](https://github.com/isogeny-labs/crypto/pull/4228) and [#4349](https://github.com/isogeny-labs/crypto/pull/4349) contain original-ideal certificate and selected-root alternatives. The native wide-prime m8 integration still lacks completed rational-root extraction.
- Petit–Kosters–Messeng, PKC 2016 (https://doi.org/10.1007/978-3-662-49387-8_1) and Amadori–Pintore–Sala, FFA 2018 (https://doi.org/10.1016/j.ffa.2018.01.009) remain the published prime-field algebraic IC comparators.
- **KB unavailable in this connector session:** research-KB MCP tools are not loaded; no repository wrapper runtime is available. GitHub source, previous reports, and primary article metadata were fallback. No unsearched/novelty conclusion.

## Mathematical branch and falsifier

Let the existing ordinary cofactor-one prime31 curve have `p=2147483647`, `n=2147446831`, base `B31`. Fix deterministic public admitted base points `A=P[0]`, `B=P[1]`, in **positive orientation**. For every natural public point `R=[a]G+[b]Q`, compute `R'=R-A-B` using two charged EC group additions. The following is an exact equivalence of full signed-point equations:

    R = A+B + (eps0 P_i0 + ... + eps5 P_i5)
    iff
    R' = eps0 P_i0 + ... + eps5 P_i5.

For finite ordinary auxiliary sums `U,V`, the residual balanced algebraic system is

    S4(x0,x1,x2,U)=0,
    S4(x3,x4,x5,V)=0,
    S3(U,V,x(R'))=0,
    H(x_i)=0 for i=0..5,

where the existing six-level Dickson tower enforces its inherited rational factor-base candidates. Semaev x-only equations cover signed projections but may include extraneous intermediate/multiplicity components. **A completed root must lift to ordinates, verify R' with signs, then add A+B and verify the original R**. Exceptional O and repeated-x branches must be handled separately; a polynomial basis without finite-root extraction is not a relation.

The direct ring now has **6×6 + 2 = 38 tower/free coordinates**, instead of 8×6+3=51. The fixed S3 pair bottleneck is removed, but the two unfixed S4 triples remain; this may still oversize. Positive control uses `P[2]..P[7]` with all-positive orientations, independently constructs R and a full tower witness, and checks every produced F4 basis element vanishes on it. It is a validator, not a natural-yield claim.

## Frozen test and reporting contract

- All **16** original `M8_NATURAL_F4_PANEL_20261115.json` public mask rows, unchanged seed/order, plus one separate known-positive control. Require recomputing each `aG+bQ` point and rejecting mismatches.
- Use the existing Rust `f4_tower`, same `degree_cap=12`, `max_nnz=15000`, `max_dense=15000`, 5-second per-call budget; no alternate engine, C++ code, group hash join or known-log oracle.
- Record verdict, censor reason, reached degree, max columns, NNZ/dense entries, elimination muladds, internal engine milliseconds, root-extraction state, and exact target coordinates for each call. A censored run is **undetermined**, never a measured no-relation or completed speed.
- Stop/go: if all 16 natural queries and positive control are capped by the same S4-subsystem matrix as before, reject naive anchor2-F4 at these caps and prioritize original-ideal-equivalent selected-root quotient. If at least one call returns a complete basis, still require actual rational-root extraction and full signed replay **before** treating it as useful.
- No runtime or security claim until a completed target-dependent extraction produces independently useful natural relations; compare completed IC oracle calls with PKM/APS, dense/tower F4, and certified quotient. Separately charge full cold Rust solver and optimized rho only when enough relations recover held-out scalar(s).
- EKS Spot/signed PCC not available here; GitHub Actions development CI is only a focused correctness/resource pilot. Do not merge, weaken policy gates, or promote leaderboard.

## Arithmetic yield sanity bounds (not observed outcomes)

With six remaining signed summands and unrestricted repetitions there are `(2*31)^6 = 56,800,235,584` ordered sign/index assignments and `n=2,147,446,831` targets. The average **number of algebraic full-point representations per group element** over uniformly sampled group targets is exactly `(62^6)/n ≈ 26.45`, by double counting; this does not imply independence, probability-one hit, or F4 feasibility. Fixing signed positive A/B transforms targets bijectively. Thus a natural source may have a substantial true witness set even while F4 is censored.

## Code and reproducibility

Extend the inherited `native/examples/prime31_m8_balanced_f4_input.rs` using its existing `six_ring`, `six_input`, `s3`, `s4`, `capped_f4`, `Arithmetic` and `Base` — no replicated cryptographic implementation — with `--anchor2-m6-f4`. Keep the existing example's behavior unchanged. Record source-specific CI stdout separately after any run.


## Separate predeclared generic **validator**, not an IC oracle

The original m8 split-control count does **not** determine how many of the frozen public R have an anchored m6 signed witness. A separate Rust-only ground-truth witness-count audit shall enumerate exactly the 62^3 labelled signed three-sums (including repeated indices and the identity) into a hash multiset, then for each of the 16 immutable public R' points sum the multiplicities of triple complements. Charge build/probe EC group work and retain counts per mask. Run the same count on the distinct public known-positive six-leaf target. Report exact number of ordered signed six-summand witnesses and whether zero occurs. This computation is **generic exhaustive MITM** and must be kept entirely outside algebraic F4 discovery; it may only classify whether a censored algebraic query had at least one full-point solution. No generic fallback, rank, log, benchmark advantage or scientific novelty follows from this diagnostic.
