# The §5a pricing band cannot show a known index-calculus win at lab sizes

- **Status:** `open` (rule amended 2026-11-12; ecbench `band.rs` support pending)
- **Kind:** methodology gap (pricing calibration)
- **Affected work:** every cross-method ECDLP comparison priced by AGENTS §5a
  at 0.1 and 1 GAE per native operation; `research/ic_positive_control_20261112/`
- **Next action owner:** owner decision (the §5a rule is the owner's)
- **Last updated:** 2026-11-12

## Problem

AGENTS §5a prices each unpriced native operation (field multiplication, LA
mulmod) at 0, 0.1 and 1 GAE, and ranks two arms only where the order holds
across that band. A positive control shows that, at the sizes the lab can run,
a method expected to cost less than rho asymptotically does not rank below rho
at 0.1 or 1 GAE [pcc:unverified]. So
"above rho at every price" is uninformative at those two prices.

## Evidence and uncertainty

`research/ic_positive_control_20261112/RESULTS.md`, protocol frozen before data
[pcc:unverified]:

- **What was run.** Genus-3 double-large-prime IC on a full Jacobian
  (`Õ(p^{4/3})` against rho's `p^{3/2}`). All 572 logs were certified.
- **Group operations alone.** IC is 0.114× rho at p = 65,537, and below rho
  from p = 257 on.
- **At 0.1 and 1 GAE per mulmod.** IC is 5.8× and 31× rho at p = 65,537 (95%
  intervals 3.8–9.4 and 20.8–52.7). Fitted crossovers are about 10^13 and
  10^20.
- **Why.** About 362 native multiplications ride on each trial (the
  `x^p mod u` smoothness test plus root finding).
- **Measured on this host.** A Cantor addition costs as much time as about
  80–200 `mod p` multiplications, which puts a multiplication at about
  0.005–0.013 GAE. That is 8–20× below the band's 0.1 lower edge. With that
  calibrated price, IC falls below rho from about p ≈ 4.7·10³; the interval
  excludes 1 only at p = 65,537.
- **Uncertainty.** The calibration is a single best-of-5 timing on a shared
  host. It is informational, not part of the frozen verdict. There are few
  curves at the largest p, and rho's run-to-run CV is 0.37–1.10.

## What this does and does not change

- **Earlier verdicts that stand.** Every transfer row in the chained-transfer
  program loses already at 0 GAE, or reduces structurally to a search on `E`.
  Those verdicts do not depend on the band.
- **Rows that cannot be distinguished from a real win.** A row whose gap at 0.1
  or 1 GAE is smaller than the control's own reading (about 6× and 31×) looks
  the same as a winning algorithm there.

## What would unblock it

- **Option 1.** Price native operations with a per-host calibrated ratio,
  measured with `perf-ipc` on an isolated host, alongside the 0/0.1/1 band.
- **Option 2.** Report the 0-GAE order plus the calibrated-price order, and keep
  the band as a sensitivity range instead of a ranking gate.
- Either option is a change to the owner's §5a rule and is not made here.

## Next discriminating step

Repeat the timing calibration with `perf-ipc` on an isolated host
(isolab-eks), and extend the positive control to p ≈ 2^18–2^20, where the
calibrated crossover should be clear of noise.

## Resolution and history

- 2026-11-12: opened from the IC positive control (step 1 of
  `research/chained_prime_transfer_20261111/NEXT_STEPS_20261112.md`).
- 2026-11-12: owner approved recalibration. AGENTS §5a gains a calibrated-weight
  reading for field-arithmetic counters (`w = 0`, `w_cal`, `10·w_cal`); the band is
  kept. Pending: emit the reading from `src/cryptanalysis/ecbench/band.rs`.
