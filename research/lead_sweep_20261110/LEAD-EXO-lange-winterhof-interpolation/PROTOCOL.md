# LEAD-EXO-lange-winterhof-interpolation

## 1. Lead as recorded
- disposition: open_next_step; family: exotic; source: atlas_survey_20261107 --pursuit; title field empty in leads.json.
- next_step_excerpt (truncated): "Read the paper and record the exact theorem statement and scope as a KN-LIT entry (it is not in ..."
- Atlas survey: "paper not yet read; zero compute"; README: read the theorems (behind publisher logins), then decide whether they give a proof-kind closure for bounded-degree statistics and advice.
- Mechanism from own knowledge (unverified here): Lange-Winterhof (COCOON 2002) lower-bound the degree of polynomials over F_p that interpolate the DL of x-coordinates of EC points (and XTR analogues). Such a bound would rule out low-degree polynomial shortcuts, not index calculus.
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 70-74, 114, 188 only); README.md (grep hit, line 217).

## 2. Hypothesis
For prime-order toy curves E(F_p), p in 300..1100, the unique F_p-polynomial interpolating k -> k mod p on the M=(n-1)/2 distinct x-coordinates x(kG), k=1..M, has full degree M-1 (deficit 0), indistinguishable from a random relabelling of the same x-set. I.e. no toy-scale degree deficiency, consistent with a Lange-Winterhof-type "high degree" statement.

## 3. Preregistered prediction and falsification criterion
- Statistic: deficit = (M-1) - deg(interpolant), per curve.
- Prediction: deficit_real = 0 on all curves (a random labelling has deficit >=1 with probability about 1/p).
- Falsified: any real deficit >= 2, or more than 1 + floor(#curves/4) curves with deficit >= 1 (a systematic low-degree structure).
- Consistent: neither of the above, with the positive control passing.
- Inconclusive: positive control fails (planted degree-5 polynomial labels not recovered with degree exactly 5) or no curves produced.
- Negative control / null: random permutation of the labels over the same x-set. A/A arm: a second independent random relabelling (same instrument, own draw); null and A/A deficits are both reported and should be 0.
- Positive control: labels f(x)=3x^5+7x^2+1; the instrument must return degree 5.
- Decides: only whether the full-interpolant degree statistic shows toy structure. Consistent adds no new information beyond a trivial bound (see section 6).

## 4. Method
Seed 20261110 (Python random.Random). For about 12 primes p (every 25th prime in 300..1100): random (a,b) until #E(F_p)=n is prime (brute-force Legendre count), pick a point G, compute x(kG) for k=1..M. Degree via Newton divided differences mod p, O(M^2) with modular inverses; four interpolations per curve (real, null, A/A, planted). Cost estimate: M<=550, about 0.15M pow() calls per interpolation, well under 2 minutes, RAM few MB, stdlib only.

## 5. Classification
runnable_local. A stdlib toy decides the stated falsifier. The literature step (record the theorem as KN-LIT) remains desk work for phase 2 and is not replaced by this run.

## 6. What a run cannot establish
- It does not read or verify the Lange-Winterhof theorem statement or scope; no paper text was available.
- Full-interpolant degree is near-trivially M-1 for any generic labelling, so a pass is weak evidence. The theorem concerns degree bounds for polynomials agreeing on large subsets, which this does not test.
- Toy sizes (p <= 1100) say nothing about cryptographic sizes or about index-calculus cost. No gap to rho is measured.
- Labels are taken mod p (k < p); the paper's value-set conventions may differ.
- Results are labelled [pcc:unverified].
