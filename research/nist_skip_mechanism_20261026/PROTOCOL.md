# Holdout test of the skipped-curve criteria on SEC 2 seeded prime curves
Frozen 2026-10-26 before any rule value was computed on these curves.

Rules (from research/nist_skip_criterion_20261025, fitted on 21 NIST/X9.62 curves):
  R2: M mod 8 in {5,7} and (b+1) mod 19 <= 13          (M = 2p+2-#E)
  R3: M mod 8 in {5,7} and t mod 7 != 0 and t mod 13 != 6   (t = p+1-#E)

Holdout set (none in the fitted 21): SEC 2 prime curves that carry a seed:
  secp112r1, secp112r2, secp128r1, secp128r2, secp160r1, secp160r2.
Primary subset = same generation method as the fitted family: a = -3, cofactor 1,
and the seed passes the ANSI X9.62 SHA-1 check r*b^2 == a^3 (mod p).
Positive control for the seed check: P-192, P-224, P-256, prime239v1 must pass it.

Prediction H1 (the rule is the historical acceptance criterion and the same
generator made these curves): every primary curve is accepted by R2 and by R3.
Any primary curve rejected falsifies H1 for that rule.
Null: per-curve acceptance probability measured on random curves y^2=x^3-3x+b over
the same field with prime order (PARI ellcard), N=2000 per field, seeded; joint
null p = product over primary curves. Evidence counts only if H1 holds.
Descriptive only, no prediction: non-primary SEC 2 curves, Brainpool r1 curves.

## Result of the holdout (recorded 2026-10-26, after computing)
Primary set = secp112r1, secp128r1, secp160r1, secp160r2 (a=-3, h=1, seed check passes;
controls P-192/224/256/384/521, prime192v1-3, prime239v1-3 all pass the seed check).
R2 and R3 reject all four. H1 is falsified for both rules. Null sampling not run:
the protocol counts evidence only if H1 holds.

## Addendum A, frozen 2026-10-26 before computing: mechanism hypotheses on the 21 labelled curves
H2 (point-counting cost): the generator abandoned candidates whose SEA count was expensive.
  Statistic: L_E = smallest bound B such that the product of Elkies primes 3<=l<=B
  (Legendre((t^2-4p)/l) != -1) exceeds 4*sqrt(p). Prediction: skipped curves have larger
  within-stream rank of L_E. Test: sum of within-stream ranks of L_E over skipped curves,
  one-sided, against all 4,480 count-preserving label assignments.
H4 (order below p): published curves have n < p (t > 0). Same permutation test on 1[t<0].
Two hypotheses: report Bonferroni alpha = 0.025 each.

## Exclusion (2026-10-26, on the owner's instruction, after the holdout result)
The SEC 2 holdout curves are excluded: they come from SECG/Certicom, not NSA/NIST.
Three of the four primary seeds embed the ASCII string "MinghuaQu" (secp160r1 is
nibble-shifted). The X9.62 SHA-1 check is public, so passing it does not identify
the generator. The holdout therefore says nothing about the NSA generator.

## Addendum A results (Python, exploratory; not landable under the no-Python rule)
H2 (skipped have larger L_E): P(>=obs) = 0.9998, falsified. Reverse tail P(<=obs) = 0.00112.
H4 (t<0): P = 0.50, null.
Post hoc: Gekeler-model null percentiles of L_E: published mean ~75 (P-384 98.5, P-521 72.9),
skipped mean ~40. Decomposition (l<200): primes dividing the twist order, P = 0.41/0.62;
other Elkies primes, P(>=) = 0.032; twist 2^16-smooth part, P = 0.50.

## Prior work found after Addendum A (recorded 2026-10-26, before Addendum B)
`research/nist_seed_provenance_20261016/PREREGISTRATION.md` H1 already preregistered
"Elkies coverage", including the smallest L at which the Elkies product exceeds 4*sqrt(p)
(= L_E here). Its result: "slight lean to fewer Elkies primes among published (best
p = 0.008, Bonferroni bound 0.004)", under a null of any 10 of the 21 curves published.
Addendum A's L_E result is therefore the same signal on the same 21 curves under a
within-stream null, not independent evidence. The atlas lead
LEAD-STR-curve-generation-provenance-seeds (reviewed 2026-10-24) is exhausted_at_scale.

## Addendum B, frozen 2026-10-26 before computing (GP, native)
B0 Port: recompute Addendum A natively in GP (L_E, within-stream exact permutation over
   the 4,480 count-preserving assignments, Gekeler-model null percentiles) and check
   agreement with the Python values recorded above.
B1 Same-organization check on the NSA binary curves B-233, B-283, B-409, B-571 (seeds
   explained as lrand48 runs; each was the first accepted candidate, so no skip
   occurred). B-163 is reported separately (seed unexplained). Statistic: mean
   Gekeler-null percentile of L_E (q = 2^m, #E = 2n). Prediction if an isogeny-structure
   filter ran in the binary pipeline as well: mean > 50. One-sided p from the exact
   Irwin-Hall distribution of a mean of 4 uniforms. Under "no filter in the binary
   pipeline" the mean is uniform, so a null result cannot separate "no filter" from
   "filter only in the prime pipeline"; it is reported as such.
B2 Elkies-failure mechanism (predicts the observed direction): the historical SEA crashed
   or gave up when some prime l in a fixed set S was Elkies (kronecker(t^2-4p, l) != -1).
   Prediction: there is a set S of odd primes l <= 100 such that every published curve
   (all 10, including P-384 and P-521) is Atkin at every l in S and every skipped curve is
   Elkies at some l in S. Take S_max = {l: all 10 published are Atkin at l}; the rule
   holds iff S_max covers all 11 skipped. Calibration: the fraction of the 4,480
   assignments (P-384 and P-521 fixed as published) for which the same construction
   covers. The rule falsifies if S_max fails to cover any skipped curve.
