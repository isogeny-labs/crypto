# IC positive control: results (2026-11-12)

Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen by [`frozen.sha256`](frozen.sha256)
before the first measured run. Data: [`control.json`](control.json) (every run),
[`control.svg`](control.svg), [`run.log`](run.log); tables regenerate with
`render_results.py control.json` and `uncertainty.py control.json`. Bench record
(left open): `benchmarks/records/20261112T2145Z-ic-positive-control.json`. Source:
`src/cryptanalysis/chained_transfer/ic_control.rs`, `src/bin/ic_positive_control.rs`.
Every numeric statement is `[pcc:unverified]`: no independent validator has replayed it.

## What was run

Genus-3 imaginary hyperelliptic curves `y^2 = f(x)`, `deg f = 7`, random monic, over
the ten primes `131, 257, 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537`
(first prime at or above `2^7 ... 2^16`), on the **full** Jacobian, `#J = h r` with `h <= 64` and
`r` prime. Arms: A (basic, every degree-1 place), B (single large prime, `nb = 2 p^{3/4}`),
C (double large prime, `nb = 2 p^{2/3}`), R (negation-map rho on the order-`r` subgroup). 572 runs
(428 IC, 144 rho): 5 curves per prime up to 2053, 3 at 4099 and 8209, 2 at 16411 and 32771, 1 at
65537, 4 seeds each; A stops at 32771. **All 572 logs certify** (`[k]G == T` by the counted multiplication and
by the independent right-to-left multiplication) and no run exhausted its budget
[pcc:unverified]. Wall clock for the whole run: 982 s [pcc:unverified]. Mean fraction of trials whose
`u` splits completely: 0.155 to 0.177 at every `p` and arm, against `1/6 = 0.1667` for a random cubic
[pcc:unverified], so the walk behaves like a random source of divisors.

## Verdict against the frozen rule

**The instrument cannot detect a win under the AGENTS.md 5a band, within `p <= 65537`.** The rule is: C below R at
all three prices at some `p`. Result: `strict_all_prices_first_p = null` [pcc:unverified].

- `w = 0`: C is below R at every `p >= 257` (C/R = 0.62 at 257, 0.114 at 65537), above R at 131 (1.80)
  [pcc:unverified]. At 65537, C spends 5.97e5 group operations against R's 5.24e6 [pcc:unverified].
- `w = 0.1`: C is above R at every `p`; C/R = 5.82 at 65537 (bootstrap 95% interval 3.84 to 9.37), 7.54 at
  16411 [pcc:unverified].
- `w = 1`: C is above R at every `p`; C/R = 31.5 at 65537 (interval 20.8 to 52.7), 40.7 at 16411 [pcc:unverified].
- Weaker reading (each price on its own): `w = 0` yes (first `p` = 257), `w = 0.1` no, `w = 1` no [pcc:unverified].
- Order of C and R across the band: it flips between `w = 0` and `w = 0.1`, so by 5a the two are **unresolved**,
  not a win for C [pcc:unverified].

Informational timing point (not part of the band, never changes the verdict): pricing a native
multiplication at its measured cost relative to one Cantor addition (`w_rel` about 0.005 for the
relation phase mod `p`, `w_la` about 0.013 for the linear algebra mod `r`; best of 5 on this shared host)
gives C/R = 1.15 at 4099, 0.77 at 16411, 0.76 at 32771, 0.58 at 65537 [pcc:unverified]. The bootstrap intervals include 1 at
every `p` up to 32771 and exclude it at 65537 (0.38 to 0.94); the first `p` with C/R below 1 (2053, ratio 0.99) is a
noisy point (the ratio is back at 1.15 and 1.36 at 4099 and 8209) [pcc:unverified]. A least-squares crossover from
the two calibrated fits sits near `p` = 4.7e3 [pcc:unverified]; the data do not support a stable crossing below 16411.

## Why C does not win at `w = 0.1` and `w = 1`

The cost is not in the linear algebra and not in the group. At `p = 65537`, C's mean native work
is 2.09e8 multiplications in the relation phase and 1.20e8 in the linear algebra, from 5.79e5 trials; the
smoothness test (`x^p mod u`, square-and-multiply on `F_p[x]/u`) plus root finding costs about 362 multiplications
per trial [pcc:unverified]. A trial costs one group operation, so each group operation drags 362 native multiplications behind it.
At a price of 0.1 or 1 GAE per multiplication that is 36 to 362 GAE per trial; rho pays about 1 native operation per
step. The asymptotic gap `p^{1/6}` between `p^{4/3}` and `p^{3/2}` is a factor of about 6.4 at `p = 65537`
[pcc:unverified], which is the size of the ratios at `w = 0.1` (5.82) and far below those at `w = 1` (31.5).
Linear algebra is not the bottleneck either way: Wiedemann on the peeled core (3120 columns at 65537) is 1.20e8
multiplications and 6.61e8 additions, less than the relation phase in multiplications [pcc:unverified].

## Per-prime tables (means over all runs of the prime)

### Table 1. Counts (mean over runs)

| p | runs | A grp | A nat.mul | B grp | B nat.mul | C grp | C rel.mul | C LA.mul | C LA.add | R grp (walk + other) | R nat |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 131 | 5c | 1,239 | 1.480e+05 | 1,243 | 1.486e+05 | 1,242 | 1.023e+05 | 31,786 | 74,825 | 689 (500 + 189) | 528 |
| 257 | 5c | 1,401 | 2.875e+05 | 1,431 | 2.981e+05 | 1,443 | 1.591e+05 | 66,210 | 1.698e+05 | 2,324 (2,024 + 301) | 2,074 |
| 521 | 5c | 2,141 | 8.367e+05 | 2,305 | 7.689e+05 | 2,337 | 3.779e+05 | 1.583e+05 | 4.528e+05 | 4,413 (4,048 + 365) | 4,120 |
| 1031 | 5c | 3,754 | 2.489e+06 | 4,428 | 1.960e+06 | 4,382 | 9.069e+05 | 4.040e+05 | 1.263e+06 | 13,884 (13,361 + 523) | 13,501 |
| 2053 | 5c | 6,814 | 8.832e+06 | 9,674 | 5.194e+06 | 8,996 | 2.104e+06 | 1.014e+06 | 3.452e+06 | 33,255 (32,698 + 557) | 32,863 |
| 4099 | 3c | 13,141 | 3.170e+07 | 22,484 | 1.440e+07 | 19,959 | 5.187e+06 | 2.722e+06 | 1.042e+07 | 70,683 (69,985 + 697) | 70,285 |
| 8209 | 3c | 25,636 | 1.221e+08 | 54,874 | 4.048e+07 | 45,114 | 1.300e+07 | 6.793e+06 | 2.830e+07 | 1.424e+05 (1.417e+05 + 732) | 1.420e+05 |
| 16411 | 2c | 51,074 | 4.613e+08 | 1.351e+05 | 1.147e+08 | 1.054e+05 | 3.413e+07 | 1.793e+07 | 8.191e+07 | 6.407e+05 (6.395e+05 + 1,201) | 6.403e+05 |
| 32771 | 2c | 1.026e+05 | 1.813e+09 | 3.418e+05 | 3.107e+08 | 2.513e+05 | 8.186e+07 | 4.653e+07 | 2.332e+08 | 1.599e+06 (1.597e+06 + 1,927) | 1.598e+06 |
| 65537 | 1c | - | - | 8.594e+05 | 8.751e+08 | 5.970e+05 | 2.095e+08 | 1.198e+08 | 6.614e+08 | 5.238e+06 (5.235e+06 + 2,876) | 5.237e+06 |

### Table: total cost in GAE at w = 0, and ratio to R

| p | A | B | C | R | A/R | B/R | C/R |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 131 | 1,239 | 1,243 | 1,242 | 689 | 1.80 | 1.80 | 1.80 |
| 257 | 1,401 | 1,431 | 1,443 | 2,324 | 0.60 | 0.62 | 0.62 |
| 521 | 2,141 | 2,305 | 2,337 | 4,413 | 0.49 | 0.52 | 0.53 |
| 1031 | 3,754 | 4,428 | 4,382 | 13,884 | 0.27 | 0.32 | 0.32 |
| 2053 | 6,814 | 9,674 | 8,996 | 33,255 | 0.20 | 0.29 | 0.27 |
| 4099 | 13,141 | 22,484 | 19,959 | 70,683 | 0.19 | 0.32 | 0.28 |
| 8209 | 25,636 | 54,874 | 45,114 | 1.424e+05 | 0.18 | 0.39 | 0.32 |
| 16411 | 51,074 | 1.351e+05 | 1.054e+05 | 6.407e+05 | 0.08 | 0.21 | 0.16 |
| 32771 | 1.026e+05 | 3.418e+05 | 2.513e+05 | 1.599e+06 | 0.06 | 0.21 | 0.16 |
| 65537 | - | 8.594e+05 | 5.970e+05 | 5.238e+06 | - | 0.16 | 0.11 |

### Table: total cost in GAE at w = 0.1, and ratio to R

| p | A | B | C | R | A/R | B/R | C/R |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 131 | 16,041 | 16,106 | 14,656 | 741 | 21.64 | 21.73 | 19.77 |
| 257 | 30,147 | 31,244 | 23,975 | 2,532 | 11.91 | 12.34 | 9.47 |
| 521 | 85,816 | 79,191 | 55,958 | 4,825 | 17.79 | 16.41 | 11.60 |
| 1031 | 2.526e+05 | 2.004e+05 | 1.355e+05 | 15,234 | 16.58 | 13.16 | 8.89 |
| 2053 | 8.900e+05 | 5.291e+05 | 3.208e+05 | 36,541 | 24.36 | 14.48 | 8.78 |
| 4099 | 3.183e+06 | 1.463e+06 | 8.108e+05 | 77,711 | 40.97 | 18.82 | 10.43 |
| 8209 | 1.223e+07 | 4.102e+06 | 2.024e+06 | 1.566e+05 | 78.09 | 26.19 | 12.93 |
| 16411 | 4.618e+07 | 1.160e+07 | 5.312e+06 | 7.047e+05 | 65.53 | 16.46 | 7.54 |
| 32771 | 1.814e+08 | 3.141e+07 | 1.309e+07 | 1.759e+06 | 103.17 | 17.86 | 7.44 |
| 65537 | - | 8.837e+07 | 3.352e+07 | 5.762e+06 | - | 15.34 | 5.82 |

### Table: total cost in GAE at w = 1, and ratio to R

| p | A | B | C | R | A/R | B/R | C/R |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 131 | 1.493e+05 | 1.499e+05 | 1.354e+05 | 1,217 | 122.68 | 123.18 | 111.27 |
| 257 | 2.889e+05 | 2.996e+05 | 2.268e+05 | 4,398 | 65.68 | 68.11 | 51.56 |
| 521 | 8.389e+05 | 7.712e+05 | 5.385e+05 | 8,533 | 98.31 | 90.38 | 63.12 |
| 1031 | 2.492e+06 | 1.964e+06 | 1.315e+06 | 27,385 | 91.02 | 71.73 | 48.03 |
| 2053 | 8.838e+06 | 5.203e+06 | 3.127e+06 | 66,118 | 133.68 | 78.70 | 47.30 |
| 4099 | 3.172e+07 | 1.442e+07 | 7.929e+06 | 1.410e+05 | 224.99 | 102.32 | 56.24 |
| 8209 | 1.221e+08 | 4.053e+07 | 1.984e+07 | 2.844e+05 | 429.20 | 142.49 | 69.74 |
| 16411 | 4.614e+08 | 1.148e+08 | 5.217e+07 | 1.281e+06 | 360.17 | 89.61 | 40.73 |
| 32771 | 1.813e+09 | 3.110e+08 | 1.286e+08 | 3.197e+06 | 567.20 | 97.28 | 40.24 |
| 65537 | - | 8.760e+08 | 3.298e+08 | 1.048e+07 | - | 83.63 | 31.49 |

### Table: total cost in GAE at calibrated (informational), and ratio to R

| p | A | B | C | R | A/R | B/R | C/R |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 131 | 2,400 | 2,408 | 2,208 | 691 | 3.47 | 3.48 | 3.19 |
| 257 | 4,022 | 4,154 | 3,151 | 2,335 | 1.72 | 1.78 | 1.35 |
| 521 | 10,671 | 9,597 | 6,397 | 4,433 | 2.41 | 2.16 | 1.44 |
| 1031 | 31,007 | 23,126 | 14,373 | 13,951 | 2.22 | 1.66 | 1.03 |
| 2053 | 1.125e+05 | 60,261 | 33,006 | 33,416 | 3.37 | 1.80 | 0.99 |
| 4099 | 4.103e+05 | 1.658e+05 | 81,757 | 71,024 | 5.78 | 2.33 | 1.15 |
| 8209 | 1.548e+06 | 4.490e+05 | 1.946e+05 | 1.431e+05 | 10.82 | 3.14 | 1.36 |
| 16411 | 5.855e+06 | 1.250e+06 | 4.945e+05 | 6.437e+05 | 9.10 | 1.94 | 0.77 |
| 32771 | 2.289e+07 | 3.389e+06 | 1.221e+06 | 1.606e+06 | 14.25 | 2.11 | 0.76 |
| 65537 | - | 9.469e+06 | 3.063e+06 | 5.262e+06 | - | 1.80 | 0.58 |

### Table: sensitivity, native additions included (mean)

| p | C native mul | C native mul+add | C total w=1 (mul) | C total w=1 (mul+add) | R total w=1 |
|--:|--:|--:|--:|--:|--:|
| 131 | 1.341e+05 | 3.057e+05 | 1.354e+05 | 3.069e+05 | 1,217 |
| 257 | 2.253e+05 | 5.392e+05 | 2.268e+05 | 5.406e+05 | 4,398 |
| 521 | 5.362e+05 | 1.338e+06 | 5.385e+05 | 1.340e+06 | 8,533 |
| 1031 | 1.311e+06 | 3.423e+06 | 1.315e+06 | 3.427e+06 | 27,385 |
| 2053 | 3.118e+06 | 8.538e+06 | 3.127e+06 | 8.547e+06 | 66,118 |
| 4099 | 7.909e+06 | 2.321e+07 | 7.929e+06 | 2.323e+07 | 1.410e+05 |
| 8209 | 1.979e+07 | 6.023e+07 | 1.984e+07 | 6.027e+07 | 2.844e+05 |
| 16411 | 5.207e+07 | 1.661e+08 | 5.217e+07 | 1.662e+08 | 1.281e+06 |
| 32771 | 1.284e+08 | 4.390e+08 | 1.286e+08 | 4.393e+08 | 3.197e+06 |
| 65537 | 3.292e+08 | 1.182e+09 | 3.298e+08 | 1.182e+09 | 1.048e+07 |

### Table: calibration (informational)

| p | ns / Cantor add | ns / mulmod mod p | ns / mulmod mod r | w_rel | w_la |
|--:|--:|--:|--:|--:|--:|
| 1031 | 1040 | 5.14 | 14.18 | 0.0049 | 0.0136 |
| 131 | 1013 | 5.13 | 14.23 | 0.0051 | 0.0141 |
| 16411 | 1084 | 5.02 | 13.96 | 0.0046 | 0.0129 |
| 2053 | 999 | 4.89 | 13.51 | 0.0049 | 0.0135 |
| 257 | 1030 | 5.13 | 14.24 | 0.0050 | 0.0138 |
| 32771 | 1081 | 4.98 | 13.75 | 0.0046 | 0.0127 |
| 4099 | 1062 | 5.15 | 14.30 | 0.0049 | 0.0135 |
| 521 | 1030 | 5.13 | 14.17 | 0.0050 | 0.0138 |
| 65537 | 1106 | 5.04 | 13.96 | 0.0046 | 0.0126 |
| 8209 | 1092 | 5.15 | 14.17 | 0.0047 | 0.0130 |

### Table: fitted exponents (least squares of ln cost on ln p)

| series | slope | coefficient | points p |
|--|--:|--:|--|
| A_full/group_ops | 0.938 | 5.61 | 521, 1031, 2053, 4099, 8209, 16411, 32771 |
| A_full/total_cal | 1.867 | 0.0785 | 521, 1031, 2053, 4099, 8209, 16411, 32771 |
| A_full/total_w0 | 0.938 | 5.61 | 521, 1031, 2053, 4099, 8209, 16411, 32771 |
| A_full/total_w01 | 1.861 | 0.653 | 521, 1031, 2053, 4099, 8209, 16411, 32771 |
| A_full/total_w1 | 1.866 | 6.22 | 521, 1031, 2053, 4099, 8209, 16411, 32771 |
| B_single_lp/group_ops | 1.240 | 0.829 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| B_single_lp/total_cal | 1.434 | 1.12 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| B_single_lp/total_w0 | 1.240 | 0.829 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| B_single_lp/total_w01 | 1.458 | 8.14 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| B_single_lp/total_w1 | 1.462 | 77.9 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| C_double_lp/group_ops | 1.158 | 1.43 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| C_double_lp/total_cal | 1.281 | 1.97 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| C_double_lp/total_w0 | 1.158 | 1.43 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| C_double_lp/total_w01 | 1.325 | 13.5 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| C_double_lp/total_w1 | 1.329 | 128 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| R_rho_negation/group_ops | 1.427 | 0.573 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| R_rho_negation/total_cal | 1.427 | 0.576 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| R_rho_negation/total_w0 | 1.427 | 0.573 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| R_rho_negation/total_w01 | 1.428 | 0.624 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |
| R_rho_negation/total_w1 | 1.433 | 1.08 | 521, 1031, 2053, 4099, 8209, 16411, 32771, 65537 |

### Crossover

```json
{
 "0": {
  "first_p_C_below_R": 257,
  "fit_crossover_p": 29.903517315754204
 },
 "0.1": {
  "first_p_C_below_R": null,
  "fit_crossover_p": 10348783237051.592
 },
 "1": {
  "first_p_C_below_R": null,
  "fit_crossover_p": 1.2142291186455482e+20
 },
 "cal": {
  "first_p_C_below_R": 2053,
  "fit_crossover_p": 4706.030054311983
 }
}
```
strict_all_prices_first_p: None
all_certified: True
runs: 572 ic: 428 rho: 144
splits/trials per arm and p (smoothness sanity):
  p=131 A_full: 0.1552
  p=131 B_single_lp: 0.1550
  p=131 C_double_lp: 0.1551
  p=257 A_full: 0.1639
  p=257 B_single_lp: 0.1633
  p=257 C_double_lp: 0.1638
  p=521 A_full: 0.1754
  p=521 B_single_lp: 0.1770
  p=521 C_double_lp: 0.1766
  p=1031 A_full: 0.1629
  p=1031 B_single_lp: 0.1631
  p=1031 C_double_lp: 0.1634
  p=2053 A_full: 0.1695
  p=2053 B_single_lp: 0.1693
  p=2053 C_double_lp: 0.1693
  p=4099 A_full: 0.1667
  p=4099 B_single_lp: 0.1666
  p=4099 C_double_lp: 0.1669
  p=8209 A_full: 0.1689
  p=8209 B_single_lp: 0.1688
  p=8209 C_double_lp: 0.1689
  p=16411 A_full: 0.1661
  p=16411 B_single_lp: 0.1651
  p=16411 C_double_lp: 0.1654
  p=32771 A_full: 0.1647
  p=32771 B_single_lp: 0.1652
  p=32771 C_double_lp: 0.1650
  p=65537 B_single_lp: 0.1664
  p=65537 C_double_lp: 0.1666

## Fitted exponents (least squares of `ln cost` on `ln p`, points `p >= 500`)

Points used are listed in the table above ("points p"). B and C use the eight primes 521 to 65537; A uses
521 to 32771 (seven points); R uses the eight primes [pcc:unverified].

| series | exponent (measured) | theory | note |
|--|--:|--:|--|
| A group ops | 0.938 | 1 | `~3 p` trials |
| A total, `w = 0.1` | 1.861 | 2 | linear algebra `~ p^2` |
| B group ops | 1.240 | 1.5 | `p^{3/4}` base; still pre-asymptotic |
| C group ops | 1.158 | 4/3 | `p^{2/3}` base; still pre-asymptotic |
| C total, `w = 1` | 1.329 | 4/3 | |
| C total, `w = 0.1` | 1.325 | 4/3 | |
| C total, calibrated | 1.281 | 4/3 | |
| R group ops (and total) | 1.427 | 3/2 | cofactor `h` varies with `p`: `sqrt(r)` carries `h^{-1/2}` |

The measured slope gap between C and R is 0.10 at `w = 1` and 0.15 at the calibrated point, against the
asymptotic 1/6 = 0.167 [pcc:unverified]. Extrapolating the two `w = 0.1` fits gives a crossover near `p` = 1.0e13, and
at `w = 1` near 1.2e20 (`crossover` block of `control.json`) [pcc:unverified]. These are extrapolations over
many orders of magnitude from a gap of about 0.1 in slope; they say "not in range", not a prediction.

## Prediction scorecard (PROTOCOL.md section 7)

| # | Prediction | Outcome | Result |
|---|---|---|---|
| P1 | `w = 0`: C below R at every `p >= 257` | C/R from 0.62 (257) to 0.114 (65537), all below 1 | held |
| P2 | `w = 0.1`: C/R > 1 everywhere; C/R at 16411 in [8, 40]; smaller at 65537 than at 16411 | C/R > 1 everywhere; 7.54 at 16411 (outside the interval, just below 8); 5.82 at 65537 (smaller) | **partly falsified** (interval) |
| P3 | `w = 1`: C/R > 1 everywhere; C/R at 16411 in [40, 250] | 40.7 at 16411, above 1 everywhere | held (at the lower edge) |
| P4 | calibrated C/R at 16411 in [0.7, 2.0] | 0.77 | held |
| P5 | decision rule fails | `strict_all_prices_first_p = null` | held |
| P6 | exponents: C group ops 1.0 to 1.4; C total `w = 1` 1.25 to 1.55; R total `w = 1` 1.35 to 1.6; A total `w = 0.1` 1.8 to 2.15; A group ops 0.95 to 1.1 | 1.158; 1.329; 1.433; 1.861; 0.938 | four held, **A group ops falsified** (0.938, below 0.95) |
| P7 | `w = 0.1` fitted crossover above 1e6 | 1.0e13 | held |
| P8 | every log certifies | 572 of 572 | held |

Two of the eight predictions have a falsified part, each on a margin: the `w = 0.1` ratio at 16411 is 0.46 below its
interval (C closer to R than predicted) and the A group-operation slope is 0.012 below its range. Neither changes the
verdict [pcc:unverified].

## Uncertainty (bootstrap over runs, 2000 resamples, seed 20261112)

Rho's run-to-run spread dominates: its coefficient of variation is 0.37 to 1.10 per prime, against 0.002 to 0.73 for C
(the largest for C at 131) [pcc:unverified]. Mean rho at 65537 rests on 4 runs.

| p | n_C | n_R | key | C/R | 95% interval | CV(C) | CV(R) |
|--:|--:|--:|--|--:|--|--:|--:|
| 131 | 20 | 20 | total_w0 | 1.804 | [1.343, 2.580] | 0.725 | 0.365 |
| 131 | 20 | 20 | total_w01 | 19.769 | [14.526, 29.218] | 0.779 | 0.372 |
| 131 | 20 | 20 | total_w1 | 111.267 | [81.246, 162.666] | 0.783 | 0.411 |
| 131 | 20 | 20 | total_cal | 3.194 | [2.445, 4.423] | 0.654 | 0.365 |
| 257 | 20 | 20 | total_w0 | 0.621 | [0.493, 0.803] | 0.042 | 0.545 |
| 257 | 20 | 20 | total_w01 | 9.470 | [7.530, 12.124] | 0.058 | 0.547 |
| 257 | 20 | 20 | total_w1 | 51.560 | [41.241, 66.320] | 0.060 | 0.561 |
| 257 | 20 | 20 | total_cal | 1.350 | [1.093, 1.724] | 0.058 | 0.545 |
| 521 | 20 | 20 | total_w0 | 0.530 | [0.400, 0.716] | 0.058 | 0.674 |
| 521 | 20 | 20 | total_w01 | 11.599 | [8.837, 15.853] | 0.089 | 0.677 |
| 521 | 20 | 20 | total_w1 | 63.115 | [47.319, 85.936] | 0.090 | 0.690 |
| 521 | 20 | 20 | total_cal | 1.443 | [1.100, 1.959] | 0.100 | 0.674 |
| 1031 | 20 | 20 | total_w0 | 0.316 | [0.221, 0.463] | 0.043 | 0.906 |
| 1031 | 20 | 20 | total_w01 | 8.893 | [6.228, 13.035] | 0.064 | 0.908 |
| 1031 | 20 | 20 | total_w1 | 48.029 | [33.357, 71.955] | 0.065 | 0.918 |
| 1031 | 20 | 20 | total_cal | 1.030 | [0.717, 1.535] | 0.071 | 0.906 |
| 2053 | 20 | 20 | total_w0 | 0.271 | [0.201, 0.397] | 0.036 | 0.781 |
| 2053 | 20 | 20 | total_w01 | 8.780 | [6.562, 12.906] | 0.070 | 0.782 |
| 2053 | 20 | 20 | total_w1 | 47.299 | [34.957, 71.427] | 0.071 | 0.785 |
| 2053 | 20 | 20 | total_cal | 0.988 | [0.728, 1.461] | 0.082 | 0.781 |
| 4099 | 12 | 12 | total_w0 | 0.282 | [0.171, 0.591] | 0.020 | 1.103 |
| 4099 | 12 | 12 | total_w01 | 10.434 | [6.388, 22.237] | 0.019 | 1.104 |
| 4099 | 12 | 12 | total_w1 | 56.245 | [33.902, 119.934] | 0.019 | 1.106 |
| 4099 | 12 | 12 | total_cal | 1.151 | [0.694, 2.445] | 0.021 | 1.103 |
| 8209 | 12 | 12 | total_w0 | 0.317 | [0.243, 0.437] | 0.016 | 0.530 |
| 8209 | 12 | 12 | total_w01 | 12.925 | [9.987, 17.740] | 0.033 | 0.530 |
| 8209 | 12 | 12 | total_w1 | 69.745 | [52.736, 96.106] | 0.033 | 0.531 |
| 8209 | 12 | 12 | total_cal | 1.360 | [1.046, 1.848] | 0.042 | 0.530 |
| 16411 | 8 | 8 | total_w0 | 0.164 | [0.112, 0.277] | 0.015 | 0.690 |
| 16411 | 8 | 8 | total_w01 | 7.538 | [5.222, 12.472] | 0.009 | 0.690 |
| 16411 | 8 | 8 | total_w1 | 40.729 | [27.900, 67.977] | 0.009 | 0.690 |
| 16411 | 8 | 8 | total_cal | 0.768 | [0.517, 1.262] | 0.009 | 0.690 |
| 32771 | 8 | 8 | total_w0 | 0.157 | [0.105, 0.251] | 0.011 | 0.704 |
| 32771 | 8 | 8 | total_w01 | 7.444 | [4.937, 11.685] | 0.005 | 0.705 |
| 32771 | 8 | 8 | total_w1 | 40.237 | [26.738, 62.560] | 0.005 | 0.705 |
| 32771 | 8 | 8 | total_cal | 0.760 | [0.503, 1.186] | 0.005 | 0.704 |
| 65537 | 4 | 4 | total_w0 | 0.114 | [0.079, 0.184] | 0.007 | 0.524 |
| 65537 | 4 | 4 | total_w01 | 5.818 | [3.839, 9.370] | 0.003 | 0.524 |
| 65537 | 4 | 4 | total_w1 | 31.485 | [20.782, 52.650] | 0.003 | 0.524 |
| 65537 | 4 | 4 | total_cal | 0.582 | [0.384, 0.938] | 0.002 | 0.524 |

## What it implies for the earlier "distance to rho" figures

1. **The instrument can read a win at `w = 0`.** In the one regime where IC should beat rho, group operations alone put C at
   0.114 of rho at 65537, with a slope gap of 0.27 to rho at `w = 0` (1.158 against 1.427) [pcc:unverified]. So the earlier
   `w = 0` ratios (for example IC2/rho_E = 47.11 at 64007 in `tower_large`) are real losses of the group-operation count, not an
   artefact of an instrument that cannot show a win [pcc:unverified].
2. **The instrument cannot read a win at `w = 0.1` or `w = 1` at these sizes, even when the algorithm wins asymptotically.**
   The best known IC variant in its winning regime sits at 5.8 to 9.4 times rho at `w = 0.1` and 20.8 to 52.7 times at `w = 1` at
   `p = 65537` (intervals) [pcc:unverified]. The earlier statements "above rho at every weight" in
   `tower_large` therefore carry no information about the transfer at `w = 0.1` and `w = 1`: the positive control reads the same
   way. Only the `w = 0` column, and the order across the band, discriminate [pcc:unverified].
3. **Distances read at `w = 0.1` and `w = 1` overstate a gap that a calibrated multiplication price would shrink.** The control's own
   C/R drops from 5.82 (`w = 0.1`) and 31.5 (`w = 1`) to 0.58 at the measured price, because the band prices a modular
   multiplication in `F_p` at 0.1 to 1 Cantor additions while a Cantor addition takes the time of about 80 to 200 of them here
   (ns ratio 0.013 to 0.005) [pcc:unverified]. Any earlier distance whose gap is below the control's own (about 6 at
   `w = 0.1`, about 31 at `w = 1`) is not distinguishable from a winning algorithm's reading [pcc:unverified].
4. **Sparse linear algebra and large primes were not what limited the earlier rows.** The genus-3 linear algebra quoted in NEXT_STEPS
   (2.2e9 mulmods at 64007, a different instance and algorithm) compares with 1.20e8 mulmods for C and 5.73e8 for B at 65537 here, so
   a Wiedemann core on large-prime relations is cheap by comparison; it is not a like-for-like figure [pcc:unverified].

The earlier rows are not revised by this record; it supplies the yardstick: a win in the known-winning regime shows up at `w = 0`, is
unresolved inside the band at 65537, and needs the calibrated price to cross 1.

## Deviations and limits

- Arm A was not run at 65537 (`--full-cap 32771`): its dense core at 65537 was the one unaffordable job. A therefore has seven fit points.
- Single curve and 4 runs at 65537; two curves at 16411 and 32771. The ratios there carry the intervals above, not a point estimate.
- `r` is about `p^{2.3}` to `p^{2.8}`, not `p^3` (cofactor `h` up to 28 in the admitted curves, plus `#J` fluctuation); rho scales with `sqrt(r)`, IC
  with `p`. R's fitted exponent (1.427) carries `h` noise from curve to curve [pcc:unverified].
- The native-operation count of rho is one canonicalisation per step (never below the step count); a fuller count of rho's table
  work would raise R and favour C slightly. The relation phase counts only the work the code does (no skipping of zero coefficients, no Legendre
  pre-filter on the discriminant, which would cut the smoothness test by less than a factor of 2).
- Linear algebra: only the final build, peel and solve are counted; every run solved at the first attempt (`attempts = 1` in all 428 IC runs),
  so no retries are hidden. The stopping rule used an uncounted incremental check, which assumes its cost is at most the counted union-find operations.
- Primary price counts multiplications (the repository convention). Counting additions too raises C's native work 2.3 to 3.6 times (table above);
  the order of C and R is unchanged at every band price [pcc:unverified].
- Timing calibration is a best-of-5 micro-benchmark on a shared host; it is an extra point, never part of the verdict.
- The protocol file was written after the code and before any data; disclosed in PROTOCOL.md.

## Checks run

`cargo test --release --lib chained_transfer::ic_control`: 4 passed (mulmod against u128, roots and smoothness, instance
and arms with relation verification, rho against IC on one instance).
`cargo clippy --release --all-targets -- -D warnings`: clean for the whole crate. `python3 tools/pcc/pcc.py lint-prose` on this file and
PROTOCOL.md: 0 findings.
