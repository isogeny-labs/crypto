#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: true)
#show table: set text(size: 8.5pt)
= Where a Jacobian-cover decomposition test spends its cost, and the next leads

2026-10-18 · study `research/ic_cover_leads_20261018` · run `run-20261018T1900Z` · build `536aba4ce` (`f4_fp.rs`, `jv_cover.rs` identical to main `3c192c34c`)

*Question.* After \#1717 made `f4_fp` 5× cheaper inside `examples/jv_cover.rs` (\#2182), what does one Nagao decomposition test cost now, part by part? Which optimisations come next? Status: *measured* means from this run; *derived* means computed from measured values, with the formula shown; *proposed* means not built.

*Answer.* At p = 101, C_cov = 1,222,627 field multiplications per test (measured). F4 takes 60 % and the linear stage after it 39 %. Relation collection is 99.99 % of end-to-end S in \#2182's dlp records. So per-test savings pass almost one-for-one into S/rho, but only as constant factors.

== Measured cost of one test (p = 101, 120 tests)
#figure(image("breakdown.svg", width: 100%), caption: [Per-test counted multiplications by part. `jv_cover --exp ccov`, run-20261018T1900Z. The parts add up to the recorded `lin_muls` (481,526) and `c_cov` (1,222,627) means.])

#figure(image("dense_cols.svg", width: 82%), caption: [Non-unit columns of each multiplication matrix (mean of 120 systems). `M_x6`, for the last grevlex variable, has 20.0 of δ = 64.])

#figure(image("f4_by_degree.svg", width: 82%), caption: [F4 rows per degree over 100 systems: 124 of 1,014 rows (12 %) reduce to zero. These are the Koszul syzygies of a regular sequence.])

#figure(image("trace_replay.svg", width: 82%), caption: [The existing F4 trace replay (`--trace`) costs 31 % (p = 53) and 35 % (p = 101) more F4 multiplications than the stock engine; 24 and 13 of 119 replays diverged.])

== Leads, ranked
#table(columns: (auto, 1.3fr, 2.2fr, 1.6fr), align: left, inset: 4pt,
  [*L*], [*lead*], [*change*], [*estimate / status*],
  [L1], [Sparse-FGLM on `M_x6` (Faugère–Mou)], [Build only `M_x6`. Get its minimal polynomial by Wiedemann / Berlekamp–Massey. Recover points by chain propagation over the 44 unit columns plus a ~20-unknown solve. Fall back when x6 does not separate points.], [Linear stage 482k → 210k–260k: 2δ·(20δ+44) ≈ 169k + BM 8k + roots 19k + NF 10k–57k + recovery ≈ 3k/root. C_cov ≈ 1.22×–1.29× (derived). Proposed.],
  [L2], [F5 / signature criteria], [Discard Koszul zero reductions before building them.], [At most 124/1,014 rows (12 %), about 1.08× on C_cov (derived bound). Proposed.],
  [L3], [Rebuild or retire the F4 trace replay], [Record only pivot rows on the current engine, or drop `--trace`.], [−23 % to −26 % of F4 multiplications against today's `--trace` (measured regression).],
  [L4], [Stop F4 once the needed normal forms are fixed], [L1 needs NF only on `M_x6`'s 20 border monomials. The degree-7/8 steps hold 37 % of the rows.], [Up to 37 % of F4 rows, if a sound early stop exists. Speculative.],
  [L5], [Measure S/rho against p], [`--exp dlp` at p = 251, 503, 1009, several seeds.], [S/rho, mean of the two seeds: 610 (p = 53), 284 (p = 101), slope ≈ p^−1.19 (2 seeds each). Extrapolated, it meets rho between p ≈ 1.7k (p^−2) and p ≈ 12k (p^−1.19); derived and wide. Proposed.],
  [L6], [Large-prime variation (Gaudry–Thomé–Thériault–Diem)], [Accept decompositions with one or two points outside the factor base and combine them through the large-prime graph.], [The only lead here that changes the exponent; not estimated. `jv_cover` has no large-prime path. Proposed.],
)

L1–L4 multiply C_cov by constant factors, at most about 1.4× together (derived: 1.29 × 1.08). The distance to rho, 194–850× after \#2182, is decided by L5's slope and L6's exponent.

== Scope and limits
Counted field multiplications only; this run records no wall-time comparisons. The host is a 4 vCPU shared VM (L0). One seed per p for the cost breakdown. The L5 slope rests on two seeds at each of two primes. No change to the engine is made or claimed here.

== References
Faugère and Mou, Sparse FGLM, JSC 80 (2017). Faugère, F5, ISSAC 2002. Berthomieu, Eder and Safey El Din, msolve, ISSAC 2021. Gaudry, Thomé, Thériault and Diem, Math. Comp. 76 (2007). Joux and Vitse, Eurocrypt 2012.
