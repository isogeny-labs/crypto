# Huge pages for every arm's tables: does the IC's TLB cost at 2^36 go away? (2026-11-18)

Frozen before any measured run.

**Setting.** At 2^36, T = 16384 the IC (k = 4, μ = 0.85, `flatf`, B = 128, `bi`, options `pf,rel`) is still about
1.22× the tuned batched rho ([`../mt_ic_rel_20261115/`](../mt_ic_rel_20261115/), [pcc:unverified]). Its table is
a 537 MB slot array plus side arrays, and every insert and probe is a random access.

This container's transparent-huge-page mode is `madvise`, so every table so far ran on 4 KB pages. A
microbenchmark here (prefetched random probes into a 512 MB array of 8-byte slots, pinned to CPU 2) measured
[pcc:unverified]:

| access | ns per probe |
|---|---|
| 4 KB pages | 16.5 |
| 2 MB huge pages | 7.4 |
| radix-sorted batches | 13.3–15.4, no better than huge pages |

So the misses are TLB-bound. Exploratory single runs of the instrument below put the IC at 2^36 at 0.80–0.86×
with huge pages, and rho unchanged.

## Instrument: `examples/mt_fieldops.rs`

- Environment `MTF_HP=1`: every flat table's arrays (`slots`, the side array `xs`, `ents`, and rho's growing
  database on resize) call madvise(MADV_HUGEPAGE) before first touch, for **every arm, IC and rho alike**. Arrays
  under 4 MB are left alone. With `MTF_HP` unset or 0 the binary behaves as before.
- **Binary.** `main` does not currently compile (duplicate fields in `src/cryptanalysis/f4_fp.rs` after a merge),
  so cargo cannot build examples. `mt_fieldops` uses only `rand`, `serde_json` and std. It is built standalone:

  ```
  rustc --edition 2021 -C opt-level=3 -C codegen-units=16 --crate-type bin examples/mt_fieldops.rs \
        -L dependency=target/release/deps --extern rand=… --extern serde_json=…
  ```

  These are cargo's default release settings. Every arm uses this one binary. Its SHA-256 is in `check.out` and
  on every row.

### Pre-run checks, recorded in `check.out` before the main run

1. The new binary with `MTF_HP=0` and the previous instrument (`17b57af9f3`, built the same way) give
   byte-identical output, all fields but times: `ic5b … flatf 128 bi pf,rel` on W28-0 and W36-0, T = 1024;
   `rhodpb` κ = 0, 64 lanes, on W32-4 and W36-0, T = 1024.
2. `MTF_HP=1` gives the same output as `MTF_HP=0`, all fields but times, for the same four executions.
3. Huge pages are in effect: during an `MTF_HP=1` IC execution at W36-0, T = 16384, the process's
   `AnonHugePages` (from `/proc/<pid>/smaps_rollup`) is at least 256 MB; with `MTF_HP=0` it is 0.

## Keys per configuration (26)

- IC: `ic5b <inst> T 43 0.85 4 1.2 flatf 128 bi pf,rel` with `MTF_HP` ∈ {0, 1}: `ic:rel:hp0`, `ic:rel:hp1`.
- Rho: `rhodpb` at L ∈ {64, 128} lanes × κ ∈ {0, 0.25} × offset ∈ {−1, 0, 1}, each with `MTF_HP` ∈ {0, 1} (24 keys).
  64 and 128 lanes were best in 20 of 24 configurations of the relation-phase round.
- Best rho is the minimum over all 24 rho keys, so rho takes whichever paging gives it the lower time.

## Configurations and order

- Instances: W32-{4..7} and W36-{0..3}.
- T ∈ {1024, 4096, 16384}; seed 43; 24 configurations × 26 keys × 3 repeats = 1,872 executions.
- One process at a time, pinned to one CPU, on this session's container (Intel Xeon @ 2.30 GHz, 4 vCPU; THP mode
  recorded in `host.json`).
- Repeats are interleaved in a fixed-seed shuffled order (20261122).

## Measures and gates

A key's time is the median of its 3 repeats of `ms_arm`. Cell = (size, T); cell value = median over its 4
configurations; a cell decision is a 4-of-4 sign rule. This host's IC repeat spread is about 14 % (three previous
rounds), so the primary is set where the exploratory effect is larger than that: the 2^36 cells.

- **Correctness (stops the reading):** every execution recovers every target, and each arm's counted fields agree
  between hp0 and hp1 in every configuration:
  - IC: table, trials, relations, p1_trials, I, M, S;
  - rho: I, M, S, walks, db.
- **Primary (IC baseline cost at 2^36):** `ic:rel:hp1` / `ic:rel:hp0` < 1, 4 of 4, in all three 2^36 cells →
  **lower**; ≥ 1 in all → **not lower**; else **mixed**.
- **Distance remaining:** `ic:rel:hp1` / best rho (both pagings), per cell. "IC ahead" ≤ 1 in 4 of 4; "IC behind"
  > 1 in 4 of 4.
- **Secondary:** the same IC ratio at 2^32; what huge pages do for rho (best rho hp1 / best rho hp0).

Priors (stated before data):
- primary "lower", 0.8;
- IC ahead in at least one 2^36 cell, 0.35;
- huge pages lower the best rho by more than 3 % in any cell, 0.15.

## §5a pricing

Every arm is timed whole, so all native work is inside `ms_arm`. The 0 / 0.1 / 1 GAE band of unpriced native work
is empty for every arm, rho included.

## Not claimed

- No `S`, no single-target change, no BSGS result.
- No leaderboard or `benchmarks/` row: there is no replay certificate or validator report, and the targets are
  planted on generated instances.
- A host whose THP mode is `always` would already give both arms huge pages. The effect measured here is a
  paging-policy choice, and it is reported as such: a real cost of this host's default that the IC pays and rho
  barely does.
