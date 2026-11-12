# F4: SAT on the bit-level x-only Montgomery ladder, conflict-count scaling and clause reuse

**Status:** run F4-RUN-02 complete; the replication that decision rule 2 prescribes (second solver, 40 targets) is pending and has not been run. Every number in this file is `[pcc:unverified]` (no pcc claim id exists for this run). No improvement over rho, or over anything, is claimed.

**Class:** boundary diagnostic; no S, no rho row, no leaderboard change.

## Purpose and mechanism

The study measures how the number of CDCL conflicts needed to recover a scalar `k` from `x(Q) = x([k]G)` grows with the field size `n`, when the problem is posed as a SAT instance of the bit-level projective x-only Montgomery ladder (the `nk = ceil(log2 r)` bits of `k` are the only free inputs; each ladder step is a differential add plus a double, Tseitin-encoded with schoolbook multipliers and modular reduction by quotient variables). The frozen reference exponent is exhaustive-like (`alpha` about 1, so `log2 conflicts` grows like `n`) against rho's `0.5`. It uses CaDiCaL 3.0.1 with default options, one prime-order curve per `n`, and 10 targets per `n`.

Two arms: FRESH builds a new solver for every target; REUSE keeps one incremental solver per curve, passes the target `x_Q` bits as assumptions, and solves the same targets in a seeded random order, so learned clauses persist from one solve to the next. Clauses learned on one target are advice for later targets, so any amortisation falls under the `S * T^2 >~ n` preprocessing bound. Metric: conflicts (primary); decisions and propagations are in `results/run.json`; wall time is descriptive only. Censored solves (budget 4e6) are reported as censored, never imputed.

## Frozen predictions and decision rules (PROTOCOL.md section 3 and 4, verbatim)

- **P1.** Exponent `alpha` of median FRESH conflicts in `n` (`log2 conflicts ~ alpha n + c`, fitted on the uncensored `n`): `alpha in [0.85, 1.15]` (exhaustive-like); rho is `0.5`.
- **P2.** REUSE / FRESH median conflict ratio at each `n` >= 0.5.
- **P3.** All models decode to the true `k`.
- Prior that `alpha` upper CI `< 0.6`: 0.01.

1. **Instrument failure:** P3 fails, or fewer than three `n` are uncensored.
2. **Lead alive:** `alpha` upper 95 % CI `< 0.6` with `n = 18` or `20` uncensored, **or** REUSE ratio `< 0.2` at two consecutive `n`. Then replicate with a second solver and 40 targets.
3. **Closed:** otherwise. Reading: CDCL on the ladder circuit is exhaustive-like and clause reuse does not amortise below the generic bound at these sizes.

## Amendment before the run (PROTOCOL.md section 6, summarised)

Predictions and rules were committed (`c8bbc80e1a`) before the tool existed. The implementer's development runs (not committed, not scored) showed about 5x more conflicts per added bit, and at `n = 10, 12` every solve censored at 2e4 conflicts, so the frozen grid `n in {10, ..., 20}` at 1e7 would have been censored throughout and tripped rule 1 by construction (a grid-design error). **Amendment (grid and budget only):** `n in {4, ..., 9}`, 10 targets per `n` (not 20), conflict budget `4e6`, `--threads 2`. P1-P3 and rules 1-3 are unchanged; rule 1's "fewer than three `n` uncensored" refers to the new grid; rule 2's thresholds are unchanged, and if its clause-reuse clause fires on these sizes the replication is run and its reading stated with `alpha_REUSE`. Instrument facts: CaDiCaL 3.0.1 via `cadical-sys 0.8.0`; propagations are `propagations.search`; x-only decoding accepts `k' in {k, r - k}`. Expectation stated before the run: `alpha_FRESH` well above 1 and P1 failing upward.

## Result table (exactly as in `results/run-02.stdout.txt`)

```
  n      vars    clauses |    FRESH med  cens |    REUSE med  cens |   ratio
  4      6562      31326 |      11393.5     0 |         57.5     0 |   0.005
  5      8651      41523 |       9856.0     0 |         29.5     0 |   0.003
  6     17833      90762 |     250058.5     0 |       1240.5     0 |   0.005
  7     23079     115102 |     296736.0     0 |       1027.0     0 |   0.003
  8     36220     188187 |         cens     6 |       4240.0     1 |     n/a
  9     43236     227543 |         cens     8 |    1080637.0     3 |     n/a
alpha FRESH = 1.877 [1.675, 2.188] over n=[4, 5, 6, 7];  alpha REUSE = 1.787 [0.365, 3.904] over n=[4, 5, 6, 7]
P1=Some(false) P2=Some(false) P3=true verdict: lead alive (rule 2)
```

Columns: medians of conflicts over 10 targets; `cens` = censored count of 10; ratio = REUSE median / FRESH median (n/a where the FRESH median is censored). `[pcc:unverified]`

- `alpha_FRESH = 1.877`, 95 % CI [1.675, 2.188] (bootstrap, 1000 resamples), fitted on the uncensored `n` = 4, 5, 6, 7. `[pcc:unverified]`
- `alpha_REUSE = 1.787`, 95 % CI [0.365, 3.904], same `n` (wide: four points, medians that include sub-1000 values). `[pcc:unverified]`
- P1 false (`alpha` above [0.85, 1.15], upward, as stated before the run); P2 false; P3 true: 102 of 120 solves found a model, all 102 verified and decoded to `k' in {k, r - k}`; 18 solves censored (6 + 8 FRESH, 1 + 3 REUSE). `[pcc:unverified]`

Figure 1 (`figures/conflicts_vs_n.svg`, `.png`): log2 median conflicts vs `n`, both arms; open markers with arrows are medians that are censored (lower bounds); the FRESH fit with its `alpha` and CI; the dotted line of slope 0.5 through the FRESH `n = 4` point is a reference slope for scale, not a cost comparison.

## Rule application (verbatim from the session log, 2026-11-11 23:10Z)

- Rule 1: **no** (P3 holds, four uncensored `n`).
- Rule 2: **yes, by its clause-reuse clause** (REUSE/FRESH ratio < 0.2 at every `n` from 4 to 7; the alpha clause does not fire: upper CI 2.188, and `n` = 18 or 20 is not in the grid).
- So the rule's consequence applies: "replicate with a second solver and 40 targets". That replication is prescribed and **pending**; this run does not carry it out. `[pcc:unverified]`

## Decomposition of the REUSE arm by solve order

Derived from the `rows` of `results/run.json` (REUSE rows with `solve_order` 0..9; the targets and their `k` are identical across the arms, checked). Conflicts of each solve, in the order the incremental solver ran them. `[pcc:unverified]`

| n | r | REUSE conflicts by solve_order 0, 1, ..., 9 (c = censored at 4e6) |
|---|---|---|
| 4 | 19 | 14679, 628, 115, 0, 0, 0, 0, 346, 411, 0 |
| 5 | 29 | 24724, 1131, 909, 34, 14, 0, 0, 43, 10, 25 |
| 6 | 71 | 351983, 13732, 2, 17207, 1417, 0, 1064, 2570, 72, 3 |
| 7 | 131 | 301358, 1011, 1021, 1004, 1033, 1005, 1300, 13439, 1064, 1021 |
| 8 | 257 | 4000001 c, 329277, 65830, 1033, 1010, 121267, 7447, 135, 0, 0 |
| 9 | 479 | 4000000 c, 4000003 c, 4000000 c, 1520918, 2683356, 640356, 395495, 572444, 1030, 9661 |

First solve against later solves (the medians of orders 1-9 are derived here: the median of the nine values per `n`):

| n | first solve (order 0) | FRESH median | warm median (orders 1-9) | warm censored | distinct x_Q among 10 targets / (r-1)/2 |
|---|---|---|---|---|---|
| 4 | 14679 | 11393.5 | 0 | 0 | 7 / 9 |
| 5 | 24724 | 9856.0 | 25 | 0 | 7 / 14 |
| 6 | 351983 | 250058.5 | 1064 | 0 | 10 / 35 |
| 7 | 301358 | 296736.0 | 1021 | 0 | 9 / 65 |
| 8 | 4000001 c | cens | 1033 | 0 | 10 / 128 |
| 9 | 4000000 c | cens | 640356 | 2 | 10 / 239 |

Description: the first solve costs about the same as a FRESH solve (n = 4..7: 1.29, 2.51, 1.41, 1.02 times the FRESH median); later solves cost far less (zero conflicts for some solves at n = 4..6 and 8; a flat band of about 1000 at n = 7, with one 13,439 at order 7); at n = 8 later conflicts fall over the solve order (329277, 65830, ... to 0); at n = 9 the first three solves censor, orders 3 to 7 cost 3.95e5 to 2.68e6, and only orders 8 and 9 are small (1030 and 9661). Figure 2 (`figures/reuse_by_order.svg`, `.png`) plots these per `n` on a log scale, zeros drawn at a floor of 0.5, with the FRESH medians as horizontal lines (drawn at the budget where censored). `[pcc:unverified]`

## Two readings, stated neutrally

- **Table exhaustion of a tiny group.** The group orders are r = 19, 29, 71, 131, 257, 479 and the 10 targets cover a sizeable share of the (r-1)/2 x-coordinate classes at small `n` (7 of 9 at n = 4, 7 of 14 at n = 5). Learned clauses could then amount to a partial table of the group, built during the first solve, so later solves are lookups; in this reading the warm per-target cost would not be below what a table of size about r gives, and would grow with r.
- **Amortised inversion.** The learned clauses could instead encode reusable structure of the ladder circuit, so that a warm solve costs less than the first by a factor that does not simply track r, and the first-solve cost is a one-off investment.

The run does not separate them. **Discriminating quantity:** the warm per-target cost (conflicts of solves after the first, per target) against the group order r, measured over more targets and on a second solver; the replication measures this. `[pcc:unverified]`

## Limitations

- Local host, 4 vCPU, shared CPU (concurrent cards ran; the receipt records waits on the cargo package-cache lock); the Section 9a isolab-eks confirmatory route has not been used. Counts are the result; wall time (`wall_ms` in `run.json`) is descriptive only.
- 10 targets per `n` (20 were frozen, amended to 10); the x_Q values are not all distinct (distinct counts in the table above), so the effective sample at n = 4, 5 is smaller.
- n = 8 and 9 are censored for FRESH (6 and 8 of 10), so the fit uses four points (n = 4..7), with a CI that is a bootstrap over 10 targets per point; the REUSE fit has a very wide CI.
- x-only decoding accepts `k' in {k, r - k}`: a model is a solve if `x([k']G) = x_Q`.
- The grid is `n <= 9`; nothing is measured at 18 or 20 bits, and "rule 2 by the alpha clause" cannot occur on this grid.
- Single solver (CaDiCaL defaults), single curve per `n`.

## Evidence pointers

- Card `cards/F4-RUN-02.json` (CARD-sat-ladder-scaling-20261110-F4-RUN-02), sha256 `89d464edf4e769b2f51d2304deefabd8057662a13884946d444748df3639a604`.
- Receipt `cards/receipts/F4-RUN-02.receipt.json` (status completed, 13 commands, each one attempt, exit 0); director assessment `cards/receipts/F4-RUN-02.assessment.md`.
- Pinned head `9465664dbe6ba5757b9cb011192215a00605e8d8` (`F4-RUN-02.git-head.stdout`).
- Binary `tool/target/release/sat_ladder_scaling`: sha256 `8e6cc0b7945ee2658ec2f92bb4a7a56ec7d2e79edb02316e8e2332792aef0ca0`.
- `results/run.json`: sha256 `a12f6c56bf42ba816a54149b13eb3cfd6d97751a68225f3331be2c7cd1425079` (120 rows = 6 n x 2 arms x 10 targets); table in `results/run-02.stdout.txt` and `cards/receipts/F4-RUN-02.result-table.stdout`.
- FREEZE.sha256 (verified OK in `F4-RUN-02.verify-freeze.stdout`): PROTOCOL.md `156fe81023a340fa7431f870b3f60a58457fa236a57325100ec582f514c0bbbc`; tool/src/main.rs `6627a93246f7f4b0445a9a380d4f2e9d016ea001a0b7b671ac328f5e18c5a9b1`; tool/Cargo.toml `b94869ee38dcd3f7038ddd9b467e481d6ba51d821057765f391348985a53b0a8`; tool/Cargo.lock `b4685bb42c0da6592fa867809a194bdfee5c2c4b63c4b0d5d3f6674d8cb496b7`.
- Tests: `F4-RUN-02.test.stdout`: 7 passed, 0 failed, 1 ignored (n = 6 planted test). Host: rustc 1.97.0, cargo 1.97.0, Linux 6.18.44-fc-v378, 4 CPUs.
- The stale `results/run.stdout.txt` and `run.stderr.txt` belong to the lost F4-RUN-01 (container restart) and are not evidence for this run.
- Sources checked: PROTOCOL.md, the F4-RUN-02 card and receipts, `docs/session-logs/2026-11-10.md` and `2026-11-11.md` (F4 entries). research-kb was not queried; no novelty or "not tried" claim is made.

## Files

- `README.md` (this file); `REPORT.typ`, `REPORT.pdf` (typst Python wheel 0.15.2 in a scratch venv).
- `figures/conflicts_vs_n.svg`, `.png`; `figures/reuse_by_order.svg`, `.png` (matplotlib 3.11.2); `figures/plot.py` (draws only from `results/run.json`); `figures/table.typ`, `table_order.typ`, `table_warm.typ`.
- `results/`, `cards/`, `tool/`, `PROTOCOL.md`, `FREEZE.sha256` (inputs, unchanged).
