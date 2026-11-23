# LEAD-TRF-jv-sieve-fp6-weak-class

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step (2026-11-01; last_reviewed 2026-10-30); family: transfer; source: atlas_survey_20261107 --pursuit
- title: Joux-Vitse cover-and-decomposition sieve on the weak class of E(F_{p^6}) (genus-3 cover), with the isogeny walk into the class
- next-step excerpt (truncated in leads.json): "Freeze numeric acceptance tolerances for the positive control (R0 S) and the rho A/A rule, then"
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 267 only, a one-line listing); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 108-116: extension-field n=3,5 are the only places a sub-sqrt exponent is "a measurement away"). The full lead text was not found; the rest is from the leads.json entry plus general knowledge of Gaudry/Diem/Joux-Vitse.

## 2. Hypothesis
Precondition of any JV-type sieve on E(F_{p^6}) with factor base {P : x(P) in F_{p^2}} (n=3 over q=p^2): the probability that a uniformly random R in E(F_{p^6}) is a sum of exactly three factor-base points is 1/3! = 1/6 up to O(1/q) corrections, for generic E over F_{p^6}, at q = 49, 121, 169. (This is the decomposition-rate ingredient of the lead's R0/S positive control. It does not test the genus-3 cover, the weak class, or the isogeny walk.)

## 3. Preregistered prediction and falsification criterion
- Statistic: rate = (#R decomposing)/(#R), 2000 random R per p; sigma0 = sqrt((1/6)(5/6)/n); dev = |rate - 1/6|.
- Consistent: for every p, dev <= 3*sigma0 + 0.02. Falsified: any p with dev > 3*sigma0 + 0.06. Anything between: inconclusive.
- Decides: consistent means the 1/n! decomposition model is the right yardstick for the lead's positive control, so tolerance "3 sigma0 + 0.02 on rate at toy size" can be frozen; falsified means the yardstick is wrong in this setup and must be re-derived before freezing tolerances.
- Negative control / null: factor base restricted to x in F_p (about p points); 3-point decomposition rate must be <= 0.02 (expected about 1/(6 p^3) scale, essentially 0), 400 R per p.
- Positive control: 20 planted R = P1+P2+P3 per p must be found 20/20 (detector can see decompositions).
- A/A arm: the main trials split by index parity into two halves of the same arm and instrument; two-proportion |z| <= 3.5 required.
- Any control failure forces verdict inconclusive.

## 4. Method
- Field F_{p^6} as the tower F_p[i]/(i^2-n) then F_{p^2}[t]/(t^3-c) with n a non-residue and c a non-cube (deterministic smallest choice). Curve y^2 = x^3 + A x + B with A, B uniform in F_{p^6} (must not lie in F_p, else all factor-base points sit in E(F_{p^2}); a first draft hit that bug, caught in smoke), seed 20261110.
- p in {7, 11, 13} (group about 2^17, 2^21, 2^22). Factor base FB = all (x,+-y) with x in F_{p^2}, f(x) a square in F_{p^6} (about q points).
- S2 = set of all P+Q, P,Q in FB (affine, Montgomery batch inversion). For each random R (uniform x in F_{p^6}, f(x) square, random sign; Tonelli-Shanks), test whether R-P in S2 for some P in FB (batch inversion). Degenerate x-collisions handled by slow add.
- Cost: pure Python, about 2-3 min total, under 50 MB; internal 480 s deadline returns inconclusive if exceeded. Run: `python3 -I run.py` (smoke: `--smoke`, p=5, 40 trials, under 1 s).

## 5. Classification: runnable_local
A stdlib toy decides the decomposition-rate hypothesis; no GPU, large memory, vault records or Groebner engine is needed for it.

## 6. What a run cannot establish (honest limits; results will be labelled [pcc:unverified])
- Not the genus-3 cover, the JV sieve's relation cost, linear algebra, the weak-class definition, or the isogeny walk into the class; the curves are generic, not weak.
- Toy q (49..169) cannot show an asymptotic exponent or beat rho; no matched rho and no 0/0.1/1 GAE pricing (AGENTS.md section 5a), so no leaderboard or benchmark row.
- The 1/6 model is textbook (Gaudry/Diem); confirming it is a sanity check on the positive-control yardstick, not a new result. Single curve per p, one seed.
