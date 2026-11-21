# PLAN (preregistration, written before any measurement run)

Date: 2026-11-15. Repo HEAD at planning time: `3c24520f7cdc1c47bac26cc7778d058e655b79a8`.
Status of every number below and in later files: [pcc:unverified] (local development
check, not an isolab/ecbench run; no replay certificate, no validator report).

## Question

Does Gaudry/Nagao decomposition index calculus run directly on
J(F_{q^2}) for a genus-2 curve y^2 = f(x) over F_{q^2} (an abelian variety of
dimension 4 over F_q; no Weil-descent cover) beat matched negation-map Pollard rho
in the repository's counted unit, at small q? We report the distance to matched
rho at 0, 0.1 and 1 GAE per native operation; we do not claim an improvement.

## Instrument (frozen before measuring)

* Field F_{q^2} = F_q[s]/(s^2 - nr), q an odd prime. Curve y^2 = f(x), f monic
  quintic with uniformly random coefficients in F_{q^2} (not defined over F_q),
  squarefree. Mumford/Cantor arithmetic over F_{q^2}
  (src/fq2.rs, src/jac.rs); associativity, inverses, order checks are unit-tested.
* Group order: #C(F_{q^2}) by direct count, then BSGS inside the Weil window
  for the L-polynomial coefficient a_2, filtered with 6 random elements.
  A curve is accepted when #J = h * ell with ell prime, h <= 16, ell > 1000.
  Curve search, order finding, factoring are NOT charged to either arm (both arms are
  given the order).
* Instance: D = h * (random element of J) of prime order ell; Q = k D, k uniform.
* Factor base: all points with x in F_q (about q points, about q/2 classes mod negation).
* Decomposition of R = eD + gQ (walk R <- R + M_j from a 16-entry pool, 1 group
  addition per trial): R + P1 + .. + P4 ~ 6*inf, x(P_i) in F_q. Weil-restriction
  system: with h = v + u (x+b)/a the quartic G = (f - h^2)/u / (-1/a^2) has
  coefficients c_k(a,b) quadratic in (a,b) in F_{q^2}; c_k in F_q gives four real
  quadrics in four unknowns (Bezout 16), solved exactly by a degree-5 Macaulay matrix
  (escalating to 6, 7), multiplication matrices, characteristic polynomial,
  gcd with x^q - x, eigenvectors; then the quartic must split over F_q. Every relation
  is verified by the group law (R + sum P_i = O). A pair-hash collinearity
  oracle (all pairs of factor-base points in the (x, t) plane, t = (y - v)/u) is the
  correctness reference; the solver must reproduce its decomposition set exactly.
  R whose Macaulay quotient does not close by degree 7 ("degenerate R", expected rate ~1/q)
  are counted as failed trials, not dropped.
* Linear algebra: dense Gaussian elimination mod ell over relations
  (unknowns: phi(class) for classes seen, plus k). Counted in mulmods. The
  block-Wiedemann count 2*m*nnz is reported as a MODEL line, not measured.
  Weight-4 rows would suit block Wiedemann; it is not implemented (limitation).
* Rho: same group, same Mumford arithmetic, r=64 additive walk, negation map
  (canonical representative by v-key), distinguished points, 8-step history
  fruitless-cycle escape by doubling.
* Counted unit: group additions (Mumford additions/doublings, one each) = GAE.
  Unpriced native work, priced at w in {0, 0.1, 1} GAE and at w_cal (see below):
  IC: F_q multiply-adds of the decomposition solver (Macaulay elimination
  counted nominally as nonzero-row-ops x row length, i.e. a dense upper
  bound), factor-base construction (30 F_q mulmods per x), root finding, equation
  assembly, mod-ell mulmods of linear algebra. Rho: partition hash, canonicalisation,
  distinguished-point test, cycle-history check (1 each per step), DP table
  operations (1 per DP insert/lookup).
  w_cal = (time of one scalar F_q mulmod) / (time of one group addition) on this
  host, best of N, label "best-of-N, shared 4-CPU VM"; not an isolated host.
* Seeds: instance seeds 1, 2, 3 per cell (curve and instance derived by
  splitmix64 from (q, seed)); IC and rho internal seeds derived from the instance seed.
  A/A: second internal seed on the same instance for both arms at q in {131, 251}, seed 1.

## Cells

q in {67, 131, 251, 509, 1021}; seeds {1,2,3}. If time is short the ladder stops
(smaller completed ladder preferred); q = 1021 is attempted only if the q = 509 IC run
took under 6 minutes. Hard cap per IC run: 300*(classes) trials; per rho run: 40*sqrt(ell) steps.
Oracle agreement test: q = 67 (300 random R) and q = 131 (100 random R).
Decomposition probability = verified-relation R / all R tried (degenerate R included
as failures), pooled over seeds, with Wilson 95% interval.

## Preregistered predictions

P1. Decomposition probability per trial is 1/24 within +-30%, i.e. in [0.0292, 0.0542],
    pooled per q for q >= 131 (q = 67 reported, finite-q effects allowed).
P2. IC exponent in q (least-squares slope of ln S_IC on ln q over cell means) is <= 1.7
    at each of w = 0, 0.1, 1 and w_cal. (Expected mechanism: relation phase is
    ~24 solver calls per relation x q/2 relations at constant solver cost; dense LA is
    cubic in q/2 but small at these q.)
P3. Rho exponent in q is in [1.85, 2.15] (S_rho ~ 0.89 sqrt(ell), ell ~ q^4/h).
P4. Macaulay solver and pair-hash oracle agree on 100% of non-degenerate R tested.
P5. Crossover (extrapolated from the two fits, per w): IC beats rho at w = 0 over the
    whole ladder; at w = 0.1, 1 and w_cal the IC arm costs more than rho over the whole
    ladder and the extrapolated crossover q* lies beyond the ladder (q* > 1021). The
    per-trial solver cost (not group additions) dominates.
P6. Per-trial solver cost grows with exponent in q within [0, 0.3] (Macaulay constant
    ~1e6 mulmods independent of q, plus O(q) root finding).
P7. Dense LA is below 5% of S_IC(0.1) in every cell (relation phase dominates).
P8. Degenerate-R rate is below 5% for q >= 131.
P9. A/A: ratio of S between two internal seeds on the same instance is within [0.5, 2]
    for IC and within [0.3, 3.3] for rho.

## Stopping rule and reporting

Report all cells run, including failures and caps. If a cell exceeds its cap it is reported as
censored with the counts reached (not dropped). No cell is re-run to change an outcome.
If per-trial decomposition cost dominates, report the measured per-trial cost and the
honest crossover projection rather than a win.

## Amendment 1 (before any registered run)

Development smoke runs (seed 99 only, q = 67 and 251; outside the registered seeds, results
discarded) showed the full pipeline works and each run takes seconds, so the time budget allows a
sixth cell: q = 2039 is added to the ladder (q <= 2039 is the limit of the lazy-reduction
u32 elimination). Ladder: q in {67, 131, 251, 509, 1021, 2039}. The "q = 1021 only if q = 509 took under
6 minutes" condition is dropped as moot. Predictions P1-P9 are unchanged.
Smoke observation (not a result): solver about 4.7e5 counted F_q mulmods and 0.46 ms per trial, so
the registered P6 (solver per-trial exponent in [0, 0.3]) and P7 stand as written. The relation pool
(16 entries for IC, 64 for rho) is charged in group additions for both arms.
Runner: run_ladder.sh (kept in this directory); raw per-run JSON in results/.
