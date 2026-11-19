# Preregistered Prime31 RR9 one-leaf graph-chart rank-one gate (2026-11-19 UTC)

Status: **PROTOCOL ONLY**; frozen before implementation, CI or benchmarks. Stacked on [PR #4780](https://github.com/isogeny-labs/crypto/pull/4780), tracked at [PR #4783](https://github.com/isogeny-labs/crypto/pull/4783). All new claims are **[pcc:unverified]** until separately signed.

## Exact hypothesis

For a valid four-row chart H=[A|B], M=A^{-1}B and a single replaced row j of A and B, let delta_a=a'_j-a_j, u=A^{-1}e_j, d=1+delta_a u, and v=b'_j-a'_j M. When d != 0, the updated chart is **M'=M+(u/d)v**. This is a rank-one *preprocessing* update, not an unknown-index solver. If d=0, singular chart must be detected and the update fail closed. Compare against full chart recomputation, full original 8x8 determinant, and exact chosen-target EC group identities where applicable. Distinct-point and signed-support restrictions stay unchanged.

## Immutable fixture before candidate execution

- Field p=2147483647; unchanged original ordinary curve y^2=x^3+847514846x+2087444724; prime order n=2147446831; G=(625037715,2100627251); original arithmetic source pinned at `1df714645aa683cf975bc76fcacb9842cb4780fd`.
- Canonical first-eight base points by increasing x with y chosen min(y,p-y); signed weighted B8 public supports with original W=[1,3,5,7,11,13,17,19].
- Public residuals: G and the sum of all first positive signed slot leaves. No secret scalars to candidate (fixture scalar only in external EC verifier).
- Start with four **fixed** left-half leaves (slot option zero); change each of four rows to each of the 16 public signed candidates, including an identity replacement. Evaluate 2 residuals x 4 rows x 16 replacements = **128** cases. Repeat each update with a fixed admissible right-half context to compare Schur determinant residues and exact 8x8 residues; discard no singular case without reporting it.
- Synthetic fixed matrices: 128 deterministic u64-xorshift over Fp; verify rank-one formula and singular d=0 handling; no probabilistic validity claim from samples.

## Program and measurement

- Reuse original `prime31_algebraic_ic::Arithmetic`, original `prime31_rr9_exterior4.rs` point rows, Schur graph chart and full determinant, without changing the affine curve arithmetic or generic ECDLP solver.
- Rust `cargo test --release --lib` on pinned original crate and focused example tests; independently authored Python BigInt verifier recomputes the emitted original-curve matrix residues from raw public coordinates and confirms on-curve points and chosen sum.
- A/B/B/A same-runner **preprocessing-only** timing of full four-plane chart recomputation versus rank-one updated charts; include old A^{-1} setup in rank-one timing and document whether amortized across 16 replacements. Pin repeat count before collection (500 trials over the fixed full 128-case schedule), use compiler black_box and identical output digest.
- Count all Fp operations when feasible; report inversions and multiplications of the rank-one kernel separately from shared precompute and full-scan candidate costs. Benchmark numbers are exploratory and **not** whole-solver runtime.
- No EC addition or scalar search inside public-only candidate. Independent EC group truth can be out-of-band and is excluded explicitly from candidate timings.

## Decision gate and nonduplication

- ACCEPT exact identity only if all 128 original-curve swaps, 128 deterministic synthetic matrices, singular cases, full 8x8 residuals, and independent Python BigInt replay agree.
- ACCEPT a preprocessing speed claim only for same-source, same-host measured A/B/B/A comparisons, not as a subquadratic extractor.
- Even a zero-cost chart build would still leave 16^8=4,294,967,296 pair predicates for exhaustive B8 negative extraction. **Any claim of cheaper complete non-generic ECDLP requires a different, public-target-only B8 unknown-index algorithm**, with no 4+4 EC sum tables, and rank31/1024 public-log integration, all fully charged against IC, BSGS and rho. Coordinate with [#4692](https://github.com/isogeny-labs/crypto/pull/4692) rather than duplicate its nonenumerative F4/remainder-tower lane.
- Prior areas checked: #4466, #4547 seven-anchor completion, #4728 four-window generic, #4774 matched BSGS, #4776 exterior, #4779 Pluecker span, #4780 graph chart; crypto-autoresearcher SA-04/SA-18 negative norm/determinant; PKM PKC 2016 DOI 10.1007/978-3-662-49387-8_1; APS FFA 2018 DOI 10.1016/j.ffa.2018.01.009; Kudo et al. 2018 hybrid symmetry. Shared research-KB unavailable in this runtime; bounded primary source and repo audit is not proof of novelty.
