# Protocol: the genus-4 sub-rho row re-priced in bitop/v1

Frozen 2026-10-20, before any measurement of this study. Changes after the
first measured row are listed under "Deviations" with their reason.

## Question

The index-calculus scoreboard (`docs/index-calculus-scoreboard.html`,
"Genus panel") has one end-to-end measurement below rho: index calculus on a
genus-4 hyperelliptic Jacobian over `F_61` (`N = 16,790,591 ≈ 2^24`) at
**0.24× rho** (renormalised; 0.15× raw), with a fitted genus-4 exponent of
−0.29 [−0.42, −0.16] in `log₂ N` (round seven,
`experiments/hyperelliptic_sparse_solve_r7/`).

That figure is in the harness's in-situ unit, "group operations". The linear
algebra and the smoothness oracle enter it through a **wall-clock**
calibration (`hyperelliptic_ic_bench::calibrate`: seconds per Jacobian
addition over seconds per BigUint modular product), so it is not machine
independent. Round twelve also found the rho reference's 8-bit branch
coefficients slowed its walk on the largest rows (2.100 against 1.481 √N at
this row); every ratio measured against it "is to be re-measured".

This study asks:

1. **Primary.** With every operation of both arms counted and priced in one
   machine-independent unit, `bitop/v1` (`docs/units/BITOPS.md`), and against
   the full-width rho walk, is index calculus below rho on the genus-4,
   `p = 61` row?
2. **Secondary.** What is the genus-4 exponent of `log₂(IC/rho)` against
   `log₂ N` in `bitop/v1`, and do genus 2 and 3 move?

No claim about deployed systems: no deployed system uses hyperelliptic
curves, and every instance here is a public toy.

## Instrument

- Both arms run in one process on the same curve, divisor and target, through
  `examples/hyperelliptic_ic_vs_rho.rs` → `hyperelliptic_ic_bench::head_to_head`,
  instrumented on branch `claude/hec-bitop-reprice`. Arithmetic is counted
  where it executes:
  - `F_p` adds, multiplications, squarings and inversions, in polynomial
    arithmetic, Cantor composition and reduction, the smoothness test, and
    factor-base membership;
  - arithmetic modulo `N`, in the linear algebra and the scalar bookkeeping;
  - hash probes and word operations, for tables, distinguished points and
    branch selection.
- Prices: `src/cryptanalysis/bitops.rs`.
  - `F_p` primitives are priced at `b = bits(p)`, and mod-`N` primitives at
    `bits(N)`.
  - Hash probes are priced at the field width, and word operations at 64
    gates.
- **Completeness check before any data.** One genus-2 row and one genus-3
  row run under Callgrind, and each arm's instructions per counted gate is
  compared.
  - The instrument is admitted only if the two arms' ratios are within 2× of
    each other.
  - A failure is fixed in the counters, not in the protocol.
- The existing in-situ outputs stay unchanged and are reported beside the new
  ones, so the effect of the unit and the effect of the rho walk can be
  separated.

## Instances and runs

- **Rows.** The 18 rows of round seven
  (`experiments/hyperelliptic_sparse_solve_r7/fbwalk_rows.json`), with the
  same curve rule and the same `p` and `N`:
  - genus 2: p ∈ {41, 61, 101, 151, 211, 251};
  - genus 3: p ∈ {23, 31, 41, 61, 101, 151, 211};
  - genus 4: p ∈ {17, 23, 31, 41, 61}.
- **Seeds and trials.**
  - Index calculus: seed 20260916, 5 trials per row.
  - Rho: seed `0xC0FFEE`, 40 trials per row, with the full-width walk.
  - Both are as in round seven.
- **Extension (secondary, run only if the primary rows finish).** Genus 4 at
  the next three primes for which the curve rule yields a prime-order `D₁`
  above 61, with the same seeds and trials. These extend the genus-4 fit
  beyond the single winning row.
- **Execution.** One thread (`RAYON_NUM_THREADS=1`) and counts only, so wall
  time is not used. The runs go through task cards (AGENTS.md "Model
  tiers").

## Measures

- **Per row:**
  - each arm's mean bitop/v1 per solve, split by phase (precomputation,
    relations or walk, oracle, linear algebra, verification);
  - the ratio IC/rho, with a 95% bootstrap interval over IC trials and rho
    seeds (2,000 resamples, seed 20261020);
  - the same ratio in the in-situ unit, with the full-width rho.
- **Per genus:** the least-squares slope of `log₂(IC/rho)` in bitop/v1 against
  `log₂ N`, with a 95% interval from a two-stage bootstrap (rows, then trials
  within rows). This is reported against the round-seven bands (genus 4:
  −0.25 to −0.09; genus 3: −0.17 to +0.07; genus 2: 0 to +0.19).

## Decision rules, fixed now

- **Primary verdict on the 0.24× row:**
  - *survives* if its bitop/v1 IC/rho upper 95% bound is below 1;
  - *does not survive* if the lower bound is above 1;
  - *unresolved* otherwise.
- **Exponent.**
  - The genus-4 advance in bitop/v1 *holds* if the slope's upper 95% bound is
    below 0.
  - It *is lost* if the lower bound is at or above 0.
  - Otherwise it is *unresolved*.
- **No pricing rescue.** A row with any unverified logarithm is reported and
  left out of the fit. A phase whose counts the completeness check flags is
  reported as a lower bound.
- **§5a band.** If any arm has work the counters do not price, it is charged
  at 0, 0.1 and 1 GAE-equivalent per operation. A ranking is stated only
  where it holds across that band.
- **Success criterion** (CLAUDE.md). This study measures. It is an
  improvement only if a change makes an IC arm cheaper; re-pricing alone is
  accounting. Every row reports its distance to rho.

## Deviations

All recorded before any primary row was measured.

1. **The instrument, fixed.**
   - **The harness:** `8e1ebba8c` adds `examples/hyperelliptic_ic_vs_rho_bitops.rs` and the counters (`src/prime_hyperelliptic/ops.rs`; counting rules in its module docs). It is run with `--arms fb-walk`, which is round seven's configuration, `--ic-trials 5 --rho-trials 40` and the default `--rho-variant full-width`.
   - **Seeds:** its defaults are round seven's: IC 20260916 and rho `0xC0FFEE` = 12648430.
   - **The completeness check passed:** under Callgrind, instructions per counted gate on IC/rho were 1.01 (genus 2, `p = 41`, fb-walk), 0.99 (genus 2 baseline) and 1.16 (genus 3, `p = 23`), all below the 2× limit.
   - **Rows seen while testing the instrument, with 1–2 trials, not used:**
     - genus 2 at `p = 41`, all three arms;
     - genus 3 at `p = 23`, fb-walk;
     - genus 4 at `p = 17` and `23`, fb-walk with one trial. This run checked that the instance rule gives round seven's `N` (5,639 and 30,223, which it does).

     None of these is the primary row.
2. **Validation reported separately.** The arithmetic validates curves and divisors as it runs (`HyperellipticCurveP::validate`, `MumfordDivisorP::validate`). That work was 39–59% of each arm's counted gates in the instrument tests.
   - **Primary:** the primary measure counts it, because both arms execute it.
   - **Secondary:** a declared secondary ratio excludes it, using the JSON's `validation_subset`. It is reported beside the primary, and the verdicts use the primary.
3. **Analysis in Rust.** The per-row ratios, bootstrap intervals and fits are computed by `examples/hec_bitop_analyse.rs`, not Python (AGENTS.md, "Implementation language").
