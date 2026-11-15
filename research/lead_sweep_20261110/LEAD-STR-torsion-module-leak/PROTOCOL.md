# LEAD-STR-torsion-module-leak
## 1. Lead as recorded
- disposition: open_next_step (atlas); disposition_20261101: closed_obstructed. family: structure. Title: "l-torsion (l != N) as a Galois/End module leaking k or attack cost". Last reviewed 2026-10-24.
- next-step excerpt (truncated): "A point-dependent l-torsion statistic computable in o(sqrt N) correlates with k beyond a k-resam..."
- Survey note: "open_next_step label, obstruction proof recorded: homomorphic invariants to l-primary l!=N trivial".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 54, 150, 264 via rg context); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (entry only). No other hits read.
## 2. Hypothesis
For E(Fp) = Z/2 x Z/2N (full rational 2-torsion, N odd prime) and G of prime order N, the 2-descent map delta: E(Fp) -> (Fp*/Fp*^2)^2, Q -> (x-e1, x-e2) mod squares, is a homomorphism killing the odd-order subgroup, so delta(kG) is trivial for every k (zero information on k). A non-homomorphic point statistic (Legendre symbol of x(kG)) has no correlation with k mod 2 either. Hence l=2 torsion-module statistics give no leak, matching the recorded obstruction.
## 3. Preregistered prediction and falsification criterion
- Statistic: count of k in 1..K with delta(kG) != trivial, over all curves; plus z-score of agreement between chi(x(kG)) and parity of k.
- Prediction: count = 0 exactly; |z| < 4 for the Legendre control and for the A/A arm.
- Falsified: any nontrivial delta(kG) (would contradict homomorphism/odd-order argument, i.e. a bug or a flaw in the obstruction). Consistent: count 0 and both |z|<4. Inconclusive: positive control < 100% or |z| >= 4 on a null arm.
- Positive control: R of order 2N with nontrivial delta; delta(kR) nontrivial iff k odd, so the detector must recover k mod 2 with accuracy 1.0.
- Negative control / null: chi(x(kG)) vs k parity (non-homomorphic statistic, expected accuracy 1/2).
- A/A arm: same chi(x(kG)) statistic against seeded random labels, expected z ~ N(0,1).
## 4. Method
Python 3 stdlib. 40 curves y^2=(x-e1)(x-e2)(x-e3) over Fp, p random prime in [3000,9000) (N ~ 2^9..2^11 so group orders ~2^11-2^13), keeping curves with #E=4N, N prime (point count by Legendre sum). R random, G=[4]R, k=1..min(N-1,1500) via repeated addition. Seed 20261110 (smoke seed 1). Cost: seconds to about a minute, < 50 MB. l=2 only.
## 5. Classification
runnable_local: a stdlib toy decides the recorded obstruction for l=2 (a cheap re-derivation).
## 6. What a run cannot establish
Only l=2 with full rational 2-torsion and N prime; does not cover odd l, Galois-nonrational torsion, statistics over extension fields, or cryptographic sizes. It re-confirms a known triviality, not a general theorem (the proof is the obstruction itself). Results are labelled [pcc:unverified].
