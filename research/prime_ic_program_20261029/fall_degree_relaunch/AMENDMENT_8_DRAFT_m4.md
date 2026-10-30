# DRAFT Amendment 8: the `m = 4` extension (`N >= 16`) of the prime fall-degree study

**Status: draft (plan), not registered.** Written 2026-10-29. It is meant to be committed to
`research/prime_fall_degree_20261027/PREREGISTRATION.md` before any `m = 4` row at `t >= 4`
exists in the relaunch. Labels: **measured (source)**, **derived**, **plan**.

## 1. Why `m = 4`, and what is already recorded

- **Derived in the PKM note** (RESEARCH_PKM_TOWER_ORACLE.md §3.7): the end-to-end cost is about
  `m!·r·T/|V|^{m−1} + m·|V|²`. That is `r^{2/(m+1)}` at best, so `m <= 3` cannot beat rho with any
  oracle, and `m = 4` (`r^{2/5}`) is the first decisive cell.
- **Measured, F4 solving degree, Kummer `m = 4` at `P1`:** `D = 6, 7, 7` at `N = 8, 12, 16`
  (§11.7, §12.7). At `N = 16`: 76,837 columns, 53 steps, 111,137 rows in the largest step, 55.5 min,
  8.06 GB peak, 2.7·10¹³ multiply-adds. The largest step's `B'` held 1.1·10⁹ entries.
- **Measured, the cap-6 confirmation at `N = 16`:** both systems "ended after 11 steps with 1,205 pairs
  above the bound, without refuting" (§12.7).
- **Derived, conditional:** under the G3 equivalence (capped F4 refutes at `c` iff `1 ∈ V_{F,c}`), the
  `N = 16` systems have `d_F = 7`. The condition is that the equivalence, validated on `m <= 4`,
  `N <= 20` (Amendment 3), applies to those systems, and that the study's draws are those systems
  (match by `x_r`).
- **Measured in this study, transcript only:** DERIVED.md D6 reports closure `d_F = 6` at `m = 4` at
  some `t` that is not stated (the rows are on the Mac).
- **Derived, projection** (§17): uncapped F4 at `m = 4`, `N = 20` needs about 240-380 GB and
  0.85-2·10¹⁶ multiply-adds, assuming `D` does not rise.
- **Context at `m = 3`, not gated:** msolve at `N = 21` "ran out of pairs" at degree 7 without the unit
  ideal, then crashed in degree 8 at 99.6 GiB
  (`research/pkm_m3_n21_msolve_20261018/RESULTS.md`, Amendment A1; reading there: inconclusive). If the
  truncated-basis argument transferred to msolve, that would mean `d_F >= 8` at `m = 3`, `N = 21`. It is
  a reason to give the registered `B-m3` cell at `t = 7` (`x_r = 221026520` is the msolve system) a
  closure at `c = 8`: 719,789 columns (derived). It is not evidence about `m = 4`.

## 2. Cells (plan)

Kummer, `m = 4`, chain presentation (2 free unknowns), prime `P1 = 2013265921`, seed `0x504B4D54`,
`--planted 0 --random 2`.

| t | N | λ at P1 (derived) | closure columns, c = 6 / 7 / 8 (derived) |
|--:|--:|--:|:--|
| 4 | 16 | 1.1·10⁻⁵ | 39,796 / 91,276 / 181,959 |
| 5 | 20 | 1.7·10⁻⁴ | 129,039 / 356,959 / 848,829 |
| 6 | 24 | 2.8·10⁻³ | 350,776 / 1,148,040 / 3,216,930 |
| 7 | 28 | 4.4·10⁻² | 833,476 / 3,166,588 / 10,291,023 |

Columns = `Σ_k C(N,k)·C(c−k+2, 2)`, the closure's monomials of degree `<= c` (square-free in the tower
variables, any power in the two free unknowns). `t = 8` has `λ = 0.71` at `P1` and is out of range
without §6's option.

The null control runs at `t = 4, 5` (the shape-matched null; same `D` as Kummer on every recorded cell,
§17). The isogeny arm cannot extend: `λ(m = 4, t = 5) = 0.44` at `P0`, and the `P1` isogeny builder stalls
(Amendment 1).

## 3. Instruments and limits (plan)

1. **Closure** (`--engine hky`), `--hky-cmax 8`, run per system (`--targets k`). Rows are 8 bytes an entry
   (`Vec<(u32, u32)>` in `hky_fp_tower.rs`; Amendment 2: `3·10⁸` entries ≈ 2.4 GB). The limit
   `max_entries = floor((request − 8 GiB) / 8 B)` is about 1.4·10¹⁰ at a 112Gi request. `max_cols` rises to
   `4·10⁶` so that `t = 6`, `c = 7` (1.15·10⁶) can start. The per-closure `--budget` is 172,800 s.
2. **Capped F4** (`--engine tower --cap c`), only as Amendment 3/7 allows after G3. It is cheap where it does
   not refute (`N = 16`, cap 6: 11 steps). It costs about as much as uncapped F4 where it refutes at the top
   degree (`N = 16`, `D = 7`: 55.5 min, 8.06 GB; `N = 20`: projected 240-380 GB, above the lab's 128 GiB pool cap).
3. **Order:** `t = 4` closure to `c = 7` (expect `1` at 7 if the derived `d_F = 7` holds), then `t = 5` to
   `c = 6`, then `t = 5` to `c = 7`, then `t = 6` to `c = 6`, then `t = 6` to `c = 7` if the earlier entry
   counts suggest it fits.

**Memory and compute estimates (derived, low confidence; no `m = 4` closure entry count is on main).** The
proxy is F4's held block. At `N = 16`, `B'` had 1.1·10⁹ entries over 76,463 columns, about 14,400 per column
(§12.7). From `N = 12` to 16 the width grew 8.1x, the largest step's nonzeros 36x and the multiply-adds 316x
(§12.7).

| cell | columns | entries (proxy band) | memory at 8 B | fits 112Gi? |
|:--|--:|:--|:--|:--|
| t = 4, c = 7 | 91,276 | 1-4·10⁹ (F4 proxy up to dense `cols²/2`) | 8-33 GB | yes |
| t = 5, c = 6 | 129,039 | ~1·10⁹ up to 8·10⁹ (dense) | 8-67 GB | likely |
| t = 5, c = 7 | 356,959 | ~1·10¹⁰ up to 6·10¹⁰ (dense) | 80-510 GB | **uncertain to no** |
| t = 6, c = 6 | 350,776 | ~1·10¹⁰ up to 6·10¹⁰ | 80-490 GB | uncertain to no |
| t = 6, c = 7 | 1,148,040 | > 10¹¹ | > 0.8 TB | no |

For compute, take the F4 multiply-adds as the scale. At `N = 16`, 2.7·10¹³ took 55.5 min, and the `N = 20`
projection is about 10¹⁶. At the `N = 16` rate, 10¹⁶ is on the order of two weeks on that host. More threads
shorten it by an unmeasured factor. The lab's ordinary jobs are non-resumable Spot jobs (HARNESS.md). So the
decisive `t = 5`, `c = 7` closure is **not a safe lab job today**. It needs a 256-512 GB host that stays up
for days, or a disk-backed echelon. The lab's 128 GiB pool cap is not raised implicitly (isolab-eks SKILL.md).

## 4. Readings, fixed before data (plan)

Medians are lower medians over the 2 draws, as §6 has them. `d_F(N)` below means that median.

- **Rise at `m = 4`:** `d_F(20) >= 8`, an exact value or a bound, with `d_F(16) = 7`.
- **Flat at `m = 4` over one level:** `d_F(16) = d_F(20) = 7`.
- **Fall:** `d_F(20) <= 6`.
- **Undecided at `m = 4`:** anything else, including a censored `t = 5`, `c = 7` closure, which leaves only `d_F(20) >= 7`.

What each reading does to the lane (the PKM tower factor base, judged by the HKY last fall degree as the
prime-field exponent route):

- **Close the lane** if C-step holds at `t = 12-14` (Amendment 6) **and** `m = 4` reads *rise*. A last fall degree
  that grows with `N`, at two values of `m`, leaves every fixed `m` far above rho (§5).
- **Keep it open, as a mechanism question only,** if `m = 4` reads *flat* or *fall* and C-step is refuted at
  `t = 12` or `t = 14` with a lower value. Then the next decisive measurement is ladder C's growth in `m` at
  fixed `t` (H-m), extended toward `m = 6...8` with the closure (`m <= 10`; at most 8 free unknowns).
- **No change** otherwise.

**Prediction (optional; the owner may strike it before registration).** `d_F(16) = 7` (from §1, derived), and
`t = 5` undecided, because `c = 7` will not fit.

## 5. What the outcome can and cannot mean at cryptographic size (derived)

**Model.** Relations needed ≈ `|V| = 2^t`. Queries per relation = `1/λ = m!·p/(|V|^m·2^{m−1})`, floored at 1.
Per-query cost `T = cols(N, m − 2, d)²` `F_p` operations: one sparse elimination of the degree-`d` closure
matrix, a floor rather than an F4 estimate. Linear algebra `m·|V|²`. The total is minimised over `t` and compared
with rho with negation, `√(πp/4)` group operations. Group operations and `F_p` operations are not converted;
the conversion favours rho by a few bits and is ignored.

**At `p = 2^256`, `m = 4`:** the optimum sits at `t ≈ 62-65`, `N ≈ 250-260`, close to one query per relation.
log₂(IC/rho) is −1, +4, +15, +25 and +35 bits at `d = 4, 5, 6, 7, 8`. The per-query matrix at `N = 256` has
2^33.1, 2^38.5, 2^43.7 and 2^48.7 columns at `d = 5, 6, 7, 8`. At `d = 7` that is about 1.4·10¹³ columns,
roughly a petabyte per query at ten 8-byte entries per column.

**At `p = 2^256`, other `m`** (same model): the largest `d` that does not lose to rho is 4, 5, 6, 7 and 7 for
`m = 4, 5, 6, 7, 8`. A degree that keeps growing like C-step, illustrated as `7 + ⌊(t − 4)/4⌋` at `m = 4`,
loses by about 143 bits.

**Honest expectation.** A *flat* `d_F = 7` at `m = 4` would be a real finding about prime-field PKM systems,
but `m = 4` would still be about 25 bits above rho at 256 bits under an optimistic per-query model. The
exponent lane survives only if `d_F` stays bounded in `N` and grows with `m` more slowly than the recorded
`D(t = 2) = m + 2`. No toy-prime measurement settles the first condition, because the `m = 4` ladder spans at
most three levels at `p < 2³²` (§6).

## 6. Limits, and an option (owner decision)

- Every engine asserts `p < 2³²` (`hky_fp_tower.rs`, `HkyRing::new`; the F4 engines, §17), and `hky_fp_tower`
  multiplies as `a * b % p` in `u64`. At `P1`, `m = 4` reaches `t = 7`, three levels above `t = 4`, which is too
  few to resolve one rise per four levels. Options:
  - widen the closure's arithmetic to 64-bit primes (`u128` products or Montgomery), so that a prime near 2^40
    keeps `λ(m = 4, t = 8) ≈ 10⁻³`;
  - condition targets on inconsistency at `λ >= 0.1`. That needs a decomposability certificate for `m = 4`,
    which is a meet-in-the-middle over pairs of base points (about `2·|V|²` sums) and does not exist yet.
    Exhaustive `V⁴` search (`pkm_tower_check verify`) is `2^32` tuples at `t = 8`.
- The tower width limit `MAX_TOWER = 56` (`hky_fp_tower.rs`) caps `N` at 56, so `m = 4` stops at `t <= 14` in
  this engine whatever the prime.
- No outcome here is an `S`, a price or a leaderboard row (PREREGISTRATION §8).
