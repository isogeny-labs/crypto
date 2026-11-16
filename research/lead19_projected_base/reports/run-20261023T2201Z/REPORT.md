# Run report: LEAD-19 projected-base protocol, card 2, the pre-registered 20-repeat extension (2026-10-23T22:01Z)

Observations only (research-visuals skill, "Every experiment run").  The
interpretation is in [`../../README.md`](../../README.md); the protocol is
[`../../PROTOCOL.md`](../../PROTOCOL.md); card 1's report is
[`../run-20261023T2154Z/REPORT.md`](../run-20261023T2154Z/REPORT.md).

## What ran

| item | value |
|:--|:--|
| why | PROTOCOL.md §9: on the five-repeat run the f2m15 published-rung interval (0.82, 1.21) straddled 1.10, so the same char2 command re-ran with `--repeats 20`, nothing else changed |
| card | `cards/CARD-lead19-projected-base-20261023-2.yaml` (sha256 `e7922074…5b04`), compiled by `experiment-director`, run by `executor-mechanical` through `python3 tools/task_card.py run` |
| receipt | `cards/CARD-lead19-projected-base-20261023-2.receipt.json` (sha256 `fb790e1e…1e1b`), status `completed`, two commands, one attempt each, exit 0 |
| commit | `1cb4924a9aec75ea575969ce248619a558dabdd2` (the card's pin: card 1's records committed on top of the code commit `6783523a3`; the code is the same) |
| binary | `target/release/ic`, sha256 `373c0f1d…e2ec`, the binary card 1 bound; the build step reported nothing to do |
| command | `ic boundary --regime char2 --char2-degrees 15,18,21,24,27 --repeats 20 --seed 123212651130 --s4-max-degree 0 --base-arms raw,projected,subgroup --out research/lead19_projected_base/char2_repeats20.json`, 66.6 s |
| control | not re-run (the extension names the char2 command only); the analysis re-reads card 1's `prime24.json` for the control check, which passes unchanged |
| host | Intel Xeon @ 2.10 GHz (cloud container), 4 logical cores, linux x86_64; no isolation tier (counted operations) |
| raw record | `../../char2_repeats20.json` (5.9 MB); stdout and stderr under `../../receipts/` |
| analysis | `../../analysis_repeats20.json`, `../../analysis_repeats20.md`, by `docs/ic/tools/lead19_projected_base_compare.py` (the frozen plan, PROTOCOL.md §8); recorded values only |

Arms, instances and seeds as in card 1; twenty targets per instance, of
which repeats 0 to 4 are card 1's targets (same seeds, same counters) and
repeats 0 to 2 the frozen round-5 targets.  5 instances × 20 repeats × 3
recorded arms (A, B, C; A′ is derived from A) × 2 rungs = 600 arm rows
requested and completed; none failed or timed out.

## Stop checks (PROTOCOL.md §6)

| check | result |
|:--|:--|
| every row `verified`, `status: complete` | yes (`all_verified: true`) |
| prime-order control bit-identical across arms (card 1's record) | yes |
| arm A, repeats 0 to 2, equals the frozen round-5 counters on every char2 instance, both rungs | yes, no drift |

## Measured values

Same definitions as card 1's report (`S` in GAE/√r, every phase, cold;
geometric mean of the per-repeat paired ratio with the 95 % percentile
interval of 10,000 paired bootstrap resamples, seed 20261023; pooled yield).

| row | h | rung | S(A) | S(A′) | S(B) | S(C) | S(A′)/S(B) (95 % CI) | S(A′)/S(C) | yield B/A | yield C/A | rho | S/rho: A′, B, C |
|:--|--:|:--|--:|--:|--:|--:|:--|--:|--:|--:|--:|:--|
| f2m15 | 2 | std (published) | 27.75 | 15.88 | 12.35 | 12.16 | 1.29 (1.12, 1.52) | 1.30 | 1.86 | 1.87 | 2.02 | 7.8×, 6.1×, 6.0× |
| f2m15 | 2 | bal | 24.02 | 24.02 | 18.20 | 17.98 | 1.31 (1.04, 1.69) | 1.32 | 1.80 | 1.74 | 2.02 | 11.9×, 9.0×, 8.9× |
| f2m18 | 8 | std | 35.90 | 17.20 | 12.75 | 35.50 | 1.32 (1.20, 1.46) | 0.47 | 3.78 | 3.69 | 1.91 | 9.0×, 6.7×, 18.6× |
| f2m18 | 8 | bal | 20.24 | 20.24 | 12.25 | 25.76 | 1.58 (1.30, 1.91) | 0.73 | 3.67 | 3.41 | 1.91 | 10.6×, 6.4×, 13.5× |
| f2m21 | 2 | std | 23.35 | 15.46 | 10.11 | 10.83 | 1.51 (1.31, 1.73) | 1.40 | 2.03 | 1.93 | 1.24 | 12.4×, 8.1×, 8.7× |
| f2m21 | 2 | bal (published) | 22.84 | 22.84 | 14.29 | 14.29 | 1.58 (1.35, 1.82) | 1.59 | 1.96 | 1.90 | 1.24 | 18.4×, 11.5×, 11.5× |
| f2m24 | 8 | std | 34.66 | 24.02 | 17.08 | 27.79 | 1.39 (1.31, 1.48) | 0.86 | 3.84 | 3.89 | 1.28 | 18.8×, 13.4×, 21.7× |
| f2m24 | 8 | bal | 22.75 | 22.75 | 10.94 | 16.49 | 2.01 (1.70, 2.34) | 1.32 | 3.90 | 3.66 | 1.28 | 17.8×, 8.6×, 12.9× |
| f2m27 | 6 | std | 43.57 | 35.54 | 19.80 | 31.26 | 1.77 (1.57, 1.95) | 1.12 | 5.78 | 5.90 | 1.02 | 34.8×, 19.4×, 30.6× |
| f2m27 | 6 | bal (published) | 50.99 | 50.99 | 14.75 | 18.84 | 3.36 (2.80, 4.02) | 2.62 | 5.57 | 5.88 | 1.02 | 49.9×, 14.4×, 18.4× |

Best rung per arm (by mean `S` over the twenty repeats) and the paired
best-rung ratio: f2m15 std/std/std, S(A′)/S(B) 1.29 (1.12, 1.52); f2m21
std/std/std, 1.51 (1.31, 1.73); f2m27 A′ std, B bal, C bal, 2.41 (2.18, 2.67).

Per-repeat `S` on the three primary rows, published rung (twenty values,
repeat order):

| row | arm | S per repeat |
|:--|:--|:--|
| f2m15 std | A′ | 13.0, 16.7, 13.6, 17.7, 17.9, 23.1, 16.5, 16.2, 16.3, 18.3, 9.7, 16.4, 12.1, 22.3, 18.7, 14.8, 11.5, 11.9, 12.5, 18.6 |
| f2m15 std | B | 18.2, 16.1, 15.1, 12.6, 17.0, 11.8, 9.0, 14.7, 9.2, 8.7, 10.4, 12.6, 12.1, 7.4, 15.6, 11.0, 11.3, 13.1, 11.6, 9.7 |
| f2m15 std | C | 12.5, 9.9, 12.4, 13.8, 14.0, 11.1, 10.1, 10.3, 10.8, 12.3, 9.8, 14.1, 12.2, 13.3, 18.7, 12.7, 11.5, 8.3, 9.6, 15.9 |
| f2m21 bal | A′ | 16.9, 28.1, 28.2, 19.5, 22.1, 20.0, 24.0, 11.9, 17.9, 15.9, 31.6, 14.9, 17.8, 17.7, 33.6, 23.7, 26.0, 26.5, 35.7, 24.8 |
| f2m21 bal | B | 13.0, 16.1, 14.5, 14.5, 15.1, 18.3, 13.9, 17.2, 16.6, 8.5, 16.9, 15.7, 9.7, 10.7, 17.8, 12.4, 13.3, 7.5, 16.1, 18.0 |
| f2m21 bal | C | 14.1, 19.0, 9.2, 10.4, 9.0, 20.0, 13.1, 14.1, 14.7, 9.2, 18.8, 16.1, 21.0, 11.7, 11.8, 10.4, 13.9, 11.6, 20.9, 16.8 |
| f2m27 bal | A′ | 25.5, 59.5, 25.7, 29.2, 67.4, 60.2, 42.7, 64.2, 60.2, 53.6, 68.3, 47.0, 51.7, 69.2, 44.4, 63.6, 31.4, 43.7, 71.0, 41.1 |
| f2m27 bal | B | 16.9, 7.5, 14.8, 13.1, 16.5, 19.5, 15.5, 18.3, 16.9, 12.5, 12.0, 12.9, 14.0, 12.9, 16.0, 16.9, 17.6, 14.4, 12.8, 14.0 |
| f2m27 bal | C | 21.9, 18.0, 16.5, 18.5, 13.8, 20.1, 20.6, 13.6, 21.9, 16.2, 18.7, 21.0, 18.9, 20.1, 26.2, 23.8, 12.5, 19.4, 15.0, 20.3 |

Trials and relations, primary rows, published rung, pooled over twenty
repeats (sums of the per-repeat values in `analysis_repeats20.json`):

| row | arm | trials (sum) | relations (sum) | relations per trial |
|:--|:--|--:|--:|--:|
| f2m15 std | A | 13,839 | 194 | 0.0140 |
| f2m15 std | B | 8,416 | 219 | 0.0260 |
| f2m15 std | C | 8,479 | 222 | 0.0262 |
| f2m21 bal | A | 403,731 | 372 | 0.00092 |
| f2m21 bal | B | 231,294 | 417 | 0.00180 |
| f2m21 bal | C | 231,445 | 406 | 0.00175 |
| f2m27 bal | A | 4,388,299 | 1,113 | 0.000254 |
| f2m27 bal | B | 1,001,477 | 1,415 | 0.00141 |
| f2m27 bal | C | 905,295 | 1,349 | 0.00149 |

`yield_over_ceiling_exact` (mean over repeats) on those rows: A 1.06 / 1.02 /
1.03, B 1.08 / 1.00 / 0.98, C 0.99 / 1.00 / 1.06 (f2m15 / f2m21 / f2m27):
every arm realises its exact counting ceiling; the projection raises the
ceiling, not the realisation.

Phase split of `S` at the std rung, mean over the twenty repeats, read from
the record's `factor_base` and `relations` phases (the linear algebra and
verification phases are the small remainder):

| row | h | A base / relations | B base / relations | C base / relations |
|:--|--:|:--|:--|:--|
| f2m15 | 2 | 13.76 / 13.84 | 2.01 / 10.19 | 1.91 / 10.10 |
| f2m18 | 8 | 24.56 / 11.23 | 6.38 / 6.24 | 29.18 / 6.20 |
| f2m21 | 2 | 11.59 / 11.73 | 3.76 / 6.32 | 3.71 / 7.09 |
| f2m24 | 8 | 23.69 / 10.94 | 13.34 / 3.71 | 24.08 / 3.68 |
| f2m27 | 6 | 22.72 / 20.84 | 14.86 / 4.93 | 26.51 / 4.74 |

Class census, base-phase counters and the class test removed for A′ are
the same as in card 1's report (same curves, same bases).

Pricing band: arm C's trace test at w = 0, 0.1, 1 GAE moves S(C) by at most
0.25 (f2m15 std, 12.16 → 12.41); every A′ > B and A′ > C order holds across
the band.

Predictions (PROTOCOL.md §7), as the analysis script evaluated them:

| row | P1 (predicted) | P2 yield B/A (band) | P3 S(A′)/S(B) published rung (predicted) | P4 yield B/C | P5 |
|:--|:--|:--|:--|:--|:--|
| f2m15 | 0.502 (0.50): within 10 % | 1.86 ([1.4, 2.6]): holds | 1.29, CI (1.12, 1.52) (1.4–1.5) | 0.99: within 15 % | identical |
| f2m21 | 0.501 (0.50): within 10 % | 1.96 ([1.4, 2.6]): holds | 1.58, CI (1.35, 1.82) (1.4–1.5) | 1.03: within 15 % | identical |
| f2m27 | 0.167 (0.17): within 10 % | 5.57 ([4.2, 7.8]): holds | 3.36, CI (2.80, 4.02) (about 3) | not applicable | identical |

Published-row ratio (accounting, not the lever; frozen round-5 mean S over
this run's mean): f2m15 26.31 / 12.35 = 2.13 (B), 2.16 (C); f2m21 24.39 /
14.29 = 1.71 (B), 1.71 (C); f2m27 36.93 / 14.75 = 2.50 (B), 1.96 (C).

The frozen comparison's verdict line (PROTOCOL.md §9): **success on rows
n = 15, n = 21, n = 27**.

## Graphs

![S per arm, rung and repeat (twenty repeats); dashed line the matched rho](s_by_arm.svg)

![Paired ratios S(A′)/S(B) with the bootstrap interval and S(A′)/S(C), against the 1.10 line](ratios.svg)

Sources beside the graphs: `../../plot_lead19.py` reading `../../analysis_repeats20.json`.

## Missing or not measured

- As in card 1: no isolation tier, no wall-clock claim, no `S₄` arm, and no
  replay certificate, validator report or `VALIDATION.json` yet.
- The record's `host.git_commit` is `null` (the binary was built outside a
  git-aware step); the card's receipt pins the commit.
