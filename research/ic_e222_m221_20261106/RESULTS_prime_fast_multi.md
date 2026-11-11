# prime_fast multi-target index calculus (frozen evaluation)

Status: [pcc:unverified] producer's own analysis, not independently replayed; no pcc standing; toy sizes.

Method (`prime_fast::ic_multi_solve`, `examples/prime_fast_multi.rs`): one shared system of relations `aG = sum s_j F_j` solves the factor-base logs once; each target Q_t then needs one relation `aG + bQ_t = sum s_j F_j` (3-term decomposition via the pair table, retried until every factor-base log it uses is determined), giving `log Q_t = (sum s_j x_j - a)/b`. Every log is verified `k G == Q_t`. Cost counted: factor base, pair table, relation collection, elimination, per-target relation searches, extraction and verification (all inside the timed region; planted targets are built outside it).

Protocol: constants frozen before the evaluation (`results/pf_multi/frozen.json`, commit 3211c092c9, verified on the remote before the run): c = 1.0, T = 1000 targets, extra relations 6. c was chosen from exploratory runs on the 24- and 28-bit curves, which the evaluation excludes. Evaluation on fresh curves of 23, 26, 30, 31 and 33 bits (a = -3, residue 3 and/or 1 as listed in the rows), 22 instance-rows, every one of the 22 000 targets verified, no failed run. Provenance: `provenance.txt` (commit and binary hash); raw rows `eval_*.jsonl`.

Reference arms: ic_solve summands 3 x T (median of 5 single solves, extrapolated), rho_parallel 1 thread with negation x T (median of 10 single solves), and an idealised multi-target rho = rho_single x sqrt(T) (not measured, favourable to rho).

Median over instance-rows (ratios are multi-target total time / reference time):

| bits | rows | setup ms | IC ms per target | vs ic_solve x T | vs rho x T | vs idealised multi-rho |
|---|---|---|---|---|---|---|
| 23 | 6 | 6 | 0.048 | 0.0075 | 0.021 | 0.65 |
| 26 | 6 | 37 | 0.21 | 0.0070 | 0.078 | 2.5 |
| 30 | 4 | 704 | 1.91 | 0.0026 | 0.41 | 12.9 |
| 31 | 4 | 852 | 2.28 | 0.0027 | 0.48 | 15.2 |
| 33 | 2 | 4207 | 8.33 | 0.0018 | 1.05 | 33 |

Findings:
1. Against the strongest implemented single-target IC repeated T times, multi-target amortization is a reproducible, verified improvement: 130x to 550x less total time at T = 1000 on fresh curves (the ic_solve arm is an extrapolation from 5 solves, not a run of T solves). This is the structure that single-target `ic_solve` lacks; it is not a new asymptotic exponent.
2. Against rho run once per target the advantage shrinks with size: 48x at 23 bits, 13x at 26, 2.5x at 30, 2.1x at 31, and none at 33 bits (ratio 1.05) at T = 1000. Larger T moves the break-even (earlier exploratory 32-bit runs: 0.46 at T = 10 000).
3. Against the idealised multi-target rho (sqrt(T) saving) IC wins only at 23 bits and loses by 2.5x to 33x from 26 bits; the gap grows with size. So no improvement over the best known rho strategy; the per-target sweep is O(fb) with fb ~ p^(1/3), which is what grows.

Limits: the idealised-rho arm is an assumption, not a run of a multi-target rho; no 5-parameter tuning per size; T = 1000 only in the frozen evaluation; 33 bits has 2 rows; 1 thread everywhere; no §5a pricing band; not independently replayed.

Next: an independent validator replay, then lower the per-target cost (the O(fb) sweep per walk step) with the table-lookup collector for the Q_t phase only, which the single-target evaluation could not exploit.

## Independent validation and required corrections (`results/pf_multi/VALIDATOR_REPORT.md`)

Validated: the measurement and finding 1. The validator replayed all 22 rows with the producer's seeds and verified 22 000 of 22 000 logs `k G = Q_t` in independent big-integer code; 16 rows re-run on a clean rebuild reproduced (identical `determined` counts, timing ratios 0.55 to 1.44, median 1.00; executable code identical, 37 bytes of build-id differ); mutation tests show `ic_multi_solve` cannot return an unverified log; the evaluation curves share no p, (p, a, b) or n with the 24/28/32-bit curves. Same machine, not an independent-host audit. Not eligible for the leaderboard (no `VALIDATION.json`, no §5a band, wall clock only).

Corrections that supersede the text above:
- Finding 1: per-size medians are 134x to 570x (rows 105x to 580x), not 130x to 550x; the `ic_solve` arm is an extrapolation.
- The rho arm is mis-sized: `rho_parallel` builds a 1024-entry table per solve (1.3 to 2.9 ms, about 80% of the 23-bit solve). With a right-sized table (32 entries, mean of 100 solves) the advantage over rho x T is **9.5x, 4.4x, 1.14x and 1.44x at 23, 26, 30 and 31 bits and IC is 1.4x slower at 33 bits**, not the figures in the table above (48x/13x/2.5x/2.1x/none).
- The idealised multi-target arm `rho_single x sqrt(T)` repeats that fixed cost 31.6 times, so it was not "favourable to rho" at small sizes. With the fixed cost paid once IC loses by **3.8x, 7.7x, 28.6x, 22.6x and 44.3x at 23, 26, 30, 31 and 33 bits**; "IC wins at 23 bits" is an artifact. No real multi-target rho was run; the validator's judgment (not a measurement) is that a real one would not be 3.8x or more worse than the corrected idealised arm.
- Remove "best known rho strategy" and the unsupported "0.46 at T = 10 000". Row medians are pseudo-replicated (10 independent rho reference measurements across 22 rows). `rho_parallel` thread sizing used the host core count (4 vCPU) before `threads = 1` was set; the count is not in `provenance.txt`. No (Q_t, k) certificate file, null control or bench record; the run was on the local sandbox, not isolab-eks.
- `RESULTS_prime_fast_vs_rho.md` probably carries the same rho-table inflation (not checked): its IC/rho ratios (5x to 257x) are therefore overstated for small sizes.

Net: multi-target amortization is a verified improvement over repeated single-target `ic_solve` (finding 1) and a marginal one over repeated single-target rho at 30 to 31 bits only; it does not approach a multi-target rho. Distance remaining against the corrected idealised multi-target rho: 3.8x to 44x, growing with size.
