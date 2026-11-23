# Multi-target IC: batched relation phase, batched combinations, and two cache attempts, at 2^32 and 2^36 (2026-11-15)

Frozen before any measured run.

**Setting.** A decomposition of the tuning round ([`../mt_lanes_20261115/`](../mt_lanes_20261115/),
[pcc:unverified]) put the IC's remaining excess over the tuned rho at 2^36 in two size-dependent costs:
- **The relation phase.** It still pays one full inversion per trial and does no prefetching: about 200–250 ns per
  trial against 44–50 ns for a target trial. It takes 19 % of the solve at 2^36, T = 1024.
- **Cache misses once the table passes about 256 MB** (2^36, T = 16384).

The 1.4× operation count is the same at both sizes. The combination step is also unbatched (3–4 % of the solve).
This round measures one lever for each and a combination.

## Instrument: `examples/mt_fieldops.rs`

`ic5b` args[11] becomes a comma-separated option set ("pf" alone is the previous option):
- **`rel`**: the relation phase computes each block of kw = 64 trial pairs (one V point, W_0..W_63) with one
  simultaneous inversion and prefetches all their probes, then processes them in the original order. Trials,
  relations, the stopping point and the factor-base logs are unchanged; only I and M move. Arithmetic of the last
  block's unprocessed trials is counted.
- **`cb`**: the (k−1)-term combination step shares one simultaneous inversion per point over its l (`padd_sib_d`).
  The combination points are unchanged.
- **`rad`** (cache attempt 1): table-build inserts are buffered in chunks of 2^22, stably bucketed by the top 12
  bits of their home slot and inserted in that order, with prefetch. Equal x share a home, so the x → entry map is
  unchanged.
- **Table mode `flatfd`** (cache attempt 2): the 8-byte fingerprint table for every p, sized to load ≤ 0.6 instead of
  ≤ 0.5. That halves the slot array whenever twice the capacity is just above a power of two. The x → entry map is
  unchanged.

**Exploration before freezing (single executions, not results).**
- `rel` cut the relation phase about 5× at 2^36, T = 1024.
- `cb` cut the combination step about a quarter.
- `rad` made the table build 3–17 % slower.
- `flatfd` made the target phase 15–40 % slower.

Both cache attempts are measured here as their own arms; neither is in the combined arm.

### Pre-run checks, recorded in `check.out` before the main run

1. The previous binary (`5151b2b0…`) and the new one give byte-identical output, all fields but times and the new
   `opts` field: `ic5b … flatf 128 bi pf` on W28-0 and W36-0, T = 1024; `rhodpb` κ = 0.25, 64 lanes, W32-4.
2. On W32-4 and W36-0, T = 1024, every IC key below has `ic:pf`'s table, trials, relations and p1_trials, and
   recovers every target.

## Keys per configuration (24)

- IC, all `ic5b <inst> T 43 0.85 4 1.2 <mode> 128 bi <opts>` (B = 128, the tuning round's best at 2^36):
  - `ic:pf`: flatf, `pf` (the current IC);
  - `ic:rel`: flatf, `pf,rel`;
  - `ic:cb`: flatf, `pf,cb`;
  - `ic:rel+cb`: flatf, `pf,rel,cb` (**the primary arm**);
  - `ic:rad`: flatf, `pf,rad`;
  - `ic:dense`: flatfd, `pf`.
- Rho: `rhodpb` at L ∈ {32, 64, 128} lanes × κ ∈ {0, 0.25} × offset ∈ {−1, 0, 1} (18 keys). These lane counts were
  best in 24 of 24 configurations of the tuning round.

## Configurations and order

- Instances: W32-{4..7} and W36-{0..3}.
- T ∈ {1024, 4096, 16384}; seed 43; 24 configurations × 24 keys × 3 repeats = 1,728 executions.
- One process at a time, pinned to one CPU, on this session's container (Intel Xeon @ 2.30 GHz, 4 vCPU).
- Repeats are interleaved in a fixed-seed shuffled order (20261121).

## Measures and gates

A key's time is the median of its 3 repeats of `ms_arm`. Best rho = min over the 18 rho keys per configuration.
Cell = (size, T): 6 cells of 4 configurations; cell value = median; a cell decision is a 4-of-4 sign rule.

- **Correctness (stops the reading):** every execution recovers every target, and the IC keys agree on table,
  trials, relations and p1_trials in every configuration.
- **Primary (IC baseline cost):** `ic:rel+cb` / `ic:pf` < 1, 4 of 4, in all 6 cells → **lower**; ≥ 1 in all → **not
  lower**; else **mixed**.
- **Distance remaining:** `ic:rel+cb` / best rho per cell; "IC ahead" ≤ 1 in 4 of 4, "IC behind" > 1 in 4 of 4.
- **Secondary:** each lever over `ic:pf`: `rel`, `cb`, `rad`, `dense`.

Priors (stated before data):
- Primary "lower", 0.6. The gain is concentrated at small T; at T = 16384 the relation phase is about 2 % of the
  solve, inside run-to-run noise.
- `rad` lower in at least 3 of 6 cells, 0.15.
- `dense` lower in at least 3 of 6 cells, 0.1.
- IC ahead in at least one cell, 0.3.

## §5a pricing

Every arm is timed whole, so all native work is inside `ms_arm`. The 0 / 0.1 / 1 GAE band of unpriced native work
is empty for every arm, rho included.

## Not claimed

- No `S`, no single-target change, no BSGS result.
- No leaderboard or `benchmarks/` row: there is no replay certificate or validator report, and the targets are
  planted on generated instances.
- Wall time on one container, one binary, read as paired ratios. This container's run-to-run variation is several
  per cent (the tuning round), so single-cell differences of that size are not resolved.
