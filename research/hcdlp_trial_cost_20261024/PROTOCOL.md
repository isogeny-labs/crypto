# Protocol: cost of one relation trial on a high-genus binary hyperelliptic Jacobian

Frozen 2026-10-24 before any measurement is taken. Study directory:
`research/hcdlp_trial_cost_20261024/`. Status labels: **measured** (counted or
timed by `trial_cost.rs`), **derived** (arithmetic on measured values),
**model** (the N = 130 note's cost model or an extrapolation), **assumption**.
This prices an abstract tower in a model; it attacks no registry curve.

## 1. Question

`research/ghs_counterfactual_n130_20261013/README.md` prices index calculus on
a genus-255 Jacobian over F_2 at 2^55.60 against rho 2^60.31 (margin 4.71
bits), charging one operation per relation trial. Finding 6 says the unit
conversion ("perhaps 2^7 curve additions") is unmeasured. This study measures it.

## 2. What one trial is in the model (read from `window.rs::price`)

`price` charges `relations = log2|FB| - log2 P(smooth)`: |FB| relations each
costing `1/P(smooth)` trials, plus `linear algebra = 2 log2|FB| + log2 g`
(`|FB|^2 g` multiplications modulo r), total `max(rel, lin) + 1`. Its doc
comment: "One operation is one divisor-class step plus one smoothness test".
`P(smooth)` is the fraction of effective degree-g divisors supported on places
of degree <= b, with place counts from `#C(F_{2^k}) = 2^k+1` (the
random-polynomial model; equal to the chance that a random monic degree-g
polynomial over F_2 is b-smooth).

**Trial (this study's definition).** For the curve `y^2 + h y = f` over F_2
(deg f = 2g+1 monic, deg h = g, nonsingular), with a table of 16 random reduced
divisor classes `A_0..A_15`: a trial is

1. one Cantor addition `D <- D + A_r` (composition + reduction to deg u <= g),
   r drawn uniformly (a random walk over the class group, one group step per
   trial: the model's "divisor-class step"), then
2. one smoothness test of the Mumford polynomial `u(x)` (degree <= g over F_2)
   against the factor base of all irreducibles of degree <= b, by
   distinct-degree factorisation with the two standard early exits (stop when
   `deg(remaining) <= b`: smooth; stop when `deg(remaining) < 2(d+1)`: the
   remainder is irreducible, smooth iff its degree <= b).

Not charged to the trial (and listed, per section 5a, as outside the measured
unit): the factor-base membership bookkeeping and writing a relation row
(only for smooth hits, 1 in 1/P trials), the walk-table setup, and linear
algebra. Not a fresh-random-combination variant (that costs about k Cantor
additions per trial and is dearer; reported only as the Cantor-addition
component, so a reader can scale it).

## 3. Units

* **Counted unit, W (primary): executed F_2[x] 64-bit word operations**, one
  per u64 XOR-with-shift executed inside carry-less multiply, polynomial
  division, squaring (one per output word) and Euclid steps. The same counter
  and the same polynomial code price the reference EC addition, so the
  conversion is a ratio of identical units. Code is bit-serial shift-xor (no
  PCLMUL); a hardware carry-less multiply would shrink both numerator and
  denominator, so the ratio is a **counted-unit** statement, not wall time.
* **Native events, N_prim**: number of polynomial-primitive calls (mul, sqr,
  divmod, gcd/xgcd) per trial. Used for the section 5a band.
* **Wall ns per trial**: recorded as a practicality note only (never the
  metric; AGENTS.md section 6).
* **Curve-addition equivalent (CAE)**: `trial W / EC-add W`, with the EC
  addition being a point addition on a binary curve over F_2^131
  (modulus `z^131+z^8+z^3+z^2+1`, checked irreducible; the field of the
  note's section 8 comparison). Two denominators are measured: (i) affine
  addition (1 inversion by Euclid + 1 mul + 1 sqr + adds), (ii) an
  inversion-free Lopez-Dahab mixed addition priced as 8 mul + 5 sqr + 9 adds
  (**model** op count, **measured** mul/sqr word counts). **Headline = the
  smaller denominator** (the larger CAE, conservative against the margin).

## 4. Design

* Genera measured: g = 7, 15, 31, 63, 127, 255, 511 (all affordable; the
  group-law work at 255 is a few words per polynomial).
* Smoothness bound b per genus is the one the model picks: the minimiser of
  `price` over 2 <= b <= 200 on `#C(F_{2^k}) = 2^k+1`. `window.rs` needs
  BigUint; this study re-implements `price` in f64 log-space (all terms
  positive, values < 2^1023 for g <= 511) and **must** reproduce the note's
  `(g, b, log2 cost)` = (128, 17, 36.41), (255, 27, 55.60), (256, 27?, 55.65)
  to 0.01 bit before its b values are used; otherwise the run is void.
* Curves: for each g, a fresh `(h, f)` from the seeded PRNG, rejected until
  nonsingular (`gcd(h, h'^2 f + f'^2) = 1`).
* Random reduced divisors: sums of random degree-d place divisors (d <= 12,
  v solved from the Artin-Schreier equation in F_2[x]/p) by Cantor addition,
  until total degree >= 2g; the walk table and start point are built this way.
* Sample sizes: **4000 timed trials per g in 8 blocks of 500**; the reported
  per-trial mean and between-block standard deviation (counted W is
  deterministic given the seed; blocks show variation across inputs).
  Costs split per trial into Cantor-add part and smoothness-test part.
* Seeds: master 20261024; per-g seed = splitmix64(master xor g). Single
  thread, one job at a time.
* Smooth-rate check (model fidelity): g in {7, 11, 15, 23, 31} at the model's
  b, 200000 walk trials each; compare the observed smooth fraction with the
  model's `P(smooth)` and with the fraction of 200000 uniformly random monic
  degree-g polynomials passing the same test. Reported as agreement/ratio, not
  gated (a walk's u has deg u = g with probability near 1/2 for q = 2, so a
  ratio against the model is expected; it is reported with the deg-g fraction).

## 5. Correctness controls (all must pass or the measurement is void)

1. **Cantor outputs**: for 200 additions at every g (and all 16 table
   entries), `u` monic, `deg u <= g`, `deg v < deg u`, `u | v^2 + h v + f`.
2. **Group law**: on 100 random triples per g, associativity
   `(A+B)+C = A+(B+C)`, commutativity, `A + 0 = A`, and `A + (-A) = 0` with
   `-(u,v) = (u, v+h)`; plus doubling `A+A` against `[2]A` through a different
   addition chain (`(A+A)+B = A+(A+B)`).
3. **Smoothness test vs full factorisation**: >= 200 random squarefree
   polynomials (degrees 8..96, random b in 2..24) against distinct-degree plus
   equal-degree (Cantor-Zassenhaus, trace map) full factorisation, and >= 200
   random polynomials of degree <= 24 (repeated factors allowed) against
   brute-force trial division by every monic polynomial of degree <= 12.
   Zero disagreements required. Also report the number of smooth cases (so the
   control is not vacuous: at least 20 smooth and 20 non-smooth in each set).
4. **EC addition**: results lie on the curve; associativity on 50 triples; the
   F_2^131 modulus is irreducible by the same DDF test.
5. **Model port**: `price` re-implementation matches the three reference
   values above.

## 6. Preregistered predictions (before data)

* Cantor addition (composition + reduction) counted W grows as g^2 per word
  count: log-log slope in [1.8, 2.3] over g = 63..511 (below 2 at small g from
  word-granularity).
* Smoothness test is about b x (squaring-mod + gcd), each g^2/64-ish, and the
  model's b grows slowly (b ~ 17 at g = 128, 27 at 255), so W/trial slope in
  [2.2, 2.8] over g = 63..511; the test dominates the Cantor addition at
  g >= 63.
* EC addition on F_2^131 costs between 2^11 and 2^13 W (affine larger).
* Headline CAE per trial at g = 255 lies in [2^5, 2^9] (the note's 2^7 is not
  presupposed; the interval is a guess from the section 3 accounting).

## 7. Analysis

* Fit `log2 W = a + e log2 g` by least squares over g = 63..511 and over
  15..511; report both; extrapolate to 255 for comparison with the direct
  measurement (the fit is a **model**, 255 is **measured**).
* Recompute the N = 130 margin: relation term `rel + log2(CAE)` with the
  measured conversion replacing the note's 2^7; linear-algebra term kept at
  the note's unit (2 log2|FB| + log2 g, one modular multiplication = one
  operation: unconverted, stated as a caveat and reported with a sensitivity
  where it is converted by the measured modmul/EC-add ratio); total
  `max(.) + 1`; rho 2^60.31.
* **Section 5a band**: unpriced native work at weight w in {0, 0.1, 1} GAE per
  native event. IC trial = 1 divisor-class step + w x N_prim(trial); rho step =
  1 + w x n_rho with n_rho = 3 native events per step (**assumption**:
  canonicalisation, distinguished-point test, partition hash; this repository's
  rho counters on a Koblitz curve are not measured here; sensitivity
  n_rho in {0, 3, 10} is reported). The margin is "survives" only if it is
  positive at every w in the band; the pinned calibrated CAE point is shown
  as a further point and never replaces the band.
* Limits: bit-serial polynomial code; one trial definition; the random-
  polynomial model for P(smooth); rho and EC-add costs on F_2^131 are the
  note's reference, not a measured rho run.
