# Factor-base line search on the JV sieve route: a frozen controlled study (2026-11-02)

**Scope.** Weak class only: full rational 2-torsion, `#E = 4l`, the genus-3 cover over `F_{p^2}`. Nothing here bears on prime-order curves.

**Labels.** *Measured* is read from a record. *Derived* is computed from records in Rust (`fit/`). *Model* is a stated cost model or extrapolation.

**Status.** Frozen protocol (`PROTOCOL.md`, hashed with the binary in `results/FROZEN.sha` at 2026-11-02T19:29:06Z by the system clock, before any arm-H/T/U run). No `VALIDATION.json`, so no leaderboard or `benchmarks/` row.

## Question and change

#3286 cut the Nagao-only arm by x1.71 with the best of 65,536 `F_p`-lines; its review found that arm dominated by the sieve route and an exploratory pilot suggested the search might transfer. Here `jv_cover::search_fb_line` is wired into `run_cover_sieve_dlp_on` (opt-in `JV_FB_LINE=n`; every context the route builds uses the moved spec; the search's multiplications go into `setup_muls`/`total_muls`, its additions and lookups into `total_plus`; `fb_line` in the record). A second mode, `JV_FB_LINE_MODE=table`, builds `chi(h(x))` for all `x in F_q` once (`38 q` multiplications, additions and lookups emitted) and then counts `B` per line with two additions and one lookup per point, no multiplication; with `n > p(p+1)/2` it enumerates every square-direction line up to equivalence. Code: `src/cryptanalysis/jv_cover.rs` and `jv_sieve.rs` only (diff summary at the end).

## Design

Instances and options are V2's sieve arm: `p in {1009, 1511, 1823}`, seeds 17-32, `--exp sieve --stop 64`, `JV_LIN=sparse`, `--rho-runs 0`. 192 runs. Arms: **P** plain; **H** Horner search, `n_H = 4096`; **T** table search, exhaustive (all `p(p+1)/2` lines, 0.51 M / 1.14 M / 1.66 M); **U** table search at `n = 32768` (secondary, declared before data).

**Rule for n (model, written before data; `results/rule.txt`).** Minimise the geometric mean over the three primes of `(route + search)/C_P`, route scaled by `(1 + k(n)/sqrt p)^-5` (`k(n)` = expected maximum of `n` standard normals), search cost counted (Horner `n p (17 + 16·331 w)`; table `38 q + 331 w (30 q + 3 n p)`), `w` in GAE per native operation, `C_P` from V2's sparse means. `gamma = 5` is the Nagao arm's measured exponent and lies in the range implied by the pilot (4.6 to 5.5); it is not derived for the sieve route. Result at w = 0: `n_H = 4096`, `n_T = exhaustive`. At w = 0.1 (and 1): `n_H = 8192`, `n_T = 32768` (arm U runs the table's).

## Controls (all pass)

| id | result |
|:--|:--|
| C1 | arm P equals V2's sparse record on **48/48** instances in every field but `wall_ms` (`total_muls`, `total_plus`, `relations`, `descent_digest` included) |
| C2h, C2t (n = 1, seed 17 per prime) | relations, digest, residuals, base, hits, lines equal P's; `d(total_muls)` equals the search's muls exactly (Horner 17,662 / 26,447 / 31,907; table 38.7 M / 86.8 M / 126.3 M) and `d(total_plus)` equals muls + adds + lookups exactly |
| C3 | table at `n = 4096` chooses the same line as Horner (B 511->554 idx 642; 768->824 idx 3928; 924->997 idx 1306) with identical route multiplications |
| C4 | all 192 runs solved, correct, 0 failed verifications, not exhausted |
| C5 | `fb_line.b_plain` = P's `base`, `b_best` = the arm's `base`, on every pair |

## Result: paired ratio arm/P (measured, derived), geometric mean over seeds, 95% bootstrap (20,000 resamples over seeds, per prime and pooled with primes as strata)

**S = total_muls/(c_add_e sqrt l)**, search charged. Below 1 is cheaper.

| arm | p = 1009 | p = 1511 | p = 1823 | pooled (48 pairs) | H1 (upper end < 1) |
|:--|:--|:--|:--|:--|:--|
| H (n = 4096) | 0.495 [0.410, 0.599] | 0.644 [0.571, 0.722] | 0.759 [0.616, 0.927] | **0.623 [0.562, 0.689]** (x1.61) | supported |
| T (exhaustive) | 0.488 [0.371, 0.655] | 0.569 [0.500, 0.662] | 0.635 [0.536, 0.736] | **0.560 [0.500, 0.631]** (x1.78) | supported |
| U (table, n = 32768) | 0.550 [0.417, 0.731] | 0.611 [0.517, 0.711] | 0.642 [0.565, 0.727] | 0.600 [0.535, 0.674] (x1.67) | supported (descriptive) |

**S+ (muls plus emitted additions and lookups, the search's included).**

| arm | p = 1009 | p = 1511 | p = 1823 | pooled | below 1 |
|:--|:--|:--|:--|:--|:--|
| H | 0.489 [0.431, 0.555] | 0.553 [0.516, 0.594] | 0.617 [0.555, 0.682] | **0.550 [0.519, 0.584]** | yes (x1.82) |
| T | 0.574 [0.492, 0.673] | 0.830 [0.785, 0.878] | **1.073 [0.968, 1.181]** | 0.800 [0.750, 0.853] | pooled yes; at p = 1823 no (interval contains 1) |
| U | 0.483 [0.410, 0.568] | 0.525 [0.488, 0.568] | 0.570 [0.522, 0.618] | **0.525 [0.491, 0.561]** | yes (x1.91) |

**Absolute (arm means, derived).** S_P = 0.02038 / 0.00615 / 0.00401; S_H = 0.00975 / 0.00394 / 0.00298; S_T = 0.00997 / 0.00351 / 0.00250. S+_P = 0.0635 / 0.0248 / 0.0165; S+_H = 0.0301 / 0.0137 / 0.0100; S+_U = 0.0300 / 0.0130 / 0.00923.

**§5a band, arm/P** (native operations = the route's emitted adds and lookups plus the search's, priced at w GAE each on both arms; F4 and descent additions are not emitted on either arm, so every cell is a lower bound on both). Pooled, with [per prime 1009, 1511, 1823]:

| arm | w = 0 | w = 0.1 | w = 1 | improves across the band (H3) |
|:--|:--|:--|:--|:--|
| H | 0.623 [0.562, 0.690] | 0.528 [0.499, 0.559]; 0.486 / 0.527 / 0.575 | 0.527 [0.498, 0.559]; 0.486 / 0.526 / 0.573 | yes |
| T | 0.560 [0.500, 0.632] | 0.876 [0.829, 0.927]; 0.608 / 0.915 / **1.207** | 0.879 [0.831, 0.929]; 0.609 / 0.918 / **1.212** | pooled yes; at p = 1823 it is worse than P at w >= 0.1, and at 1511 the interval reaches 0.98 |
| U | 0.600 [0.534, 0.672] | 0.497 [0.470, 0.527]; 0.448 / 0.499 / 0.550 | 0.496 [0.469, 0.525]; 0.446 / 0.498 / 0.550 | yes (descriptive) |

The best arm differs by weight: **T at w = 0** (0.560), **U at w >= 0.1** (0.497). H sits between. The exhaustive table's `3 p L` native operations are 53% of `total_plus` at p = 1823.

## Phase shares (derived; share of the arm's own `total_muls`, mean over seeds)

| arm | p | enumeration | sieve | descent | linear algebra | search | other |
|:--|--|--|--|--|--|--|--|
| P | 1009 / 1511 / 1823 | 31.2 / 30.6 / 26.6% | 20.1 / 27.5 / 28.7% | 44.1 / 32.7 / 32.6% | 4.4 / 8.7 / 11.7% | 0 | 0.3 / 0.4 / 0.4% |
| H | 1009 / 1511 / 1823 | 27.7 / 22.5 / 18.4% | 22.1 / 24.8 / 23.6% | 34.8 / 31.0 / 35.2% | 10.2 / 16.2 / 17.7% | 4.55 / 4.92 / 4.53% | 0.6% |
| T | 1009 / 1511 / 1823 | 22.9 / 20.3 / 17.8% | 19.6 / 23.8 / 24.2% | 43.0 / 31.1 / 29.7% | 11.4 / 19.5 / 22.3% | 2.57 / 4.56 / 5.32% | 0.6-0.7% |
| U | 1009 / 1511 / 1823 | 22.8 / 21.8 / 20.1% | 18.9 / 24.8 / 26.2% | 45.7 / 30.9 / 27.0% | 9.8 / 17.6 / 20.9% | 2.30 / 4.25 / 5.19% | 0.5-0.7% |

The search costs 2.3-5.3% of the multiplications (about 1.2-3% of `total_plus` for H and U; 26-53% for T). Linear algebra roughly doubles its share as relations fall, because it does not shrink with `B^-5`: its share is small in P but grows with `p` (4.4% to 11.7%) and then bounds further gains. Full tables and S+ shares: `analysis/analysis.txt`.

## B model check (derived against the model)

Per run, the route-only ratio `(total_muls - search)/P total_muls` against `(B_plain/B_best)^5` has geometric-mean ratio 0.930 (H), 0.973 (T), 0.962 (U), i.e. the route gains slightly more than the B^-5 model, with sd of log about 0.36 per run (the descent's geometric number of tests per success dominates run-to-run noise). The fitted exponent through the origin is `gamma_hat` = 5.92 [4.83, 6.94] (H), 5.48 [4.55, 6.29] (T), 5.41 [4.31, 6.48] (U): all contain 5; the point estimates are above it. The rule's J (model) against the measurement: H 0.655 vs 0.623 [0.562, 0.689]; T 0.564 vs 0.560 [0.500, 0.631] at w = 0. At w = 0.1 the model's T (exhaustive) J = 0.989 and U's 0.611 are conservative against the measured 0.876 and 0.497. At p = 1823 the measured H ratio (0.759) is worse than the model's 0.697, within its interval.

## Reading

- **H1 is supported for H and T** (and descriptively for U): with the search fully charged the sieve route gets 1.6-1.8x cheaper in F_p multiplications, and 1.8-1.9x in `S+` for H and U. This is a measured improvement over the plain sieve route in each of the three primes (the pilot's x1.42 net at p = 1511, n = 4096 was a lower effect than the frozen run's x1.55 for H there (0.644)).
- **The table makes the multiplication count cheapest but not the total.** T has the lowest S (0.560) and the highest S+ among the improving arms: its additions and lookups are priced and at p = 1823 it loses to P in `S+` (1.073 [0.968, 1.181]) and at w >= 0.1. The rule at w = 0.1 picks a smaller table search (U, n = 32768), which wins in S+ and in the band: x1.91 in `S+`, x2.0 at w >= 0.1.
- **The gain decays with p** in H (0.495, 0.644, 0.759; model: (1 + k/sqrt p)^-5), as in the Nagao arm.
- **Distance remaining to the matched rho (derived, not a ranking).** V2's R3 reference (harness price, S = 0.705 / 0.833 / 0.993; V2 withheld its rho comparisons because its A/A rule A4 failed) is not re-measured here. Against it H's S is 0.0138 / 0.0047 / 0.0030 and U's S+ is 0.036 / 0.013 / 0.0078 of rho, against 0.029 / 0.0074 / 0.0040 (S) and 0.076 / 0.025 / 0.014 (S+) for P. The sister study re-measures the matched rho; this one does not rank against it.

## Deviations, limits, not counted

- The run was local (4 cores, a rho study running concurrently), not on isolab-eks; counts are host-independent and the P control reproduces V2's records bit-for-bit. Wall: about 53 minutes (`results/walls.txt`), no cut.
- `gamma = 5` is a model input taken from the Nagao arm and the pilot's range. n_H and n_T were fixed by the rule before data; the rule's `n_H = 4096` coincides with the pilot's, whose seeds 17-24 at p = 1511 overlap this study's instances (the rule used only the pilot's exponent range, not its records).
- Not counted in the search: the table index arithmetic `x0 p + x1`, loop control, and 17 multiplications per direction in the enumeration are charged but the `p+1` inversions' additions are not. The F4 and descent additions are not emitted on any arm, so the S+ and band values are lower bounds on both sides.
- `cargo fmt` reformatted 9 unrelated files; those were restored, only `jv_cover.rs` and `jv_sieve.rs` carry changes.
- `fit/target` was deleted after the analysis.

## Files

`PROTOCOL.md`, `run.sh` (frozen); `results/` (all 21 records and logs, `FROZEN.sha`, `FIT.sha`, `rule.txt`, `versions.txt`, `walls.txt`); `fit/` (Rust: `sievefb_fit rule|analyze`); `analysis/` (`analysis.txt`, `ratios.svg`, `ratios.png`); `REPORT.pdf`.
