# LEAD-STR-safecurves-parameter-audit

## 1. Lead as recorded
- disposition: closed_known (last_reviewed 2026-10-29)
- family: structure
- title: SafeCurves-style parameter audits (twist security, completeness, ladder, cofactor, transfer rows)
- next-step excerpt (truncated in leads.json): "Revisit when: A deployed row is found to violate a criterion that bears on ECDLP cost (rho size,"
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 52-58, 322-328 region via grep: lead listed closed_known, sibling TRF-curve-parameter-transfer-audits closed_known).
  /home/user/crypto/docs/known-label-next-leads-20261028/atlas-inputs.json did not exist at that path; no cryptanalysis hits read.
  [pcc:unverified] for everything below.

## 2. Hypothesis
The recorded obstruction is that the SafeCurves criteria split into (a) criteria that bear on ECDLP cost (group order size, trace != 1, embedding degree, CM discriminant) and (b) criteria that bear only on implementation or twist/ladder safety. Claim tested: twist security (largest prime factor of the twist order) is class (b): for random prime-order prime-field curves, generic rho cost on E(F_p), measured as log(steps / sqrt(N)), is the same for twist-insecure and twist-secure curves, within a log-scale margin of 0.2.

## 3. Preregistered prediction and falsification criterion
- Statistic: D = mean log(steps/sqrt N) over twist-insecure curves minus the same over twist-secure curves; 95% bootstrap CI (1000 resamples, seed 20261111).
- Threshold: margin 0.2 (log scale, about 22%), equivalence (TOST-style) if CI inside (-0.2, 0.2).
- Outcomes: consistent_with_prediction if CI inside margin AND A/A and positive control pass. falsified if CI excludes 0, |D| > 0.1, A/A and positive control pass (would mean twist class correlates with rho cost, contradicting the closed disposition). Otherwise inconclusive (including too few insecure curves, fewer than 3 per arm).
- Classes: secure = largest prime factor of twist order * 4 >= twist order; insecure = largest prime factor * 64 <= twist order; middle band discarded.
- Negative control / null: the twist classes themselves (null is no effect); A/A arm: random split of the secure arm in halves, must also have CI inside margin.
- Positive control: a size arm with p in [2^12, 2^15) versus the main arm p in [2^14, 2^17) with the same instrument; raw log(steps) difference must have 95% CI above 0 (expected about 0.5*ln(N ratio), reported). Shows the detector sees a real cost change.
- Side tally (descriptive only): count of anomalous (trace 1) curves among prime-order draws.

## 4. Method
- Python 3 stdlib only, one core. Seed 20261110 (main RNG), 20261111 (bootstrap). Deterministic.
- Draw prime p = 3 mod 4 in [2^14, 2^17); random (a, b); count points by Legendre scan with a squares set; keep curves with prime order N; twist order 2p+2-N factored by trial division; classify.
- One discrete-log instance per curve (random P of prime order N, Q = kP), solved by Floyd rho with a 16-point r-adding walk, restarts on the degenerate case, answer verified (kP == Q). Steps counted as 3 per Floyd iteration.
- Up to 600 prime-order curves in the main arm (time-capped at 384 s), then 150 in the small-size positive-control arm (capped at 470 s total budget). Expected runtime about 6-8 minutes, memory under 50 MB.
- `python3 -I run.py --smoke` uses p in [2^10, 2^11) and n = 6 (under 1 s).

## 5. Classification
runnable_local. A stdlib toy can measure whether twist-security class changes rho cost; closing evidence is a cheap re-derivation (expected null, as rho cost depends only on N for generic groups).

## 6. What a run cannot establish
- Nothing about deployed 256-bit curves or any real deployed row; toy orders 2^14..2^17 only.
- Does not test completeness, ladder, rigidity or cofactor criteria, nor index-calculus or transfer attacks; says nothing about beating rho.
- Twist insecurity matters for invalid-curve/twist attacks, outside the ECDLP-on-E model here.
- Insecure class may be sparse at toy size, risking inconclusive. Equivalence is within 22% only. All results are [pcc:unverified].
