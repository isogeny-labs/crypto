# F6: local search and annealing on the m = 2 decomposition landscape

**Answer.** In run F6-RUN-01 (n = 12, 16, 20, 24, 28; B = ceil(sqrt p) = 64 ... 16384; 20 planted instances per n), none of the six solvers (uniform random sampling, greedy with restarts, simulated annealing at three schedules, parallel tempering) needed fewer cost evaluations than root-solving enumeration, and none has a fitted exponent in B near it: the six solver exponents over all n are 1.820 to 2.197, the root-solving enumeration measured 0.901. Uniform sampling, the control, has e = 1.948. Greedy is the lowest at 1.820 (all n) and 1.851 (n >= 20). All 100 instances x 7 solvers (700 rows) verified (0 verification failures, 0 instrument errors). `[pcc:unverified]` Decision rule 3 applies (closed). The run measured these solvers on this landscape only.

**Class:** boundary diagnostic; no S, no rho row, no leaderboard change.

All numbers below are measured values copied from `results/run.json` / `results/run.stdout.txt`; each sentence carrying one is `[pcc:unverified]` (no pcc claim id exists for this run yet).

## Decision-rule reading (PROTOCOL.md §4)

1. **Instrument failure** (P3 fails, or a reported solution does not verify): not triggered. P3: the uniform exponent is 1.948 (all n), inside 2 ± 0.25 (|1.948 − 2| = 0.052); `verification_failures = 0`, `instrument_errors = []`, `P3_ok = true`. `[pcc:unverified]`
2. **Lead alive** (some solver's e upper 95 % CI < 1.0 on uncensored n >= 20, or P2 fails at lag 1 on >= 3 sizes): not met. The smallest n >= 20 upper CI over the six solvers is 1.614 (greedy; CIs: uniform 2.251, greedy 2.105, sa_B 2.440, sa_4B 2.486, sa_16B 2.474, pt 2.506); none is < 1.0 (`lead_alive_solver_ci_hi_lt_1_on_n_ge_20 = false`). P2 at lag 1 is in band at all three landscape sizes (walk mean −0.0006 / +0.0003 / −0.0012 for n = 12 / 16 / 20, bands about ±0.094), so `P2_lag1_fail_sizes = 0`, not >= 3. `[pcc:unverified]`
3. **Closed** (applicable line, verbatim): "Closed: otherwise: no local-search method beats root-solving enumeration on the `m = 2` decomposition landscape at these sizes." The run's own verdict is `closed`. `[pcc:unverified]`

## Results

p = largest prime ≡ 3 (mod 4) below 2^n (the generator reproduces the committed rungs at n = 16, 20, 24, 28); B = ceil(sqrt p); mean box solutions = mean over 20 instances of `box_solutions_ordered` (ordered solutions in the box, planted one included). Entries are the median over 20 instances of cost evaluations to first verified c = 0, with the number of instances censored at the cap in brackets. root_enum counts x1 values tried (one quadratic solve each), not cost evaluations of the same unit. `[pcc:unverified]`

| n | p | B | B² | mean box solutions | uniform | greedy | sa_B | sa_4B | sa_16B | pt | root_enum |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 12 | 4091 | 64 | 4,096 | 2.80 | 922 [0] | 2,466 [0] | 3,636 [0] | 2,354 [0] | 5,162 [0] | 4,783 [0] | 25 [0] |
| 16 | 65519 | 256 | 65,536 | 3.40 | 13,792 [0] | 35,242 [0] | 59,672 [0] | 89,760 [0] | 71,861 [0] | 33,464 [0] | 42 [0] |
| 20 | 1048571 | 1,024 | 1,048,576 | 2.75 | 191,126 [0] | 394,924 [0] | 2,466,308 [0] | 1,722,998 [0] | 893,260 [0] | 939,996 [0] | 290 [0] |
| 24 | 16777199 | 4,096 | 16,777,216 | 3.20 | 3,898,006 [0] | 4,346,316 [0] | 19,692,566 [0] | 13,430,037 [0] | 34,937,164 [0] | 30,191,004 [0] | 640 [0] |
| 28 | 268435399 | 16,384 | 268,435,456 | 3.30 | 40,138,032 [0] | 66,891,984 [0] | 820,856,178 [2] | 765,670,044 [2] | 367,805,855 [0] | 395,638,322 [2] | 3,324 [0] |

(Medians are listed as rounded in `run.stdout.txt`; `run.json` holds the half-integer values, e.g. uniform n = 12 is 921.5.) The six censored rows are all at n = 28 (below).

### Exponent e (median ~ B^e, ln-ln OLS, bootstrap 95 % CI over instances, 1000 reps) `[pcc:unverified]`

| solver | all n (5 sizes): e [CI] | n >= 20 (3 sizes): e [CI] |
|---|---|---|
| uniform | 1.948 [1.790, 2.115] | 1.929 [1.572, 2.251] |
| greedy | 1.820 [1.668, 2.003] | 1.851 [1.614, 2.105] |
| sa_B | 2.197 [2.050, 2.321] | 2.095 [1.761, 2.440] |
| sa_4B | 2.192 [1.993, 2.372] | 2.199 [1.875, 2.486] |
| sa_16B | 2.058 [1.835, 2.304] | 2.171 [1.593, 2.474] |
| pt | 2.124 [1.981, 2.325] | 2.179 [1.791, 2.506] |
| root_enum (baseline) | 0.901 [0.797, 1.038] | 0.879 [0.626, 1.135] |

The n >= 20 CIs rest on three sizes and are wide (about ±0.3 to ±0.4 around the estimate). The root_enum n >= 20 CI (upper 1.135) is itself not below 1.0.

### Landscape autocorrelation (bit-flip walks vs random landscape with the same marginal; 640 walks per n; band = random mean ± 3 sd) `[pcc:unverified]`

| n | lag-1 walk mean | lag-1 random mean (sd) | lag-1 in band | lags 2..8 in band | lag-2 walk mean (band) | lag-2 advisory z of mean difference |
|---|---|---|---|---|---|---|
| 12 | −0.0006 | +0.0003 (0.0313) | yes | yes | +0.0836 (−0.095, +0.090) | 49.6 |
| 16 | +0.0003 | +0.0012 (0.0316) | yes | yes | +0.0618 (−0.096, +0.090) | 36.6 |
| 20 | −0.0012 | −0.0004 (0.0315) | yes | yes | +0.0486 (−0.089, +0.094) | 26.3 |

Landscape statistics are computed for n <= 20 only. The band is ± 3 sd of single walks, so it is wide; the run records an advisory z of the difference of means, which is large at lag 2 (49.6, 36.6, 26.3), and 11.4, 7.1, 3.0 at lag 4. Under the protocol's in-band convention every lag passes (`P2_all_lags_in_band = true`); the lag-2 and lag-4 mean excess over the random landscape is visible in the advisory z and is reported here, not scored. The lag-2 excess is a property of the walk, not of the landscape `[derived]`: the walk flips one of the `2·log₂B` Gray-code bits per step, so with probability `1/(2·log₂B)` a step undoes the previous one and the state (hence `c`) repeats exactly, contributing that probability to the lag-2 autocorrelation. Predicted excess 1/12 = 0.0833, 1/16 = 0.0625, 1/20 = 0.0500 at n = 12, 16, 20; measured lag-2 walk means 0.0836, 0.0618, 0.0486 (random landscape −0.003 to +0.003). The lag-4 excess is the same mechanism over two-step return paths. The shuffled random-landscape control draws an independent state per step and so has no returns; it is the right null for landscape smoothness but not for walk revisits. The lag-1 statistic, which P2 and rule 2 gate on, is unaffected. `[pcc:unverified]` for the measured values.

## Predictions

- **P1** (median evaluations ∝ B^e with e >= 1.8 for every solver): **held under the protocol's wording, by point estimates, with the margin at greedy**. The protocol states the threshold on e, not on its interval. Every solver's fitted e (all n) is >= 1.8: uniform 1.948, greedy 1.820, sa_B 2.197, sa_4B 2.192, sa_16B 2.058, pt 2.124 (`P1_all = true`, `e_ge_1.8` true for each). Greedy clears the threshold by 0.020 (all n) and 0.051 (n >= 20: 1.851). Its CIs, [1.668, 2.003] and [1.614, 2.105], contain 1.8, so the data do not separate greedy's exponent from values below 1.8; the protocol froze no CI test for P1. `[pcc:unverified]`
- **P2** (autocorrelation within the random band at every lag >= 1): **held** under the in-band convention (mean within random mean ± 3 sd over walks): all 8 lags at n = 12, 16, 20. With the caveat above (advisory z large at lags 2 and 4). `[pcc:unverified]`
- **P3** (uniform sampling has e ≈ 2): **held**: 1.948 (n >= 20: 1.929), |e − 2| = 0.052 <= 0.25. `[pcc:unverified]`
- Priors (e <= 1 for any solver: 0.005; e < 0.5: 0.0005): no solver reached e <= 1; the minimum solver e is 1.820 (point), minimum n >= 20 lower CI 1.572 (uniform). `[pcc:unverified]`

## Cap note (a labelling mismatch, disclosed)

The protocol text in §2 says the cap is `50·B²`; the §6 amendment (frozen before the run, in the hashed PROTOCOL.md) sets `min(50·B², 2·10⁹)`. The tool source computes `cap = min(50·B², 2,000,000,000)` (`tool/src/main.rs` line 920), and the rows follow it: every censored row records `evals = 2,000,000,000`. But three outputs still carry the unamended label: the stdout header "cap 50 B^2", `header.eval_cap` ("50*B^2 cost evaluations (censored at the cap)") and the per-n field `cap_50B2` in `summary.baselines_per_n` (13,421,772,800 at n = 28). So for n = 28 the labelled cap (1.34·10¹⁰) is not the cap that was applied (2·10⁹). For n <= 24, 50·B² (204,800; 3,276,800; 52,428,800; 838,860,800) is below 2·10⁹ and the two coincide; the largest recorded evaluation count at n = 12, 16, 20, 24 is 32,765; 984,039; 19,547,648; 233,918,313 (all below the cap). What the censored n = 28 rows record: 6 of 140 n = 28 solver-instance rows are censored at 2·10⁹, namely sa_B instances 13 and 18, sa_4B instances 9 and 11, pt instances 1 and 6 (2 of 20 each). The medians at n = 28 are exact (fewer than half censored); the censored rows enter the per-n medians and the bootstrap resamples as the recorded value 2·10⁹, which is a lower bound on their true count. No prediction or rule changed; the amendment's own statement is that a solver censored at 2·10⁹ already has e > 1.9 there. `[pcc:unverified]`

## Cost counts

- Total cost evaluations over all 700 solver-instance rows (sum of `evals`, derived): sa_4B 17,216,285,921; sa_B 17,132,574,438; pt 15,666,652,911; sa_16B 11,568,795,778; greedy 1,980,966,200; uniform 1,372,459,516; root_enum 105,635 (x1 values tried). `[pcc:unverified]`
- Spurious zero-cost states (c = 0 that did not lift to a verified decomposition): 0 in all rows. `[pcc:unverified]`
- Instances: 100 (20 per n); curves: 5, one per n, `b_candidates_tried` recorded in `run.json`. Walks: 640 per landscape size. Bootstrap reps: 1000.
- No wall time or CPU time is reported or a result.

## Execution route

Local host (4 vCPU Intel Xeon @ 2.10 GHz, Linux 6.18.44-fc-v338, rustc 1.97.0 per the receipt). The §9a default route (isolab-eks, EKS Spot) confirmatory run is open: it has not been done. The host was shared; the results are counts and no timing is a result. The tool ran with 4 threads (`header.threads = 4`, taken from `available_parallelism`) because it has no `--threads` flag. Each row is seeded per instance and solver (`F6/inst/<n>/<i>`, `F6/solver/<name>/<n>/<i>`), so by the tool's design the counts do not depend on the thread count; this is an assertion about the design, not a measurement (no 1-thread run was made). The receipt records a dirty working tree before and after (other studies' files and this study's card outputs).

## Scope

m = 2 only, these solvers only (uniform, greedy, SA at three schedules, parallel tempering with 8 replicas), n <= 28 (n <= 20 for the landscape statistic), moves limited to bit flips, ±1 steps and restarts; no QUBO hardware, no quantum annealer, no other encodings, no m = 3. The run says nothing about an exponent below those measured for other search methods or sizes, and does not bear on any S, T or rho row.

## Sources checked

- `research/semaev_qubo_annealing_20261110/PROTOCOL.md` (frozen protocol; sha256 7c427397…87eb)
- `/home/user/crypto/research/prime_moonshot_20261110/ideas-generator.md`, idea record G1
- `/home/user/crypto/RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §1–2
- research-kb: unavailable in this session (not queried); no novelty or "not tried" claim is made here.

## Provenance

- Card: `CARD-semaev-qubo-annealing-20261110-F6-RUN-01`, sha256 `2ba610affa718256691e81900195fff1b7c6a9859b44ef8c6a8e7a2d72bd9ece`
- Commit: `8c3b356ff45ec268ea7d0a21f28050200aa4c515`
- Receipt `cards/receipts/F6-RUN-01.receipt.json`: sha256 `7fce30d49275716076fa84a840441a24e896bd4d95059491388714e481fd1920` (status completed, 13 commands)
- Binary (`tool/target/release/semaev_qubo_annealing`): sha256 `07371fffa1bb890de18d3d000ee5418987499e01bc78e9526f96a6653a3d9cae` (receipt `binary-sha256` and the file on disk agree). The card's `binds` entry for the binary lists a different hash, `c70c8310f6752cd64f326c6feb8aa625f5b9a556e6b0ea93a535a250c187ba9f`; the card was written before the build step, so that entry is not the built binary. Not reconciled here.
- `results/run.json`: sha256 `9bbea2ea5dd017b85461ce9fc619cf99768bcdc2cdfb0a0a9e58586d786b3cdd` (equals the receipt's `result-sha256`)
- `FREEZE.sha256` (file sha256 `6284fe55cdf859eb057ac2095d6853a10cb492d36a394e6afdbc5a6cb338ff0e`, equals the card bind; verified OK in the receipt): PROTOCOL.md 7c427397febd48f48ba9941152997628ab6f0ec308fcb0dc7d550c12222387eb; tool/src/main.rs 88d7dd340c8b48164696db4d0a670537e1e574c3d32938f3d0c8fbd540dad61f; tool/Cargo.toml 69fa9a7ee61e4e8814462bb57765f56295802a455d1795a631936c702fc8f61f; tool/Cargo.lock a1e70b445783b45e612a8ab1c862018b98769babefefb8c293684f7ec1c94aea

## Files

- `README.md` (this file); `REPORT.typ`, `REPORT.pdf` (built with the typst Python wheel 0.15.2 in a scratch venv)
- `figures/evals_vs_B.svg`, `figures/evals_vs_B.png` (matplotlib 3.11.2), `figures/evals_vs_B.data.csv`, `figures/make_figure.py`
- `results/run.json`, `results/run.stdout.txt`, `results/run.stderr.txt` (empty); `cards/`, `tool/`, `PROTOCOL.md`, `FREEZE.sha256`
