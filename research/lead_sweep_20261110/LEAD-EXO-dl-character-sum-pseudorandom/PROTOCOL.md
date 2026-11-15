# LEAD-EXO-dl-character-sum-pseudorandom

## 1. Lead as recorded
- Disposition: exhausted_at_scale (last_reviewed 2026-10-24; disposition_20261101 same). Family: exotic.
- Title: Additive-character bias of the discrete log on structured factor bases (H-PSEUDO).
- Next-step excerpt (truncated in leads.json): "Revisit when: R measured against a negation-matched null exceeds 1 beyond its CI and trends upwa[rd with p ...]". Atlas survey adds: reopen only for R>1 beyond CI trending up beyond 2^30.
- Recorded facts: old R = C_EC/C_null ~1.28-1.34 used an asymmetric null; against a negation-matched null R = 1.040 [0.999,1.092] (2^20), 0.993 [0.972,1.017] (2^22) (HPSEUDO-SYMNULL-20261104).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_leads_20261031/C4_pseudo_generic/REPORT.md (118 lines; energy variant, null_confirmed at 2^12..2^18);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 71-75, 152-156, 301-305 via grep).

## 2. Hypothesis
Let E/F_p have prime order N, F the h = round(sqrt N)/2 smallest-x point pairs {P,-P}, S the folded DL indices min(j,N-j). Define C(S) = max_{1<=k<=(N-1)/2} |sum_{j in S} cos(2 pi k j/N)|. Claim: R = C(S)/mean C(S_null), with S_null uniform random h-subsets of {1..(N-1)/2} (this is the negation-matched null), has pooled mean 1 within noise and no upward trend in log2 N over N ~ 2^11..2^14 (toy range of the recorded obstruction; real reopen range 2^30 is out of reach).

## 3. Preregistered prediction and falsification criterion
- Statistic: per-curve R; pooled mean M with 95% CI M +- 1.96 se over curves; least-squares slope of per-size mean R on log2 p.
- Prediction: CI contains 1 (or lies below), and slope <= 0.01 per bit.
- Falsified: CI lower bound > 1 AND slope > 0.01 per bit (the recorded revisit condition, at toy scale), given controls valid.
- Inconclusive: controls invalid (below) or fewer than 8 usable curves, or mixed signals (CI > 1 with flat slope, etc.).
- Consistent: otherwise.
- Negative control / null: random negation-closed (folded) subsets of the same size on the same curve (10 per curve).
- A/A arm: the 10 nulls split 5/5; ratio of group means must average within [0.9,1.1], else inconclusive.
- Positive control: S = interval of h consecutive DL indices (a known highly biased set); mean ratio to null must exceed 1.4, else the instrument is deemed blind and verdict is inconclusive.
- Decides: consistent keeps the lead closed at toy scale; falsified would justify reopening for a larger-scale run.

## 4. Method
Python 3 stdlib only. For p = largest prime below 2^b, b in {11,12,13,14}, 10 random Weierstrass curves y^2=x^3+ax+b each with prime non-anomalous order N (points counted by enumeration). Generator random; full DL table by repeated addition (affine, pow(.,-1,p)). S built from the h smallest x-coordinates (one point per x). C computed exactly over all k via a cosine lookup table. Seed 20261110 (single random.Random stream, fixed order). Cost: about 12 sets per curve x N/2 x h table lookups, order 10^8 simple operations at the top size; estimate 1-5 minutes, memory < 50 MB, limit 10 minutes. `--smoke` uses b in {8,9}, 2 curves, 4 nulls (<1 s).

## 5. Classification
runnable_local: a stdlib toy at N<=2^14 decides the preregistered toy-scale statistic.

## 6. What a run cannot establish
- Nothing about N ~ 2^30 where the lead's reopen condition lives; a toy null does not extend there.
- Only max-|character sum| over one structured set (smallest x); not other factor bases, higher moments, or algebraic structure.
- Per the weak-class calibration note, such instruments may be blind to known weak classes; random curves only.
- Single-core sizes give wide CIs (about 10 curves per size). Results are [pcc:unverified] and are not a replay certificate or rho comparison.
