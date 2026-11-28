# Round 39: replay at 64 samples and an independent recompute of the holdout ratio

Written 2026-11-17 by the producing session, so this is a producer-side check, not an independent validation [pcc:unverified]. Claim `PCC-20261117-15fb13` stays at unverified standing; a validator run by another session is still the step toward a gate row.

## Replay at eight times the loop's depth

`ecbench verify --dir sessions/holdout-k28 --replay 64` (the loop's audit uses 8): 5760 records, 5760 verified, 64 replays reproduced, 0 problems. Receipt: `verify-holdout-k28-replay64.json`. The replay seed was drawn by the tool (17408933821038403668); 5120 runs were eligible. [pcc:unverified]

## Recompute of the statistic from raw records

`paired_ratio.py` reads the extracted `records.jsonl` (committed as `sessions/holdout-k28/records.jsonl.xz`, see `README-records.md` there; extract with `xz -dk`) without ecbench code, pairs `inc` and `sel` by (workload, round) over 128 workloads x 8 rounds = 1024 pairs, and gives (`paired_ratio.out`):

| quantity | recompute | `ecbench compare` (round README) |
|:--|--:|--:|
| sel / inc, ratio of means, w = 0 | 0.9439 | 0.944 |
| 95% interval, workload-clustered bootstrap | [0.9184, 0.9690] | [0.919, 0.970] |
| 95% interval, plain pair bootstrap | [0.9206, 0.9673] | - |
| mean S inc / sel | 1.9030 / 1.7962 | 1.903 / 1.796 |

The two agree to the third decimal. The recompute covers w = 0 only; the w = 0.1 and w = 1 ratios and the band order stay as `ecbench compare` and `band --ranks` report them.

## What this does not establish

- The A/A ratio is exactly 1.0000: counted GAE is deterministic and `inc-aa` runs the same method on the same seeds, so the A/A arm shows reproducibility, not a noise floor. Variation across workloads is what the interval measures.
- It does not check the cost model (which operations are priced, `unpriced` list in each record) or the matched rho reference; the §5a band is the loop's, unchanged.
- The checks use the same binary (Nov-14 `ecbench`) and the same session; a second host or an independently written counter would be a separate step.

## Replay with a binary built from a known source (2026-11-18)

Validator finding F2 was that the Nov-14 `ecbench` (sha256 581b3c6f...) had no recorded build commit and the standalone `hashed-ecdl-tensor` crate did not build on `main`, so "same source as the freeze commit" was unproven. The crate builds on `origin/main` now (`cargo check --release -p hashed-ecdl-tensor` passes). I built `ecbench` and `ic_autolab` from `origin/main` at `ad2a014c0022cf000845014499a9649b6c2cef2d` (`cargo build --release --bin ecbench --bin ic_autolab`, 8m38s, clean tree) and ran `ecbench verify --dir sessions/holdout-k28 --replay 256` on the committed session (extracted `records.jsonl`, sha256 3347420e...).

Result: 5760 records bound, 5760 verified, **256 replays reproduced, 0 problems**; the auditor binary is sha256 `1b86126f59cc2c4e59d01cafb987c4dc98a28e2eb95dfd91580d1a89252d1bda`. Receipt: `verify-holdout-k28-replay256-main-build.json`. [pcc:unverified]

What this adds: the recorded counted costs of round 39 are reproduced by a binary of identifiable source that differs from the one that produced them (different sha256, about three weeks of `src/` changes in between), 256 sampled runs across all five arms. What it does not add: it is still this session's replay, on the same host class, so it is not an independent validation; it re-executes runs and does not re-derive the ratio or re-measure with fresh targets; and the freeze commit's own source (`7a1cd931e3`) was not rebuilt. Claim `PCC-20261117-15fb13` stays at unverified standing; no `VALIDATION.json`.
