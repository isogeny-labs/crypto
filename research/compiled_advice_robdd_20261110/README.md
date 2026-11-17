# F5: ROBDD size of the decomposable set X₂ and the Legendre set X_E against random sets

**Answer.** In run F5-RUN-01 (n = 16, 18, 20, 22, 24; four variable orders; 10 random sets per row), the ROBDD node counts of X₂ and X_E lie between 0.994 and 1.029 times the mean node count of size-matched random subsets of [0, p), inside the frozen P1 band [0.9, 1.1] at every n and order; no ratio is at or below 0.9. The interval control INT has 12, 17, 20, 22, 24 nodes (at most 2n), and every ROBDD passed its verification. `[pcc:unverified]` Decision rule 3 applies (closed). The run measured ROBDD size only.

**Class:** boundary diagnostic; no S, no rho row, no leaderboard change.

All numbers below are measured values copied from `results/run.json` / `results/run.stdout.txt`; each is `[pcc:unverified]` (no pcc claim id exists for this run yet).

## Decision-rule reading (PROTOCOL.md §4)

1. **Instrument failure** (P2 fails or any verification fails): not triggered. INT nodes are 12, 17, 20, 22, 24 for n = 16, 18, 20, 22, 24 against bounds 2n = 32, 36, 40, 44, 48, identical in all four orders; `p2_all_interval_le_2n = true`. `all_verifications_ok = true` (all 60 ROBDDs verified: 15 rows x 4 orders). `[pcc:unverified]`
2. **Lead alive** (ratio ≤ 0.9 at every n for some order, and decreasing in n): not met. The smallest ratio over all 40 (n, set, order) cells is 0.994 and the largest is 1.029; no cell is ≤ 0.9, and `lead_alive_rule2 = false` for all eight (set, order) series in `summary`. `[pcc:unverified]`
3. **Closed** (applicable line, verbatim): "Closed: otherwise: the decomposable and Legendre sets are as incompressible as random sets under ROBDDs at these sizes. Says nothing about other compressed representations." The run's own `decision_rule_outcome` is `closed`. `[pcc:unverified]`

## Results

Per n: p = largest prime below 2^n; |X₂| and |X_E| are exact set sizes; node counts are exact ROBDD sizes in the orders msb, lsb (fixed) and sift_msb, sift_lsb (one budgeted Rudell sifting pass from msb / lsb). Ratio = nodes(X) / mean nodes(RAND, 10 sets); relsd = sample sd of the 10 RAND node counts divided by their mean. INT = interval [0, |X₂|). `[pcc:unverified]`

| n | p | set | \|X\| | nodes msb / lsb / sift_msb / sift_lsb | ratio to RAND mean (relsd), same order | INT nodes (4 orders) |
|---|---|---|---|---|---|---|
| 16 | 65521 | X2 | 400 | 892 / 834 / 872 / 831 | 1.029 (0.012) / 1.010 (0.006) / 1.000 (0.011) / 1.007 (0.009) | 12 / 12 / 12 / 12 |
| 16 | 65521 | X_E | 32775 | 8307 / 8290 / 8311 / 8284 | 1.001 (0.001) / 1.001 (0.001) / 1.001 (0.001) / 1.000 (0.001) |  |
| 18 | 262139 | X2 | 1018 | 2003 / 1942 / 2010 / 1943 | 0.994 (0.012) / 1.003 (0.010) / 0.996 (0.013) / 1.001 (0.007) | 17 / 17 / 17 / 17 |
| 18 | 262139 | X_E | 130988 | 31052 / 31031 / 31102 / 30992 | 0.998 (0.001) / 1.001 (0.001) / 1.000 (0.001) / 0.999 (0.001) |  |
| 20 | 1048573 | X2 | 1521 | 3510 / 3348 / 3482 / 3352 | 1.023 (0.007) / 1.005 (0.007) / 1.014 (0.010) / 1.005 (0.005) | 20 / 20 / 20 / 20 |
| 20 | 1048573 | X_E | 524784 | 107055 / 106938 / 107202 / 106863 | 1.000 (0.001) / 1.000 (0.000) / 1.001 (0.000) / 1.000 (0.000) |  |
| 22 | 4194301 | X2 | 5767 | 10288 / 10137 / 10293 / 10138 | 1.001 (0.006) / 1.002 (0.003) / 1.000 (0.006) / 1.005 (0.002) | 22 / 22 / 22 / 22 |
| 22 | 4194301 | X_E | 2097809 | 326495 / 326410 / 326428 / 326403 | 1.000 (0.000) / 1.000 (0.000) / 1.000 (0.000) / 1.000 (0.000) |  |
| 24 | 16777213 | X2 | 17411 | 28158 / 27794 / 28263 / 27905 | 0.997 (0.003) / 0.999 (0.002) / 1.003 (0.004) / 1.003 (0.003) | 24 / 24 / 24 / 24 |
| 24 | 16777213 | X_E | 8387895 | 1114066 / 1114057 / 1114064 / 1114055 | 1.000 (0.000) / 1.000 (0.000) / 1.000 (0.000) / 1.000 (0.000) |  |

RAND means (10 sets each, same size as X): `[pcc:unverified]`

| n | set | RAND mean nodes msb / lsb / sift_msb / sift_lsb (10 sets) |
|---|---|---|
| 16 | X2 | 866.7 / 826.0 / 871.7 / 824.9 |
| 16 | X_E | 8301.0 / 8282.2 / 8306.5 / 8281.0 |
| 18 | X2 | 2014.9 / 1937.0 / 2017.1 / 1941.6 |
| 18 | X_E | 31100.7 / 31001.2 / 31088.2 / 31008.4 |
| 20 | X2 | 3430.1 / 3332.4 / 3434.7 / 3336.0 |
| 20 | X_E | 107073.9 / 106903.3 / 107075.6 / 106904.4 |
| 22 | X2 | 10278.6 / 10112.7 / 10294.5 / 10092.5 |
| 22 | X_E | 326472.1 / 326418.6 / 326467.8 / 326404.0 |
| 24 | X2 | 28242.6 / 27835.5 / 28188.9 / 27825.1 |
| 24 | X_E | 1114069.6 / 1114052.2 / 1114069.1 / 1114054.0 |

Further recorded sizes: |F| = 40, 64, 78, 152, 264 and B = 41, 64, 102, 162, 256 for n = 16..24; X₂ raw events 1580, 4064, 6045, 23028, 69564 and collisions 1180, 3046, 4524, 17261, 52153. `[pcc:unverified]`

Largest single-cell deviations from the RAND mean in sd units (`z_score` in run.json, 40 cells): +3.44 (n = 20, X₂, msb), +2.37 (n = 20, X_E, sift_msb), +2.35 (n = 16, X₂, msb), -2.34 (n = 22, X_E, sift_msb). The X₂ ratios above 1 at n = 16 and 20 in the msb order are within the frozen band; no multiplicity correction was applied and none was preregistered. `[pcc:unverified]`

## Predictions

- **P1** (ratio in [0.9, 1.1] at every n and order, for X₂ and for X_E): **held**. X₂ ratios range 0.994 to 1.029; X_E ratios range 0.998 to 1.001 (X_E relsd ≤ 0.001). `p1_ratio_in_0.9_1.1` is true in all 40 cells. `[pcc:unverified]`
- **P2** (INT ≤ 2n nodes): **held**. 12 ≤ 32, 17 ≤ 36, 20 ≤ 40, 22 ≤ 44, 24 ≤ 48. `[pcc:unverified]`
- The frozen prior that P1 fails downward with a decreasing trend in n (0.003) was not realised: no ratio is below 0.994. `[pcc:unverified]`

## Exact null

`RAND(X)`: 10 uniformly random subsets of [0, p) with |RAND| = |X| (seed `F5/rand/<n>/<set>/<j>`, j = 0..9), built and measured with the same instrument, the same four orders and the same verification as X. The reference value is the mean over the 10 sets; the band in the figure is ±3·relsd with relsd computed from those 10 counts. Ten sets per row is the frozen sample size; it is a small sample for the relsd. `[pcc:unverified]`

## Cost counts (all recorded in run.json)

- Unique-table operations over the 15 X/INT rows x 4 orders (excluding RAND builds): 346,541,069. `[pcc:unverified]`
- Sifting swaps over the same rows and orders: 21,415 (nonzero only in the lsb and sift_lsb orders, which swap during the pass; the msb and sift_msb entries record 0). RAND-build swaps are not recorded separately. `[pcc:unverified]`
- RAND builds: unique-table operations are recorded as a per-row mean per order; summed as mean x 10 sets over all rows and orders they are 3,423,975,762 (derived). `[pcc:unverified]`
- Peak RSS: 610,432 kB (`peak_rss_kb`). `[pcc:unverified]`
- Per-row unique-table operations, hits, creations, reductions and swaps are under `rows[].orders.*.ops` in `results/run.json`.

## Execution route

Local host (4 vCPU, Linux 6.18.44-fc-v338, rustc 1.97.0 per the receipt). The §9a default route (isolab-eks, EKS Spot) confirmatory run is open: it has not been done. The host was shared; counts are exact and deterministic given the seeds, and no timing was taken, so no timing is a result. The receipt records a dirty working tree before and after (the card file and other studies' files); the card, protocol, source and lock hashes are listed below.

## What this does not show

The run says nothing about other compressed representations (e.g. BDD variants with other variable orders beyond the four tried, ZDDs, decision-tree or algebraic circuits, learned models), nothing about X₃ (three-sums), and nothing about n > 24. It measures ROBDD node counts of two sets at five sizes against 10 random sets each. It does not bear on any S, T or rho row.

## Sources checked

- `research/compiled_advice_robdd_20261110/PROTOCOL.md` (frozen protocol; sha256 e57b8ef0…76d1)
- `/home/user/crypto/research/prime_moonshot_20261110/ideas-generator.md`, idea record E1 (MOON-E1-compiled-advice-bdd-compressibility)
- `/home/user/crypto/RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §1 (the S·T² ≳ n preprocessing-bound framing)
- research-kb: unavailable in this session (not queried); no novelty or "not tried" claim is made here.

## Provenance

- Card: `CARD-compiled-advice-robdd-20261110-F5-RUN-01`, sha256 `e946af9134b86caed6dd735ad8904f262f88cb65f0aae12825de425357fe4a79`
- Commit: `da7ac2dbb8dc920652be41efb0be3ee14eaa17a7`
- Receipt `cards/receipts/F5-RUN-01.receipt.json`: sha256 `dd28fdb8f727c92a644ce34878d92c0d3b33860c1595e455c26f74a51bdeed05` (status completed, 13 commands)
- Binary (`tool/target/release/compiled_advice_robdd`): sha256 `7bb1a710a6ae3d3df15a78cde927275195f5cf3c521337d058c6bcbf339f8bb2`
- `results/run.json`: sha256 `1ab1b4fded683de2fd55b9e53e5f3830a47b9a7229d2f5b2c99d6e18631f0f9a`
- `FREEZE.sha256`: PROTOCOL.md e57b8ef0449a0353244770e439fc1c1ee7798f9a915e0d8d3cf49513fe4f76d1; tool/src/main.rs c0f41c930679fc016f29864e541bac351272dd49976dd8a23fd2a579e015d019; tool/Cargo.toml 01cec37983a65158e913ab81b4bc6d8ede694b1fb4050e526f6723d243155278; tool/Cargo.lock 2912a6c720234c63c2c8ed0622fc99d8ff566ee0b80f873b8df3e8aea0c08c29 (verified OK in the receipt)

## Files

- `README.md` (this file); `REPORT.typ`, `REPORT.pdf` (built with the typst Python wheel 0.15.2 in a scratch venv)
- `figures/ratio_vs_n.svg`, `figures/ratio_vs_n.png` (matplotlib 3.11.2; no rsvg-convert, cairosvg or inkscape on the host), `figures/ratio_vs_n.data.csv`, `figures/make_figure.py`, `figures/make_tables.py`, `figures/table.md`, `figures/table_rand.md`, `figures/table.typ`
- `results/run.json`, `results/run.stdout.txt`, `results/run.stderr.txt` (empty); `cards/`, `tool/`, `PROTOCOL.md`, `FREEZE.sha256`
