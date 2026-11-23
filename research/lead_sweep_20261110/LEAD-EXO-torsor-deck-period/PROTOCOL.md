# LEAD-EXO-torsor-deck-period

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- id LEAD-EXO-torsor-deck-period; family exotic; last_reviewed 2026-10-24.
- disposition: open_next_step (leads.json, atlas_survey_20261107 --pursuit); disposition_20261101 = closed_obstructed (atlas_tally.json). The survey notes it carries an `obstruction:` block while staying in the pursuit queue.
- title: "Torsor, deck-orbit, cubical/biextension and scalar-orbit elliptic-period descent".
- next_step_excerpt (truncated in source): "A rational, target-compatible selector or section of a nontrivial cover (or a non-Galois cover e..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context only, lines 72-74, 170, 201); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (grep context only). The lead's own YAML and full obstruction text were not available; the mechanism below is my reading (labelled inference).

## 2. Hypothesis
Toy model of the recorded obstruction (inference): for the 2-cover G=E(F_p) -> G/<T> (T the unique rational 2-torsion point, deck group {0,T}), a selector bit b(Q) (cheap function of affine coordinates, separating Q from Q+T) defines a section s(class)=the representative with b=0. The hypothesis: among the cheap selectors y-parity, y>p/2, x-parity, x>p/2, Legendre(y), none is a homomorphic section, i.e. for pairs of b=0 representatives the event b(Q1+Q2)=0 occurs with probability 1/2 (no excess), at p in [9000,16000), for both N=#E mod 4 = 2 and 0. Only the 2-descent character Legendre(x-x0) (and the DL-parity oracle) is expected to be homomorphic, and it yields only the known DL mod 2 bit, no more.

## 3. Preregistered prediction and falsification criterion
- Statistic: z = (f-0.5)/sqrt(0.25/n), f = fraction of normalised pairs with b(Q1+Q2)=0, n=3000 pairs per (curve, selector) cell.
- Threshold: |z|>5 is a "strong" cell.
- Prediction: each of the five cheap selectors has <=2 strong cells over all curves (20 curves, 10 per N mod 4 class).
- Falsified: any cheap selector has >=3 strong cells (a cheap, replicable homomorphic or anti-homomorphic section exists; this would be a lead to follow, not a result).
- Consistent: all of the below controls pass and no selector falsifies.
- Inconclusive: any control fails.
- Positive control: ORACLE_dl_parity (b = discrete-log parity, N=2 mod 4 cyclic curves only) must be strong in every one of its cells; leg_x_minus_x0 is a second expected-positive, reported not gated.
- Negative control / null: hash_null, a seeded hash of x; must have 0 strong cells.
- A/A arm: every cell is rerun with an independent sampling seed on the same selector; none of the non-descent selectors may exceed |z|>5 in the second run.
- Skips: cells with fewer than n/4 usable pairs are dropped and reported.

## 4. Method
Python 3 stdlib, one core. Seed 20261110 (curve choice, per-curve seeds SEED+i, sampling seeds SEED*7+i, SEED*13+i for A/A). For 20 draws pick prime p in [9000,16000), random short-Weierstrass curves with exactly one root x0 of the cubic (T=(x0,0) unique), N from Legendre counting, classes N=2 and N=0 mod 4 (one curve each per draw). Enumerate points, sample random pairs, normalise each to the b=0 representative (skip if b does not separate Q and Q+T), add, test b(S) (S+T must separate too). Oracle: dl table from a generator of order N where one exists. Cost: roughly 1-3 minutes, memory < 50 MB.

## 5. Classification
runnable_local. A stdlib toy decides the sharpened question (whether cheap selectors are sections compatible with the group law) without any heavy resource.

## 6. What a run cannot establish
- It is not the lead's real object: no cubical/biextension structure, no non-Galois cover, no isogeny to a different curve, only the 2-cover of the group by its 2-torsion deck group, at p ~ 2^13-2^14.
- A null on five hand-picked selectors says nothing about all rational sections; the lead's selector may be target-specific.
- Even a positive homomorphic selector would give only log2 of the kernel order bits, not beating rho.
- Results are [pcc:unverified]; no matched rho / validation gate / S5a pricing is attempted.
