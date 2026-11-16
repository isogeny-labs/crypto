# LEAD-EXO-split-jacobian-projection

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- id LEAD-EXO-split-jacobian-projection; disposition open_next_step (2026-11-10 leads.json; disposition_20261101 = closed_obstructed); family exotic; last_reviewed 2026-10-24.
- Title: "Split-Jacobian projected smoothness and the pair (P,Q) as a genus-2 divisor".
- Next-step excerpt (truncated): "A same-field embedding whose ambient group order is O(N) rather than inflated, or a projection w..."
- Recorded obstruction (atlas_survey A6/D9): Hasse-Weil inflation, #Jac = N*Theta(N^(g-1)), outweighs genus-2 IC (KN-FIND-61347e); desk row 15: ic.hyperelliptic measured 1.47x rho at 2^17-2^20, exponent q^1.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 69-74, 139-170); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 83-95). Not read: candidate_matches.json, KN-FIND-61347e source.

## 2. Hypothesis
For genus-2 curves C: y^2=g(x^2) over F_p (split Jacobian, J ~ E1 x E2), embedding the DL group G (largest prime factor of #E1 or #E2) into J(F_p) has ambient/N = #J/|G| >= p/4 in every sampled case (p in 37..139), i.e. the ambient order is never O(N); and generic sextics have #J/p^2 within the Hasse-Weil band. This re-derives the recorded inflation bound; it does not test the lead's open projection idea.

## 3. Preregistered prediction and falsification criterion
- Statistic: m = min over all split cases (both elliptic factors) of (#J/|G|)/p.
- Prediction: m >= 0.25 in both seeded arms. Falsified if m < 0.25 in either arm (an O(N) ambient exists at toy size, which would reopen the lead). 
- Positive control: #J = (N1^2+N2)/2 - p (from #C(F_p), #C(F_p^2)) must equal #E1*#E2 (isogeny) for every split curve; any mismatch or Hasse-Weil band violation => verdict inconclusive (counter bug).
- Negative control / null: random squarefree sextics (non-split); #J/p^2 should sit in the band (sqrt p -1)^4..(sqrt p +1)^4.
- A/A arm: identical protocol with seed+1; medians of generic #J/p^2 must agree within 0.15, else inconclusive.
- Outcomes: consistent => recorded inflation bound reproduced at toy size (no change to lead status); falsified => escalate; inconclusive => rerun after fixing instrument.

## 4. Method
Python 3 stdlib. Primes 37..139 (25 primes, about 12 split + 12 generic curves each, per arm), seeds 20261110 and 20261111. Point counts by brute force over F_p and F_p^2 (square test via norm to F_p); squarefree check by gcd. |G| by trial-division largest prime factor. Cost estimate: roughly 1-3 minutes, under 20 MB RAM, one core.

## 5. Classification
runnable_local: a stdlib toy decides the re-derivation. run_cmd: `cd /home/user/crypto/research/lead_sweep_20261110/LEAD-EXO-split-jacobian-projection && timeout 660 python3 -I run.py`.

## 6. What a run cannot establish
Results are [pcc:unverified]. It shows only that Jacobian order inflation holds at p<=139 for random split curves; it says nothing about asymptotics, about a cleverly chosen projection or special families (e.g. Jacobians with #J = O(N) via CM or Weil restriction constraints), about IC relation-yield on J, or about actual cost versus rho. Hasse-Weil makes #J >= (sqrt p -1)^4 a theorem, so a "consistent" verdict is expected and is largely a sanity check.
