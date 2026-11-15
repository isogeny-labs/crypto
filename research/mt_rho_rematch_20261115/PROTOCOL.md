# Multi-target IC vs a fairly tuned rho: the rematch (2026-11-15)

Frozen before any measured run. `../mt_table_layout_20261114/` gave the multi-target IC a flat table with lock-step
batched, prefetched probes (`ic5b flat 32`, "IC-B"): target phase 0.67–0.73× at 2^32, but the whole solve still
1.07–1.18× the best rho, and the rho arms had not been given the same treatment. This round gives it to them.

## Instrument (this PR): `examples/mt_fieldops.rs`

- `Tab` becomes generic over its value type (a type change only); the flat form doubles when half full, which
  `ic5b` never reaches (its capacity bound is exact).
- **`rho2xksp`** = `rho2xks` with a flat database and a one-step pipeline: after each step its two probe slots are
  prefetched, the next step's arithmetic is done, then the previous step's lookups are made. Targets stay sequential,
  so retention of solved walks is `rho2xks`'s. A hit is seen one step late. That moves restarts, and with them the
  RNG stream, so `rho2xksp` is a different sample of the same algorithm, not a replay. Its expected cost is
  `rho2xks`'s plus at most one step per target.
- **`rhodpup`** = `rhodpu` with a flat database and flat pending tables, each lookup slot prefetched as soon as the
  distinguished point is known. Walks, counted operations and logs are identical to `rhodpu`'s.
- Pre-run check (`check_rho.py`, 3 configurations): `rhodpup` equals `rhodpu` exactly; `rho2xksp` recovers every
  target (counted target steps within −4.2 to +7.9 per target of `rho2xks`, about 1–2 %); `ic5b std 1` and
  `flat 32` still equal `ic5`.

## Keys per configuration

IC-A `ic5b std 1`, IC-B `ic5b flat 32`; rho, untuned: `rho2xks` κ ∈ {0.05, 0.1, 0.25, 0.5}, `rhodpu` κ ∈ {0, 0.25} ×
offset ∈ {−1, 0, 1}; rho, tuned: `rho2xksp` and `rhodpup` on the same grids. 22 keys.

## Configurations and order

As `../mt_table_layout_20261114/`: instances W{24,28,32}-{0..3}, T ∈ {1024, 4096, 16384}, seed 43; 36 configurations ×
22 keys × 3 repeats = 2,376 executions, one process at a time, pinned to one CPU, repeats interleaved in a fixed-seed
shuffled order. Host: one RunPod `cpu5c` pod, deleted after the run and in any case 4 h after creation.

## Measures and gates

Time of a key = median of its 3 repeats of `ms_arm` (the whole solve). Per configuration: best tuned rho = min over
the 10 tuned keys; best rho = min over all 20 rho keys. Cell value = median of the cell's 4 configurations; a cell
decision is a 4-of-4 sign rule.

- **Correctness (stops the reading):** every execution recovers every target; `rhodpup`'s counted totals equal
  `rhodpu`'s for the same configuration and κ, offset.
- **Primary (the rematch):** at 2^32, IC-B / best rho:
  - **IC ahead:** ≤ 1 in at least 2 of the 3 T cells;
  - **IC behind:** ≥ 1 in all 3 T cells;
  - otherwise mixed.
- **Secondary:** best tuned rho / best untuned rho per cell (how much the treatment buys rho), and IC-B / best rho
  at 2^24 and 2^28.

Prior (stated before data): IC behind at 2^32, 0.8. Tuned rho faster than untuned rho at 2^32 in all 3 cells, 0.6.

## Not claimed

No `S`, no single-target change, no BSGS result, no leaderboard or `benchmarks/` row. Wall time on a shared cloud pod,
one binary, read as paired ratios.
