# Protocol: torsion anchors on and off on the cofactor-596 curve (a = 0, n = 37), frozen before data

Frozen 2026-11-01, before the first run of either arm in this protocol.

## Question
`ic_m2_cycle` with `torsion = 1` solved a = 0, n = 37 (cofactor 596, 2,011 columns at d = 12)
for two targets in 26 s and 31 s (`research/ic_m2_cycle_leads_20261030/raw/cyc_tors_a0_n37.json`),
where the frozen ladder (90% coverage stop) was stopped unfinished at 99 s. That run had no
torsion-off arm and two targets. Here: (1) does torsion on solve this curve reliably, (2) does
torsion off (the same cycle-stop code) solve it at all within a cap, and (3) how far is it from
strong rho. Claim class: comparative (paired), exploratory, not for the leaderboard.

## Arms and design
- Arm ON: `ic_m2_cycle 0 37 0 1 <seed> 1 <out>`. Arm OFF: same with `torsion = 0`. Same binary
  (SHA-256 recorded), same host, one target per process, d = 12 (the producer default).
- Pairs: 8 seeds, one target (ti = 0) each: 20261024 and 20261101 to 20261107. The secret depends
  only on the seed, so both arms solve the same discrete log. Seed 20261024 with torsion on
  reproduces row 0 of the earlier record (14,720,702 trials): the determinism check. A different
  count means the binary or host differs and is reported as such.
- Order: all ON runs first, then all OFF runs, at most 4 processes at a time (4 vCPU, one
  thread each). Trials are counted, so order cannot move them; wall time is secondary and noisy.
- The same process runs strong rho on the same target (`rho_s`), giving the distance to rho.

## Metrics
- Primary: decomposition trials to recover `t` (`ic_trials`). Secondary: wall seconds `ic_s`,
  relations, columns, `ic_largest_component`, `ic_equations`, `ic_useless_cycles`.
- Every recovered scalar must satisfy `[t]G = T` (`ic_verified`); a failing row is excluded and
  reported. Rho's scalar is checked the same way (`rho_verified`).

## Cap and censoring
- Each run has a hard cap of 1,800 s (`timeout`). A capped run has no trials count: it is a
  censored lower bound of 1,800 s. OFF runs are expected to hit it on some or all seeds (the
  ladder's graph splits into about 298 cofactor classes at this size), so this is the main
  outcome to report: k of 8 OFF runs finished.
- No run is restarted or given a longer cap after seeing results.

## Analysis (frozen)
- Arm ON alone: mean and range of trials and wall, the 8 per-seed values, mean wall divided by
  mean rho wall (distance to rho).
- Pairs where OFF finished: per-pair ratio OFF/ON of trials, with the geometric mean and a 95%
  bootstrap interval over those pairs (10,000 resamples, seed 20261101). Pairs where OFF was
  censored: ratio lower bound = cap / ON wall; the summary is then a lower bound and labelled so.
- No threshold is set; the result is a measured distance in either direction.

## Stated limits
- Eight pairs; heavy-tailed trial counts (at n = 41 per-pair ratios spanned 0.30 to 2.60).
- One cofactor (596) and one size (n = 37). It does not reach the N83 a = 1 curve (cofactor about
  1.1e9, n = 83), whose projected cost is far beyond this method at any cofactor handling.
- Local shared cloud container, not the default isolab-eks route (no AWS identity in this
  session). Exploratory; no replay certificate or validator report.
