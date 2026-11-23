# Batched (Montgomery) inversion for both the multi-target IC and rho, at 2^32 and 2^36 (2026-11-15)

Frozen before any measured run.

**Setting.** A phase profile of IC-B (k = 4, μ = 0.85, flat table, batch 32) on this container (W32-4, T = 4096)
puts the table build at about 47 % of the whole solve and the target phase at about 50 % [pcc:unverified]. Both
are dominated by modular inversions: about 1.1 M in the table build and 2 M in the target phase. One inversion
(extended Euclid on i128) costs about 180 ns here, about 30 multiplications. Every rho step also pays an
inversion. Montgomery's simultaneous inversion replaces L inversions by 1 inversion and 3(L − 1)
multiplications. It is standard for both methods, so both get it.

## Instrument: `examples/mt_fieldops.rs`

- `F::batch_inv` (Montgomery's trick; the products and the one inversion are counted as M and I), plus `xpair_d`
  and `padd_d`, the existing formulas with the inverse supplied.
- **`ic5b … inv = "bi"`** (args[10]) batches the inversions in two places: the table build, per combo, over its l;
  and the target phase, per lock-step round of up to 32 targets. The table, every trial and every recovered log
  are unchanged; only the I and M counts move. `"plain"` is `ic5b` as before.
- **`rhodpb`** is `rhodpup` run as 32 lock-step lanes, each lane one target at a time, with retention of finished
  walks. Every round's additions share one simultaneous inversion; walk starts and the rare second addition of the
  fruitless-cycle rule are batched too. Precomputation (κ > 0) also runs as 32 lock-step walks. The order in which
  targets meet the database differs from `rhodpup`, so `rhodpb` is a different sample of the same algorithm.

### Pre-run checks, recorded in `check.out` before the main run

1. On W28-0, T = 1024: `ic5b … plain` gives byte-identical output to the previous binary, all fields but times
   and the new `inv` field.
2. On W32-4 and W36-0, T = 1024: `ic5b … bi` has `table`, `trials`, `relations` and `p1_trials` equal to
   `plain`, with I lower and M higher.
3. `rhodpb` at κ ∈ {0, 0.25} recovers every target on W24-0, W32-4 and W36-0.

## Keys per configuration (18)

- `ic:plain`, `ic:bi`: `ic5b <inst> T 43 0.85 4 1.2 flat 32 plain|bi`.
- Unbatched rho:
  - `rho2xksp` at κ ∈ {0.05, 0.1, 0.25, 0.5};
  - `rhodpup` at κ ∈ {0, 0.25} × offset ∈ {−1, 0, 1}.
- Batched rho: `rhodpb` with the same 6 (κ, offset) points and 32 lanes.
- `rho2xksp` has no batched form here; its lookups make it a walk-at-a-time method. It stays in the "best rho"
  minimum.

## Configurations and order

- Instances: W32-{4..7} (`../mt_fieldops_v5_20261112/instances/`) and W36-{0..3} (`../mt_ic_scale_20261115/instances/`).
- T ∈ {1024, 4096, 16384}; seed 43; 24 configurations × 18 keys × 3 repeats = 1,296 executions.
- One process at a time, pinned to one CPU, on this session's container (Intel Xeon @ 2.30 GHz, 4 vCPU; RunPod
  refused pods for a low balance).
- Repeats are interleaved in a fixed-seed shuffled order (20261118).

## Measures and gates

A key's time is the median of its 3 repeats of `ms_arm` (the whole solve). Cell = (size, T): 6 cells of 4
configurations; cell value = median; a cell decision is a 4-of-4 sign rule.

- **Correctness (stops the reading):** every execution recovers every target, and `ic:bi` has `ic:plain`'s
  table, trials, relations and p1_trials in every configuration.
- **Primary (IC baseline cost):** `ic:bi` / `ic:plain` < 1, 4 of 4, in all 6 cells → **lower**; ≥ 1 in all → **not
  lower**; else **mixed**.
- **Distance remaining:** `ic:bi` / best rho (min over all 16 rho keys), per cell. Reported, not a gate.
- **Secondary:** best batched rho / best unbatched rho (what batching buys rho); `ic:plain` / best unbatched rho
  (the previous round's distance, on this host).

Priors (stated before data): primary "lower", 0.9. Batching helps rho by at least as much as it helps the IC at
2^32 (best-rho gain ≤ IC gain), 0.5. IC ahead of rho in any cell, 0.15.

## §5a pricing

Every arm is timed whole, so all native work is inside `ms_arm`. The 0 / 0.1 / 1 GAE band of unpriced native work
is empty for every arm, rho included.

## Not claimed

- No `S`, no single-target change, no BSGS result.
- No leaderboard or `benchmarks/` row: there is no replay certificate or validator report.
- Wall time on one container, one binary, read as paired ratios.
- A different inversion algorithm (binary GCD, Montgomery-form arithmetic) would change every arm and is not
  tested here.
