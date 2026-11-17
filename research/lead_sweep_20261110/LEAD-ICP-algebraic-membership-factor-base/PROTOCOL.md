# LEAD-ICP-algebraic-membership-factor-base

## 1. Lead as recorded
- disposition: open_next_step (disposition_20261101: closed_obstructed); family ic-prime; last_reviewed 2026-10-29.
- title: Factor bases with low-degree algebraic membership over F_p (subgroup, coset, Dickson, j=0 orbit, images of low-degree maps, character-constant sets).
- next-step excerpt (truncated in leads.json): "KN-OPEN-020 is answered positively (a size-B set with sub-B-degree membership surviving substitu..."; cost: est. cheap-medium (theory/measurement).
- why it might beat sqrt(p): a size-B base with sub-B-degree membership surviving substitution would break the conservation-based no-go; all measured geometries give yield ratio 1.000000.
- Recorded obstruction (survey): mean yield binomial(B+m-1,m)/N for every size-B base (KN-FIND-007); EV-FBG-001: 288 cells within 0.0003 bits of random at m=2.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 16, 139, 229); /home/user/crypto/research/prime_field_lead_assessment_20261107/README.md (grep of KN-OPEN-020 lines only). Full atlas YAML and KN-FIND-007 text not read.

## 2. Hypothesis
At m=2 on cyclic toy elliptic-curve groups (N ~ 2^11..2^17), a size-B base (B = 1.5 sqrt(N), x-coordinates +-P merged) selected by x in an algebraic locus (QR/character-constant, cubes, Dickson D_3 image, image of t^2, image of t^2+t, lowest-x interval) has the same coverage of Z_N by {+-f_i +- f_j} as a size-matched uniformly random base: pooled mean coverage ratio < 1.02 for every family. (Mean yield is fixed by conservation; coverage/variance is the quantity the recorded "reopen" condition names.)

## 3. Preregistered prediction and falsification criterion
- Statistic: per cell, ratio = coverage(family)/mean coverage of 60 random size-B bases, and z = (coverage - mean)/sd of the random bases; pooled per family across cells (mean ratio; Stouffer z = sum z / sqrt(cells)).
- Prediction: for all six families, mean ratio < 1.02 or Stouffer z <= 5.
- falsified: any family with mean ratio >= 1.02 AND Stouffer z > 5 (a coverage gain surviving the pooled test), provided controls pass. That would be evidence for a structural excess and justify a degree-of-membership study.
- consistent_with_prediction: no such family and controls pass.
- inconclusive: controls fail.
- Null / negative control: the size-matched random bases themselves (reference distribution).
- A/A arm: a fresh random base of size B scored against the same reference; pooled |Stouffer z| must be <= 4.
- Positive control: a planted subgroup of Z_N (multiples of a divisor d with N/d <= B), which must show ratio < 0.8 in every cell where such d exists (at least one cell required). Shows the detector sees structure.
- Smoke mode always reports inconclusive (single tiny cell).

## 4. Method
- Stdlib Python. For each prime p in {2053-ish, 8209-ish, 32771-ish, 131101-ish} (first prime >= 2^11, 2^13, 2^15, 2^17) and seeds {20261110, 20261111, 20261112} (derived seed = s*1000003 + p): draw random short Weierstrass curve y^2=x^3+ax+b until a random point has full order N = #E (cyclic group); build x(kG) table by repeated addition.
- B = int(1.5 sqrt(N)). Family base = first B x-values (ascending x) satisfying the locus predicate, mapped to discrete-log index k in 1..N/2.
- coverage(F) = |{+-f_i +- f_j, i<=j}| / N. Reference: 60 random B-subsets of 1..N/2; A/A: 1 more random subset per cell.
- Cost: dominated by p=2^17 cells (point enumeration plus order walk, about 1e5 affine additions) and ~120 coverage evaluations of ~1.2e5 sums per cell; estimate 2-6 minutes total, <100 MB. A wall guard of 420 s stops starting new cells.
- Output: one JSON object with pooled stats, controls, cells and verdict.

## 5. Classification
runnable_local. Pure counting on toy cyclic groups; no solver, GPU or records needed. It tests only the coverage version of the recorded obstruction (KN-OPEN-020 itself is about solving degree and is not run).

## 6. What a run cannot establish
- Nothing about KN-OPEN-020 proper (sub-B-degree membership surviving substitution) or Groebner solving degree; only coverage at m=2, B ~ sqrt(N), N <= 2^17.
- Not a statement about cryptographic sizes, m>=3, or complete-DLP cost against rho (no section 5a pricing).
- Bases are x-coordinate loci chosen by ascending x; other selections, j=0 orbits and true subgroup/coset bases are only covered by the positive control.
- A null result repeats EV-FBG-001-type evidence at toy scale; it cannot rule out coverage excess growing with N.
- Results are [pcc:unverified]; no replay certificate or validator report.
