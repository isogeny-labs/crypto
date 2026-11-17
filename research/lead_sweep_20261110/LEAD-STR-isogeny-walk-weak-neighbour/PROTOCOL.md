# LEAD-STR-isogeny-walk-weak-neighbour

## 1. Lead as recorded
- Disposition: closed_known (atlas_tally_20261101; last reviewed 2026-10-24). Family: structure.
- Title: "Walk or construct isogenies from a deployed curve to find a weak neighbour".
- next_step_excerpt: empty in leads.json (no recorded next step).
- Files read: research/lead_sweep_20261110/leads.json (entry); research/prime_field_lead_assessment_20261107/atlas_survey.md (line 54: "closed_obstructed; weaknesses are class invariants; P-256 million-model walk no hit"; sibling closures isogeny-class-generic-invariance: 403,168-run census, 33/35 level ratios TOST-equivalent, floor/crater rho 1.004; TRF-prime-field-isogeny-to-weak: Tate, special set empty); research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (line 893). Nothing found in /home/user/cryptanalysis was searched for; only the crypto hits above were read.

## 2. Hypothesis
Over F_p, every curve reached from E0 by rational 2-isogenies has the same group order #E0 (Tate), hence the same weakness indicators determined by the order (anomalous, largest prime factor, embedding degree <= 6), and Pollard rho on the largest prime-order subgroup costs the same on the neighbour as on E0 (ratio of mean steps in [0.8, 1.25]). So walking isogenies cannot produce a weak neighbour from a non-weak curve (recorded obstruction: weaknesses are class invariants).

## 3. Preregistered prediction and falsification criterion
- Statistics: (a) count of walk curves with #E != #E0 or a different weakness-flag tuple; (b) rho ratio mean_steps(neighbour)/mean_steps(root).
- Prediction: (a) = 0 of all walk curves; (b) in [0.8, 1.25].
- Falsified if any order or flag mismatch occurs (would contradict Tate or the Velu implementation; a bug is checked before any claim). Consistent only if (a)=0, positive control passes, and both (b) and the A/A ratio lie in [0.8, 1.25]. Otherwise inconclusive.
- Positive control: quadratic twists, which are not isogenous over F_p; the detector must see an order change in >= 80% of roots (traces 0 excepted).
- Negative control / null: random curves over the same p; fraction with the same order as E0 (expected near 0, reported, not gated).
- A/A arm: root vs root with independent seeds in the rho comparison; its ratio must also lie in [0.8, 1.25], which sets the noise floor.

## 4. Method
Pure Python stdlib, seed 20261110. Primes p = 4091, 16381, 65521 (p = 3 mod 4, taken downward from 4093/16381/65521). For each prime, 12 random roots E0: y^2=x^3+ax+b. Walk 8 steps, each a Velu 2-isogeny from a random rational 2-torsion root x0 (t=3x0^2+a, w=x0*t, a'=a-5t, b'=b-7w); orders by an O(p) Legendre-count table. Compare order and flags (N==p, largest prime, embedding degree <= 6). Rho: p=4091, 6 root curves with largest prime factor >= 300, each paired with its 2-isogenous neighbour; 150 independent Floyd rho runs per arm (root, root A/A, neighbour), 16-adding walk, base point of prime order q. Cost estimate: well under 5 minutes, memory < 50 MB.

## 5. Classification: runnable_local
A stdlib toy decides both statistics directly; the closed lead gets a cheap re-derivation of its recorded obstruction.

## 6. What a run cannot establish
- It tests only rational 2-isogenies at toy sizes (p <= 2^16); it says nothing about the P-256 million-model walk or odd-degree isogenies beyond the cited theory.
- Equal order is a theorem, so the run mainly checks the implementation; it does not rule out weaknesses that are not class invariants (e.g. implementation, side-channel, or non-prime-field settings).
- The rho ratio has sampling noise (150 runs/arm); the 0.8-1.25 band is a coarse equivalence, not a TOST certificate.
- Results are labelled [pcc:unverified].
