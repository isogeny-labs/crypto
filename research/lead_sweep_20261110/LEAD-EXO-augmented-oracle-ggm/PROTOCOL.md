# LEAD-EXO-augmented-oracle-ggm

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (atlas_tally_20261101; last reviewed 2026-10-29; theorem recorded 2026-11-04). Family: exotic.
- Title: Augmented oracles (x-, y-, full-point, halving, endomorphism-image, incidence) and GGM simulability.
- Next-step excerpt: empty in leads.json (closed lead, no next step).
- Recorded obstruction (atlas_survey.md A6): label-valued oracles have explicit GGM simulators; endomorphism image = public scalar lambda.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 69-74, 155). Other hits (atlas_tally.json, candidate_matches.json, cryptanalysis ATLAS_INPUTS) are index files, not read.

## 2. Hypothesis
In a toy generic group Z_N (N prime, 2^12..2^20) with random-hash labels, the endomorphism-image oracle (multiply by public lambda) and the halving oracle are simulated exactly by the group-add oracle with at most ~2*log2(N) add queries per call, while a non-label-valued oracle (parity of the discrete log) is not simulable (simulator accuracy 0.5).

## 3. Preregistered prediction and falsification criterion
- Statistic: per-instance match rate of simulated vs real oracle outputs over 3000 random queries (200 in smoke); mean simulator add queries per call.
- Prediction: match rate exactly 1.0 for endo and halving; mean adds <= 2*log2(N); parity-simulator accuracy within 4 sigma of 0.5 (|acc-0.5| <= 4*sqrt(0.25/n)).
- Falsified if any endo/halving match < 1.0 or overhead exceeds the bound (the recorded simulator claim fails). Inconclusive if the simulable claims hold but the null or A/A check fails.
- Positive control: endo/halving oracles (detector sees a perfect simulator). Negative control/null: parity oracle with a coin-flip simulator. A/A arm: same protocol, different label key and seed (777), must also match 1.0.

## 4. Method
Labels are keyed blake2b(key, x) 64-bit values (collision probability negligible at these sizes); the oracle keeps a label-to-exponent dict. Simulator: double-and-add using only add queries. Sizes N in {4093, 65521, 1048573}, 3 seeds each (seeds 1000*bits+i), lambda drawn from seeded RNG, 3000 queries each. Cost: a few seconds, <50 MB, stdlib only.

## 5. Classification
runnable_local. A stdlib toy can re-derive the recorded simulability bound directly.

## 6. What a run cannot establish
It is a toy generic-group model, not real elliptic curves: it cannot say anything about coordinate-revealing oracles on real curves, nor about incidence or x/y oracles, whose simulability is a proof matter. It re-checks the easy direction (public-scalar oracles are simulable) and cannot establish the obstruction theorem itself or any speedup. Results are labelled [pcc:unverified].
