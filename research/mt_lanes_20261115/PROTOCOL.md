# Tuning rho's lanes and the IC's lock-step batch: does "IC below rho at 2^32" survive a tuned rho? (2026-11-15)

Frozen before any measured run.

**Setting.** The memory round ([`../mt_ic_mem_20261115/`](../mt_ic_mem_20261115/), [pcc:unverified]) put the IC
(k = 4, μ = 0.85, `flatf`, batch 32, `bi`, `pf`) at 0.935–0.951× the best batched rho at 2^32 and 1.17–1.29× at
2^36. That rho, `rhodpb`, ran 32 lock-step lanes, a number never tuned, while the IC had three tuning rounds. More
lanes share each simultaneous inversion over more additions but cost more multiplications and more walks in
flight. The IC's lock-step batch B (32) is likewise untuned since the prefetch round. This round tunes both on the
same configurations and reads the IC/rho distance with each arm at its best.

## Instrument

`examples/mt_fieldops.rs` as built from `0bc69900d4` (the memory round's binary, SHA-256 `5151b2b0…`); nothing
changes. `rhodpb` takes the lane count as args[7]; `ic5b` takes B as args[9]. B changes only the grouping of the
target-phase inversions and prefetches: table, trials, relations and p1_trials are the same for every B.

Pre-run check, in `check.out`: the payload binary's SHA-256 equals the memory round's.

## Keys per configuration (27)

- IC: `ic5b <inst> T 43 0.85 4 1.2 flatf B bi pf`, B ∈ {32, 128, 256}.
- Rho: `rhodpb <inst> T 43 κ offset L`, L ∈ {16, 32, 64, 128} × κ ∈ {0, 0.25} × offset ∈ {−1, 0, 1} (24 keys).

The rho grid is 8 times the IC's. Taking the minimum over more noisy keys biases best rho downward, against the IC.

## Configurations and order

- Instances: W32-{4..7} and W36-{0..3}, as the memory round.
- T ∈ {1024, 4096, 16384}; seed 43; 24 configurations × 27 keys × 3 repeats = 1,944 executions.
- One process at a time, pinned to one CPU, on this session's container (Intel Xeon @ 2.30 GHz, 4 vCPU).
- Repeats are interleaved in a fixed-seed shuffled order (20261120).

## Measures and gates

A key's time is the median of its 3 repeats of `ms_arm` (the whole solve). Best IC = min over the 3 IC keys; best
rho = min over the 24 rho keys, per configuration. Cell = (size, T): 6 cells of 4 configurations; cell value =
median; a cell decision is a 4-of-4 sign rule.

- **Correctness (stops the reading):** every execution recovers every target, and the IC keys agree on table,
  trials, relations and p1_trials in every configuration.
- **Primary (distance with both arms tuned):** best IC / best rho per cell. "IC ahead" if ≤ 1 in 4 of 4; "IC
  behind" if > 1 in 4 of 4. The memory round's two "IC ahead" cells (2^32, T = 1024 and 4096) **hold** if they are
  "IC ahead" here, else they **do not hold**.
- **Secondary:** what tuning buys each arm: best rho (all lanes) / best rho at L = 32; best IC (all B) / IC at
  B = 32; and the untuned pair IC B32 / rho L32 (the memory round's comparison, re-measured).

Priors (stated before data): the two cells hold, 0.45. Lane tuning lowers best rho by more than 3 % in at least 3
of 6 cells, 0.4. Larger B lowers the IC by more than 3 % in at least 3 of 6 cells, 0.5.

## §5a pricing

Every arm is timed whole, so all native work is inside `ms_arm`. The 0 / 0.1 / 1 GAE band of unpriced native work
is empty for every arm, rho included.

## Not claimed

- No `S`, no single-target change, no BSGS result.
- No leaderboard or `benchmarks/` row: there is no replay certificate or validator report, and the targets are
  planted on generated instances, so no T0 claim under `docs/ic/SUCCESS_CRITERIA.md`.
- Wall time on one container, one binary, read as paired ratios. Tuned on the same configurations it reports: both
  arms' minima carry the same kind of selection, rho's over a larger grid.
