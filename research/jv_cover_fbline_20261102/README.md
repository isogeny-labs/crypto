# Factor-base line search: the JV cover gets 1.7× cheaper and passes the negation-map rho in group additions (2026-11-02)

**Scope.** This applies only to the weak class: the Joux–Vitse genus-3 cover of `E(F_{p^6})` over `F_{p^2}`, with full rational 2-torsion and `#E = 4l`. It runs on the same 9 instances as #3194 and #3278. It makes no claim about prime-order curves.

**Labels.** *Measured* means read from a run record. *Derived* means computed from measured values. *Model* means an extrapolation.

**Status.** The study is not yet validated. There is no `VALIDATION.json`, so it adds no leaderboard or `benchmarks/` row until a validator and a red team review it.

## The change

A Nagao decomposition test asks for the six roots of a sextic to lie in an `F_p`-line of `F_q`. The baseline uses the line `F_p` itself. Any line `a·F_p + b` with `a = t²` works just as well: under `x = a·s + b` the cover becomes `H' : y'² = h(as+b)/a⁷`, which is isomorphic over `F_q`. The test is still six quadrics in six unknowns. Only the factor-base size `B` changes.

The residuals a run needs scale as `B·(p/2B)⁶ ∝ B⁻⁵`, because the rate is `(2B/p)⁶/720` and `B+1` relations are needed. `B` moves by about `√p/2` from one line to the next. A search over 65,536 lines therefore finds bases about 4σ larger.

- **Code:** `src/cryptanalysis/jv_cover.rs`, `search_fb_line`, `line_poly` and `line_div`. It is opt-in through `JV_FB_LINE=n`. When unset, the run is bit-identical to before (control C1).
- **What is charged in `setup_muls`:** every multiplication of the search (Horner, the norm, the square table, and moving `G` and `Q`). That is about 1.5·10⁹ multiplications, 0.51–0.57% of each run.

Before this study, the measured `residual_ratio` spread of the 9 baseline records (0.79–1.44, far above the ~4% noise of an 800-relation count) already tracked `(p/2B)⁶`. The study's frozen protocol (`PROTOCOL.md`, hashed in `results/FROZEN.sha` and pushed in 795eb00bed before any run) turned that observation into a test.

## Result: paired against the plain cover, same instances

The full table is in `analysis/paired.txt`, and the graph is `analysis/paired.svg`.

| p | seed | B plain → best | S plain | S line | ratio (measured) | model (B_plain/B_best)⁵ |
|---|---|---|---|---|---|---|
| 1301 | 1 | 634 → 728 | 1.494 | 0.729 | 0.488 | 0.501 |
| 1301 | 2 | 651 → 725 | 1.337 | 0.781 | 0.585 | 0.584 |
| 1301 | 3 | 662 → 730 | 1.125 | 0.703 | 0.625 | 0.613 |
| 1451 | 1 | 735 → 811 | 0.982 | 0.602 | 0.612 | 0.611 |
| 1451 | 2 | 729 → 800 | 1.036 | 0.633 | 0.611 | 0.628 |
| 1451 | 3 | 703 → 804 | 1.241 | 0.610 | 0.492 | 0.511 |
| 1601 | 1 | 764 → 884 | 1.118 | 0.523 | 0.468 | 0.482 |
| 1601 | 2 | 820 → 883 | 0.716 | 0.504 | 0.704 | 0.691 |
| 1601 | 3 | 824 → 883 | 0.665 | 0.506 | 0.761 | 0.708 |

- **Primary result (derived):** the geometric mean of `S_line/S_plain` is **0.587 [0.530, 0.651]**, a **1.71× [1.54, 1.89]** speed-up. The interval is a 95% bootstrap over instances. The model predicted 0.587. Every instance got cheaper, so **H1 is supported**.
- **Why the gain is fairly even:** the line search also removes most of the instance-to-instance spread. The 9 plain values of S range over 2.2×; the 9 line-search values range over 1.55×.

## Result: re-scored against the negation-map rho

The reference is #3278's, unchanged. It is the 126 `rhostrong --neg 1` runs on these instances, and the estimator is (b), pooled. The analysis is #3278's own `negrho_rescore` tool run unchanged (`fit/src/main.rs` sha `af4e1355…`). On the plain cover records it reproduces #3278's `table.md` byte-for-byte.

| unit | p=1301 | p=1451 | p=1601 |
|---|---|---|---|
| U1 group additions | **0.791** [0.722, 0.871] below | **0.659** [0.608, 0.718] below | **0.548** [0.506, 0.596] below |
| U2 textbook F_p mults | 1.900 [1.735, 2.092] above | 1.585 [1.460, 1.725] above | 1.318 [1.217, 1.432] above |
| U1 before (#3278) | 1.405 above | 1.160 above | 0.870 not resolved |
| U2 before (#3278) | 3.376 above | 2.787 above | 2.090 above |

- **U1:** the cover is now **below rho at every prime under all three reference estimators**. It is ahead by 1.26×, 1.52× and 1.82×.
- **U2, distance remaining:** the cover still needs to get 1.90×, 1.58× and 1.32× cheaper.
- **§5a band:**
  - U1 is below at w = 0, 0.1 and 1 at all three primes, so the order holds.
  - U2 is above at w = 0 and below or not resolved at w ≥ 0.1, so the order does **not** hold across the band.
  - The cover's native count is a lower bound: its F_p additions are not emitted. A "below" verdict could therefore narrow under complete counts.
- **Crossover (model, slope −2.10):**
  - U1: p* ≈ **1185** [1141, 1233], against ≈ 1528 before.
  - U2: p* ≈ **1799** [1732, 1872], against ≈ 2320 before. That is now inside the u64 limit p ≤ 2039, so it can be tested directly.

The graph is `analysis/rescore_scaling.svg`, and the full tables are in `analysis/rescore_table.md` and `rescore_fit.txt`.

## Controls (all pass)

- **C1 (baseline unchanged):** with `JV_FB_LINE` unset, p=1301 seed 3 reproduces the crossover record exactly. Residuals, relation digest, total multiplications and decompositions all match.
- **C2 (machinery null):** `JV_FB_LINE=1` gives the same residuals and digest as C1. Total multiplications are higher by exactly the search's own 22,772.
- **C3:** all 9 line-search runs were solved and correct (the planted logarithm), with the moved `G` and `Q` asserted valid on `H'`.
- **C4:** the measured rate divided by `(2B/p)⁶/720` lies in 0.92–0.99 on every run; the band was [0.8, 1.25].
- **Reference controls, inherited from #3278:**
  - Positive control: z = +1.15 against theory.
  - Homogeneity: permutation p = 0.31.

## Deviations and limits

- The `fit/` tool was written after the freeze. Its hash was recorded (`results/FIT.sha`, commit 112da2ea31) before any line-search result existed.
- The runs were local, on the same 4-core box as the baseline arm, not on isolab-eks.
- The cover side has 3 seeds per prime. The reference side is 42 runs per prime.
- U2 prices the cover at textbook (harness) multiplication counts and the rho at tuned counts, as in #3278. A tuned cover F4 would narrow U2 further, but no such cover was measured here.
- The search size `n = 65536` was fixed before data. It was not tuned on these instances.

## Next steps

1. **Validator and red team:** run both on this study, and add a `VALIDATION.json` so it can be admitted to the leaderboard.
2. **Measure past the U2 crossover:** run cover instances at p ≈ 1850–2000, with a matched negation rho, to test U2 p* ≈ 1800 directly.
3. **Cut the per-test cost:** the test costs about 9.5·10⁵ F_p multiplications. A tuned F4 or a structured solver is the remaining lever for U2.

## Files

- `PROTOCOL.md` and `run.sh`, both frozen.
- `results/`: the raw records and logs, `FROZEN.sha`, `FIT.sha`, `versions.txt` and `walls.txt`.
- `fit/`: the paired analysis, in Rust.
- `analysis/`: `paired.*` and `rescore_*`.
- `REPORT.pdf`.
