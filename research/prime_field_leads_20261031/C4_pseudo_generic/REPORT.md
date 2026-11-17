# C4: the interval predicate is pseudo-generic (conjecture plus toy energy measurement)

Lead C4 of `RESEARCH_PRIME_FIELD_LEADS_20261031.md`; thresholds frozen in `../PLAN.md`.
Counted metric: curve additions (72,367,912 in total). No wall-time claim.

## 1. What this is not

The lab card `LEAD-EXO-dl-character-sum-pseudorandom` (H-PSEUDO, `exhausted_at_scale`) measured the
additive-character sum of the **discrete log** over the small-x base, `sum_{P in F} e(2 pi i k DL(P)/N)`,
against a matched null (ratio flat at 1.28-1.34 over 2^20..2^26). C4 measures a different object: the
additive **energy of the base itself inside the group**,

    E(F) = #{(P1,P2,P3,P4) in F^4 : P1 + P2 = P3 + P4}    (ordered, trivial solutions included),

which needs no discrete log (it is computed by hashing the m^2 pair sums). The two are related through
Fourier duality over the cyclic group (E(F) = (1/N) sum_k |hat 1_F(k)|^4), so a flat character sum bounds
energy from above, but the card's measured constant 1.28-1.34 is a constant factor in the *sup norm*
and says nothing about the 4th moment at the 1 + o(1) level. C4 asks the 4th-moment question directly.

## 2. Conjecture (precise form)

Let E/F_p have prime order N (cyclic group). For B in [1,p] let F_B = {P in E(F_p) \ {O} : x(P) < B}
(closed under negation). Let R_m be a uniformly random **negation-closed** subset of E(F_p)\{O} with |R_m| = |F_B| = m.

**Conjecture (pseudo-genericity of the interval predicate).** For every fixed delta > 0, uniformly over
curves E over F_p, B with m = |F_B| >= N^delta (and m <= N^{1-delta}),

    E(F_B) / E[E(R_m)] = 1 + o(1)    as p -> infinity,

with E[E(R_m)] = 3m^2 + m^4/N (1 + o(1)) + O(m)  (trivial classes P1=P3,P2=P4 ; P1=P4,P2=P3 ; P1=-P2,P3=-P4,
plus the generic term m^4/N).

Remark on the null. Against a *plain* uniform random subset the ratio is not 1 even in theory: negation
closure adds the third trivial class, so the plain-null ratio tends to (3m^2 + m^4/N)/(2m^2 + m^4/N), i.e.
4/3 at m = sqrt(N) and 3/2 at m = N^{1/3}. This is exact bookkeeping and does not depend on x < B. The
conjecture is therefore stated against the negation-closed null (design choice made before running; the
plain-null ratio is also recorded below).

**Why it would be the missing yield lemma of L1.** A sieve-style relation shortcut on the interval base needs
the base to have more additive coincidences than a random set of its density (excess energy = excess
relations among m^2 pair sums, i.e. super-random decomposition yield; compare the Betti-yield duality
KN-FIND-9d2f56 cited by the H-PSEUDO card). The conjecture says the coincidence count of F_B equals the
random one to 1 + o(1); by the density/self-reducibility ceiling this would leave the sieve with exactly the
generic yield L1 prices. It closes the predicate-level shortcut only for energy (4th moment); higher
moments (8-fold coincidences) and non-additive algebraic structure (Semaev summation polynomials) are not
covered. Nothing here proves the conjecture.

## 3. Literature (named with the confidence stated)

- Shparlinski and coauthors have a body of work on the distribution and exponential sums of points of
  elliptic curves, including points with coordinates in short intervals/boxes, and on sums of x-coordinates
  (e.g. Kohel-Shparlinski, "On exponential sums and group generators for elliptic curves over finite fields",
  ANTS 2000 - confident of that title; the box-distribution papers I describe but do not cite by title, to
  avoid fabrication). These give *equidistribution* of x in intervals (counts of points per box to
  p^{1/2+o(1)} error) - a first-moment statement. They do not, to my knowledge, give the 4th-moment /
  energy statement here at m ~ N^{1/2}.
- Additive-combinatorial background: additive energy, Balog-Szemeredi-Gowers; sum-product style results in
  elliptic-curve groups. Only used as the framework; no specific theorem is invoked.
- The H-PSEUDO card (KN-FIND-e7a3b1) records that six standard analytic proof routes to the related
  character-sum statement are closed; C4 inherits that obstruction for a proof and offers only evidence.

## 4. Toy measurement

Program `energy.rs` (standalone, `rustc -O`, own splitmix RNG, affine curve addition over u64; no crate
changes). For k = 12..18, p = largest prime below 2^k (7 sizes); 8 random Weierstrass curves per p with prime
non-anomalous order N (candidates with N composite or N = p are rejected; tries per p: 96, 123, 108, 152, 88, 67, 183). Per curve and
size m in {round(sqrt N), round(N^{1/3})} (rounded up to even so F_B is negation-closed): F_B = first m points
ordered by x; E(F_B) exact by hashing/sorting all m^2 pair sums; 8 uniform random m-subsets and 8 random
negation-closed m-subsets on the same curve. Reported: ratio of E(F_B) to the mean of the 8 negation-closed
nulls (primary) and to the mean of the 8 plain-uniform nulls (frozen-text version). Sizes reach m = 512,
N ~ 2.6e5. Files: `data_per_curve.csv` (every curve, every cell; no cell failed or timed out),
`data_summary.csv`, `graph_energy_ratio.svg/.pdf`, `slope_check.txt`.

Observed (mean over 8 curves, sd over curves in parentheses):

| log2 p | sqrt N: ratio vs closed null | cbrt N: ratio vs closed null | sqrt N: vs plain | cbrt N: vs plain |
|---|---|---|---|---|
| 12 | 1.0038 (0.0178) | 0.9943 (0.0255) | 1.317 | 1.436 |
| 13 | 1.0014 (0.0173) | 1.0126 (0.0176) | 1.330 | 1.471 |
| 14 | 0.9927 (0.0080) | 1.0012 (0.0144) | 1.317 | 1.461 |
| 15 | 0.9973 (0.0119) | 1.0032 (0.0112) | 1.328 | 1.468 |
| 16 | 0.9999 (0.0053) | 1.0052 (0.0091) | 1.331 | 1.481 |
| 17 | 0.9995 (0.0045) | 1.0011 (0.0065) | 1.330 | 1.482 |
| 18 | 1.0023 (0.0029) | 1.0002 (0.0058) | 1.335 | 1.483 |

- Largest |mean ratio - 1| is 0.0073 (sqrt N, p=2^14) and 0.0126 (cbrt N, p=2^13), each below 1 sd over curves
  (0.91 and 0.71 of the curve sd at their worst point); least-squares slope of the mean ratio against log2 p:
  -3.8e-5 (sqrt N) and -4.9e-5 (cbrt N) per bit, i.e. no growth.
- Within-curve relative sd of the null energies falls from 0.024 to 0.004 across the range
  (`null_rel_sd_within_curve`), the same scaling as the spread of the ratio over curves.
- Versus the plain null the ratio sits at 1.32-1.34 (sqrt N) and 1.44-1.48 (cbrt N), approaching the exact
  4/3 and 3/2 predicted for negation closure alone (Section 2). It does not grow beyond that.
- Binomial/Poisson reference: the generic term lambda = m^4/N is about 262,000 at sqrt N, p~2^18 (about 25% of E) and about 64 at cbrt N (about 0.5% of E). The observed within-curve relative sd of the 8 negation-closed nulls at p~2^18 is 0.0038 (`null_rel_sd_within_curve`); no separate Poisson-variance computation was made (pair sums are clustered, so Poisson would understate it).

## 5. Decision (frozen rule)

A ratio departing from 1 by more than the spread AND growing with p: not observed against the matched
(negation-closed) null, in either size. Against the plain null the departure is large but constant and is the
exact negation-closure effect. Decision: **null_confirmed** at toy scale.

## 6. Limits

- At m = N^{1/3} the generic term m^4/N (about 64 at N~2^18) is dwarfed by the trivial 3m^2 (about 12,000);
  the cbrt N energy ratio is dominated by trivial classes and is nearly blind to non-trivial structure. The
  separate "non-trivial part" ratio was undefined (negative or noise-level after subtracting) there, hence NaN
  in `data_summary.csv`. Only the sqrt N column carries real power, and even there the signal is the
  m^4/N term, about 25% of E at m = sqrt N.
- Range 2^12..2^18 only; the card's own revisit condition (growth beyond 2^30) is not touched.
- The test sees the 4th moment only. Structure in the Semaev polynomial system or higher-order
  coincidences is not measured.
- The closure formula 3m^2 - 2m for trivial solutions is approximate (overlaps); the primary ratio does not
  use it, only the diagnostic non-trivial column does.

## Reading

Energy of the interval base matches a random negation-closed set of the same size to within 1% at every
tested size and shows no trend in p, which is what the conjecture predicts; this is evidence consistent with
the conjecture at toy scale, not a proof, and says nothing about higher moments or algebraic structure.
