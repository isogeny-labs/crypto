# LEAD-EXO-ml-learned-models

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last_reviewed 2026-10-30). Family: exotic.
- Title: Machine learning / neural and RL approaches to ECDLP and its subroutines.
- Next-step excerpt (truncated in leads.json): "Run a learnability control on toy prime-order groups: train a model to predict a log bit from co..."
- Atlas context: a learned WDSat branching policy was worse than default (16,097 vs 3,092 conflicts); learned log-bit predictors were inconclusive x3 at N<=2^20. README line: "a learnability control with a planted homomorphic leak; prediction is chance on real curves."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 72-74, 155-166, 194-208 only);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/README.md (grep context only).

## 2. Hypothesis
For prime-order toy curves E(F_p), N in ~2^12..2^18, a logistic-regression learner on point-coordinate features (x and y residues mod small m, y parity, top bits of x), trained on 8000 (P=kG, bit of k) pairs, predicts the log bits lsb(k), bit1(k) and msb(k) on 8000 held-out points no better than chance: held-out accuracy z-score vs 0.5 stays below 4.0 for every curve x label x split-seed, while the same pipeline detects a planted leak.

## 3. Preregistered prediction and falsification criterion
- Statistic: z = (acc - 0.5)/sqrt(0.25/n_test), per (curve, label, split seed); 4 curves x 3 labels x 2 seeds = 24 tests; threshold |z| >= 4.0.
- Prediction: no test reaches 4.0 (chance on real curves).
- Falsified: any (curve, label) with |z| >= 4.0 on both independent split seeds (replicated). A single-seed exceedance is inconclusive.
- Negative control: same features with labels randomly permuted; max |z| must be < 4.0, else the run is inconclusive (pipeline biased).
- Positive control: planted leak feature that agrees with the label with probability 0.65 (expected z ~ 27 at n=8000); min z over all runs must be >= 4.0, else inconclusive (detector blind).
- A/A arm: identical data and split, different SGD shuffle seed; max |acc difference| reported as the training noise floor; not a gate.
- Decides: consistent_with_prediction supports (does not prove) the recorded "chance" outcome; falsified would mean a coordinate-residue leak of a log bit and would reopen the lead.

## 4. Method
- Per size bits in {12,14,16,18}: pick p = next prime above 2^bits + rand (seed 1000+i), random (a,b), naive O(p) point count, accept first prime N>1000. Enumerate all kG, k=1..N-1.
- Features (one-hot indices): x mod m for m in {2,3,4,5,7,8,9,11,13,16,17,19,23,32,64}; y mod {2,3,4,8}; y>p/2; 8 top bits of x.
- Split: sample 16000 distinct k (cap (N-1)/2 each side for small N), first half train, second test; seeds 11 and 22.
- Learner: logistic regression, SGD lr 0.05, 4 epochs, seeded shuffle. Labels: k&1, (k>>1)&1, [2k>=N].
- Cost estimate: well under 10 min on one core (dominated by SGD, ~72 fits x ~3 each); RAM < 150 MB (2^18 point list). Python stdlib only.

## 5. Classification
runnable_local: the question is decidable by a stdlib toy; run.py implements it (`--smoke` tested only).

## 6. What a run cannot establish
- Only linear learners on a small hand-made feature set at N <= 2^18; nothing about deep nets, RL, bigger N, other feature maps (e.g. digit characters, Fourier of x), or ML as a solver subroutine (SAT branching, factor-base choice).
- Chance result does not rule out leaks the features cannot express; positive control only proves detection of an extra-feature leak, not of a leak hidden in the coordinates.
- Real curves only: prime order, so no subgroup-structure leak is tested. Results are labelled [pcc:unverified].
