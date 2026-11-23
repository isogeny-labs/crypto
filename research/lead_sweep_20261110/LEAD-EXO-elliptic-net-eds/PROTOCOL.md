# LEAD-EXO-elliptic-net-eds

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step (disposition_20261101: closed_obstructed); family: exotic; last_reviewed 2026-10-24.
- title: Elliptic nets / elliptic divisibility sequences as a relation or annihilator source.
- next-step excerpt (truncated): "Someone supplies a target-specific NONLINEAR locator outside the closed Hankel/translated-block ..."
- Recorded obstruction (atlas_survey.md, lines 74, 155): charged k-recovery exponent 1.97-2.80 vs BSGS 1.17-1.32; sieve/BSGS ratio 24.2 -> 134.7 for p 101 -> 1601; class "GGM-simulable oracles".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context only, lines 72-74, 153-172).
  Not read: ecdlp_subexp_desk_20261101/data/*.json (tallies). The lead's own atlas file is encrypted here.

## 2. Hypothesis
For an elliptic net / EDS built from (P, Q=kP) on E(F_p), the only available "locator" is the vanishing test W(a,b)=0 iff a+bk=0 mod ord(P), which is equivalent to one group-law zero test. Hence a net sieve over a (b=1) costs Theta(N) charged group operations, with no sub-linear structure: its log-log cost slope in N exceeds that of BSGS (about 0.5) by at least 0.3, and the sieve/BSGS ratio grows monotonically with N. (This re-derives the recorded obstruction in a different unit: exponent in N rather than p.)

## 3. Preregistered prediction and falsification criterion
- Statistic: slope_gap = slope_sieve - slope_bsgs, least-squares slope of log(charged cost) vs log(N) over 4 sizes (charge: add = 1 inversion (10 mult units) + 3 mults = 13).
- Consistent with prediction: slope_gap >= 0.3 in BOTH seed arms, ratio sieve/BSGS strictly increasing in N, all recoveries correct, A/A difference of slope_gap <= 0.25.
- Falsified: slope_gap < 0.15 in the primary arm (the sieve would then be sub-linear relative to BSGS, i.e. a real locator).
- Otherwise, or any control failing: inconclusive.
- Positive control: every target k is recovered exactly by both sieve and BSGS; the EDS W_n(P) from the division-polynomial recurrence has first zero exactly at ord(P) (rank of apparition = group-law order) for orders up to 4000 (larger sizes: no zero in the first 4000 terms is accepted).
- Null/negative control: random-order enumeration of the k-space (probe count uniform on 1..N); sieve probe count / null probe count should be about 1 (reported; no extra structure shortcut). Sieve probe counts are the same statistic as the null by construction, so this checks bookkeeping rather than power.
- A/A arm: same procedure, disjoint seed (2002 vs 1001, different curves and targets); noise floor is the slope_gap difference.

## 4. Method
Python 3 stdlib only. Primes near 2^12, 2^14, 2^16, 2^18 (each plus a seeded offset < p/8); random curve y^2=x^3+Ax+B; N0 by Legendre-sum point count; random P; ord(P) by factoring N0 trial division. 6 random targets k per size. Sieve: R=Q, add P until R=O (k = -a mod ord). BSGS: table of j*P, giants Q - i*m*P. Seeds 1001 (primary) and 2002 (A/A). Estimated wall: under 2 minutes, a few MB RAM. `--smoke`: primes 400/800/1600, 3 targets.

## 5. Classification
runnable_local: a stdlib toy decides the slope question directly.

## 6. What a run cannot establish
- It tests only the GGM-simulable zero-test locator, not a "target-specific nonlinear locator" supplied from outside; it cannot show none exists.
- Group order <= ~2^18 only; exponents are fits over 4 points, not asymptotics, and differ in unit from the recorded p-exponents (1.97-2.80).
- The 2-D net recurrence of Stange is not implemented; the 1-D EDS is only the correctness control.
- All results are [pcc:unverified].
