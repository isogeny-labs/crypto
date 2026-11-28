# Deviations from PROTOCOL.md (frozen at b5971d86dd)

Each deviation is written before any data it affects is read.

## D1 (2026-11-20): genus-4 third prime 60013 -> 54293

- **What happened.** Every genus-4 seed at p = 60013 was rejected as "other" in about 4 ms. The cause
  is the binary's range guard (`src/cryptanalysis/genus_ladder.rs`, `JCurve::new`): it refuses a
  curve when (sqrt p + 1)^(2g) >= 9.0e18, so that #J fits 63 bits. At g = 4 that bound is
  p <= 54,293 among primes. p = 60013 (#J about 1.3e19) is outside it, so the frozen genus-4 cell cannot run.
  No curve was admitted, and no count or time was produced.
- **Why it was missed.** The protocol checked #J against 2^64, not the guard's 9.0e18.
- **Runaway.** The search driver's extension rule ("extend by 500 seeds until enough are admitted")
  never terminates when nothing can be admitted. It ran to seed 7,082,000 before it was stopped,
  writing about 283,000 empty chunk files. Those beyond seed 1500 were deleted. The 112 chunks for
  seeds 1-1500 are kept in `failed_p60013/` as evidence of the guard.
- **Change.** The third genus-4 prime is **p = 54293**, the largest prime the guard admits. Selection,
  arms, blocks and analysis are otherwise unchanged; `run.sh`, `curves.py` and `analyze.py` now name
  54293.
- **Effect on the predictions.** P1 was stated for p = 60013 as x\* / ideal negation, w = 1, in
  [0.42, 0.58]. It is graded as written. For reference only: scaling by the validated slope
  (-0.454) gives about 1.05x at the smaller prime, i.e. [0.44, 0.61]. The slope fit loses lever arm:
  ln(54293/16411) = 1.20 against 1.30.
- **Driver fix.** The driver now stops a cell after 3000 seeds with nothing admitted, and the
  stop is reported, not extended silently.
