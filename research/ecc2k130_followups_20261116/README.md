# ECC2K-130 follow-ups F1–F3 (2026-11-16)

The three follow-ups named by the 2026-11-14 open-items sweep
(`research/ecc2k130_open7_20261114/README.md`) ran as parallel lanes. Rules are in `COMMON.md`:
Rust only; F2 had CPU core 3 to itself; no paid compute. Every claim is `[pcc:unverified]`:
these are local producer runs, with no signed claims and no independent replay.

## Verdicts

| Lane | Question | Outcome | Key numbers |
|---|---|---|---|
| F1 | Cold SAT solve with R folded in as constants, seeded with warm heuristic state | *pending* | |
| F2 | GF(2^n) w_cal: price of unpriced native ops in GAE | Instrument built; provisional local calibration; all controls pass in 11/11 cells | Per GAE at n = 11 / 29 / 47: mul (latency) 5.7e-2 / 3.1e-2 / 2.1e-2; word XOR at row width 1 5.0e-2 / 2.7e-2 / 1.9e-2, at width 64 about 3e-3 to 1e-3. Prices fall as n^-0.69, because the addition time grows while mul and XOR stay flat. At n = 131 the measurement uses a three-word addition 13.4× slower than ecbench's wide one at n = 83; rescaled, mul is about 0.096 GAE and word XOR about 4e-4. Cross-check against `ecbench price-units` (`full_prices.json`, n = 13–41): mul/sqr/inv/double agree within 0.96–1.11×. |
| F3 | Complete m = 83 counted IC row against matched rho | **Blocked on method, not only budget.** Launch specs frozen; smoke run at n = 41/53/61 | Smoke IC/rho at w=0: 14.96 [10.3, 21.8], 19.38 [14.3, 26.2], 24.42 [16.6, 36.0]; rho ahead at every §5a weight. Projected m = 83 with the planner's own recipe: pair table about 1.1 PiB, IC/rho about 1,200× (476× with drift). With a 128 GiB table cap: about 7.6·10^4×. Matched rho alone: 2^37.2 GAE, about 28 single-core hours per target. |

## What changes

- **L2's open trade from the 2026-11-14 sweep is settled at measured prices.** The m=2 degree-4
  filter loses in every cell. At the nearest cell the measured word-op price is 37× above
  break-even, and up to 8.7·10^4× above it elsewhere. Under the frozen §5a rule the pair is
  still not ranked, because the filter is ahead at w = 0.
- **L4's gcd-vs-inherited-f4 order stays open, now for a known reason.** A field multiplication
  costs 18.2 word XORs latency-bound and about 5.7 throughput-bound. The order flips between
  those two: I/G is 0.46–0.56 with latency-bound muls and 1.38–1.63 with throughput-bound ones.
  It is settled only by timing both side by side, or by porting G onto the shared `bitop/v1`
  tally.
- **An m=83 counted IC row is not a budget question yet.**
  - ecbench has no IC method for n ≥ 64 (`methods.rs::solve_wide`), and `ic plan --koblitz 0:83`
    has no recipe.
  - Even after a port, the nearest-to-rho Koblitz arm scales to 10^3–10^5× rho at m=83.
  - No isolab contract can host it: request/v1 jobs are capped at 3,600 s, and resumable/v1
    has a 24 h deadline and requires L2 with 5 rounds.
  - The owner decision needed first is scientific: a better-scaling wide IC method, or a
    different §8a reading.
  - A cheap rate-probe request (`requests/request-v1-rate-probe-l2.json`, under 30 minutes)
    passes `remote/validate-request` and could go given a live budget readback.

## Integrity notes
- **F2 freeze not verifiable from Git.** The frozen PROTOCOL (`38cab9bf…`, its own copy in
  `raw/PROTOCOL.frozen.md`) first reached Git at 01:46:08Z, 40 s after F2's first measurement
  file (01:45:28Z). The version Git recorded before data (01:21Z, `56768d99…`) differs only by
  disclosed changes that came from an unscored smoke run: curve availability, known op limits,
  the n=131 unit, and a secondary range that decides nothing. No decision rule moved. The freeze
  rests on the lane's own hash, not on Git.
- **`main` does not compile** (`koblitz_index_calculus.rs:1646`, from merge `16aa66283b`).
  - F2 built against a scratch copy carrying the two-line fix in
    `F2-binary-wcal/crypto-head-build-fix.patch`.
  - F3 built from `70b57c2b81`, the last commit that compiles.
  - F2's `Cargo.toml` points at an absolute scratchpad path and must be repointed to `../../..`
    once `main` builds.
- F3's freeze is verifiable from Git: `5d14d9bb…` was committed at 01:09Z, before the smoke runs.

## Files
- `F2-binary-wcal/`: `RESULTS.md` (§7 holds the isolab task card for the authoritative
  calibration), `tables.md`, `results.json`, `fig_prices_vs_n.svg`, `fig_l2_breakeven.svg`,
  `raw/` (`SHA256SUMS`, 86 entries).
- `F3-m83-counted/`: `RESULTS.md`, `specs/`, `requests/`, `results.json`,
  `fig_cost_vs_log2r.svg`, `raw/` (sessions, audits, `validators.txt`, `SHA256SUMS`).
- `F1-warm-cold-sat/`: pending.
