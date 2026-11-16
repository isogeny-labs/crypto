# ksum `prune=1` against `prune=0` on Table B's eight curves

**Standing: counted cost at L0, `[pcc:unverified]`** (no pcc claim filed; no second-host replay, independent validator or `VALIDATION.json`: the validation gate is not met). Wall time is not claimed. Track: generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS).

## Outcome

- **`prune=1` lowers the counted cost of the k-sum arm on every one of the 8 rows, at all three weights of the section 5a band, pooled and in each replicate.** `ksum5-p / ksum5`: w = 0 0.831-0.941, w = 0.1 0.840-0.947, w = 1 0.890-0.973. `ksum4-p / ksum4`: w = 0 0.943-0.985, w = 0.1 0.946-0.987, w = 1 0.965-0.993. The largest 95% upper end of any pooled ratio is 0.9735 (k = 5) and 0.9936 (k = 4); of any replicate ratio 0.9741 and 0.9937.
- **Correctness: no mismatch.** 9216 per-target pairs (24 sessions x 2 k x 192 runs incl. warm-up): the `-p` arm recovered the same log, with identical counters (relations, targets tried, matrix rank/rows/work), identical `native` and `work_native` of every phase (so identical relation, trial, lookup, probe and `table_insert_attempts` counts), identical factor base and identical cost in every phase except `oracle_setup`, whose adds never increased. The record does not expose `ksum_table_entries`; insert attempts and hash probes are the proxy. 24/24 sessions pass `ecbench verify --replay-all --exit-code` (27,648 records verified, 20,736 replays reproduced, 0 problems; same binary, same host).
- **The gain is larger than the director's prediction** (about 5% at k = 5, w = 0): it is 6-17% at w = 0, because the saved build adds are 10-36% of the k = 5 build here (commit message: 10-12%) and 3-18% at k = 4 (message: 3%), so the build adds are a larger share of S than the director assumed (build share of S not measured here). The prediction `R_0(ksum5) = 0.95 +/- 0.03` held on 3 of 8 rows (P24, B24, B27); the gain is smaller at w = 1 (native work, unchanged) as predicted.
- **Noise gates.** The paired A/A gate (verdict) holds on all rows for both k: `rho-neg-aa / rho-neg` is exactly 1.0000 on every row and weight (same code, same seeds, deterministic count), so the gate reduces to the upper end lying below 1. The sibling delta gate (largest max/min ratio across replicate sessions of one arm's mean S, which compares *different seeds* and does not enter a paired ratio) holds for `ksum5` on P20, P22, B15, B18 and for `ksum4` on B15, B18, and fails on the rows where seed spread (delta up to 1.127) exceeds the gain. Both are reported; the registered verdict is the first.
- **Distance remaining.** Matched negation rho is still ahead on every row: `ksum5-p` costs 5.03-5.70x rho at w = 0 (unpruned 5.56-6.85x), 5.07-6.66x at w = 1. `ecbench band --ranks`: rho-neg below every k-sum arm on every row at every weight. This is a measured improvement over the unpruned k-sum baseline, not a result against rho (the long-term goal is a gap below 1).
- **Not an accounting change:** the removed work is group additions of the table build that the unpruned build performed and never used (correctness check: nothing else moved). Both arms are priced by the same harness pricing.

Study: `research/ic_ksum_prune_20261115/` (PROTOCOL.md registered in commit fff219f63f before any data; binary sha256 `6c60b4c931294bf770039a5b47800e39c424eb2e7f4876b426cd084f7c364d77`). Twin of `research/ic_tableb_ksum_20261115` and `research/ic_tableb_binary_ksum_20261115`: same curves, seeds, targets (48 per curve), 3 rounds, 1 warm-up, factor-base sizes; arms `rho-neg`, `rho-neg-aa`, `ksum4`, `ksum4-p`, `ksum5`, `ksum5-p`.

## ksum5-p / ksum5 (S: counted GAE per sqrt(r), mean over 3 sessions; ratio from `ecbench compare`, 4,000 resamples, seed 20261002)

| row | w | S ksum5 | S ksum5-p | ratio p/u [95%] | per replicate | A/A | gap to rho: u | gap to rho: p |
|:--|--:|--:|--:|:--|:--|--:|--:|--:|
| P20 | 0 | 7.111 | 6.441 | **0.9057** [0.9046, 0.9069] | 0.9056 / 0.9049 / 0.9066 | 1.0000 | 5.60 | 5.07 |
|  | 0.1 | 7.859 | 7.189 | **0.9147** [0.9136, 0.9158] | 0.9146 / 0.9139 / 0.9155 | 1.0000 | 5.70 | 5.21 |
|  | 1 | 14.588 | 13.917 | **0.9540** [0.9533, 0.9548] | 0.9540 / 0.9535 / 0.9546 | 1.0000 | 6.14 | 5.86 |
| P22 | 0 | 6.649 | 6.111 | **0.9191** [0.9181, 0.9201] | 0.9185 / 0.9195 / 0.9193 | 1.0000 | 5.67 | 5.22 |
|  | 0.1 | 7.391 | 6.853 | **0.9272** [0.9263, 0.9282] | 0.9267 / 0.9276 / 0.9274 | 1.0000 | 5.78 | 5.36 |
|  | 1 | 14.065 | 13.527 | **0.9618** [0.9611, 0.9624] | 0.9614 / 0.9620 / 0.9619 | 1.0000 | 6.25 | 6.01 |
| P24 | 0 | 6.383 | 5.978 | **0.9366** [0.9358, 0.9374] | 0.9372 / 0.9372 / 0.9354 | 1.0000 | 5.87 | 5.50 |
|  | 0.1 | 7.158 | 6.753 | **0.9435** [0.9427, 0.9443] | 0.9441 / 0.9440 / 0.9423 | 1.0000 | 6.04 | 5.70 |
|  | 1 | 14.132 | 13.727 | **0.9714** [0.9709, 0.9719] | 0.9717 / 0.9717 / 0.9706 | 1.0000 | 6.85 | 6.66 |
| B15 | 0 | 13.469 | 11.198 | **0.8314** [0.8304, 0.8325] | 0.8311 / 0.8313 / 0.8319 | 1.0000 | 6.85 | 5.70 |
|  | 0.1 | 14.186 | 11.916 | **0.8400** [0.8389, 0.8410] | 0.8397 / 0.8398 / 0.8404 | 1.0000 | 6.74 | 5.66 |
|  | 1 | 20.640 | 18.370 | **0.8900** [0.8890, 0.8910] | 0.8898 / 0.8897 / 0.8905 | 1.0000 | 6.13 | 5.46 |
| B18 | 0 | 11.257 | 9.654 | **0.8576** [0.8560, 0.8592] | 0.8555 / 0.8576 / 0.8596 | 1.0000 | 6.00 | 5.14 |
|  | 0.1 | 11.981 | 10.378 | **0.8662** [0.8646, 0.8678] | 0.8641 / 0.8662 / 0.8682 | 1.0000 | 5.92 | 5.13 |
|  | 1 | 18.498 | 16.895 | **0.9133** [0.9120, 0.9147] | 0.9116 / 0.9133 / 0.9151 | 1.0000 | 5.55 | 5.07 |
| B21 | 0 | 7.007 | 6.337 | **0.9043** [0.9032, 0.9055] | 0.9040 / 0.9044 / 0.9046 | 1.0000 | 5.56 | 5.03 |
|  | 0.1 | 7.733 | 7.063 | **0.9133** [0.9122, 0.9144] | 0.9130 / 0.9134 / 0.9135 | 1.0000 | 5.64 | 5.15 |
|  | 1 | 14.266 | 13.596 | **0.9530** [0.9522, 0.9538] | 0.9528 / 0.9530 / 0.9532 | 1.0000 | 6.05 | 5.76 |
| B24 | 0 | 6.991 | 6.517 | **0.9322** [0.9310, 0.9335] | 0.9331 / 0.9322 / 0.9314 | 1.0000 | 5.77 | 5.38 |
|  | 0.1 | 7.872 | 7.399 | **0.9398** [0.9386, 0.9410] | 0.9407 / 0.9398 / 0.9390 | 1.0000 | 5.96 | 5.61 |
|  | 1 | 15.810 | 15.337 | **0.9700** [0.9693, 0.9707] | 0.9705 / 0.9700 / 0.9696 | 1.0000 | 6.86 | 6.66 |
| B27 | 0 | 6.414 | 6.038 | **0.9413** [0.9407, 0.9420] | 0.9413 / 0.9416 / 0.9410 | 1.0000 | 6.00 | 5.65 |
|  | 0.1 | 7.168 | 6.791 | **0.9475** [0.9469, 0.9481] | 0.9475 / 0.9477 / 0.9472 | 1.0000 | 6.14 | 5.82 |
|  | 1 | 13.948 | 13.572 | **0.9730** [0.9726, 0.9734] | 0.9730 / 0.9732 / 0.9728 | 1.0000 | 6.78 | 6.59 |

## ksum4-p / ksum4

| row | w | S ksum4 | S ksum4-p | ratio p/u [95%] | per replicate | A/A | gap to rho: u | gap to rho: p |
|:--|--:|--:|--:|:--|:--|--:|--:|--:|
| P20 | 0 | 6.635 | 6.461 | **0.9738** [0.9734, 0.9742] | 0.9741 / 0.9739 / 0.9733 | 1.0000 | 5.23 | 5.09 |
|  | 0.1 | 7.468 | 7.294 | **0.9767** [0.9763, 0.9771] | 0.9770 / 0.9769 / 0.9762 | 1.0000 | 5.41 | 5.29 |
|  | 1 | 14.964 | 14.790 | **0.9884** [0.9881, 0.9886] | 0.9885 / 0.9885 / 0.9881 | 1.0000 | 6.30 | 6.23 |
| P22 | 0 | 6.284 | 6.139 | **0.9770** [0.9767, 0.9774] | 0.9771 / 0.9768 / 0.9772 | 1.0000 | 5.36 | 5.24 |
|  | 0.1 | 7.090 | 6.945 | **0.9796** [0.9793, 0.9799] | 0.9797 / 0.9794 / 0.9798 | 1.0000 | 5.54 | 5.43 |
|  | 1 | 14.341 | 14.196 | **0.9899** [0.9897, 0.9901] | 0.9900 / 0.9898 / 0.9900 | 1.0000 | 6.38 | 6.31 |
| P24 | 0 | 6.365 | 6.256 | **0.9828** [0.9826, 0.9830] | 0.9827 / 0.9826 / 0.9831 | 1.0000 | 5.85 | 5.75 |
|  | 0.1 | 7.224 | 7.115 | **0.9848** [0.9846, 0.9850] | 0.9847 / 0.9847 / 0.9851 | 1.0000 | 6.10 | 6.01 |
|  | 1 | 14.955 | 14.846 | **0.9927** [0.9926, 0.9928] | 0.9926 / 0.9926 / 0.9928 | 1.0000 | 7.25 | 7.20 |
| B15 | 0 | 11.194 | 10.552 | **0.9427** [0.9421, 0.9432] | 0.9423 / 0.9428 / 0.9428 | 1.0000 | 5.69 | 5.37 |
|  | 0.1 | 11.891 | 11.250 | **0.9460** [0.9455, 0.9466] | 0.9457 / 0.9462 / 0.9462 | 1.0000 | 5.65 | 5.34 |
|  | 1 | 18.169 | 17.527 | **0.9647** [0.9642, 0.9651] | 0.9644 / 0.9647 / 0.9648 | 1.0000 | 5.40 | 5.21 |
| B18 | 0 | 9.789 | 9.336 | **0.9537** [0.9530, 0.9544] | 0.9534 / 0.9541 / 0.9536 | 1.0000 | 5.22 | 4.97 |
|  | 0.1 | 10.523 | 10.069 | **0.9569** [0.9562, 0.9576] | 0.9566 / 0.9573 / 0.9568 | 1.0000 | 5.20 | 4.98 |
|  | 1 | 17.124 | 16.671 | **0.9735** [0.9730, 0.9741] | 0.9733 / 0.9738 / 0.9735 | 1.0000 | 5.14 | 5.00 |
| B21 | 0 | 6.727 | 6.554 | **0.9741** [0.9737, 0.9746] | 0.9745 / 0.9737 / 0.9743 | 1.0000 | 5.34 | 5.20 |
|  | 0.1 | 7.570 | 7.397 | **0.9770** [0.9766, 0.9774] | 0.9773 / 0.9766 / 0.9771 | 1.0000 | 5.52 | 5.40 |
|  | 1 | 15.158 | 14.984 | **0.9885** [0.9883, 0.9887] | 0.9887 / 0.9883 / 0.9886 | 1.0000 | 6.43 | 6.35 |
| B24 | 0 | 6.577 | 6.396 | **0.9725** [0.9722, 0.9728] | 0.9726 / 0.9726 / 0.9723 | 1.0000 | 5.43 | 5.28 |
|  | 0.1 | 7.333 | 7.152 | **0.9753** [0.9751, 0.9756] | 0.9754 / 0.9754 / 0.9752 | 1.0000 | 5.56 | 5.42 |
|  | 1 | 14.144 | 13.963 | **0.9872** [0.9871, 0.9874] | 0.9873 / 0.9872 / 0.9871 | 1.0000 | 6.14 | 6.06 |
| B27 | 0 | 6.702 | 6.601 | **0.9849** [0.9848, 0.9851] | 0.9850 / 0.9849 / 0.9848 | 1.0000 | 6.27 | 6.18 |
|  | 0.1 | 7.578 | 7.477 | **0.9867** [0.9865, 0.9868] | 0.9867 / 0.9867 / 0.9866 | 1.0000 | 6.49 | 6.40 |
|  | 1 | 15.461 | 15.360 | **0.9935** [0.9934, 0.9935] | 0.9935 / 0.9935 / 0.9934 | 1.0000 | 7.51 | 7.46 |

## Registered rule per row

| row | build adds saved k5 | build adds saved k4 | k5 max upper (pooled / replicate) | k5 holds (A/A gate) | k5 holds (sibling delta) | k4 max upper | k4 holds (A/A gate) | k4 holds (sibling delta) | delta (sibling) k5 |
|:--|--:|--:|:--|:--|:--|:--|:--|:--|--:|
| P20 | 20.0% | 7.9% | 0.9550 / 0.9561 | True | True | 0.9887 / 0.9890 | True | False | 1.027 |
| P22 | 15.5% | 5.3% | 0.9625 / 0.9633 | True | True | 0.9902 / 0.9904 | True | False | 1.019 |
| P24 | 12.4% | 3.8% | 0.9720 / 0.9729 | True | False | 0.9928 / 0.9931 | True | False | 1.062 |
| B15 | 36.3% | 17.7% | 0.8913 / 0.8927 | True | True | 0.9653 / 0.9659 | True | True | 1.035 |
| B18 | 36.3% | 17.7% | 0.9150 / 0.9179 | True | True | 0.9742 / 0.9748 | True | True | 1.034 |
| B21 | 20.0% | 7.9% | 0.9540 / 0.9550 | True | False | 0.9888 / 0.9892 | True | False | 1.074 |
| B24 | 20.0% | 5.3% | 0.9709 / 0.9720 | True | False | 0.9874 / 0.9876 | True | False | 1.127 |
| B27 | 10.1% | 2.8% | 0.9735 / 0.9741 | True | False | 0.9936 / 0.9937 | True | False | 1.098 |

Verdicts (`summary.json`): `ksum5_p_holds_all_rows` = True, `ksum4_p_holds_all_rows` = True, `correctness_ok` = True, `all_sessions_verified` = True.

## Deviations and limits

- Route: local, `--cpus none`, L0, counted cost only, instead of the isolab-eks default (as both siblings, at the director's instruction). Replay on the same host with the same binary, not a second host class.
- The two siblings' arms `inc-*`, `bsgs-neg` are not run; this study compares prune against no prune only.
- Warm-up round records are included in the correctness check but not in the ratios (ecbench excludes warm-up).
- `ksum_table_entries` is not in the record; correctness rests on identical insert attempts, hash probes, relation/trial/lookup counts and recovered logs.
- The `.jsonl` records of the 24 sessions are kept gzipped as the siblings do (`gunzip` before `ecbench band/compare/verify`).
- Table B rows, not new curves; the factor-base sizes were frozen from the siblings and not re-tuned for the pruned build (a smaller-base re-tune could move the optimum; not tested).
- `analysis/correctness.py` was edited after registration (hash in REGISTRATION.sha256 is of the first version) to read `records.jsonl` as well as `.jsonl.gz`; the checks themselves are unchanged. `analysis/summary.mjs`, `plot.mjs`, `report.py` were written after the data and are not in REGISTRATION.sha256 (they copy numbers; they do not decide a verdict).
