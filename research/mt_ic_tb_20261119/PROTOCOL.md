# Multi-target IC table build at 2^36: batched in-order inserts and zero-encoded empty slots (2026-11-19)

Frozen before any measured run.

**Setting.** With huge pages ([`../mt_ic_hp_20261118/`](../mt_ic_hp_20261118/), [pcc:unverified]), the IC's table
build at 2^36, T = 16384 costs about 67 ns per entry against 44–45 ns per target trial, with the same arithmetic per
operation. An exploration on 2026-11-18, recorded in that day's session log, split the table build on a scratch
build:
- arithmetic and loop alone take 430–500 ms;
- the inserts take about 700 ms more, about 39 ns each, against about 9 ns for the same insert in a microbenchmark.

Today's host is quiet: A/A spread ±2.5 % for the IC and ±0.6 % for rho (four executions each, W36-0, T = 16384).
Exploratory single runs of the instrument below put the table build at about 0.78× with batched inserts alone and
about 0.73× with zero-encoded slots added. Cross-combination batched inversion (`xb`) made it slower, so it is not
an arm here.

## Instrument: `examples/mt_fieldops.rs`

- **`ic5b` option `ib`:** table-build inserts are buffered 1,024 at a time and inserted in order by
  `Tab::insert_batch`. That function matches the table variant once per chunk, inlines the insert and prefetches
  each slot 32 inserts ahead. The insert order, and so the table, equals in-order `insert_if_absent`.
- **Environment `MTF_Z=1`**, for every arm's flat tables, IC and rho alike: an empty slot is marked by a stored index
  of 0, and entries store index + 1. The slot array can then be allocated zeroed (lazily, by the kernel) instead of
  filled with a non-zero "empty" pattern. Contents and lookups are unchanged.
- Also present but not measured here: option `xb`, and the `MTF_Z` support in rho's growing tables, which is the
  same code path.
- Every execution runs with `MTF_HP=1` (huge pages; THP mode `madvise` on this host).
- **Binary.** Built standalone with `rustc` at cargo's release settings, as in the huge-page round:

  ```
  rustc --edition 2021 -C opt-level=3 -C codegen-units=16 --crate-type bin examples/mt_fieldops.rs \
        -L dependency=target/release/deps --extern rand=… --extern serde_json=…
  ```

  The SHA-256 is in `check.out` and on every row.

### Pre-run checks, recorded in `check.out` before the main run

1. The new binary and the huge-page round's binary (`c049a323…`) give byte-identical output with `MTF_HP=1`,
   `MTF_Z=0` and the previous options, all fields but times: `ic5b … flatf 128 bi pf,rel` on W28-0 and W36-0,
   T = 1024; `rhodpb` κ = 0, 64 lanes, on W32-4 and W36-0, T = 1024.
2. For the same four executions, `MTF_Z=1` gives output identical to `MTF_Z=0`, all fields but times.
3. On W32-4 and W36-0, T = 1024, `rel,ib` (with both `MTF_Z` values) has `pf,rel`'s table, trials, relations,
   p1_trials, I, M and S, and recovers every target.

## Keys per configuration (28), all with `MTF_HP=1`

- IC, `ic5b <inst> T 43 0.85 4 1.2 flatf 128 bi <opts>`:
  - `ic:pf`: `MTF_Z=0`, opts `pf,rel` (the current IC);
  - `ic:ib`: `MTF_Z=0`, opts `rel,ib`;
  - `ic:z`: `MTF_Z=1`, opts `pf,rel`;
  - `ic:z+ib`: `MTF_Z=1`, opts `rel,ib` (**the primary arm**).
- Rho: `rhodpb` at L ∈ {64, 128} lanes × κ ∈ {0, 0.25} × offset ∈ {−1, 0, 1}, each with `MTF_Z` ∈ {0, 1} (24 keys).
  Best rho is the minimum over all 24, so rho takes whichever encoding gives it the lower time.

## Configurations and order

- Instances: W32-{4..7} and W36-{0..3}.
- T ∈ {1024, 4096, 16384}; seed 43; 24 configurations × 28 keys × 3 repeats = 2,016 executions.
- One process at a time, pinned to one CPU, on this session's container (Intel Xeon @ 2.10 GHz, 4 vCPU).
- Repeats are interleaved in a fixed-seed shuffled order (20261123).

## Measures and gates

A key's time is the median of its 3 repeats of `ms_arm`. Cell = (size, T); cell value = median over its 4
configurations; a cell decision is a 4-of-4 sign rule.

- **Correctness (stops the reading):** every execution recovers every target. The IC keys agree on table, trials,
  relations and p1_trials. Each rho key agrees with itself across `MTF_Z` on I, M, S, walks and db.
- **Primary (IC baseline cost at 2^36):** `ic:z+ib` / `ic:pf` < 1, 4 of 4, in all three 2^36 cells → **lower**;
  ≥ 1 in all → **not lower**; else **mixed**.
- **Distance remaining:** `ic:z+ib` / best rho per cell. "IC ahead" ≤ 1 in 4 of 4; "IC behind" > 1 in 4 of 4.
- **Secondary:**
  - attribution `ic:ib` / `ic:pf` and `ic:z` / `ic:pf`;
  - the same IC ratio at 2^32;
  - rho's `MTF_Z` effect (best rho z1 / best rho z0).

Priors (stated before data):
- primary "lower", 0.85;
- IC ahead in at least one 2^36 cell, 0.5;
- `MTF_Z` lowers best rho by more than 3 % in any cell, 0.1.

## §5a pricing

Every arm is timed whole, so all native work is inside `ms_arm`. The 0 / 0.1 / 1 GAE band of unpriced native work
is empty for every arm, rho included.

## Not claimed

- No `S`, no single-target change, no BSGS result.
- No leaderboard or `benchmarks/` row: there is no replay certificate or validator report, and the targets are
  planted on generated instances.
- Wall time on one container, one binary, read as paired ratios. This host differs from the huge-page round's (rho
  is slower here relative to the IC), so its distances are compared within this run only.
