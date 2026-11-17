# LEAD-RHO-cheon-toy-reproduction

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step (2026-11-01 also open_next_step); family: generic; last_reviewed 2026-10-24; source atlas_survey_20261107 --pursuit.
- title: Toy reproduction of Cheon's algorithm with memory charged and supplied-input CRT composition.
- next-step excerpt (truncated in leads.json): "Run EXP-AUXIN-25a29a (H-AUXIN-a86953: 56-bit r, divisor 65536, stage counts and gain fractions)".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 65-67, 247 only). Related recorded context: cheon-deployed-applicability open_blocked ("values need d-degree supply, not a vulnerability"), cheon-aux-acquisition closed_obstructed, hnp-advice-leakage (aux-walk exponent 0.23 vs 0.50 only if [alpha^i]G leaked). ecdlp_subexp_desk atlas_tally.json hit not read. The 56-bit/65536 experiment itself is not reproducible here (56-bit r too large); this protocol is a scaled-down analogue.

## 2. Hypothesis
In a generic group of prime order r with d | r-1, given G, aG and a^d G, Cheon's two-stage algorithm (stage 1: a mod (r-1)/d by BSGS in the exponent orbit of h=g^d; stage 2: the remaining t in [0,d) by BSGS on the order-d subgroup) recovers a with a mean count of known-scalar multiplications equal to 1.5(sqrt((r-1)/d)+sqrt(d)) within [0.8,1.25]x, giving a raw gain over plain BSGS (1.5 sqrt r group additions) of at least 0.7x the theoretical sqrt(r)/(sqrt(r/d)+sqrt(d)); and when each scalar multiplication is charged 1.5 log2 r group operations, the charged gain falls below 1 in every toy cell (the gain is an accounting artefact of free scalar multiplication).

## 3. Preregistered prediction and falsification criterion
Statistic: per cell (r bits, d), ratio_meas_over_pred = mean Cheon scalar-mult count / 1.5(sqrt(r/d)+sqrt d), raw_gain = mean plain ops / mean Cheon mults, charged_gain = raw_gain / (1.5 log2 r). 120 random a per cell.
- consistent_with_prediction: all cells recover a in 100% of trials AND every ratio in [0.8,1.25] AND raw_gain >= 0.7 theory_gain, with controls valid.
- falsified: controls valid but any ratio outside [0.8,1.25] or any raw_gain < 0.7 theory (the cost model is wrong).
- inconclusive: any control invalid.
- The charged_gain<1 claim is reported per cell, descriptive, not part of the verdict.
Controls: negative control = supply a divisor not dividing r-1 (stage structure invalid); must succeed in <5% of trials. Positive control = d>1 cells themselves must recover a in 100%. A/A = same cells re-run with disjoint derived seeds (first 3 cells); relative deviation in mean Cheon cost must be <= 10%. Baseline arm = plain BSGS with additions only. Null for "no gain" is plain BSGS (d=1 is equivalent).

## 4. Method
Generic group modelled as Z_r with G=1, element = discrete log (so the check is exact; algorithm only uses scalar mult by known scalars and table collisions). Python stdlib only, one core.
- Prime r = k*d+1 near 2^bits (deterministic search), g = primitive root. Grid (bits,d): (16,16),(16,64),(20,16),(20,64),(20,256),(22,64),(22,256),(22,1024).
- Stage 1: n=(r-1)/d, h=g^d, BSGS with m=ceil(sqrt n) baby steps (stored table) and giant steps of h^-m on a^d G; yields x0 = x mod n.
- Stage 2: R = g^-x0 (aG) = zeta^t G, zeta=g^n of order d; BSGS yields t; a = g^(x0+n t); verified against a.
- Counts: scalar mults (every table and giant step), table entries (memory). Memory is reported (mean_table_entries) and charged via the 1.5 log2 r scalar-mult price.
- Seeds: 20261110 + 7*i (main), +1000+7*i (A/A), +2000 (negative control). Cost: well under 1 minute total, < 50 MB.
- Supplied-input CRT composition (second half of the title) is NOT implemented; see section 6.

## 5. Classification
runnable_local. The cost model and the memory/scalar-mult accounting are decidable with a stdlib toy at r <= 2^22.

## 6. What a run cannot establish
- Nothing about 56-bit r, d=65536, or any real curve; this is a generic-group toy where scalar multiplication is a primitive.
- Does not test supplied-input CRT composition, nor how a^d G could be obtained (cheon-aux-acquisition is closed_obstructed); it assumes the aux input is given.
- Charged-gain constant 1.5 log2 r is a modelling choice, not a measured curve cost.
- Expected result is a re-derivation of a known bound, not an improvement over index calculus or rho. Results are [pcc:unverified].
