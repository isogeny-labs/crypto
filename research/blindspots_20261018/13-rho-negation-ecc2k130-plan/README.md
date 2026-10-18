# Item 13: the negation-map shortfall (H-RHO-bc6ea6) and an ECC2K-130 completion plan

**Verdict: TRADES (level: constant factor).** Part B is a plan and is **DELIVERED**.
Frozen rule: [`PROTOCOL.md`](PROTOCOL.md). Read-out: [`raw/summary.md`](raw/summary.md), [`raw/summary.json`](raw/summary.json), [`raw/plan.md`](raw/plan.md).
Graph: [`graph.svg`](graph.svg), [`graph.png`](graph.png). PDF: [`REPORT.pdf`](REPORT.pdf). Benchmarks copy: `benchmarks/20261018-blindspots-rho-negation-ecc2k130-plan/`.

## Result in one paragraph

On a prime-field walk (counted `Z/N` model, N near 2^40, 2^44, 2^48; 64 paired targets per cell; r in {16, 32, 128, 2048}) the negation walk **does not robustly reach the preregistered 1.35 gain over the plain Teske walk, and its gain flips sign inside the pricing band**. Every gate passed. Under the most favourable pricing (native work free) 11, 14 and 12 collision-preserving cells reach 1.35 at 2^40, 2^44, 2^48, with the look-ahead plus periodic rules best (`laper16` r=128: 1.557 [1.325, 1.825] at 2^40, 1.488 [1.254, 1.779] at 2^44; `laper16` r=32: 1.552 [1.277, 1.895] at 2^48). No cell has a 95% lower bound of 1.35 and a point gain of 1.35 at all nine pricing points with every target solved; so "robustly reaches" is false at every size. Frozen rule, first prediction: **INCONCLUSIVE** (neither SUPPORTED nor REFUTED). Second prediction: a sign change **exists in the tested range** (12 collision-preserving settings are clear net losses at all three sizes: `min2`, `la`, `per2` at r=16 and 32; `per4`, `per16`, `per32`, `per64` at r=16, `per32`, `per64` at r=32; all rules that detect only short cycles lose below r=128). Against the incumbent (un-quotiented walk, constant level) the 44 collision-preserving (rule, r) settings read: **0 ADVANCES, 30 TRADES, 2 MATCHES, 12 REGRESSES**, 0 INCONCLUSIVE. TRADES means the order flips inside the nine-point band (JAC30/AFF3/AFF30 times native work 0, 0.1, 1 group-addition equivalents): every collision-preserving cell has a worst-of-nine point gain below 1.0 (nine-point minimum 0.2 to 0.8).

## Distance remaining (gap to matched rho; the baseline is rho itself)

Gain 1.0 is the plain walk. The ideal map control sees 1.49 [1.40, 1.59], 1.41 [1.32, 1.51], 1.41 [1.33, 1.50] at 2^40, 2^44, 2^48 (target sqrt 2 = 1.414). Fully charged JAC30, native work 0, best collision-preserving cell per size: 1.557 (r=128, `laper16`), 1.488 (r=128, `laper16`), 1.552 (r=32, `laper16`), i.e. best/sqrt 2 = 1.10, 1.05, 1.10 with 95% intervals of roughly +-0.25 that include 1.35 and sqrt 2. A typical long-r cell (r=2048, any rule from `min2` up) sits at 1.20 to 1.40, i.e. 0.85 to 0.99 of sqrt 2; `la` at r=2048: 1.361, 1.292, 1.272 (0.90 to 0.96 of sqrt 2). At the most pessimistic point of the band (native work priced at one group addition per operation, Jacobian model) the r=2048 cells fall to 0.58 to 0.81. The gap to rho is therefore not closed under any pricing that charges the hash, canonicalisation and cycle-comparison work at 1 GAE, and is more than closed in point estimates only when that work is free; the intervals do not separate the best cells from the 1.35 line. Per-setting table with all three sizes: [`tables.md`](tables.md).

## Gates and controls (all pass)

* C1 baseline vs ideal random-map control (pooled r>=128, `base`+`baseb`, 256 targets): ratio 0.948 [0.882, 1.019] at 2^40, 1.023 [0.943, 1.104] at 2^44, 1.008 [0.932, 1.087] at 2^48 against the band [0.92, 1.08]: PASS at every size. (Convention corrected, PROTOCOL Deviation 2: 0.886 sqrt N is the ideal negation cost, the plain walk is 1.253 sqrt N; measured 1.237, 1.270, 1.287.)
* C2 ideal random-map control sees sqrt 2 at every size (above): PASS.
* C3 no-escape arm absorbed at r=16 at every size: 0/64 solved, capped-walk fraction 0.962, 0.981, 0.990 against the model 1.000: PASS.
* A/A (`base` vs `baseb`): the interval covers 1 in 12 of 12 cells: PASS.
* C4 two-implementation agreement and selftest positive controls (card 01, [`raw/selftest.txt`](raw/selftest.txt)): all PASS, no wrong logarithm.
* HEUR-RHO-1 (fruitless 2-cycle entry rate per draw against 1/(2r), DP-ending-walk estimator): 48 measured settings, ratio 0.948 to 1.039, all within a factor 2, **not refuted**.
* HEUR-RHO-2 (size independence, intervals at 2^40 and 2^48 disjoint): 4 of 44 collision-preserving pairs flagged (`la`, `min2`, `per2` at r=128, `per4` at r=16), plus `dbl` r=128 (not collision-preserving). The three r=128 flags are a shrinking gain with size; see the theta extension below, which suggests the cause is the DP bit count (10, 11, 12) and not N.

## Caveats on what the numbers mean

* **Censoring.** Cells marked + in the table left at least one target unsolved at the cap of 2 sqrt(pi N / 2) group operations; their gain is an upper bound. All 12 REGRESSES and both MATCHES settings are censored, so they are loss or no-gain statements that are conservative; no censored cell supports a reach claim. The cells that reach 1.35 at the favourable point and are fully solved (for example `laper16` r=128 at all three sizes, `la` at r=2048 at 2^40) are the uncensored ones.
* **Toy scale.** N is 2^40 to 2^48, not 2^130, and the group is the cyclic `Z/N` model with counted operations (the curve agreement is checked at 2^20 only). DP bits are bits/4. Nothing here is an ECC2K-130 or NIST-curve measurement, and the ECC2K-130 iteration function is a Frobenius (sigma) walk, not this negation walk. No exponent is touched (KN-TECH-005).
* **Pricing.** Costs are counted group operations priced by three models; native work at 0, 0.1 and 1 GAE; the same weights for every arm including the baseline (AGENTS.md section 5a). The 1 GAE point is a deliberately harsh bound (a hash is not a point addition); the verdict TRADES is the statement that the order is unresolved across that band, not that the walk loses at the realistic point.
* **Theta extension (outside the frozen rule, enters no verdict).** At 2^44, r=128, DP bits 8 / 11 / 14: `la` 1.344 / 0.810 / 0.443, `min2` 1.269 / 0.801 / 0.442, `per8` 1.313 / 1.261 / 1.053, `laper16` 1.474 / 1.488 / 1.179 (full tables in `raw/summary.md`). Rules that only see cycles of length <= 2 lose to longer fruitless cycles as walks get longer; windowed rules decay more slowly.
* **Host.** Shared 4-vCPU container at isolation level L0, one thread per job, `prlimit --as=4294967296`. Counted operations do not depend on the host. The CPU rate in part B does: it is a statement about this host.
* **Pilot disclosure.** The instrument was debugged on pilot runs with disjoint seed offsets (PROTOCOL Deviation 7); no threshold was set from a measured gain.

## Relay check (executor reports against receipts)

Saved as `raw/relay-1.txt` to `raw/relay-15.txt`; result in [`raw/relay-check.txt`](raw/relay-check.txt). `task_card.py check-relay` as a command refuses now because each card's `lint` fails once its outputs exist on disk (the output paths are not in `binds`); the check was run with its own `check_relay()` function from `tools/task_card.py` on each card and its receipt, without changing the tool or any card. All 15 receipts are `completed`, every command exit 0, and the 33 recorded output digests match the files on disk. Relay result: **9 verbatim (1, 2, 3, 5, 8, 9, 10, 13, 14), 6 deviated (4, 6, 7, 11, 12, 15)**. The deviations, reported and not repaired: cards 4, 7, 11 and 15 do not carry the `status: completed` line in the position the tool prints it (4, 7 and 15 put it before the card line, 11 omits it; card 15 also lists `plan.md` before `plan.json`, reversing the receipt order); card 6 drops the `card:` prefix and moves status to the head; card 12's report carries a card sha256 that differs from the receipt in one hex digit (`...29b3c1bb...` against `...29b3b1bb...`). No measured number differs in any report. The analysis here reads `raw/` and the receipts, never the executor text.

## Part B: costed plan for ECC2K-130 (DELIVERED, no verdict)

Measured here: current `ecc2k130/` client built from `ecc2k130/src/main.cu` with the Makefile `ecc2k130-cpu` flags, `--curve 131 --bench --threads 1`, five runs: 14.702, 15.189, 14.532, 15.206, 15.605 M iterations/s per core; **median 15.189, min 14.532, max 15.605** (host load average 0.83 before, 1.45 after, L0). Committed in the repository, cited and not re-measured: GPU rates (sigma-table 15.884369 B/s RTX PRO 6000; x-only bridge 12.499203 B/s RTX PRO 4500, work factor 1.006696), 2^60.9 expected iterations, DP intervals 2^28.41 (w32) and 2^25.27 (w34), 2026-09 cloud prices.

| quantity | value |
|:--|--:|
| expected work 2^60.9 | 2.1514e18 iterations |
| CPU alone at 15.189 M/s | 3.935e7 core-hours = 4,488 core-years |
| DPs to collect, storage at 32 B (w32 / w34) | 6.03e9 / 193 GB; 5.32e10 / 1.70 TB |
| RTX PRO 6000 sigma-table | 37,623 GPU-h; $45,629 at $1.2128/h (spot), $49,286 at $1.31, $113,998 at $3.03 (Modal), $155,760 at $4.14 (on-demand) |
| RTX PRO 4500 x-only bridge (different iteration function) | 48,133 GPU-h; $27,460 at $0.5705/h |
| work for 50% / 90% / 99% (Rayleigh model, labelled model) | 35,345 / 64,419 / 91,103 GPU-h; $42.9k / $78.1k / $110.5k |
| wall clock, 100 / 1,000 / 10,000 GPUs (sigma-table) | 15.7 / 1.6 / 0.2 days |
| break-even CPU core-hour price vs the 6000 at $1.2128/h | $0.00116 |

Claims nothing about a live price, a GPU rate measured here, campaign progress already collected, or network and orchestration cost. The measured CPU rate (15.2 M/s) is about 1/1045 of one sigma-table RTX PRO 6000. Part B has no unpriced-native-operation arm to put through the section 5a band; its cost rows are price-times-hours at cited prices only.

## Deviations, files

All deviations are in [`PROTOCOL.md`](PROTOCOL.md) (arms added, baseline control re-based on the ideal map, Z/N model, pilots, part B reuse). Files: `PROTOCOL.md`, `card-01.yaml` to `card-15.yaml`, `smoke/`, `raw/` (sweeps, controls, receipts, relay texts, `summary.*`, `plan.*`, `cpu-bench-*.log`), instrument `examples/blindspots_13_rho_negation.rs`, `plot_graph.py` and `make_tables.py` (layout only, read `raw/summary.json`), `tables.md`, `tables.typ`, `REPORT.typ`, `REPORT.pdf`, `graph.svg`, `graph.png`.
