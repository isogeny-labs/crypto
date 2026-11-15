# LEAD-EXO-invertible-hint-subset

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24); family: exotic; source: atlas_survey_20261107 --pursuit.
- Title: "The invertible half: dense, cheaply recognizable AND cheaply invertible subsets of E(F_p)".
- Next-step excerpt (truncated in leads.json): "First obtain and read the theorem statements of Lange-Winterhof (COCOON 2002, LNCS 2387, pp. 137..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 72-102 region, grep context); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (grep context, rows on this lead: low-degree variant measures maximal agreement A(d) of degree-d polynomials with the log graph; paper statements "not retrieved: unverified pointer"). The paper text itself is not available here.

## 2. Hypothesis
Over toy prime-order curves E(F_p) (p ~ 800-1700, N prime, N != p), define the half-log graph S = {(x(kG), k) : 1 <= k <= (N-1)/2} and A(d) = max over polynomials f in F_p[x], deg f <= d, of #{(x,k) in S : f(x) = k}. Claim H: for d in {1,2,3}, the agreement that degree-d polynomials interpolated through d+1 random points of S achieve is statistically indistinguishable from the same x-set with labels replaced by a uniformly random bijection (null), i.e. the log graph has no low-degree dense subset beyond the trivial d+1 interpolated points (consistent with Lange-Winterhof-type lower bounds on interpolation degree; the lower-bound statements themselves are unverified here).

## 3. Preregistered prediction and falsification criterion
- Statistic: per degree d, z = (mean excess agreement EC - mean excess agreement NULL) / pooled s.e., where excess = agreement - (d+1) for a RANSAC trial (d+1 random points, Lagrange interpolation mod p, count matches over all n = (N-1)/2 points); EC pooled over 3 curves, NULL pooled over 3 matched nulls, T = 2500 trials per arm.
- Prediction: z <= 4 for every d in {1,2,3}. Falsified (a detectable excess exists at toy scale) if z > 4 for any d, and only if both controls below pass.
- Negative control / null: same x-set, labels shuffled by a fixed-seed random bijection (3 of them, one per curve).
- A/A arm: a second independent shuffle of curve 0; |z_AA| must be <= 4 for every d, else the instrument is noisy and the verdict is inconclusive.
- Positive control: on curve 0, a random degree-d polynomial planted on 25% of the labels; RANSAC max agreement must reach >= 0.15 n for every d, else inconclusive (detector cannot see a planted dense subset).
- Verdicts: consistent_with_prediction (H not rejected at this scale, no excess seen); falsified (low-degree dense structure in the EC log graph); inconclusive (control failure).

## 4. Method
Pure stdlib python (run.py). Curves y^2=x^3+ax+b found by point counting for primes p >= 800, 1100, 1400 (seeds 20261110+i), requiring N prime, N != p (non-anomalous). Points kG by affine addition, k=1..(N-1)/2, one representative per +-P pair so x values are distinct. RANSAC per arm/degree as in section 3, with fixed per-arm seeds. Cost: 7 arms x 3 degrees x 2500 trials x n ~ 400-850 evaluations of degree<=3 polynomials, plus 3 positive-control runs; estimated 2-6 minutes on one core, < 50 MB RAM. Smoke mode (--smoke): p~61, T=60, d in {1,2}, < 1 s.
Run: `cd /home/user/crypto/research/lead_sweep_20261110/LEAD-EXO-invertible-hint-subset && timeout 660 python3 -I run.py`.

## 5. Classification: runnable_local
A stdlib RANSAC toy decides or sharpens the A(d) question at group orders ~2^10-2^11 without compute beyond one core. (Groups of order 2^12+ would also fit but the controls already fix the sensitivity at this size; the paper-reading step stays a separate desk item.)

## 6. What a run cannot establish
- It does not read or verify any Lange-Winterhof theorem; no literature closure is claimed.
- RANSAC finds only agreement reachable by random (d+1)-subsets; a dense subset of fraction q is found with probability ~q^(d+1) per trial, so sets below ~15% density at d=3 are invisible (positive control quantifies only 25% planting).
- Toy size (p ~ 10^3), degrees <= 3 and the half-log labelling (one fixed representative per +-P) say nothing about asymptotic A(d), higher degrees, or other cheaply recognisable/invertible subsets (the lead's actual "invertible half"), which this run does not address.
- A null result is not a cost or speedup claim and says nothing about the gap to rho. All results are labelled [pcc:unverified].
