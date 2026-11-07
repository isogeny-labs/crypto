# R7.1: complete-cost ordering of kernel-finding routes

Toy and synthetic instances only (the `j = -3375`, `D = -7` vertical instances of round-4 lead 16.3, 61-bit `p`, one instance per `ell`, seeds `0x1630 ^ ell`). No sub-rho claim; no claim about any deployed curve. Code: `examples/r7_1_complete_cost.rs` and the library change in `src/cryptanalysis/isogeny_eval/` (native Rust; Python only plots recorded values and renders the PDF). Preceding results: `round5/r5_3_ntt_sqrt_velu`, `round6/r6_4_vertical_margin_2` (frozen, not modified).

## Hypothesis, falsifier, verdict (stated in the task)

- **H.** The ordering of routes (ladder/NTT, `psi_ell` + factoring, `Phi_ell(j, Y)`) and the crossover with rho at `r = 2^128` survive when additions and `Phi_ell` construction are counted, with the addition weight `a` swept over `{0, 0.1, 0.3, 1}` M (one addition or subtraction priced at `a` M).
- **Falsifier.** The ordering of the cheapest two routes changes at some `a` in the band for some `ell` in the measured range, or the exponent CI moves its lower endpoint below 1.5 or its upper endpoint above 2.3.
- **Verdict: the ordering part of the falsifier is triggered; H is not confirmed as stated.** The exponent part is not triggered.
  1. *Ordering, triggered in one accounting.* With `Phi_ell` construction charged (cold) and ascending edges, the cheapest two routes are the ladder and one of `{psi, Phi cold}`; their second place swaps with `a` at `ell = 89, 109, 131` (3 of 22 `ell`; over the four weights the two routes come within 1.6%, 0.3%, 0.4% of each other, so this is a near tie in one instance per `ell`, not a robust reordering). The ladder is the cheapest route at every `ell`, every `a`, both directions, in the cold accounting. In the other three accountings the cheapest two (and the full order) never change with `a`: amortised (`Phi_ell` given), descending and ascending, 25 and 22 values of `ell`; cold descending, 25 values of `ell`. Amortised order: `Phi(j,Y)` < ladder < `psi`, every adjacent cost ratio at least 4.7 (`ell = 13`) and at least 55 at `ell = 131`. Cold order at `a = 0`: ladder < `Phi` < `psi`, descending, at every `ell` (smallest adjacent ratio 1.6 at `ell = 11`, 2.1 at `ell = 131`).
  2. *Exponent, not triggered.* The generic-kernel exponent (projected ladder, `PolyMul::Auto`, `ell >= 2^13`) is `1.919` at every `a`, with 95% OLS interval at worst `[1.717, 2.121]` and bootstrap interval at worst `[1.781, 2.126]` over the four weights (the best-of Karatsuba/Auto series: OLS `[1.717, 2.121]`, bootstrap `[1.776, 2.146]`); all lie inside `[1.5, 2.3]`. At `a = 0` the numbers are R6.4's `1.919 [1.724, 2.115]`. This remains insufficient to show an exponent below 2 (the intervals contain 2 and values below it); it is also not evidence that the exponent is at least 2.
  3. *Crossover with rho (supplementary; not part of the falsifier).* At `r = 2^128`, `g = 6`: ladder crossover `log2 ell*` shifts by `-0.24` to `-0.39` bit between `a = 0` and `a = 1` (`21.74 -> 21.35` measured fit, `25.07 -> 24.81` projected fit), `psi` by `-0.37` bit, `Phi` cold by `-0.01` bit, `Phi` amortised by `-1.48` bit (an extrapolation of a fit over `ell <= 103` out to `2^30`). The rank order of the routes by `ell*` does not change with `a`. All are extrapolations.

## What counts as one addition (library tally, opt-in)

`PrimeField::counted_with_adds(p)` tallies two new counters in an `AddTally` (`PrimeField::add_tally`, `complete_tally`); `PrimeField::counted(p)`, `PrimeField::new(p)` and the type `OpTally` are unchanged, so `M`, `W`, `I` and every frozen output are as before. One *addition* is one modular addition, subtraction or negation of canonical residues (the conditional correction is part of it):

- `A` (in `F_p`): every call of `PrimeField::add`, `sub`, `neg`; the `la*lb - (la + lb - 1)` accumulations of a schoolbook product (an output coefficient with `t` terms needs `t - 1`; the lazy `u128` accumulation and its periodic `% p` stay inside `M`, as before); `2n` for an NTT product (the two `u128` accumulations per output coefficient of `r1 + (q1 mod p) t2 + (q1 q2 mod p) t3`). Karatsuba's sums and subtractions are `add`/`sub` calls and are counted.
- `WA` (modulo an NTT prime): `3 T N log2 N` butterfly additions and subtractions (`T = 3` transforms, `2` for a square; each butterfly is one addition and one subtraction), `3n` Garner differences, and the conditional subtractions that actually happen when inputs are brought below each NTT prime and in Garner's cross-reductions (data dependent; counted at run time). Closed form without the data-dependent part: `ntt_mul_add_cost`.
- Priced at `Meq + a (A + WA)`, `a` in `{0, 0.1, 0.3, 1}`. `PolyMul::Auto` still picks Karatsuba or the NTT by `M + W` alone (so frozen counts do not depend on whether additions are counted); a best-of Karatsuba/Auto whole-ladder series is reported as a check.
- Not additions here: inversions' internal operations (`I = 91 M` as before), multiplications by small constants (`M`, as before), reductions `% p`, RNG, table construction.

Proof that frozen outputs are unchanged when additions are not requested:

- unit tests `additions_are_opt_in_and_leave_frozen_counts_unchanged` and `addition_counts_of_products_are_exact` (`src/cryptanalysis/isogeny_eval/tests.rs`): same results and same `M`, `W`, `I` from `counted`, `counted_with_adds` and `new` over a workload that reaches schoolbook, Karatsuba, NTT (products and squares), Newton, remainder and product trees, and sqrt-Velu; additions exactly zero when not requested; exact addition counts for schoolbook, NTT within the closed form. All 18 `isogeny_eval` tests pass (`build_mini/test_isogeny_eval.txt`); the existing exact-count tests are unmodified and pass.
- the *unmodified* round-6 example rebuilt against the modified library reproduces its frozen records: `frozen_check/` (9/9 ladder rows, 2/2 `psi` rows, 5/5 `modpoly` rows identical in every field except wall-clock keys; `frozen_check/compare.py`).
- the new example's `a = 0` columns reproduce R6.4's counts (`analysis.json`, `reproduction_of_frozen_r6_4`): ladder 20/20 rows (`M, W, I` of find, ladder and minpoly, and the expected find), `psi` 9/9, `Phi` evaluation + roots 23/23, projection 119/119 arm rows (all three `PolyMul` arms).

## Route (c): `Phi_ell` construction and the Elkies step

- **Construction (counted).** A counted port of the library's `isogeny_walk::modpoly::modular_polynomial` (q-series of `j`, the powers `A[a]`, `B[b]`, triangular solve for the symmetric unknowns with every other exponent checked), on the counted `F_p`. Same loops; each executed `(i, j)` of the library's `poly::mul`/`mul_trunc` is one product and one addition (zero left factors skipped, zero right factors not), so the construction has `A` close to `M`. The fast version accumulates in `u128` and adds the `M` and `A` counts in bulk; `selftest` shows it equals a per-operation port (same table, same tally) and the library's table for `ell` in `11..31` (`selftest.json`); the table equals the library's for all 23 `ell <= 103` (`port_equals_library_phi`, `library_phi_wall_s_contended`).
- **Elkies kernel polynomial (counted).** A counted port of `kernel::elkies_isogeny` (without its `verify_kernel` certificate, which no route's count includes) that gets `Phi_X`, `Phi_XX`, `Phi_XY`, `Phi_YY` from shared row sums (the library's `partial()` recomputes powers per term; the port is the cheaper schedule, which favours route (c)); the kernel polynomial and codomain equal the library's for all 28 `ell` in both directions.
- **Sutherland's CRT evaluation of `Phi_ell(j, Y) mod p` is not implemented.** Not feasible in this session without the CM-method / volcano-walk infrastructure it needs (it computes `Phi_ell` modulo many small primes by isogeny-graph walks, then lifts); this is a scope limit of this session, not a mathematical obstruction. The cold cost below is therefore the library's builder (exponent 4.69), not a state-of-the-art one; Sutherland's method is recalled to be far cheaper in `ell` (heuristically about `ell^3` for the whole polynomial, less for one `Phi_ell(j, Y)`; recalled, not verified here and not used). Construction/ladder at `a = 0`: `7.5` (`ell = 37`), `28` (`67`), `81` (`103`), `129` (`131`); a construction method would have to be that many times cheaper than the library's for cold `Phi` to match the ladder at those `ell`.
- **Cold and amortised, reported separately.** Cold: construction + evaluation + roots + Elkies, charged once per edge. Amortised: `Phi_ell` already available (a table of `(ell + 2)^2` words reused over targets); evaluation + roots + Elkies only. R6.4's row (c) was evaluation + roots only (still reported: `Phi eval+roots only`).

## Commands

```
B=target/release/examples/r7_1_complete_cost; D=research/isogeny_fibers_20261010/round7/r7_1_complete_cost
# research/isogeny_fibers_20261010/round7/r7_1_complete_cost/run_all.sh runs, three at a time:
$B selftest 11,13,17,19,23,29,31 $D/selftest.json
$B ladder 37 470 0.25 100000 auto 3 $D/ladder_grid_a.json
$B ladder 500 900 0.25 100000 auto 1 $D/ladder_grid_b.json
$B ladder_list 11,13,17,19,23,29,31,43,53,61,71,73,83,89,101,103 100000 auto 3 $D/ladder_extra_a.json
$B project 5 13 0.25 auto,karatsuba $D/project_a.json
$B project 13.25 16 0.25 auto,karatsuba $D/project_b.json
$B project 5 12 0.25 ntt $D/project_ntt.json
$B psi 37,41,47,59,67 100000 $D/psi_grid_a.json ; $B psi 79,97 ... psi_grid_b.json ; $B psi 109,131 ... psi_grid_c.json
$B psi 11,13,17,19,23,29,31,43,53,61,71,73 100000 $D/psi_extra_a.json ; $B psi 83,89,101,103 ... psi_extra_b.json
$B modpoly 11 67 100000 103 $D/modpoly_grid_a.json ; $B modpoly 71 103 100000 103 $D/modpoly_grid_b.json
$B modpoly 107 131 100000 0 $D/modpoly_extra_a.json
$B analyze $D research/isogeny_fibers_20261010/round6/r6_4_vertical_margin_2 $D/analysis.json > $D/analysis.txt
python3 $D/plot.py
```

The raw records were produced by the example before its analysis gained the `additions_per_meq` block (the measurement modes are unchanged since the runs; `analysis.json`/`analysis.txt` were regenerated afterwards from the same records).

Build. In this session the full crate's build was OOM-killed (rustc peaked at 6.5 GB in the shared 15 GB container), so the example was built against a minimal crate named `crypto_lib` that includes the same source files (`isogeny_eval`, and the `isogeny_walk` modules `field`, `curve`, `poly`, `modpoly`, `kernel`) through `#[path]`: `build_mini/make.sh <repo> <dir>`, then `cargo build --release --offline --examples` in `<dir>`. The example source is the repository file; with enough memory use `cargo build --release --example r7_1_complete_cost` in the repository. `cargo +1.98 clippy --release --example r7_1_complete_cost --tests -- -D warnings` and `rustfmt --edition 2021 --check` are clean on the changed files (run in the same minimal crate). The full-crate `cargo test --release --lib` was not run in this session (same memory limit); CI will run it.

Instances, unit and weights. `Meq = M + W + 91 I` (R6.4's unit, `W = 1 M`), cost `= Meq + a (A + WA)`. Arms as in R6.4: (a) `F_(p^k)` ladder, `k = (ell - 1)/2`, `PolyMul::Auto`, expected find `= 2 x` (per-attempt ladder) `+` minpoly; (a') the per-step cost measured in `F_(p^k)` up to `ell = 2^16` times the exact step count (projection/measured `1.0006..1.0062`, all weights); (b) `psi_ell` recurrences and Cantor-Zassenhaus (crater) or gcd with `x^(p^k) - x` (floor); (c) as above. Grids: ladder `37..863` (20 `ell`, as R6.4) plus 16 extra primes `11..103`; projection `5..16` in steps of `2^0.25` (45 `ell`); `psi` the 9 R6.4 primes `37..131` plus 16 extra; `Phi` all 23 primes `11..103` (as R6.4's grid) plus `107..131` (5 `ell`). Fits and the exponent falsifier use the R6.4 grids only; the extra rows enter the ordering tables. The ladder arm, instances and RNG streams are R6.4's, so the same `ell` gives the same `p` and the same counts at `a = 0`. Wall times are contended (4 shared cores; `*_wall_s_contended`) and not used.

## Exponents per route and addition weight

`log2(cost) = log2 c + e log2 ell`, OLS; both intervals are 95%: the OLS t-interval (as R6.4) and a pairs bootstrap (2000 resamples, percentile, seed from the series name). Series ranges as R6.4. `n` is the number of `ell`.

| series | n | a = 0 | a = 0.1 | a = 0.3 | a = 1 |
|---|---|---|---|---|---|
| (a) ladder, measured desc, ell >= 256 | 8 | 2.554 OLS [2.352, 2.756] boot [2.376, 2.713] | 2.566 OLS [2.380, 2.753] boot [2.399, 2.710] | 2.584 OLS [2.420, 2.748] boot [2.434, 2.715] | 2.612 OLS [2.482, 2.742] boot [2.498, 2.720] |
| (a) ladder, measured asc, ell >= 256 | 8 | 2.554 OLS [2.352, 2.756] boot [2.375, 2.707] | 2.566 OLS [2.380, 2.752] boot [2.406, 2.710] | 2.584 OLS [2.419, 2.748] boot [2.442, 2.714] | 2.612 OLS [2.482, 2.742] boot [2.496, 2.725] |
| (a') projected, Karatsuba, ell >= 2^10 | 25 | 2.579 OLS [2.553, 2.605] boot [2.556, 2.604] | 2.581 OLS [2.558, 2.604] boot [2.562, 2.603] | 2.584 OLS [2.564, 2.603] boot [2.567, 2.603] | 2.588 OLS [2.575, 2.601] boot [2.576, 2.601] |
| (a') projected, Auto, ell >= 2^10 | 25 | 2.088 OLS [2.020, 2.155] boot [2.023, 2.158] | 2.092 OLS [2.024, 2.160] boot [2.029, 2.163] | 2.097 OLS [2.028, 2.167] boot [2.029, 2.169] | 2.105 OLS [2.034, 2.177] boot [2.035, 2.179] |
| (a') projected, Auto, ell >= 2^13 | 13 | 1.919 OLS [1.724, 2.115] boot [1.795, 2.113] | 1.919 OLS [1.722, 2.116] boot [1.781, 2.113] | 1.919 OLS [1.720, 2.118] boot [1.790, 2.126] | 1.919 OLS [1.717, 2.121] boot [1.783, 2.125] |
| (a') projected, best of Karatsuba/Auto, ell >= 2^13 | 13 | 1.919 OLS [1.724, 2.115] boot [1.779, 2.123] | 1.919 OLS [1.722, 2.116] boot [1.780, 2.122] | 1.919 OLS [1.720, 2.118] boot [1.791, 2.146] | 1.919 OLS [1.717, 2.121] boot [1.776, 2.122] |
| (b) psi + CZ desc, 37..131 | 9 | 3.565 OLS [3.302, 3.828] boot [3.316, 3.792] | 3.588 OLS [3.331, 3.845] boot [3.346, 3.819] | 3.618 OLS [3.370, 3.867] boot [3.384, 3.833] | 3.665 OLS [3.428, 3.903] boot [3.454, 3.856] |
| (b) psi + gcd asc, 37..131 | 8 | 3.439 OLS [3.087, 3.791] boot [3.187, 3.724] | 3.456 OLS [3.106, 3.806] boot [3.222, 3.738] | 3.478 OLS [3.130, 3.826] boot [3.228, 3.743] | 3.512 OLS [3.166, 3.859] boot [3.270, 3.764] |
| (c) Phi amortised desc, 11..103 | 23 | 1.860 OLS [1.785, 1.936] boot [1.779, 1.932] | 1.882 OLS [1.808, 1.956] boot [1.804, 1.959] | 1.914 OLS [1.842, 1.987] boot [1.838, 1.988] | 1.976 OLS [1.906, 2.045] boot [1.900, 2.043] |
| (c) Phi amortised asc, 11..103 | 20 | 1.710 OLS [1.631, 1.790] boot [1.608, 1.770] | 1.735 OLS [1.657, 1.813] boot [1.639, 1.791] | 1.773 OLS [1.697, 1.849] boot [1.676, 1.830] | 1.845 OLS [1.771, 1.920] boot [1.745, 1.899] |
| (c) Phi cold desc, 11..103 | 23 | 4.638 OLS [4.581, 4.696] boot [4.574, 4.751] | 4.639 OLS [4.582, 4.696] boot [4.577, 4.750] | 4.640 OLS [4.583, 4.697] boot [4.572, 4.753] | 4.642 OLS [4.586, 4.698] boot [4.576, 4.745] |
| (c) Phi cold asc, 11..103 | 20 | 4.703 OLS [4.655, 4.751] boot [4.644, 4.805] | 4.703 OLS [4.656, 4.751] boot [4.647, 4.804] | 4.704 OLS [4.657, 4.752] boot [4.648, 4.807] | 4.705 OLS [4.658, 4.752] boot [4.650, 4.802] |
| (c) Phi construction alone, 11..103 | 23 | 4.685 OLS [4.641, 4.729] boot [4.632, 4.768] | 4.685 OLS [4.641, 4.729] boot [4.635, 4.769] | 4.685 OLS [4.641, 4.729] boot [4.636, 4.771] | 4.685 OLS [4.641, 4.729] boot [4.634, 4.768] |
| (c) eval + roots only (R6.4's definition), 11..103 | 23 | 1.884 OLS [1.804, 1.964] boot [1.796, 1.962] | 1.904 OLS [1.825, 1.982] boot [1.813, 1.981] | 1.934 OLS [1.858, 2.010] boot [1.853, 2.011] | 1.990 OLS [1.917, 2.062] boot [1.912, 2.064] |

Reading (derived from the table and the ordering costs): at `a = 1` the cost relative to `a = 0` is `2.0` (`Phi` cold, `ell = 103` and `131`), `2.4` and `2.3` (ladder), `2.4` and `2.3` (`Phi` amortised), `2.8` and `2.8` (`psi`), for `ell = 103` and `131`; the additions per `Meq` behind this are (`additions_per_meq` in `analysis.json`): `Phi` construction `1.00`; ladder `0.99` at `ell = 37`, `1.12` at `97`, `1.47` at `223`, `1.61` at `863`; `psi` find `1.42` at `37`, `1.81` at `97`, `1.82` at `131`; projected `xDBL`/`xADD` at `ell = 65537` `1.59`/`1.61` with Karatsuba and `1.78`/`1.79` with the NTT. The exponents move by at most `+0.06` (ladder), `+0.10` (`psi`), `+0.12` (`Phi` amortised, `a = 1`), `+0.004` (`Phi` cold); the projected NTT exponent is unchanged to three digits (the NTT's `N log N` additions scale with its `N log N` multiplications). Best of Karatsuba/Auto equals Auto at `ell >= 2^13` at every weight (the NTT stays cheaper than Karatsuba even though it has more additions per `Meq`).

## Ordering of routes per `(ell, a)`

`L` ladder, `P` `psi` + factoring, `F` `Phi` amortised, `C` `Phi` cold. "order, amortised" ranks `{L, P, F}`, "order, cold" ranks `{L, P, C}`; cheapest first. The full set (all `ell`, all four weights, both directions) is in `analysis.json` (`ordering`, `ordering_summary`); `analysis.txt` lists the 94 `(direction, accounting, ell)` cases with the smallest adjacent cost ratio.

#### Descending edges (crater to floor)

Cost in M-equivalents (Meq + a(A+WA)); order = cheapest first (ladder / psi / Phi-amortised / Phi-cold)
| ell | a | ladder (a) | psi (b) | Phi amortised (c) | Phi cold (c) | order, amortised | order, cold |
|---|---|---|---|---|---|---|---|
| 11 | 0.0 | 1.891e+05 | 5.580e+06 | 3.774e+04 | 3.036e+05 | F < L < P | L < C < P |
| 11 | 0.1 | 2.071e+05 | 6.290e+06 | 4.078e+04 | 3.331e+05 | F < L < P | L < C < P |
| 11 | 0.3 | 2.431e+05 | 7.710e+06 | 4.685e+04 | 3.921e+05 | F < L < P | L < C < P |
| 11 | 1.0 | 3.689e+05 | 1.268e+07 | 6.813e+04 | 5.987e+05 | F < L < P | L < C < P |
| 23 | 0.0 | 1.899e+06 | 7.730e+07 | 1.707e+05 | 6.816e+06 | F < L < P | L < C < P |
| 23 | 0.1 | 2.085e+06 | 8.847e+07 | 1.861e+05 | 7.495e+06 | F < L < P | L < C < P |
| 23 | 0.3 | 2.456e+06 | 1.108e+08 | 2.170e+05 | 8.854e+06 | F < L < P | L < C < P |
| 23 | 1.0 | 3.754e+06 | 1.890e+08 | 3.252e+05 | 1.361e+07 | F < L < P | L < C < P |
| 37 | 0.0 | 8.132e+06 | 6.367e+08 | 4.877e+05 | 6.129e+07 | F < L < P | L < C < P |
| 37 | 0.1 | 8.934e+06 | 7.271e+08 | 5.405e+05 | 6.742e+07 | F < L < P | L < C < P |
| 37 | 0.3 | 1.054e+07 | 9.078e+08 | 6.461e+05 | 7.968e+07 | F < L < P | L < C < P |
| 37 | 1.0 | 1.615e+07 | 1.540e+09 | 1.016e+06 | 1.226e+08 | F < L < P | L < C < P |
| 59 | 0.0 | 2.576e+07 | 2.586e+09 | 1.039e+06 | 5.677e+08 | F < L < P | L < C < P |
| 59 | 0.1 | 2.885e+07 | 3.027e+09 | 1.168e+06 | 6.245e+08 | F < L < P | L < C < P |
| 59 | 0.3 | 3.503e+07 | 3.909e+09 | 1.426e+06 | 7.380e+08 | F < L < P | L < C < P |
| 59 | 1.0 | 5.667e+07 | 6.997e+09 | 2.330e+06 | 1.136e+09 | F < L < P | L < C < P |
| 79 | 0.0 | 6.180e+07 | 7.617e+09 | 1.886e+06 | 2.337e+09 | F < L < P | L < C < P |
| 79 | 0.1 | 6.892e+07 | 8.939e+09 | 2.112e+06 | 2.571e+09 | F < L < P | L < C < P |
| 79 | 0.3 | 8.315e+07 | 1.158e+10 | 2.562e+06 | 3.038e+09 | F < L < P | L < C < P |
| 79 | 1.0 | 1.329e+08 | 2.083e+10 | 4.140e+06 | 4.675e+09 | F < L < P | L < C < P |
| 97 | 0.0 | 1.131e+08 | 1.845e+10 | 2.566e+06 | 6.367e+09 | F < L < P | L < C < P |
| 97 | 0.1 | 1.258e+08 | 2.179e+10 | 2.896e+06 | 7.003e+09 | F < L < P | L < C < P |
| 97 | 0.3 | 1.513e+08 | 2.845e+10 | 3.556e+06 | 8.277e+09 | F < L < P | L < C < P |
| 97 | 1.0 | 2.403e+08 | 5.176e+10 | 5.867e+06 | 1.273e+10 | F < L < P | L < C < P |
| 103 | 0.0 | 1.048e+08 | 2.113e+10 | 2.489e+06 | 8.541e+09 | F < L < P | L < C < P |
| 103 | 0.1 | 1.194e+08 | 2.487e+10 | 2.833e+06 | 9.396e+09 | F < L < P | L < C < P |
| 103 | 0.3 | 1.486e+08 | 3.235e+10 | 3.523e+06 | 1.110e+10 | F < L < P | L < C < P |
| 103 | 1.0 | 2.508e+08 | 5.853e+10 | 5.936e+06 | 1.708e+10 | F < L < P | L < C < P |
| 109 | 0.0 | 1.248e+08 | 2.354e+10 | 2.772e+06 | 1.127e+10 | F < L < P | L < C < P |
| 109 | 0.1 | 1.419e+08 | 2.771e+10 | 3.161e+06 | 1.240e+10 | F < L < P | L < C < P |
| 109 | 0.3 | 1.761e+08 | 3.606e+10 | 3.939e+06 | 1.466e+10 | F < L < P | L < C < P |
| 109 | 1.0 | 2.959e+08 | 6.529e+10 | 6.664e+06 | 2.255e+10 | F < L < P | L < C < P |
| 131 | 0.0 | 2.151e+08 | 5.815e+10 | 3.839e+06 | 2.783e+10 | F < L < P | L < C < P |
| 131 | 0.1 | 2.433e+08 | 6.875e+10 | 4.357e+06 | 3.061e+10 | F < L < P | L < C < P |
| 131 | 0.3 | 2.997e+08 | 8.994e+10 | 5.393e+06 | 3.617e+10 | F < L < P | L < C < P |
| 131 | 1.0 | 4.971e+08 | 1.641e+11 | 9.019e+06 | 5.565e+10 | F < L < P | L < C < P |

#### Ascending edges (floor to crater; only ell where the ascent reached the crater)

Cost in M-equivalents (Meq + a(A+WA)); order = cheapest first (ladder / psi / Phi-amortised / Phi-cold)
| ell | a | ladder (a) | psi (b) | Phi amortised (c) | Phi cold (c) | order, amortised | order, cold |
|---|---|---|---|---|---|---|---|
| 23 | 0.0 | 1.912e+06 | 4.510e+07 | 7.497e+04 | 6.720e+06 | F < L < P | L < C < P |
| 23 | 0.1 | 2.099e+06 | 5.166e+07 | 8.143e+04 | 7.390e+06 | F < L < P | L < C < P |
| 23 | 0.3 | 2.472e+06 | 6.478e+07 | 9.434e+04 | 8.731e+06 | F < L < P | L < C < P |
| 23 | 1.0 | 3.779e+06 | 1.107e+08 | 1.396e+05 | 1.342e+07 | F < L < P | L < C < P |
| 37 | 0.0 | 8.141e+06 | 3.387e+08 | 2.029e+05 | 6.101e+07 | F < L < P | L < C < P |
| 37 | 0.1 | 8.943e+06 | 3.885e+08 | 2.243e+05 | 6.710e+07 | F < L < P | L < C < P |
| 37 | 0.3 | 1.055e+07 | 4.881e+08 | 2.670e+05 | 7.930e+07 | F < L < P | L < C < P |
| 37 | 1.0 | 1.616e+07 | 8.367e+08 | 4.166e+05 | 1.220e+08 | F < L < P | L < C < P |
| 59 | 0.0 | 2.584e+07 | 1.244e+09 | 3.763e+05 | 5.670e+08 | F < L < P | L < C < P |
| 59 | 0.1 | 2.894e+07 | 1.461e+09 | 4.228e+05 | 6.237e+08 | F < L < P | L < C < P |
| 59 | 0.3 | 3.514e+07 | 1.895e+09 | 5.158e+05 | 7.371e+08 | F < L < P | L < C < P |
| 59 | 1.0 | 5.684e+07 | 3.415e+09 | 8.413e+05 | 1.134e+09 | F < L < P | L < C < P |
| 79 | 0.0 | 6.186e+07 | 3.535e+09 | 6.595e+05 | 2.336e+09 | F < L < P | L < C < P |
| 79 | 0.1 | 6.898e+07 | 4.162e+09 | 7.374e+05 | 2.570e+09 | F < L < P | L < C < P |
| 79 | 0.3 | 8.322e+07 | 5.416e+09 | 8.932e+05 | 3.037e+09 | F < L < P | L < C < P |
| 79 | 1.0 | 1.331e+08 | 9.805e+09 | 1.439e+06 | 4.672e+09 | F < L < P | L < C < P |
| 97 | 0.0 | 1.134e+08 | 9.110e+09 | 8.616e+05 | 6.365e+09 | F < L < P | L < C < P |
| 97 | 0.1 | 1.261e+08 | 1.076e+10 | 9.732e+05 | 7.001e+09 | F < L < P | L < C < P |
| 97 | 0.3 | 1.516e+08 | 1.406e+10 | 1.196e+06 | 8.274e+09 | F < L < P | L < C < P |
| 97 | 1.0 | 2.409e+08 | 2.562e+10 | 1.977e+06 | 1.273e+10 | F < L < P | L < C < P |
| 103 | 0.0 | 1.049e+08 | 9.781e+09 | 8.515e+05 | 8.540e+09 | F < L < P | L < C < P |
| 103 | 0.1 | 1.195e+08 | 1.154e+10 | 9.709e+05 | 9.394e+09 | F < L < P | L < C < P |
| 103 | 0.3 | 1.487e+08 | 1.507e+10 | 1.210e+06 | 1.110e+10 | F < L < P | L < C < P |
| 103 | 1.0 | 2.510e+08 | 2.741e+10 | 2.045e+06 | 1.708e+10 | F < L < P | L < C < P |
| 109 | 0.0 | 1.249e+08 | 1.048e+10 | 9.613e+05 | 1.127e+10 | F < L < P | L < P < C |
| 109 | 0.1 | 1.421e+08 | 1.236e+10 | 1.094e+06 | 1.240e+10 | F < L < P | L < P < C |
| 109 | 0.3 | 1.763e+08 | 1.611e+10 | 1.359e+06 | 1.465e+10 | F < L < P | L < C < P |
| 109 | 1.0 | 2.962e+08 | 2.926e+10 | 2.287e+06 | 2.254e+10 | F < L < P | L < C < P |
| 131 | 0.0 | 2.155e+08 | 2.597e+10 | 1.378e+06 | 2.782e+10 | F < L < P | L < P < C |
| 131 | 0.1 | 2.437e+08 | 3.072e+10 | 1.559e+06 | 3.061e+10 | F < L < P | L < C < P |
| 131 | 0.3 | 3.002e+08 | 4.023e+10 | 1.921e+06 | 3.617e+10 | F < L < P | L < C < P |
| 131 | 1.0 | 4.980e+08 | 7.349e+10 | 3.189e+06 | 5.565e+10 | F < L < P | L < C < P |

Change of the cheapest-two ordering with `a` (counted over the `ell` where all three routes of the accounting are measured): descending amortised 0/25, descending cold 0/25, ascending amortised 0/22, **ascending cold 3/22 (`ell = 89, 109, 131`)**. At `ell = 89` the second and third route (ascending, cold) cost `psi 4.113e9`, `Phi 4.178e9` at `a = 0` and `4.832e9`, `4.596e9` at `a = 0.1`; at `ell = 109`: `1.048e10` vs `1.127e10` (`a = 0`), `1.236e10` vs `1.240e10` (`a = 0.1`), `1.611e10` vs `1.465e10` (`a = 0.3`); at `ell = 131`: `2.597e10` vs `2.782e10` (`a = 0`), `3.072e10` vs `3.061e10` (`a = 0.1`). Reason: additions are `1.8` per `Meq` in `psi` against `1.0` in the `Phi` construction, so `a` moves `psi` up more than `Phi` cold; the two ascending costs come within `1.6%`, `0.3%`, `0.4%` of each other (closest weight) at these `ell`. Fitted lines (measured range, extrapolated): ladder `<` `Phi` cold `<` `psi` is the order up to `ell` about `2^8.1` (`a = 0`) to `2^8.7` (`a = 1`), after which `psi` is cheaper than `Phi` cold (exponents `3.57..3.67` vs `4.64`); `Phi` amortised is cheaper than the ladder at every `ell > 2` in the fits (`analysis.txt`, "fitted pairwise crossings"). These are extrapolations.

## Crossover with rho (extrapolated)

Smallest `ell` with `s x find(ell) + eval(ell) >= rho` where `s = log2 r / 61`, `eval` is the measured cost of evaluating one image point from the kernel polynomial (fitted per weight), and rho `= sqrt(pi r / 4) (g + g a)` M-equivalents: a group operation is `g` multiplications and `g` additions (affine addition with batched inversion: `6 M`, `6` add/sub; stated assumption). `r = 2^128`, `g = 6`, `log2 ell*`; `ell_max = 2^65`, all rows inside the possible range. `g = 12` and `r = 2^64, 2^256` are in `analysis.json` (`crossover`).

| model | a = 0 | a = 0.1 | a = 0.3 | a = 1 | shift a=1 vs a=0 |
|---|---|---|---|---|---|
| ladder (a), measured desc fit l>=256 | 21.74 | 21.66 | 21.54 | 21.35 | -0.39 |
| ladder (a), measured asc fit l>=256 | 21.74 | 21.66 | 21.54 | 21.35 | -0.39 |
| ladder (a'), projected auto fit l>=2^10 | 24.23 | 24.16 | 24.07 | 23.92 | -0.31 |
| ladder (a'), projected auto fit l>=2^13 | 25.07 | 25.02 | 24.94 | 24.81 | -0.25 |
| ladder (a'), projected auto l>=2^13, exponent at OLS CI low end | 26.10 | 26.05 | 25.98 | 25.85 | -0.24 |
| ladder (a'), projected best of karatsuba/auto fit l>=2^13 | 25.07 | 25.02 | 24.94 | 24.81 | -0.25 |
| ladder (a'), model l^2 log l calibrated at top | 24.55 | 24.51 | 24.44 | 24.32 | -0.23 |
| psi + CZ desc (b), measured fit | 15.38 | 15.30 | 15.19 | 15.01 | -0.37 |
| psi + gcd asc (b), measured fit | 16.02 | 15.95 | 15.84 | 15.69 | -0.34 |
| Phi amortised desc (c), measured fit 11..103 | 30.26 | 29.97 | 29.54 | 28.77 | -1.48 |
| Phi cold desc (c), measured fit 11..103 | 13.69 | 13.69 | 13.69 | 13.68 | -0.01 |

R6.4's `a = 0` table is reproduced exactly for the ladder, `psi` and `Phi` rows it had (`21.74`, `24.23`, `25.07`, `26.10`, `24.55`, `15.38`, `16.02`; its `Phi` lower bound `29.98` (evaluation + roots) is now `30.26` for the amortised row, which includes the Elkies step, and `13.69` for the cold row). Observation (derived from the table): with `Phi` construction counted, the extrapolated `Phi` cold crossover (`13.7`) is below `psi`'s (`15.4`), whereas R6.4 had `psi` smallest; in the measured range `Phi` cold is cheaper than `psi` (descending, every `ell`).

## What is still not counted

- Sutherland's CRT evaluation (not implemented); cold route (c) is the library's q-series builder, exponent `4.69`.
- The setup of `F_(p^k)` for the ladder (irreducible polynomial, as in R6.4), the group orders over `F_(p^k)` used for the cofactor, verification/certification of any kernel (`verify_kernel`, image-order checks), key-size dependence (every find cost is scaled by `log2 r / 61`, as in R6.4).
- Additions inside inversions (`I = 91 M` fixed), `% p` reductions, memory traffic and storage of the `Phi_ell` table (`(ell + 2)^2` words), cache effects, any wall time.
- `PolyMul::Auto` chooses by `M + W`, not by the weighted cost; only whole-ladder best-of Karatsuba/Auto is checked (a per-product weighted choice is not implemented).
- The ratio `a` of an addition to a multiplication is swept, not measured on any hardware; the rho step's `g` additions per operation is an assumption.
- One instance per `ell` (the `psi`/CZ and ladder costs are random variables; the near ties above are within that noise); `ell` ranges are those of R6.4 (ladder `<= 863`, `psi <= 131`, `Phi <= 103`, `131` added for the ordering table), larger `ell` and the `2^16` projection are extrapolations; all crossovers are extrapolations of fits.
- Library `cargo test --lib` over the whole crate (see Build).

## Limits and claim hygiene

Toy instances; counted unit, not time; no sub-rho claim and no claim about a deployed curve. The hypothesis was about orderings under a cost model; it is not evidence about any attack on a real curve.

## Files

`analysis.json`, `analysis.txt` (all fits, orderings, crossovers, reproduction checks); raw records `ladder_grid_*.json`, `ladder_extra_a.json`, `project_*.json`, `psi_grid_*.json`, `psi_extra_*.json`, `modpoly_grid_*.json`, `modpoly_extra_a.json`, `selftest.json`, `logs/`; `run_all.sh`, `jobs.txt`, `jobs.sh`; `frozen_check/`; `build_mini/`; figures `fig1_costs_by_route.svg`, `fig2_exponents_by_weight.svg`, `fig3_crossover_r128.svg`, `fig4_phi_breakdown.svg` (`plot.py`); `report.typ`, `report.pdf`.

## Note text (ready to paste into the consolidating section)

R7.1 re-priced the three kernel-finding routes of R6.4 (same instances, `j = -3375`, `D = -7`, 61-bit `p`, one instance per `ell`) with an opt-in addition tally in the library counter (`M`, `W`, `I` and all frozen outputs unchanged; the `a = 0` columns reproduce R6.4 exactly) and added what R6.4's route (c) left out: `Phi_ell` construction by a counted port of the library's q-series builder (equal to the library's table for all 23 `ell <= 103`) and the Elkies kernel polynomial (equal to the library's for all 28 `ell` in both directions). Additions priced at `a` M each, `a` in `{0, 0.1, 0.3, 1}`. Sutherland's CRT evaluation was not implemented, so cold route (c) is the library builder (exponent 4.69, `ell <= 131`); route (c) is reported both cold and amortised (`Phi_ell` given).

Result. The exponent of the projected generic-kernel ladder (`ell >= 2^13`) is `1.919` at every `a` (OLS 95% at worst `[1.717, 2.121]`, bootstrap at worst `[1.781, 2.126]`), so the R6.4 exponent statement stands unchanged: insufficient to show an exponent below 2. The route ordering holds for the amortised accounting (`Phi` < ladder < `psi`, `ell = 11..131`, both directions, every `a`) and for cold descending (ladder < `Phi` < `psi`), but the preregistered ordering falsifier fires for cold ascending edges: at `ell = 89, 109, 131` the second place swaps between `psi` and `Phi` cold as `a` grows (over the weights the two come within 1.6%, 0.3%, 0.4%; one instance per `ell`); the ladder stays cheapest at every `ell` and `a`. At `r = 2^128`, `g = 6`, the ladder's extrapolated crossover with rho moves by `-0.24` to `-0.39` bit from `a = 0` to `a = 1` (`2^21.7..2^25.1` becomes `2^21.4..2^24.8`), `psi`'s by `-0.37` bit, `Phi` amortised's by `-1.5` bit (extrapolated from `ell <= 103`), `Phi` cold's by `-0.01` bit; the rank order of the routes by crossover does not change with `a`. Additions add `1.0` (`Phi` construction) to `1.8` (`psi`, NTT ladder) additions per M-equivalent, raising the costs by `2.0` to `2.8` at `a = 1`.

Limits. Conditional on the counted unit with `W = 1 M`, `I = 91 M`, `PolyMul::Auto` chosen by `M + W`, the library's `Phi_ell` builder (Sutherland not implemented), additions of one weight `a` for every kind, a rho step of `g` multiplications and `g` additions, and one instance per `ell`; ladder `ell <= 863`, `psi <= 131`, `Phi <= 131`; projection to `2^16` and every crossover are extrapolations. No sub-rho or deployed-curve claim.
