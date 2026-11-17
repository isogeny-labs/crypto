#set page(margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")
#show raw: set text(size: 8.5pt)

= Large-prime variation in the cover attack: lead L6 of \#2216 (2026-10-18)

*Verdict (derived, with measured constants).* For the cover decomposition path of `examples/jv_cover.rs`, a large-prime variation lowers `S` by at most 0.02 % (one large prime), 0.27 % (two) and 0.56 % (an information bound no scheme is known to reach) for every p ≤ 12000. The 10 % gate fails by more than a factor 18. No attack code was changed; the study adds opt-in logging to the dlp path and a model example.

Labels: *measured* = a recorded run; *derived* = computed from measured values (formula in `COSTMODEL.md`); *proposed* = not built. No wall-clock claims; no new cross-method comparison with rho is made, so the §5a price band is not needed (rho figures quoted from other studies are lower-bound comparisons as recorded there).

== Boundary, target, falsification

- *Boundary (derived).* Any elimination of the points outside the sub-base `F'` needs at least `touched + 1` solutions (generic coefficients), `c* N` with `c* = 0.9975`, whatever `|F'|` is. The plain method uses `N + 1`.
- *Target set before measuring.* Model saving ≥ 10 % of `S_full` for some p ≤ 12k; otherwise write the negative result.
- *Class.* Negative result; no row improves on the baseline.

== Cost model

`S(f) = X(f) · T · C_test + 419 (f N)^2`, `N = |F| ≈ p/2`, `f = |F'|/|F|`, `α = log|F'|/log|F|`.

- `T` (tests per decomposition), measured 779 ± 33 (95 %; 14 runs, 2139 decompositions; the heuristic is 6! = 720).
- `C_test`, measured 1.2216M counted multiplications at p = 101 (\#2182 post-\#1717 records), growing 0.75 % per doubling of p (pre-\#1717 ratio, derived).
- LA, measured 26.2 Wiedemann operations × 16 multiplications per `u^2` at row weight 7.
- `X(f)/N`: solutions needed, Monte Carlo over uniform 6-subsets of `F` (`curves.json`).
- A decomposition test returns decompositions over the whole `F` at a cost independent of `F'` (argued): the sub-base only filters afterwards. Yields: full `f^6`, 1-partial `6 f^5 (1-f)`, 2-partial `15 f^4 (1-f)^2`.
- Graph cost priced 0, merged-row weight ignored: both favour large primes.

== Measured against modelled yields

#figure(image("figs/fig_yields.svg", width: 100%),
  caption: [Share of real decompositions with 0, 1, 2 points outside a sub-base (first f|F| base elements). Points: measured with Wilson 95 % intervals (322 decompositions at p = 101, 6 seeds; 254 at p = 251 (2 seeds), 1 seed). Lines: hypergeometric model. Records `raw/yields.json`, logs `raw/dlp_p*_s*.log`.])

The six abscissae are uniform (chi-square z between -1.5 and +0.6 per seed; all six distinct). The largest |z| over 72 cells is 2.6. Real decompositions give each scheme the same number of relations as the uniform model (p = 101, f = 0.5: single prime 1.10 real against 1.07 modelled; bound 27.67 against 27.71). Controls: seeds 1, 2 at p = 101 and seeds 1, 2 at p = 251 reproduce the earlier records' residual counts and multiplication totals; every discrete logarithm verified (`correct = true`).

== Solutions needed

#figure(image("figs/fig_solutions_needed.svg", width: 70%),
  caption: [Monte Carlo, N = 6000: solutions needed over N for one large prime, two, and the information bound, against f. The plain method needs N + 1. Mean of the trials in `raw/curves.json` (N-independence checked from 484 to 6000 within 3 %).])

== Result

#figure(image("figs/fig_cost_vs_alpha.svg", width: 100%),
  caption: [Derived S(α)/S_full for one (solid) and two (dashed) large primes. Left: p ≤ 12k, where no curve goes below 0.997. Right: p ≥ 1e6, where the LA is large enough to trade (single-prime and double-prime rows use the N = 6000 curve; extrapolated below f = 0.05 for the bound only). `raw/cost.json`.])

#figure(image("figs/fig_la_share.svg", width: 62%),
  caption: [Derived LA share of the plain method, `419 N^2 / ((N+1) T C_test + 419 N^2)`, N = p/2. 1 % at p ≈ 4.9e4, 10 % at 5.5e5, 50 % at 5.1e6.])

#table(columns: (auto, auto, auto, auto, auto), align: right,
  [p], [LA share], [1 large prime], [2 large primes], [bound],
  [101], [2.2e-5], [0.9995], [0.9981], [0.9980],
  [251], [5.4e-5], [0.9991], [0.9978], [0.9944],
  [1009], [2.2e-4], [0.9997], [0.9973], [0.9966],
  [4096], [8.7e-4], [0.9999], [0.9973], [0.9957],
  [12000], [2.5e-3], [0.9998], [0.9975], [0.9944],
  [1e6], [0.166], [0.996], [0.980], [0.832],
  [1e7], [0.661], [0.916], [0.778], [0.34],
  [1e8], [0.950], [0.542], [0.378], [0.050],
  [1e12], [1.000], [0.029], [0.013], [7e-6])

Best S/S_full over f (derived). Below 12k the differences from 1 are at the Monte-Carlo noise level. The 10 % gate is passed at p between 1e6 and 1e7 (two large primes) and between 1e7 and 1e8 (one), far above the crossing range. Asymptotically a single large prime gives `S ∝ N^(2 - 4/13)` against `N^2` (derived): the exponent changes, but only where the LA already dominates.

== What would change the exponent, ranked

+ The sieve path (`jv_sieve.rs`) with a large prime (proposed): LA share there is measured 8.5–9.9 % at p = 1823 (0.04–0.07 % at 251), relation cost 2.1–2.7M multiplications at p = 1823 against about 0.95G for 779 cover tests (derived).
+ A different m: free in the sieve (11 at p = 101, 9 from 1009), fixed at 6 in the decomposition path.
+ A cheaper test: constant factor, L1–L4 of \#2216 reach about 1.4×.
+ Large primes in the decomposition path: at most 0.56 % for p ≤ 12k.

== Correctness

`cargo test --release --lib jv_cover`: see `raw/test_jv_cover.log`. Lean (`lean/LpBound.lean`, toolchain v4.34.1, axioms `propext`, `Quot.sound`): the conditional arithmetic step only ("if a sub-base scheme needs at least the plain solutions, it saves at most the plain LA cost"). The rank bound itself is open (needs linear algebra, no Mathlib).

== Open, limits

Proposition 1 (no F'-restricted test is cheaper) is argued, not proved (`problems/jv-cover-subbase-restricted-test.md`). Coefficients are generic in the bound. Monte Carlo N ≤ 6000. Two seeds at p = 251. The sieve with large primes is not modelled here.

== Sources

\#2216 `research/ic_cover_leads_20261018/README.md`; \#2182 `research/f4_fp_ic_effect_20261018/` (records `raw/ref/pr2182_*.json`); `experiments/30_jv_cover_dlp*.json`, `30_jv_cover_ccov.json`, `32_*`, `35_*`, `38_*`; Gaudry–Thomé–Thériault–Diem, Math. Comp. 76 (2007); Joux–Vitse, Eurocrypt 2012.
