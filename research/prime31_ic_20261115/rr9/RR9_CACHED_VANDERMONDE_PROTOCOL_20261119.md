# Prime31 RR9 B8 cached Vandermonde graph-chart gate — preregistered protocol

Status: **protocol frozen before source or benchmark**, 2026-11-19 UTC. Extends rank-one chart lane PR #4783, stacked on #4780; no claim of a cheaper complete solver. [pcc:unverified]

## Mathematical prediction

On unchanged original Prime31 curve `p=2147483647`, `E:y²=x³+847514846x+2087444724`, cofactor one, group order `2147446831`, generator `G=(625037715,2100627251)`, construct original RR9 signed supports with eight slots, B=8 unsigned leaves per slot and two signs.

For one four-row half indexed by signed leaves `q_i=(x_i,y_i)`, target `R=(r,s)`, and `V_{i,j}=x_i^j` (0<=i,j<4), define `z_i=(y_i+s)/(x_i-r)`. Under **all** distinct `x_i` and `x_i != r`,

- `A=diag(x_i-r)V`, `B=diag(y_i+s)V`;
- `M=A^{-1}B=V^{-1}diag(z_i)V`;
- `det(A)=det(V)prod_i(x_i-r)`.

Because `x(-Q)=x(Q)`, only `B^4=4096` distinct unsigned four-row Vandermonde systems occur per half, reused by the `2^4` sign combinations. Cache each `V^{-1}` and `det V` **once per factor-base setup**, then rebuild target-dependent `M` with ordinary finite-field operations, not per-tuple inversion. Cache `1/(x_i-r)` **once per target and signed slot choice** (at most 128 signed leaves, actually at most 64 unique abscissae).

This is classical Vandermonde/interpolation linear algebra applied to the prior RR9 graph chart, not a new subexponential ECDLP algorithm. Singular input must be flagged as **unsupported by this chart** rather than misclassified as relation-negative.

## Frozen experiment controls

1. Reuse original Rust `prime31_algebraic_ic::Arithmetic` and `row`, `graph_chart`, `graph_halves`, and `det8` from #4783. Do not introduce new EC arithmetic, generic sum tables, or altered curve.
2. Implement `cached_vandermonde_halves`: one target-independent unsigned cache for both halves, target-denominator precomputation, and 16 signed coordinate choices per slot. Populate the same `(signed Word, graph M, detA)` tuples as `graph_halves`. Fail closed on any `x_i=r` or Vandermonde collision. Verify all 65,536 entries for *both* halves on at least two original-curve public targets (G and nontrivial planted aggregate), and add a synthetic/nontrivial sign case.
3. Independent Python `int` modular arithmetic reconstructs independently selected x/y four-row charts and checks `V^-1 diag(z)V`, `det(A)`, and `det8` against raw exported samples. Include at least 128 real-curve half-chart checks, not just producer-generated hashes.
4. Paired **A–B–B–A** elapsed measurements with identical target inputs: A is fresh `graph_halves`, B is the proposed cached target online path. Time setup/cache construction and per-target denominator precomputation separately. `std::hint::black_box` output digests to retain work; compare exact digests/order-independent equality. Report values **only after CI** and retain raw source-bound JSON. The target-independent amortization count (1 and 1,024 residuals) must be disclosed; no extrapolation of local ratio to extraction.
5. Original Rust library regression and exact-focused Rust tests in source-bound GitHub Actions. An external independent signed PCC remains a separate gate.

## Predeclared failure modes, cost frontier, and decision

- If baseline and cached chart differ on one tuple, **FAIL**. If zero-denominator appears, **INCONCLUSIVE** for that fibre and report fallback needed. If CI is unavailable, report **UNMEASURED**, never estimate a result.
- Even a positive chart-only result leaves unchanged the 16^8 = 4,294,967,296 graph-pair checks on a fully exhausted B8 public target, as shown in #4780/#4783. The new path is chart construction, *not* public-only unknown-index extraction. Do not use this result to assert non-generic solver cost advantage.
- If cache time > savings over observed workload, reject the optimization for that workload.
- Competitive advancement is separately owned by #4692: pass frozen 46-positive/18-negative natural targets with genuinely nonenumerative unknown-index extraction (zero generic EC four-sum tables). Only then replace the generic 4+4 oracle in #4728, retain 31/31 rank and 1024/1024 blinded logs, and measure same-runner complete costs against optimized IC/frontier, signed shared BSGS (#4774), and rho, including cache/setup, failures and memory.

## Prior work checked before writing code

- #4776 exterior-four, #4780 four-plane graph chart, #4783 rank-one rowupdate, #4779 fixed exterior-span audit, #4773 prefix reduction, #4728 generic 4-window end-to-end, #4692 remainder/F4 extractor, #4774 matched BSGS, autoresearcher SA-04/18 failures and cryptanalysis Rust index calculus.
- Primary prior art: Petit–Kosters–Messeng, PKC 2016, DOI:10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, Finite Fields Appl. 2018, DOI:10.1016/j.ffa.2018.01.009. No claim of originality for Vandermonde interpolation or generic finite-field index calculus.
- **KB unavailable**: no shared research-KB connector or networked CLI in this runtime. Sources checked are GitHub paths/PRs and primary literature only. 
