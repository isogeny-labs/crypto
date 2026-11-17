# LEAD-STR-isogeny-class-generic-invariance

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (disposition_20261101 same); family: structure; last_reviewed 2026-10-29.
- Title: "Generic (rho, BSGS) ECDLP cost across an isogeny class and across volcano levels".
- Next-step excerpt: empty in leads.json (no next step recorded; lead is closed).
- Recorded obstruction (survey): theorem plus a 403,168-run census, 33/35 level ratios TOST-equivalent, floor/crater rho ratio 1.004 [0.990,1.016]. Generic cost is an isogeny-class invariant (same #E).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 54); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (line 60); research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (id/title only). The census evidence itself (EV-ENDO-10109d) was not available here.

## 2. Hypothesis
For a fixed prime p and a fixed trace t, all F_p-curves with prime order N = p+1-t (one isogeny class, distinct j-invariants) have the same expected generic Pollard-rho cost: the mean number of r-adding-walk steps to a verified discrete log differs between any curve and the pooled rest of the class by a ratio inside [1/1.08, 1.08]. Cost depends on N only, not on the curve model.

## 3. Preregistered prediction and falsification criterion
- Statistic: for each class curve i, ratio_i = mean steps(curve i) / mean steps(other class curves pooled), delta-method CI.
- Consistent: every class curve has its 90% CI inside [1/1.08, 1.08] (TOST), AND the A/A arm is equivalent, AND the positive control is detected.
- Falsified: positive control and A/A valid, and some class curve's 99% CI lies wholly outside the band.
- Inconclusive: anything else (including a failed positive control or A/A).
- Null / negative control: the class itself is the null (same N, so the predicted ratio is 1).
- Positive control: one curve over a larger prime p2 = nextprime(2.25p) with prime order, whose expected ratio is about sqrt(2.25) = 1.5; must be detected (99% CI lower bound > 1.08) versus class curve 0. This shows the instrument sees a real cost difference.
- A/A arm: two independent seed streams on class curve 0; ratio must be TOST-equivalent, giving the noise floor.

## 4. Method
- Python 3 stdlib only. p = nextprime(2^14) = 16411. Choose the smallest |t| with p+1-t prime, then collect NCURVES = 8 curves y^2=x^3+ax+b with random (a,b) (seed 20261110), exact Legendre point count equal to N, distinct j.
- Rho: 16-adding walk (partition by x mod 16), random start, dictionary collision detection (steps counted until the first collision that yields a log verified by kk*P == Q); a collision with equal b or at infinity restarts, and steps accumulate. Random P, k per run.
- RUNS = 1500 per curve; per-curve seeds 20261110*1000+i; positive control seed +99; A/A seeds +77, +78.
- Cost estimate: about 17k rho runs of about 160 steps each, plus class search (up to 4000 point counts of 16k terms); roughly 1-2 minutes, below 200 MB, well under 10 minutes. `--smoke` uses p about 1031, 3 curves, 40 runs (verdict forced to inconclusive).

## 5. Classification
runnable_local. The question is decided by a stdlib toy at group order about 2^14, and a closed lead's recorded census can be cheaply re-measured at small scale.

## 6. What a run cannot establish
- Only generic rho at N about 2^14 on one class; says nothing about BSGS, large N, volcano levels (no isogeny walking, no endomorphism-ring/level control), or non-generic algorithms (index calculus).
- 8 curves of one class may not span several volcano levels; a power margin of 8% is bounded by run count.
- Equivalence here is empirical, not the theorem; the original 403,168-run census is not re-verified. Result is labelled [pcc:unverified]; no leaderboard or benchmark use.
