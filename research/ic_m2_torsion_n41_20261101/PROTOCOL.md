# Protocol: torsion anchors on and off at n = 41 (frozen before any data)

Frozen 2026-11-01, before the first run of either arm in this protocol.

## Question
At a = 0, n = 41 (cofactor 4, d = 14), what does the `torsion` option of
`examples/ic_m2_cycle.rs` do to the cost of one cold single-target solve? Earlier
exploratory rows (cofactor 4): n = 23 on/off 1.5x more trials, n = 31 16x more trials
(4 targets, `research/ic_m2_cycle_leads_20261030/raw/`). Claim class: comparative
(paired), exploratory, not for the leaderboard.

## Arms and design
- Arm OFF: `ic_m2_cycle 0 41 0 1 <seed> 0 <out>`. Arm ON: same with `torsion = 1`.
  Same binary (SHA-256 recorded), same host, same build, one target per process.
- Pairs: 8 seeds, one target (ti = 0) each: 20261024 and 20261101 to 20261107. The secret
  depends only on the seed, so both arms solve the same discrete log. Seed 20261024 reproduces
  row 0 of the merged n = 41 record (26,535,644 trials) and is the determinism check on the
  OFF arm: any other count means the binary or host differs, and the run is reported as such.
- Order: all OFF runs first, then ON runs, at most 4 processes at a time (4 vCPU, one thread
  each). No interleaving; trials are counted, so order cannot move them. Wall time is
  secondary and carries host noise (shared cloud container, not isolated).

## Metrics
- Primary: decomposition trials to recover `t` (counted by the producer, `ic_trials`).
- Secondary: wall seconds `ic_s` (cold, includes the factor base), group additions.
- Every recovered scalar must satisfy `[t]G = T` (`ic_verified`); a row that does not is
  excluded and reported.

## Cap and censoring
- Each ON run has a hard cap of 3,600 s (`timeout`). A capped run has no trials count; it is
  reported as censored with a lower bound of cap seconds, and its pair ratio as a lower
  bound (cap divided by the pair's OFF wall seconds).
- OFF runs get the same cap; none is expected to reach it (about 90 s at n = 41).

## Analysis (frozen)
- Per pair: ratio ON/OFF of trials (wall for censored). Report all pairs.
- Summary: geometric mean of the ratios with a 95% bootstrap interval over pairs (10,000
  resamples); with censored pairs the summary is a lower bound and is labelled so.
- Also report the arithmetic mean trials of each arm. No threshold is set: the result is a
  measured distance, in either direction.

## Stated limits
- Eight pairs; per-target trial counts are heavy-tailed (earlier n = 41 ratios to the frozen
  ladder were 7.2, 3.1, 2.9 and 50.7), so the interval will be wide.
- One host class, local container, not the repository's default isolab-eks route (no AWS
  identity in this session). Exploratory; no replay certificate or validator report.
- Cofactor 4 only. This does not test the large-cofactor case torsion is meant for.
