# Line-search sieve route vs a matched negation rho, same instances, three primes (frozen 2026-11-03)

**Scope.** The `jv_cover` weak class only (full rational 2-torsion, `#E = 4l`, genus-3 cover over `F_{p^2}`); never prime order. Item 3 of "Next round" in [`../jv_cover_next_leads_20261102/LEADS.md`](../jv_cover_next_leads_20261102/LEADS.md): one study, cover arms and rho on the same instances, instead of a product of ratios. Accounting and distance to rho, not a new improvement; no leaderboard or `benchmarks/` row (no `VALIDATION.json`).

Labels: **measured** = read from a run record; **derived** = computed from records by the Rust tool [`fit/`](fit/); **model** = extrapolation or weight model (only the bitop proxy below).

Files: [`PROTOCOL.md`](PROTOCOL.md), [`run.sh`](run.sh), [`results/`](results/) (`cover/` 18 files, `rho/` 39 records and logs, `rho_supp/`, `replay/`, `inputs/`, `FROZEN.sha`, `versions.txt`, `walls.txt`, `analysis.out`), [`fit.txt`](fit.txt) / [`fit.json`](fit.json) / [`table.md`](table.md) (every statistic), [`graph.svg`](graph.svg) / [`graph.png`](graph.png), [`REPORT.pdf`](REPORT.pdf); secondary outputs [`bitop.txt`](bitop.txt), [`runboot.txt`](runboot.txt), [`runboot_with_s17.txt`](runboot_with_s17.txt), [`supp_fit_with_s17.txt`](supp_fit_with_s17.txt), [`supp_table_with_s17.md`](supp_table_with_s17.md).

Design (frozen before any new run, hashes and UTC time in `results/FROZEN.sha`): frozen binary `jv_cover_round3_frozen` (main aa1c89ab2a, sha256 d21002a309374e1a06caf3f93a4c6da212362a72e5987b4e80aa8a96fdc44f2c); arms P (plain), H (`JV_FB_LINE=4096`, Horner), U (`JV_FB_LINE=32768`, table) on p in {1009, 1511, 1823}, seeds 17-24 (24 instances); matched `rhostrong --neg 1 --batch 16 --r 256 --dp-bits 16` on the same seeds: p = 1009 two runs per seed, p = 1823 one run per seed, p = 1511 study B's 16 runs reused. Primary metric: cover / rho **per instance**, geometric mean over instances, 95% bootstrap (20000) jointly over cover seeds and rho runs.

## Controls (measured)

| control | result |
|:--|:--|
| C1 arms P, H, U equal study A's records on seeds 17-24, every field but `wall_ms` | **72/72 pass** |
| C2 every cover run solved, correct, 0 failed verifications, not exhausted, arm settings as declared | **72/72 pass** |
| R1 study B `p=1511 seed=17 run=0` replayed with the frozen binary (before any new rho run) | **bit-identical** |
| R2 #3278 `p=1451 seed=1 run=0` replayed (instrument control) | **bit-identical** |
| I1 `l` and `c_add_e` of every rho run equal the cover record of that prime and seed | **pass** (all 39 + supplementary) |
| R3 every rho run correct, uncapped, negation, batch 16, r 256, dp-bits 16 | pass; one run killed by the time rule, see Deviations |
| PC positive control, mean steps/sqrt(l) vs 0.88796 within 3 se | **pass at all primes**: p1009 0.779 (n 16, z -0.83), p1511 0.806 (16, z -0.68), p1823 0.723 (7, z -0.88); with the supplementary run 0.909 (8, z +0.08) |

## Homogeneity decision (derived, frozen rule): NOT homogeneous, so the primary reference is per-prime

Across the three primes' matched groups the tests do not reject (ANOVA p 0.94, Kruskal-Wallis p 0.89). Against #3278's 126 runs the 39 matched runs (mean 0.780 vs 0.932) give |diff| permutation p 0.063 but **KS permutation p 0.012 (< 0.05)**; the frozen rule needs all four p >= 0.05, so **the primary reference is the per-prime matched runs**; the pooled reference (126 + new, 165 runs) is secondary. The decision is the same with the supplementary p1823 run (40 runs: |diff| p 0.166, KS p 0.018). With 8-16 heavy-tailed runs per prime (sd about 0.5) the power is modest.

## Result (derived; primary = per-prime reference; 95% bootstrap in brackets)

Headline units follow the red team's request (reporting choice made after the freeze; the frozen primary statistics are all in `table.md`): **U2** (tuned rho, `s_tuned`) and **S+ over the rho priced like-for-like** (`S+/rhoS+`: rho native counters and `fp_adds` counted one per mult). Every cell is below rho. Ratios are cover / rho (1 = parity).

| arm | scope | S/rho U2 | S+/(rho S+) U2 | S/rho U1 |
|:--|:--|--:|--:|--:|
| P | pooled 24 | 0.0244 [0.0206, 0.0292] | 0.0624 [0.0546, 0.0727] | 0.0101 [0.0086, 0.0122] |
| H | pooled 24 | 0.0154 [0.0135, 0.0180] | 0.0344 [0.0307, 0.0394] | 0.0064 [0.0056, 0.0075] |
| U | pooled 24 | 0.0141 [0.0122, 0.0167] | 0.0324 [0.0288, 0.0371] | 0.0059 [0.0051, 0.0070] |
| P | p1009 / p1511 / p1823 | 0.066 / 0.0177 / 0.0125 | 0.140 / 0.0507 / 0.0344 | 0.0273 / 0.0074 / 0.0052 |
| H | p1009 / p1511 / p1823 | 0.0314 / 0.0124 / 0.0094 | 0.0646 / 0.0283 / 0.0223 | 0.0131 / 0.0052 / 0.0039 |
| U | p1009 / p1511 / p1823 | 0.0306 / 0.0115 / 0.0081 | 0.0603 / 0.0268 / 0.0210 | 0.0127 / 0.0048 / 0.0034 |

Per-prime intervals, S+ over unpriced rho (U1 and U2), the pooled-reference table and every instance are in [`table.md`](table.md). Distance remaining (pooled, S/rho U2): the plain route is x41 below rho, H x65, U x71; in S+/(rho S+) U2: x16, x29, x31. Pooled-reference (secondary, 165 runs) S/rho U2: P 0.0209, H 0.0132, U 0.0121; S+/(rho S+) U2: 0.0535 / 0.0295 / 0.0277.

**Trend with p** (derived): the ratio to rho falls as p grows for every arm (S/rho U2, arm U: 0.031, 0.0115, 0.0081), because rho's cost grows as sqrt(l) ~ p^3 faster than the route's. The line-search gain over P (cover-only, paired, point): H S x0.632 / S+ x0.551, U S x0.580 / S+ x0.518 pooled; per prime H 0.48 / 0.70 / 0.75, U 0.47 / 0.65 / 0.65, shrinking with p as in study A.

**Section 5a band** (native operations priced at w = 0, 0.1, 1 GAE on both arms; verdict below rho at all three w in every cell; both layers, both units, all arms and scopes in `table.md`). Layer B (as emitted; pooled, U2, w = 0 / 0.1 / 1): P 0.0244 / 0.0168 / 0.0150, H 0.0154 / 0.0103 / 0.0091, U 0.0141 / 0.0115 / 0.0108. Layer A (V2's upper bound, prices unemitted F4/descent/linear-algebra mults as native on the cover side; pooled, U2): P 0.0244 / 0.164 / 0.172, H 0.0154 / 0.091 / 0.095, U 0.0141 / 0.086 / 0.090, i.e. still about x10 below rho at w = 1. The order holds across the band everywhere; H and U trade places across the band at the pooled level (U lower at w = 0, H lower in layer B at w = 1), which is not resolved by the intervals.

**Comparison with the product estimate** (p = 1511, seeds 17-24, primary reference): measured / product = H S/rho U1 0.00518 [0.00412, 0.00687] vs 0.0042 (x1.23); H U2 0.01245 [0.0099, 0.0165] vs 0.010 (x1.25); U S+/rho U2 (rho unpriced) 0.0388 [0.0309, 0.0514] vs 0.034 (x1.14); U S+/rho U1 0.0162 vs 0.014 (x1.15). The product lies inside every interval and is 13-20% optimistic (the 8-seed matched reference mean 0.805 is below the pooled 142's 0.918 used by the product; the product used the 142 mean).

## Secondary reporting declared after the freeze (red-team relay)

1. **Headline unit:** U2 and S+/(rho S+) are the headline above; U1 and S+ over an unpriced rho are in the tables.
2. **bitop/v1 proxy (MODEL, not a measurement)**, [`bitop.txt`](bitop.txt), tool `fit/src/bin/bitop.rs` (hashed after the data): per `sqrt(l)*331`, cover cost = `total_muls - la_muls + 31.2 la_ops + 0.096 (sieve_adds + enum_adds + search adds) + 0.043 (sieve_lookups + search lookups)`; rho cost = `muls_tuned + 0.096 fp_adds + 0.043 (six native counters)`. Weights as relayed (field_add/field_mul 0.096 at b = 11, word op 0.043, mod-l LA op 31.2 F_p mults instead of the record's 16 per op); the weights were not re-derived here from `docs/units/BITOPS.md`. F4 and descent additions are unemitted, so the cover is a lower bound. Per-prime reference, pooled over 24: **P 0.0315 [0.0272, 0.0373], H 0.0200 [0.0177, 0.0232], U 0.0187 [0.0165, 0.0218]**; per prime (p1009 / p1511 / p1823) P 0.078 / 0.024 / 0.017, H 0.039 / 0.016 / 0.013, U 0.038 / 0.015 / 0.011 (pooled reference: 0.0270 / 0.0172 / 0.0161). Below rho in all cells.
3. **Arm T / table `x0*p + x1`:** arm T is not re-run here. Arm U's table walk does an uncounted index computation per lookup (~1% of its search cost, per the red team; not independently re-measured); the arm-U S and S+ are lower bounds by that amount.
4. **Resolution:** the descent dominates per-run noise (sd of the log ratio about 0.7-1.2 per the red team); with 8 seeds per prime **the per-prime arm rankings (H vs U) are not resolved**: their intervals overlap at every prime and in every cell (see `table.md`). What is resolved: every arm is below rho at every prime, and H and U are below P pooled (cover-only ratios: study A's 48-instance intervals).

## Deviations and limits (all stated)

* **Time rule kill, p1823 seed 17 run 0.** Under 8 busy threads on 4 cores (cover arms and the rho queue concurrently) it hit the frozen 3600 s limit (exit 124) and was excluded as the protocol states, so the frozen analysis has **7 rho runs at p = 1823** (flagged as a cut; the fallback to 4 runs was not needed). The exclusion is a right-censoring of a long run and biases the p1823 reference downward (mean 0.723), hence the S/rho ratios at p1823 upward, i.e. conservatively against the cover. As a declared supplement the same run was re-run alone without the kill (finished in 1601 s; steps/sqrt(l) = 2.21). With it: p1823 mean 0.909 (n 8); pooled S/rho U2 P 0.0226, H 0.0143, U 0.0131 (frozen: 0.0244, 0.0154, 0.0141); p1823 U S/rho U2 0.0064 (frozen 0.0081). Same decisions (homogeneity still fails on KS, p 0.018; everything below rho). Files: `results/rho_supp/`, `supp_*`.
* **The frozen bootstrap understates rho-run uncertainty.** It resamples rho runs only within an instance group (as study B), so at p = 1823 (one run per instance) the rho runs do not vary at all and at p = 1009 and 1511 only two ways. The frozen p1823 intervals are therefore cover-seed intervals conditional on the rho runs. The secondary run-level bootstrap ([`runboot.txt`](runboot.txt); resamples all of a prime's rho runs) is wider: p1823 U S/rho U2 0.0081 [0.0052, 0.0131] (frozen [0.0074, 0.0088]); pooled U 0.0142 [0.0114, 0.0182] (frozen [0.0122, 0.0167]); P pooled [0.0194, 0.0317]. No verdict changes. Use the run-level intervals where they differ.
* **Analysis tool.** `fit/src/main.rs` and `Cargo.toml` hashed before any new run (`FROZEN.sha`); `main.rs` later had a graph-only edit (canvas height and legend offset), re-hashed, `fit.txt` byte-identical. `bitop.rs` and `runboot.rs` were written and hashed after the data existed (red-team request and the interval issue above).
* **run.sh re-hash.** The binary-hash grep in `run.sh` did not match an absolute path; fixed and re-hashed once before any run (the first hash line is overwritten; noted in `FROZEN.sha`).
* **Local, not isolab-eks**, as instructed (counted records, host-independent). Budget: all runs finished in about 1 h 35 min wall including the supplementary run; no cut other than the one above. Walls are in `results/walls.txt` (observation only; they were inflated by running everything concurrently).
* **Reference choice.** The reference runs at p = 1009 and 1511 have means 0.779 and 0.806, below theory 0.888 and below #3278 (0.932); both are within noise (z about -0.7 to -0.8) and the choice of reference moves S/rho by up to about 15% (pooled-reference values in `table.md`). Cover instances are the study A / V2 seeds; the same 8 seeds were used by the red-team pilot at p = 1511.
* Distances are to a negation-map rho at these primes (l about 2^55-2^64); nothing is extrapolated. Weak class only.
