# LEAD-TRF-small-genus-cover-general

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- id: LEAD-TRF-small-genus-cover-general; family: transfer; source: atlas_survey_20261107 --pursuit
- disposition: open_next_step (leads.json); disposition_20261101: closed_obstructed (ecdlp_subexp_desk_20261101 tally); last_reviewed 2026-10-29
- title: Find small-genus GHS/Kummer covers beyond the Joux-Vitse weak class (other configurations over F_{q^3}, F_{p^k} with k = 6, 10, 14)
- next_step_excerpt (truncated): "a non-Kummer cover family (Hess Artin-Schreier analogue, non-x-line function) is constructed in ..."
- recorded obstruction (atlas_survey.md line 60): "genus floors 9/49/41 at k=6/10/14".
- Files read:
  - /home/user/crypto/research/lead_sweep_20261110/leads.json
  - /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 58-60, 270-274)
  - /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (entry at line 1089)
- Not found: the underlying derivation of the floors 9/49/41 (only the numbers are in the files read). Not searched in cryptanalysis.

## 2. Hypothesis
H: For a degree-k extension F_{p^k} (k in {6,10,14}) the smallest genus of a GHS/Kummer-type cover of an elliptic curve E/F_{p^k} that has an F_{p^k}/F_p-descended Jacobian factor decomposition is at least the recorded floors 9 / 49 / 41, so that no genus below those floors yields a transfer with Gaudry-type cost q^(2-2/g) below rho cost p^(k/2) via a non-Kummer family. (Recorded floors are taken as the claim under audit.)

## 3. Preregistered prediction and falsification criterion
- Statistic: for each k, the minimum genus g_min(k) obtainable from the recorded cover-degree formula once re-derived in phase 2 (Riemann-Hurwitz for a degree-2^m or degree-ell Artin-Schreier/Kummer compositum, with ramification forced by the Frobenius orbit of the x-line function over F_{p^k}).
- Prediction: g_min(6)=9, g_min(10)=49, g_min(14)=41 reproduced exactly.
- Falsified if any re-derived g_min is smaller than the recorded floor (a smaller cover would exist) or the derivation needs an assumption the atlas does not state. Consistent if all three match. Inconclusive if the derivation cannot be reconstructed from the recorded text.
- Negative control: a k where the formula is known (k=3, Joux-Vitse weak class) must give the known small genus. Positive control: the same derivation must reproduce a published GHS genus in char 2. A/A arm: not applicable (no stochastic measurement).

## 4. Method
Desk derivation in phase 2, no compute: write the Riemann-Hurwitz computation per k, tabulate against floors, compare Gaudry cost q^(2-2/g) against p^(k/2) at those genera. No code is run.

## 5. Classification
desk_only. The step is a derivation and literature audit (Hess Artin-Schreier analogue, Kummer covers); a stdlib toy cannot construct function-field covers of the needed genus, and group orders in 2^12..2^22 do not reach genus 9+.

## 6. What a run cannot establish
- A floor derivation is a bound for the specified cover family only; it does not exclude other constructions.
- It says nothing about the actual DLP cost of any prime-order prime-field curve.
- The recorded floors are cited from atlas text; results are [pcc:unverified].
