# Held-out EC relation-feature control for proposals 27 and 31

Frozen 2026-11-24 (America/Los_Angeles) under draft crypto PR #5165. This is a deterministic secondary analysis of the exact signed-point hit table in [EC_LEADS_001_034.stdout.txt](EC_LEADS_001_034.stdout.txt), SHA-256 `ab4e92a1762ad1782b40966f50604bf2ef78a742ba9995fdb3886acb6d14fbe0`. The input was generated on public `E/F_1009:y²=x³+x+14`, with the same 16-point base, eight on-curve targets, four equal-cost candidate blocks, and exact group-sum labels. The producer output is `[pcc:unverified]` pending independent review. This control does not train a GNN or transformer, and cannot establish their cost or accuracy.

The latest user instruction resumes local parallel E(F_p) runs after an earlier stop; the parent confirmed the local-only exception. Today's `scripts/checkpoint.sh gate` failed because `.resume/RESUME.md` is absent; the prior S3 attempt returned `InvalidAccessKeyId`. This control uses the same explicit local bypass, with no cloud spend, external target, or credential claim.

## Frozen classifier and controls

- Input parser must find exactly eight `lead29_target` rows and assert target abscissae `[42,43,44,45,48,49,54,55]`, block hit counts as printed by the frozen raw file, and the input SHA-256 checked before launch. Targets 1–4 are training; 5–8 are held out. No shuffling or tuning after seeing holdout results.
- Label a target/block cell **positive** when its exact signed-point hit count is at least six. A predicted negative would skip that equal-cost block and lose any hits there; report false-negative cells and the number of exact hits they contain.
- **Bit-feature arm:** for each held-out abscissa, find the training abscissa with minimum bitwise Hamming distance (`popcount(x xor x_train)`), tie to lower training index. Copy that training target's four binary labels. For one-block scheduling, choose the training target's highest-hit block, tie to lower block index.
- **Controls:** predict all-negative if negatives are the training majority (otherwise all-positive); choose static block 1 without training; choose the highest total-hit training block (tie to lower block index). Compare all arms on the same four held-out targets. The one-block choices each inspect 8192 signed candidate patterns per target.
- **Charged cost accounting:** acquiring four training target profiles required `4×32768=131072` target comparisons against already generated signed point sums, plus one shared 32768-pattern group-sum enumeration. Bit inference uses 16 XOR/popcount comparisons, plus fixed small selection work; constant/static baselines require no label acquisition, while training-best shares it. Evaluating all held-out labels for this retrospective audit required another `4×32768=131072` target comparisons; do not count that as online inference. Source generation, label acquisition, and model selection are not free.

The native Rust source reads the existing raw file, checks the exact fixed target/count table, computes predictions and confusion counts, and prints all held-out choices and costs. It must not modify the input or write files. Stop on parse mismatch or any source/output error. Preserve source, stdout, stderr and exit separately; no silent retry. The output is a small feature-screen component with no measured runtime, relation-rank, or end-to-end result.

## One-shot command

Frozen source `EC_CLASSIFIER_001_034.rs` SHA-256: `15a62bf7f4c77c72aaf5d8f3d7e2618090605469fd386b5e0470e8d371743d51`. Input stdout SHA-256 was rechecked before launch and remained `ab4e92a1762ad1782b40966f50604bf2ef78a742ba9995fdb3886acb6d14fbe0`. Compile with Rust 1.99.1 to a **local-only** binary under `/tmp`; do not stage the binary. Then invoke once from repository root, capturing both streams:

```sh
RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc rustup run 1.99.1 rustc -O --edition 2021 research/prime_field_100_ideas_20261123/EC_CLASSIFIER_001_034.rs -o /tmp/ec_classifier_001_034.bin
/tmp/ec_classifier_001_034.bin research/prime_field_100_ideas_20261123/EC_LEADS_001_034.stdout.txt > research/prime_field_100_ideas_20261123/EC_CLASSIFIER_001_034.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_CLASSIFIER_001_034.stderr.txt
```

Record compile and run statuses, local binary SHA-256, and source/raw hashes in this file. No run has occurred at protocol creation.

## Observed local control — 2026-11-24 (America/Los_Angeles)

The frozen standalone Rust source compiled with Rust 1.99.1 and ran once against the unchanged input file. Compile and run both exited 0; stderr was empty. The source asserted the eight target abscissae and all 32 exact group-hit counts before computing predictions. The local binary remains under `/tmp` and is **not** to be staged. These are producer observations `[pcc:unverified]`.

| Held-out target `x` | Nearest training `x`; Hamming distance | NN-chosen block; exact hits | Training-best block 3; hits | Fixed block 1; hits |
|---:|---|---|---:|---:|
| 48 | 42; 3 | 4; 4 | 6 | 2 |
| 49 | 43; 3 | 3; 8 | 8 | 4 |
| 54 | 42; 3 | 4; 12 | 13 | 17 |
| 55 | 43; 3 | 3; 4 | 4 | 4 |
| **Total** | | **28** | **31** | **27** |

At the predeclared positive threshold of six exact hits, training had 6/16 positive cells. On 16 held-out cells, nearest-neighbor confusion `(TP,FP,FN,TN)` was `(5,3,4,4)`; the all-negative majority control was `(0,0,9,7)`. The four nearest-neighbor false-negative cells contained **33 exact signed-point hits**, so using this classifier as a sound reject filter would discard valid relations. The one-block NN scheduler found 28 hits versus 31 for the simpler training-best block and 27 for the fixed block. Label acquisition cost 131072 training target comparisons plus the shared 32768 group-sum patterns; NN inference added 16 XOR/popcount comparisons. The retrospective holdout audit used another 131072 target comparisons, kept separate from online inference. This small control supplies neither a GNN nor transformer result and makes no speed or rank claim. `[pcc:unverified]`

| Artifact | SHA-256 |
|---|---|
| `EC_CLASSIFIER_001_034.rs` | `15a62bf7f4c77c72aaf5d8f3d7e2618090605469fd386b5e0470e8d371743d51` |
| `EC_CLASSIFIER_001_034.stdout.txt` | `2401c4ef716ff4741e071dfcb9a9141733ed46fea426b5e1ee54356a0104c60d` |
| `EC_CLASSIFIER_001_034.stderr.txt` (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `EC_CLASSIFIER_001_034.exit.txt` | `f36e5fc8345e61efb7e13cf97c63801e4faf72beb4f636b9cfbf2539de66e235` |
| `/tmp/ec_classifier_001_034.bin` (local-only) | `8085d3d268084aca33fcffb8bb0593df17706df9977439d3c44a017ba7f2d569` |

The charged static-block control is feasible without any training labels. The training-best and NN arms share the training-label expense; NN did not exceed that simpler same-data arm on these four held-out targets. A future actual GNN/transformer test would need a much larger independently labeled EC candidate corpus and a predeclared generalization split. This control does not establish a general limit for those models.
