# Protocol: reduced-round neural distinguishers under crypto-ml-v1 — 2026-11-02

Status: **frozen before data** at the commit that adds this file (recorded in the PR). The dry runs in §8 used
scratch output only and preceded the freeze.

## 1. Question

`docs/crypto-ml-v1.md` defines the lab's ML-cryptanalysis contract and says "generators and training pipeline remain
to be implemented". This study implements it end to end for one non-AES primitive and AES, and asks, per cipher and
round count: **does a small model distinguish real ciphertext pairs from random pairs on held-out keys, and does it
beat classical baselines on the same data?**

## 2. Claims and classes

| claim | class | what licenses it |
|:--|:--|:--|
| "model M distinguishes cell C" | comparative | Bonferroni-corrected Wilson lower bound on held-out accuracy > 0.5 (§6), with the cell's negative control at chance and positive controls detected |
| "MLP beats baseline B on cell C" | comparative | MLP accuracy − B accuracy > 0, with the 95% Wilson intervals disjoint |
| "no distinguisher found for C at this size" | comparative (bounded) | lower bound ≤ 0.5 *and* positive controls detected; scoped to these models, samples and epochs |
| key recovery, or a break of full-round Speck/AES | **out of scope** | not established: distinguisher accuracy is neither key recovery nor a full-round result (crypto-ml-v1 §controls) |

## 3. Task, data and splits (crypto-ml-v1 row contract)

Gohr's real-vs-random task (CRYPTO 2019). Label 1: `(E_k(P), E_k(P ⊕ Δ))`. Label 0: `(E_k(P), E_k(P'))` with P'
uniform. Speck32/64: Δ = (0x0040, 0x0000). AES-128 reduced to r rounds (final round without MixColumns): Δ = 0x01 in
byte 0. **Every sample has its own fresh uniformly random key**, so the independent unit is the key and train/test
are key-disjoint by construction (no key is reused within or across splits; collision probability ≤ 2^-60).
Classes are exactly balanced. Features are bits: C0 ‖ C1 ‖ C0⊕C1 (Speck 96, AES 384). Only pre-outcome values are
features; the label and P/P'/k never are.

| cell | role | train | test | epochs |
|:--|:--|--:|--:|--:|
| speck-r3 | positive control | 2^18 | 2^17 | 4 |
| speck-r5, speck-r6, speck-r7 | target | 2^20 | 2^17 | 10 |
| aes-r2 | positive control | 2^17 | 2^17 | 4 |
| aes-r3, aes-r4 | target | 2^19 | 2^17 | 6 |

Reference points from the literature, not reproduced here and not used by the decision rule: Gohr's residual network
with 10^7 training samples reports 0.929 / 0.788 / 0.616 accuracy at 5 / 6 / 7 Speck rounds. These models are smaller
and see 10× fewer samples, so lower accuracies are expected.

## 4. Models, baselines, controls, seeds

- **MLP**: 96/384 → 128 → 128 → 1, ReLU, sigmoid, BCE, Adam (lr 2·10^-3, linear decay to 10%, weight decay 10^-5),
  batch 512. Native Rust (`src/main.rs`), no Python (AGENTS.md).
- **Logistic regression**: the same trainer with no hidden layer (linear classical baseline).
- **Difference-table naive Bayes** (classical baseline): empirical distribution of each ciphertext-difference word
  (Speck: two 16-bit words; AES: sixteen bytes) under the real class, Laplace-smoothed, against uniform for the random
  class; summed log-likelihood ratio with a threshold fit on training data. This is weaker than Gohr's full
  Markov-chain DDT distinguisher, which is not implemented; the report says so.
- **Positive controls**: speck-r3 and aes-r2. Every model is expected to exceed 0.9. If the MLP misses, the run is
  invalid and no "not found" verdict is written.
- **Negative control**: per cell, an MLP trained on permuted training labels and scored against permuted test labels.
  Labels are then independent of every feature, so accuracy must lie within its 95% Wilson interval around 0.5.
- **A/A**: per cell, a second MLP training seed on the same data; its accuracy difference is the seed noise floor.
- **Seeds**: data per (cell, split) and training per arm from FNV-1a(name) XOR 0x6e64697374201102, fixed in source.

## 5. Execution

`neural-distinguishers run <out>`: all cells, appending one JSON row per (cell, model, arm) to `rows.jsonl`, then
`summary` builds `summary.json` and `fig_accuracy.svg`. Session container, 4 vCPU, CPU only, no spend (owner choice,
2026-11-02). Training times are recorded but are not a performance claim. Preflight (2^16 samples, contended host):
MLP 0.71 s/epoch (Speck) and 1.03 s/epoch (AES) → about 6 min per Speck target cell, 2.5 min per AES target cell,
about 30 min total.

## 6. Decision rule (unchanged after freezing)

- Family: 7 cells × 3 models = 21 one-sided tests at α = 0.01. Bonferroni gives per-test α = 4.8·10^-4, z = 3.52.
- "distinguishes": the Bonferroni Wilson lower bound on test accuracy is > 0.5.
- "beats baseline": the MLP's 95% Wilson interval lies entirely above the baseline's.
- A cell's verdicts are **invalid** if its negative control's 95% interval excludes 0.5, or if the positive control
  for that cipher is not detected.
- Effect size: report accuracy − 0.5 (advantage) and the A/A difference beside it.

## 7. What does not count

Changing cells, sizes, epochs, architecture, seeds, baselines or the decision rule after seeing target-cell data.
Any change is a dated amendment saying whether it preceded the data it affects; both versions are reported.

## 8. Dry runs before freezing (scratch output, positive-control cells only)

1. Shuffling only the training labels left the negative control significantly *below* chance (0.4835, 95%
   [0.4808, 0.4862]) on speck-r3. Test labels were still correlated with the features, so a noise-fitted model's
   chance offset between the structured real cluster and the random cluster moved accuracy away from ½. **Changed**:
   permute labels in both train and test.
2. With that fix the control read 0.5035, 95% [0.5008, 0.5062]. Labels were drawn independently, so the test
   prevalence was not exactly ½ and a near-constant predictor scored the prevalence. **Changed**: exactly balanced
   classes.
3. After both changes: negative controls 0.5000 (speck-r3) and 0.4996 (aes-r2); positive controls MLP 0.9993 and
   1.0000. No target-cell data was generated before freezing.

## Amendments

**A1 (2026-11-02, execution only; no design change).** Card `-1` was killed by the agent harness's 30-minute limit
on background shell commands after 33 of 35 arms (aes-r4's negative control and difference-table baseline
missing); no receipt. Card `-2` re-ran only cell aes-r4 with the frozen code, seeds and binary
(`41b8d479…`) and appended its rows. The three aes-r4 arms present in both runs agree exactly on `correct`. The
summary uses the last row per (cell, model, arm), as `summary` already did before any data. Cells, sizes, models,
seeds and the decision rule are unchanged. Target-cell results of the other six cells were visible before card
`-2` ran; card `-2` could not change them.
