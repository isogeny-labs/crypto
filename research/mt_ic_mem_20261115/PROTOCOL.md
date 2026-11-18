# Multi-target IC memory round: prefetched table inserts and 8-byte fingerprint slots, at 2^32 and 2^36 (2026-11-15)

Frozen before any measured run.

**Setting.** With batched inversion ([`../mt_batch_inv_20261115/`](../mt_batch_inv_20261115/), [pcc:unverified]),
the IC (k = 4, μ = 0.85, flat table, batch 32, `bi`) does about 1.4× as many group operations as the best batched
rho (`rhodpb`) in every cell. At 2^32 it is already cheaper per operation (62–68 ns against 71–79 ns), because an
x-only pair costs about 3.5 multiplications per point against rho's 6. At 2^36 it is dearer (90 against 73 ns).
Derived from that round's rows, a table entry costs 88–135 ns against 37–60 ns for a target trial with the same
arithmetic [pcc:unverified]. The difference is memory: target probes are prefetched, table inserts are not, and
above 2^32 the slots are 16 bytes. Rho's distinguished-point table is small and pays neither.

## Instrument: `examples/mt_fieldops.rs`

- **`ic5b … pf`** (args[11]): every table-build insert first prefetches its home slot and waits in a FIFO of 32
  entries. Inserts happen in the same order, so the table, every trial, every count and every recovered log are
  unchanged.
- **Table mode `flatf`** (args[8]): as `flat` for p < 2^32; for p ≥ 2^32 the slots are 8 bytes (the low 32 bits of
  x as a fingerprint, plus the entry index), with the full x of every entry in a side array read only on a
  fingerprint match. The table is exactly the 16-byte table's: same keys, first insert wins, no false hits.
- Nothing else changes; `flat`/`std` without `pf` is `ic5b` as before. Rho arms are unchanged.

### Pre-run checks, recorded in `check.out` before the main run

1. The previous binary (`ea2f038e…`, the batched-inversion round) and the new one give byte-identical output, all
   fields but times and the new `pf` field: `ic5b … flat 32 bi` on W28-0 and W36-0, T = 1024; `rhodpb` κ = 0.25
   on W32-4, T = 1024.
2. On W32-4 and W36-0, T = 1024: `ic:pf` and `ic:pf+fp` have `ic:bi`'s table, trials, relations, p1_trials, I, M
   and S, and recover every target.

## Keys per configuration (10)

- `ic:bi`: `ic5b <inst> T 43 0.85 4 1.2 flat 32 bi` (the current IC).
- `ic:pf`: `… 0.85 4 1.2 flat 32 bi pf`.
- `ic:pf+fp`: `… 0.85 4 1.2 flatf 32 bi pf` (**the primary arm**).
- `ic:pf+fp:mu1.0`: `… 1.0 4 1.2 flatf 32 bi pf` (cheaper table entries may favour a larger table).
- Rho: `rhodpb` at κ ∈ {0, 0.25} × offset ∈ {−1, 0, 1}, 32 lanes. `rhodpb` was the best rho in 24 of 24
  configurations of the batched-inversion round, by about 3× over every unbatched rho (`rho2xksp`, `rhodpup`), so
  the unbatched arms are left out.

## Configurations and order

- Instances: W32-{4..7} (`../mt_fieldops_v5_20261112/instances/`) and W36-{0..3} (`../mt_ic_scale_20261115/instances/`).
- T ∈ {1024, 4096, 16384}; seed 43; 24 configurations × 10 keys × 3 repeats = 720 executions.
- One process at a time, pinned to one CPU, on this session's container (Intel Xeon @ 2.30 GHz, 4 vCPU).
- Repeats are interleaved in a fixed-seed shuffled order (20261119).

## Measures and gates

A key's time is the median of its 3 repeats of `ms_arm` (the whole solve). Cell = (size, T): 6 cells of 4
configurations; cell value = median; a cell decision is a 4-of-4 sign rule.

- **Correctness (stops the reading):** every execution recovers every target, and `ic:bi`, `ic:pf` and `ic:pf+fp`
  have identical table, trials, relations, p1_trials, I, M and S in every configuration.
- **Primary (IC baseline cost):** `ic:pf+fp` / `ic:bi` < 1, 4 of 4, in all 6 cells → **lower**; ≥ 1 in all → **not
  lower**; else **mixed**.
- **Distance remaining:** `ic:pf+fp` / best rho (min over the 6 `rhodpb` keys), per cell. "IC ahead" in a cell if
  ≤ 1 in 4 of 4; "IC behind" if > 1 in 4 of 4. Reported, not a gate.
- **Secondary:** attribution `ic:pf` / `ic:bi` and `ic:pf+fp` / `ic:pf`; `ic:pf+fp:mu1.0` / `ic:pf+fp` per cell.

Priors (stated before data): primary "lower", 0.9. IC ahead of rho in at least one cell, 0.35. μ = 1.0 lower in at
least 4 of 6 cells, 0.3.

## §5a pricing

Every arm is timed whole, so all native work is inside `ms_arm`. The 0 / 0.1 / 1 GAE band of unpriced native work
is empty for every arm, rho included.

## Not claimed

- No `S`, no single-target change, no BSGS result.
- No leaderboard or `benchmarks/` row: there is no replay certificate or validator report.
- Wall time on one container, one binary, read as paired ratios. Memory-system effects depend on the host; the
  same comparison on another CPU may differ.
- Field arithmetic (u128 remainder, extended-Euclid inversion) is unchanged for every arm; a different field implementation would
  change every arm and is not tested here.
