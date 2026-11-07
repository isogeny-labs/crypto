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
