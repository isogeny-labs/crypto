# LEAD-ICB-cofactor-projected-ladder-base

## 1. Lead as recorded
- disposition: open_next_step (last_reviewed 2026-10-31; source atlas_survey_20261107 --pursuit)
- family: ic-binary
- title: Cofactor-projected factor base ([h]P into the prime-order subgroup) on the random-binary ladder rows with cofactor h > 1
- next-step excerpt (truncated in leads.json): "Land the three primary rows through the validation gate (independent replay certificate on a sec..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 203, one-line row only, no further detail).
  Hits in ecdlp_subexp_desk_20261101/data/{atlas_tally,candidate_matches}.json were only id matches, not read. Nothing in cryptanalysis.
- The real next step (validation-gate landing of three ladder rows) needs replay certificates and repo launchers: not reproducible here.
  This protocol tests only the mechanism behind the lead, on a toy.

## 2. Hypothesis
For a curve with #E = h*n (n prime), a decomposition factor base F' = [h]F (F random points of E, projected into the order-n subgroup G)
gives, for a uniformly random target T in G and a base of B points, an expected number of ordered 2-term relations f1+f2 = T that is
exactly h times that of an unprojected random base F of the same size B (E[reps] = B^2/n versus B^2/(hn)), because only a 1/h fraction
of unprojected sums land in G. Prediction: ratio R = reps(proj)/reps(full) = h.

## 3. Preregistered prediction and falsification criterion
- Statistic: R_h = total relations with projected base / total with unprojected base, pooled over curves, per cofactor h in {1,2,4}.
  Poisson relative SE = sqrt(1/c_proj + 1/c_full).
- Threshold: consistent if |R_h - h| <= 3 * R_h * relSE for every h, AND the A/A check passes.
  Falsified if any h violates this (with counts >= 30 in every arm). Inconclusive if any arm count < 30.
- Null / negative control: h = 1 (no cofactor, projection is the identity map's analogue), predicted R = 1.
- Positive control: h = 2 and 4 themselves (the effect is known to be exactly h by the counting argument; a detector that cannot
  see R = 2 or 4 would be broken). The unprojected arm is also checked to be ~ 1/h of the subgroup-only rate via R.
- A/A arm: two independently drawn projected bases of equal size; ratio must lie within 3 SE of 1.
- Decides: consistent means the projection yields an h-fold relation-rate gain at fixed base size at toy scale (an accounting-neutral
  gain only if the projection cost, one scalar multiplication per base point, is amortised; it is not counted here).
  Falsified means the lead's premise (projection raises yield) fails even for this idealised random base.

## 4. Method
Pure Python, prime-field short Weierstrass toy (binary-field arithmetic omitted; the counting argument is field independent).
- p in {nextprime(6000), nextprime(9000)}; for each h in {1,2,4}, 2 curves per (p,h) found by random (a,b) with #E = h * prime exactly
  (point count by Euler-criterion table, O(p)). Seeds: seed0 = 20261110, +1 per curve; random.Random only.
- Q = [h]P of prime order n (asserted). B = int(1.5*sqrt(n)). 300 targets T = [k]Q, k uniform in [1,n).
- Arms per curve: full = B random points of E; proj = [h]f for the same points; proj2 = [h] of an independent B points (A/A).
- Count ordered pairs (f1,f2) in base with f1+f2 = T by hash lookup of T - f.
- Cost: well under 1 minute, < 50 MB. `--smoke` uses p=307, h in {1,2}, 20 targets and always reports inconclusive at best.

## 5. Classification
runnable_local. A stdlib toy decides the counting claim. (The recorded next step, gate landing, is blocked here, but the mechanism is testable.)

## 6. What a run cannot establish
- Nothing about the three primary ladder rows, replay certificates, validation gate, or the gap to matched rho.
- Only 2-term relations on prime-field toys; not binary Jacobians, not the Weil-descent decomposition, not linear algebra or
  relation-matrix rank, and not whether projected relations still solve the DLP (kernel/coset information is lost for h > 1).
- Projection cost and the cofactor-subgroup discrete-log side (log in the h-part) are not priced; no 0/0.1/1 GAE band (§5a) applied.
- The expected result follows from a counting identity, so confirmation is a sanity check, not novel evidence. Results are [pcc:unverified].
