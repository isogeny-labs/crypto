# Block-filled filter tests in the aimed scan: same-session A/B with its own A/A (protocol)

Written on 2026-11-15, **before any row**. The owner asked to keep
optimizing the index calculus.

## Why this stage

A probe of current main (`a95274095f`; outputs in
[`../microbench/`](../microbench/)) prices collection at 197 of 310 ms of
set-up at k0n53 and 1,075 of 1,281 ms at k0n61. The filter stage is the
largest part of a scanned summand: 5.30 and 5.72 ns, of 14.4 and 14.9.

A filter probe costs about 2.3 ns even when the filter sits in L1
([`../../key-chains-20261114/`](../../key-chains-20261114/README.md)), so
much of the stage is instructions rather than memory. Per key, the scalar
loop hashes (two multiplies), computes the word and three bit positions
twice (once to prefetch, once to test), counts a probe, and branches.

## The change

- **`FilterGeom::fill_block`:** fills each block's hashes, filter words and
  bits with AVX-512F/DQ, eight keys at a time, before the test loop. The
  loop then prefetches and tests from these arrays.
- **Probe count:** counted once per block, the same total.
- **Answers:** admitted keys, answers and counted work are identical. The
  test `a_filled_block_is_the_per_key_hash_and_slot` checks every lane
  against the scalar hash and slot under both layouts, both probe counts,
  every width and tails of every length.
- **Control:** `KIC_SCAN_VFILTER=0`.
- **Micro-benchmark** ([`../microbench/vf.rs`](../microbench/vf.rs), random
  keys, an 8 MB and a 4 MB filter): 4.1–4.3 → 3.6–3.7 ns per key.

## Arms: one binary, three labels

- **Binary:** `ic` built at this protocol's commit with `--features
  scan-probes`. Its SHA-256 goes in `runs/arms.sha256` before the first
  row.
- **Labels:**
  - **a**: `KIC_SCAN_VFILTER=0`, the baseline;
  - **q**: the same as `a`, its A/A twin;
  - **v**: the default.

## Workload, schedule, metrics and statistics

The same as the key-chains A/B
([`../../key-chains-20261114/ab/PROTOCOL.md`](../../key-chains-20261114/ab/PROTOCOL.md)):
- **Rows:** `M1-T01` at `k0n53` and `k0n61`.
- **Isolation:** `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1`.
- **Schedule:** 42 rounds cycled over the six orders of `a q v`.
- **Metrics:** per-process medians of build, collect and set-up.
- **Statistics:** per-round A/B `a / v` and A/A `a / q`; the median with a
  95% percentile bootstrap over rounds (2,000 resamples, exact LCG step,
  high-bit index, seed 20261116).
- **Dropped rounds:** a contended or failed round is dropped and counted.

## Decision rule (applied once, to all rounds)

- **Established:** a gain is established if the A/B lower end is above 1
  and above the A/A upper end.
- **Regression:** a build regression is established if the A/B upper end
  is below 1 and below the A/A lower end.
- **Kept:** the block fill stays the default if the **collect** gain is
  established on at least one row and no build regression is established.
- **Otherwise:** the default reverts to the per-key loop, the switch stays
  (opt-in), and the reason is recorded.

## Prediction (before any row)

Medians `a / v`:

| row | collect | build | set-up |
|:--|:--|:--|:--|
| k0n53 | [1.01, 1.07] | [0.98, 1.02] | [1.00, 1.05] |
| k0n61 | [1.01, 1.07] | [0.98, 1.02] | [1.01, 1.06] |

## Identity

Every process of a row must report the same certificates and `counts` as
that row's first `a` process. Any mismatch voids the session.

## Route

This session's container (Intel Xeon @ 2.1 GHz, AVX-512). The Iso Lab
bridge cannot submit while its CodeBuild quota reads 0. The result holds
for this host class only.
