# LEAD-ICP-gfp5-oa-sdhp-jv

## 1. Lead as recorded
- Disposition: open_next_step (2026-10-29 review; source atlas_survey_20261107 --pursuit). Family: ic-prime.
- Title: Joux-Vitse oracle-assisted static DH and (n-1)-point decomposition on E(F_{p^5}), p = 2^64 - 2^32 + 1.
- Next-step excerpt (truncated in leads.json): "Freeze the JV/Granger sources, measure C_pdp4 at toy p' and project to p = 2^64 (H-GFPN-3366ea),"
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; grep hits in
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 236, listing only) and
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines ~108-118: the lead is one of three places where a below-sqrt(N) exponent is "a measurement away"). No detail on H-GFPN-3366ea or C_pdp4 was found; the sources themselves (JV, Granger) are not present here.

## 2. Hypothesis
For random E over F_{p^5} (p = 7, 11) with factor base FB = {P : x(P) in F_p} (JV/Gaudry decomposition base, negation closed), the number of distinct sums of exactly four FB points (repeats allowed) equals, within 10%, the number obtained from a random negation-closed base of the same size. I.e. the relation-existence side of the (n-1)=4 decomposition is generic: it contributes no extra collisions or coverage beyond the counting heuristic (probability of a point being a 4-sum ~ c/p), so any gain of the lead must come from the polynomial-system solving cost C_pdp4, not from decomposition density.

## 3. Preregistered prediction and falsification criterion
- Statistic: ratio R = (distinct 4-sums of FB) / (mean distinct 4-sums of 3 random bases of equal size m), per curve.
- Threshold: consistent if every R in [0.9, 1.1]; falsified if any R is outside this band while controls pass; inconclusive if controls fail.
- Null / negative control: random negation-closed bases of size m drawn from E(F_{p^5}) (x not in F_p).
- Positive control: arithmetic-progression base {+-kG, k<=m/2} (strong additive structure); detector must see pos_ctrl / null < 0.5.
- A/A arm: two independent random bases per curve; relative difference of their counts < 0.05 (noise floor).
- Outcome decides: consistent -> decomposition density is generic at toy size, C_pdp4 is the only open quantity; falsified -> FB has structure worth following up.

## 4. Method
Pure-Python F_{p^5} = F_p[t]/(t^5+at+b) (irreducible by trial division), curve y^2 = x^3+Ax+B with A,B random in F_{p^5}. Group order N from full enumeration of squares table (p^5 elements). FB points: x = c in F_p, y a root. Distinct 4-sums via S2 = all pair sums, then all pairs of S2. Plan: p=7 curve seeds 11-14, p=11 seeds 21-22; null seeds = curve_seed*1000+k, k<3. N ~ 1.7e4 and 1.6e5 (about 2^14 to 2^17). Estimated cost: well under 10 min, < 200 MB (dict of 161k entries). Run: `python3 -I run.py` (smoke: p=5 only). Also reported (information only): Poisson prediction and p * (distinct/N).

## 5. Classification
runnable_local. This tests only the decomposition-density part of the lead. Measuring C_pdp4 itself needs a Groebner/polynomial-system solver (absent), so that part is not done.

## 6. What a run cannot establish
- Nothing about C_pdp4, the oracle-assisted static DH step, or the projection to p = 2^64 - 2^32 + 1.
- Toy p (7, 11) is far from asymptotic; small-p effects (negation, repeated points, Frobenius-invariant structure) may mask or fake genericity.
- Curves are random over F_{p^5}, not subfield or special curves.
- No pcc claim; results are [pcc:unverified] and not a validated benchmark or leaderboard row.
