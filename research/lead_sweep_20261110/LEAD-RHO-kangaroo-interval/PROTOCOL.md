# LEAD-RHO-kangaroo-interval

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_dominated (disposition_20261101 the same; last_reviewed 2026-10-24). Family: generic.
- Title: "Kangaroo / lambda (van Oorschot-Wiener, Gaudry-Schost) as a full-group ECDLP solver".
- Next-step excerpt: "Revisit when: A target with a known short interval or low-entropy secret (LHW leads)."
- Recorded obstruction (atlas_survey.md line 66): kangaroo-interval closed_dominated at 2.203 x floor.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 64-67);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (entry and line 312 context).
  No hit in /home/user/cryptanalysis was read. The 2.203 figure is quoted from the survey, not re-verified.

## 2. Hypothesis
On a full prime-order group of order q (secret uniform in [0,q)), a two-kangaroo tame/wild solver with
distinguished points costs more group operations than a matched r-adding rho with the same
distinguished-point rule. The ratio R = ops(kangaroo)/ops(rho) lies in [1.3, 2.4] for q = 2^14..2^20,
consistent with the recorded domination (about 2 sqrt(q) vs about 1.25 sqrt(q) plus overhead). Kangaroo's
advantage is confined to short intervals: cost scales with sqrt(W) for interval width W.

## 3. Preregistered prediction and falsification criterion
- Statistic: R = mean over sizes of (mean kangaroo ops / mean rho ops), 95% bootstrap CI (1000 resamples, seed 20261110).
- Threshold: consistent_with_prediction if CI lies inside [1.3, 2.4]. falsified if the CI lies wholly
  outside (below 1.3 would mean kangaroo is not dominated, which reopens the lead; above 2.4 means the recorded
  2.2 figure is optimistic). Otherwise inconclusive.
- Null / A/A arm: rho vs rho with independent walk seeds on the same secrets. The CI of the ratio must contain 1.
- Positive control: kangaroo on the same group with secret in a short interval W = q/256. The ratio
  short/full cost must fall in [0.03, 0.15] (expected near 1/16), showing the instrument sees an interval
  benefit.
- If either control fails, or more than 2% of runs fail to recover x, the verdict is inconclusive.
- Same secrets, same DP rule (D = sqrt(W)/16), same step counting for all arms.

## 4. Method
- Group: Z_q additive, q = next prime above 2^e for e in {14,16,18,20}; generator 1, element labels only.
  Walks use a multiplicative hash of the element for branch choice and DP test; x is never used by the walk.
- Kangaroo: tame at W/2, wild at x, alternating steps, 2^i jumps with mean about sqrt(W)/2, DP table keyed by
  element, solve on tame/wild DP match, x = mid + d_tame - d_wild mod q, verified.
- Rho: 20-branch r-adding walk on (a,b) with elem = a + b x, restart after each DP or 20 D steps, solve on
  DP match with b1 != b2, verified.
- 300 trials per size, derived fixed seeds (base 20261110). Cost: about 6M group steps total, expected under 2 minutes, memory well under 200 MB.
- Smoke: `python3 -I run.py --smoke` (q about 2^12, 8 trials; its verdict is meaningless).

## 5. Classification
runnable_local. A stdlib toy can decide the ratio and sharpen the closed bound.

## 6. What a run cannot establish
- Toy sizes (2^14..2^20) in an additive group stand in for elliptic curves. Operation counts exclude
  field-cost differences, negation-map and automorphism gains, and the section 5a GAE pricing band.
- It does not re-derive the 2.203 x floor figure exactly (different floor, DP overhead, toy constants).
- Gaudry-Schost two-set variants and multi-target settings are not tested.
- The smoke run showed large small-size overhead (kangaroo about 7.8 sqrt(q) at q = 2^12), so the full run's
  ratio may fall outside the band for finite-size reasons; that would be reported, not tuned away.
- Results are [pcc:unverified] and give no claim about cryptographic curves.
