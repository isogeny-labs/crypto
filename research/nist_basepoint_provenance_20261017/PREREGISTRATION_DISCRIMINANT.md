# Preregistration: CM-discriminant screen of the skip filter

Written 2026-10-17 before any value below was computed. Self-dated; its
SHA-256 is recorded in the session transcript before the script runs.

Population: the 21 accepted candidates in census/summary.json. Only the 19 in
runs with skips (p239, p192, p256, p224) can discriminate; P-384 and P-521 are
reported but not scored.

For each candidate: t = p + 1 - N0, D = t^2 - 4p (D < 0, odd, so the
discriminant is fundamental iff D is squarefree). Trial-divide |D| by all
primes below 2^24, then test the cofactor for being a perfect square.
Conductor proxy f = product of l^(floor(e/2)) over the small primes found.

Hypotheses, scored exactly:
  H1 "published iff D squarefree": supported only if every published candidate
     has f = 1 and every skipped candidate has f > 1.
  H2 "published iff f below some bound B": supported only if a threshold on f
     separates the two groups perfectly.
Anything short of perfect separation is reported as not supporting the rule,
with the counts. Expectation under no rule: about 19% of random odd D have a
square factor (1 - 8/pi^2), so roughly 3-4 of 19 candidates.
Positive control: the classifier is run on synthetic D = q^2 * r (q = 3, 101,
65537) and on D = r squarefree, and must label them correctly.
