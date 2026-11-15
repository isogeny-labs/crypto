# LEAD-TRF-same-field-genus-g-embedding

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (leads.json); disposition_20261101: closed_obstructed. Family: transfer. Last reviewed 2026-10-29.
- Title: Embed E/F_p as an isogeny factor of a genus-g Jacobian over the same field (covers, quadric pencils, quartic/bielliptic models).
- Next-step excerpt (truncated): "never for same-field covers; extension-field sources are separate leads".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (A4, line 59: #Jac = N*#A, #A = Theta(N^(g-1)), IC O~(q^(2-2/g)) >= q); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 59, 84-89, 145: ic.hyperelliptic measured 1.47x rho at 2^17-2^20, exponent q^1).

## 2. Hypothesis
For bielliptic genus-2 curves C: y^2 = a x^6+b x^4+c x^2+d over F_p, Jac(C) is (2,2)-isogenous to E1 x E2 with E1: y^2=au^3+bu^2+cu+d and E2 the reversed cubic, so #Jac(F_p) = #E1 * #E2 exactly, and the cofactor #E2 >= (sqrt p - 1)^2 (Hasse). Hence the group containing the embedded E1 has order N*Theta(p), and same-field genus-2 index calculus (q^(2-2/g) = p) costs exponent 1 against rho's 1/2; the embedding gives no advantage.

## 3. Preregistered prediction and falsification criterion
- Statistic: fraction of curves with #Jac == #E1*#E2 (Jac order from #C(F_p), #C(F_p^2): (N1^2+N2)/2 - p), plus count of Hasse cofactor violations.
- Threshold: positive-control rate must be exactly 1.0 and violations 0. Any failure FALSIFIES the recorded obstruction's arithmetic (or exposes a bug); success is consistent_with_prediction only (it re-derives a known bound, no new improvement).
- Positive control: the bielliptic curves themselves (identity must hold). Negative control / null: generic sextics with odd-degree terms compared with the same E1*E2 product; match rate must be < 0.5 (detector can reject non-split Jacobians). A/A arm: not applicable (deterministic exact counting, no noise; seeds fixed).
- Inconclusive: positive rate 1 and no violations but negative control match >= 0.5.

## 4. Method
Python stdlib only. Seed 20261110. Primes 23..69 (about 12 primes), 12 random squarefree-cubic bielliptic curves each; F_p^2 as F_p[s]/(s^2-n), n a nonresidue; explicit quadratic-character counting, 2 (or 1 + chi(a)) points at infinity. Plus one generic sextic per curve as the null. Cost: ~p^2 evaluations per curve, a few minutes at most, <50 MB. `--smoke` uses p in {11,13}, 3 curves each.

## 5. Classification
runnable_local: a stdlib toy decides the arithmetic identity and Hasse cofactor bound behind the recorded closure.

## 6. What a run cannot establish
- It does not measure index-calculus cost, only the group-order identity; the q^(2-2/g) exponent is Gaudry's published bound, taken as given.
- Only genus 2 bielliptic, p < 70; general g and non-split covers (GHS, quartic models) are untested.
- Does not touch extension-field sources (separate leads). Results labelled [pcc:unverified].
