# PLAN (preregistration, written before any measurement run of this round)

Date: 2026-11-21. Repo HEAD at planning time: `473c4064294aea5131fb869b8aaa30810f9ed228`.
Status of every number in this directory: [pcc:unverified] (local development check at small
parameters, not an isolab/ecbench run; no replay certificate, no validator report).
Round 1 (`../extfield-jacobian-decomp_20261115/`) is not edited; its recorded results are read, not rewritten.

## Question

Round 1 built the exact Weil-restriction decomposition (R + P1+..+P4 = 0, x(P_i) in F_q) and [pcc:unverified]
showed that with factor base = all x-in-F_q classes, no large primes and dense LA the algorithm has
exponent 2 in q, the same as rho. This round builds the lever round 1 did not: a reduced factor base F of
|F| = round(c q^alpha) classes (deterministic subset of the x-in-F_q classes), partial relations with up
to two points outside F (large primes, still x in F_q), merged through the repository's large-prime graph
filter, and sparse linear algebra (sequential Wiedemann mod ell). Heuristic exponents: relation phase
q^(3-2 alpha) (about q relations are needed to close cycles in a graph on ~q/2 large-prime vertices, each
costing ~ (q/|F|)^2 decomposition trials), LA q^(2 alpha); balance at alpha = 3/4 gives 1.5. We
measure whether this shows up at q <= 2039 and report the distance to matched rho at 0, 0.1 and 1 GAE.
No improvement is claimed.

## Instrument (frozen before measuring)

* Reused unchanged from round 1 (included by #[path]): F_{q^2} arithmetic, Cantor arithmetic, curve/instance [pcc:unverified]
  generation (`make_instance`, same seeds so instances equal round 1's), Macaulay solver, group-law
  verification of every decomposition, matched negation-map rho (`run_rho`, same code, same seeds).
* New: arm = (alpha, c). F = the min(n_classes, round(c q^alpha)) classes with smallest splitmix64 hash
  of x (nested across arms; deterministic). alpha = 1 means F = all classes (control, no large primes).
  A verified decomposition is accepted if the number of distinct classes outside F is <= 2
  (rows with 0, 1 or 2 large primes). Points with y = 0 carry no column (as round 1).
* Merging: `large_prime_filter::filter_large_primes` (copied unchanged into `src/` with its
  `jv_bitops` hooks replaced by counting shims and the `trace` submodule dropped; the repo's lib is not
  linked). Its field operations and data words are counted and priced.
* Sparse LA: sequential Wiedemann mod ell on the normal equations (M^T M) x = M^T b over the merged complete rows
  (`src/sparse_la.rs`, written here because `pq_wiedemann` needs BigUint and crate-internal types), every mulmod
  counted. Acceptance of the DLP: k = x[k column] and `[k]D = Q`.
* Paired design: for each instance one decomposition stream (same R walk, same solver calls) is consumed by all
  five arms; each arm stops when its own attempt solves the DLP, and is charged the stream's counters up to its
  own stopping trial plus its own filter/LA/verification work. Common random numbers, so arm differences are
  paired; arms differ only in which of the same decompositions they accept. Attempts: a filter pass whenever
  accepted rows grow by 10 percent (first at nF + 10); Wiedemann only if the merged rows number >= columns + 2.
* Counted unit: group additions (GAE). Unpriced native work at w in {0, 0.1, 1} GAE per operation (plus w_cal from
  round 1's calibration, shared VM, best of 3, reported for continuity): solver F_q multiply-adds (round 1's nominal dense
  Macaulay count), root finding, factor-base construction, the filter's field ops and words (an inversion counts
  40 ops), Wiedemann mulmods. Rho: round 1's native tally unchanged. S = adds + w * native. S_relation = stream
  part, S_LA = filter + Wiedemann + kD check part.
* Seeds: instance seeds 1, 2, 3 per q; stream tag 0. A/A: tag 1 (new internal walk start) on seed 1 at q in
  {131, 251, 509}.
* Ladder: q in {131, 251, 509, 1021, 2039}. q = 4093 is not attempted: the round-1 solver's lazy u32 elimination
  is limited to q <= 2039 and decomp.rs is not edited this round. Exponents are fitted on the 5 q values
  (>= 4), bootstrap over the 3 instance seeds per q (2000 resamples), arm-wise, for total S at each w.
* Arms: control (alpha=1), (0.75,1), (0.75,2), (0.6,1), (0.6,2). When round(c q^alpha) >= n_classes the arm equals the
  control at that q and is flagged `clamped`.
* Stopping rule: per stream, 1.5e6 trials or 20 minutes wall, whichever first; unsolved arms are reported censored
  with the counts reached. No cell is re-run to change an outcome.

## Preregistered predictions

P1 (as assigned). With alpha = 0.75 and up to two large primes the total-S exponent (fit over the ladder) is <= 1.7 at [pcc:unverified]
   w = 0, 0.1 and 1, for both c = 1 and c = 2. The control exponent is 2.0 +- 0.1.
   Own expectation, recorded before running: the alpha = 0.75 part is confirmed (about 1.3 to 1.6; mechanism
   q^(3-2 alpha) = q^1.5 relation phase); the control part is refuted: at q <= 2039 the control is
   solver-dominated, so its fitted exponent is about 0.8 to 1.2 (round 1 measured 0.74 to 0.83 with dense LA), well
   below the asymptotic 2.
P2. LA share (S_LA / S_total) at q = 2039 is below 50 percent for every arm at w = 0.1 and w = 1.
   Own expectation: far below (under 1 percent): ~4.5e5 mulmods per trial times 1e4..1e6 trials dwarfs sparse LA.
P3. For the best arm (lowest mean S at w = 0.1 and q = 2039), mean S/S_rho at w = 0.1 improves (decreases)
   monotonically along q = 131, 251, 509, 1021, 2039. Own expectation: probably refuted by seed noise at one step
   (round 1's control went 991 -> 1001 at the last step).
P4. Predicted crossover q* (IC below rho), from round 1's constants (p = 0.0402, 4.52e5 mulmods/trial, rho
   S ~ 0.99 sqrt(ell)(2+4w)+9.5e3, ell ~ q^4/8.5; `prereg_model.py`): w = 0: IC already below rho on the whole ladder (mulmods
   free); w = 0.1: control ~2e6, (0.75,2) ~1e8, (0.75,1) ~2e9, alpha=0.6 arms none below 1e15; w = 1: control none, (0.75,2) ~2e9.
   i.e. every q* is far outside the ladder (>> 2039); S/rho at q = 2039 is predicted > 100 at w = 0.1 for
   every arm. The model's rho is about 3 times too high against round 1's measured rho, so true q* are larger still.
P5. Ordering at q = 2039, w = 0.1 (paired): S(control) < S(0.75,2) < S(0.75,1) and S(0.6,2) < S(0.6,1);
   reduced arms cost MORE than the control at every ladder q at w > 0 (their relation phase is q^(3-2 alpha) > q).
P6. Per-trial solver cost is constant in q: mean native per trial within +-10 percent of 4.5e5 for q >= 251, and the
   solver share of S(0.1) is >= 90 percent for every arm at q >= 509 (relation phase dominates everything).
P7. Partial-relation acceptance rate per trial = p x P(<= 2 outside F) with p = 0.0402 and the realised F fraction
   f = nF / n_classes (binomial approximation over 4 points), within +-30 percent, for non-clamped reduced arms with f <= 0.5.
P8. A/A: ratio of S between tag 0 and tag 1 on the same instance within [0.5, 2] for every arm at q in {131, 251, 509}.
P9. Every uncensored run satisfies `[k]D = Q` with k equal to the instance's k; correctness of merged rows is also
    unit-tested on synthetic relations.
P10. Raw accepted relations at the solving attempt, for non-clamped reduced arms, are within a factor 2 of
    1.1 (n_classes - nF) + nF + 10 (graph-cycle model), pooled over cells.

## Reporting rules

All cells, including censored ones. Report S, S/rho with seed-bootstrap intervals at w = 0, 0.1, 1 and the
distance to matched rho. If the per-trial decomposition cost (Macaulay solve) dominates, state the measured
per-trial cost and the honest crossover projection instead of any win.
