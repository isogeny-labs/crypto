# LEAD-ICE-curve-presentation-invariance

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step (last reviewed 2026-10-29); family: ic-engine; source: atlas_survey_20261107 --pursuit.
- title: "Does decomposition difficulty change across an isogeny class, curve model or special j (transport to an easier presentation)?"
- next-step excerpt (truncated): "Execute EXP-DREG-dd668d (spec frozen, runs/ empty): last-fall and grevlex solving degree of the ..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 218-222);
  /home/user/crypto/docs/session-logs/2026-11-06.md (grep context, lines ~857, 1769-1777, 2827).
- Recorded context: binary Koblitz toy runs found no curve effect (ICC upper bounds < 0.05). EXP-DREG-dd668d compares solving degree of native vs Velu-eliminated ideals plus one special-model curve. An owner decision on base-curve order reading is pending. research-kb was not queried.

## 2. Hypothesis
H0 (predicted): among prime-field curves of identical group order (same Frobenius trace, different j), the coverage of the 2-decomposition sumset of a fixed-size x-window factor base shows no between-curve component beyond within-curve window noise: ICC < 0.05 (permutation p >= 0.01). Falsified if some prime shows ICC > 0.05 with p < 0.01 while the controls behave.
This is a toy proxy (set-theoretic decomposition yield), not the frozen Groebner solving-degree test.

## 3. Preregistered prediction and falsification criterion
- Statistic: one-way random-effects ICC of coverage = |{P_i+P_j}|/N across k=8 curves x R=6 window replicates, with a permutation p-value (2000 shuffles of replicates across curves).
- Prediction: consistent_with_prediction (no presentation effect).
- Falsified: xwindow arm has perm_p < 0.01 and ICC > 0.05 at any prime, with positive control detected and null arm clean.
- Inconclusive: the positive control is missed (perm_p >= 0.01) or the null arm is flagged (perm_p <= 0.01).
- Negative control: random subsets of the same size drawn from each curve's group (no x-structure); must give perm_p > 0.01.
- Positive control: same pipeline, but curve 0 gets a factor base 15% larger; must give perm_p < 0.01.
- A/A arm: the within-curve replicate variation (different random window starts on the same curve) is the noise floor in the ICC denominator.
- Outcome decides only whether the toy gives any hint of a presentation effect; a null does not close the lead.

## 4. Method
- Primes p = nextprime(2^13) and nextprime(2^15). Sample 900 random (a,b), count points via Legendre table, group by order N and distinct j; take the order class with most distinct j and use up to 8 curves.
- Factor base: m = floor(1.5 sqrt(N)) consecutive points in x-sorted order starting at a random cyclic offset (both y signs included).
- Coverage = distinct sums over unordered pairs (i<=j) divided by N. Python RNG seed 20261110, one stream, fixed order of arms.
- Cost: about 8 curves x 3 arms x 6 reps x ~m^2/2 point adds; est. 2-6 minutes total, < 50 MB RAM, one core.

## 5. Classification: runnable_local
A stdlib toy can decide the proxy question. The actual EXP-DREG-dd668d needs Groebner engines (blocked here); this protocol is a cheap sharpening only.

## 6. What a run cannot establish
- Nothing about solving degree, Semaev summation polynomials, Velu transport or special j (j=0/1728 are in other order classes and are not tested).
- Coverage of a sumset is a count-level statistic; equal coverage does not imply equal algebraic difficulty.
- Toy sizes ~2^13-2^15; tiny k and R give low power; same-order but not necessarily isogeny-equivalent curves (same trace implies isogenous over F_p by Tate, so same class, but the j set is whatever was sampled).
- Results are [pcc:unverified].
