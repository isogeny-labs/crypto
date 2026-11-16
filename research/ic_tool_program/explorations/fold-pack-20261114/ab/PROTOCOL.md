# One packed stream a partition in the folded build: same-session A/B with its own A/A (protocol)

Written on 2026-11-14, **before any row**. The owner asked to keep
optimizing the index calculus. After the partition-local filter
([`../../filter-layout-20261113/`](../../filter-layout-20261113/README.md)),
pushing pairs into the partition streams is the build's largest step, at
12.3–13.9 ns per stored pair.

## The change

- **What:** `fold_partitioned` pushes one `u64` a pair instead of a hash
  (`u64`) plus a stored word (`u32`).
- **Why it fits:** no step reads a hash's bits 34–37, and a partition's
  top 8 bits are its own. Those 12 bits carry the pair's orbit, and the
  stored word is rebuilt as the pair is placed.
- **When it applies:** a tagged table, no duplicate harvest, 256
  partitions, at most 26 bucket bits and at most 4,096 orbits. The suite
  rows have about 264–320 orbits.
- **Control:** `KIC_FOLD_PACK=0` keeps two streams.
- **Tables:** identical word for word. The partitioned build is checked
  against the two-pass build
  (`the_partitioned_fold_build_is_one_table_word_for_word_on_any_thread_count`
  and `the_partitioned_fold_build_stores_the_two_pass_table`).
- **Micro-benchmark** ([`../microbench/pb.rs`](../microbench/pb.rs),
  random keys, the top layout): push 11.5 → 5.6 ns at 3.7 M pairs and
  11.0 → 6.7 ns at 6.3 M; placement 7.2 → 6.2 and 7.9 → 7.7.

## Arms: one binary, three labels

- **Binary:** `ic` built at this protocol's commit with `--features
  scan-probes`. Its SHA-256 goes in `runs/arms.sha256` before the first
  row.
- **Labels:**
  - **a**: `KIC_FOLD_PACK=0`, today's two streams, the baseline;
  - **q**: the same as `a`, its A/A twin;
  - **p**: the default, one packed stream.

## Workload, schedule, metrics and statistics

The same as the filter-layout A/B's
([`../../filter-layout-20261113/ab/PROTOCOL.md`](../../filter-layout-20261113/ab/PROTOCOL.md)):
- **Rows:** `M1-T01` at `k0n53` and `k0n61`.
- **Isolation:** `isolated_bench run --wait --cpus 2` with
  `RAYON_NUM_THREADS=1`.
- **Schedule:** 42 rounds cycled over the six orders of `a q p`.
- **Metrics:** per-process medians of build, collect and set-up.
- **Statistics:** A/B `a / p` and A/A `a / q` per round; the median with a
  95% percentile bootstrap over rounds (2,000 resamples, exact LCG step,
  high-bit index, seed 20261114).
- **Dropped rounds:** a contended or failed round is dropped and counted.

## Decision rule (applied once, to all rounds)

- **Established:** a gain is established if the A/B lower end is above 1
  and above the A/A upper end.
- **Regression:** a collect regression is established if the A/B upper end
  is below 1 and below the A/A lower end.
- **Kept:** packing stays the default if the build gain is established on
  at least one row and no collect regression is established.
- **Otherwise:** the default reverts to two streams, the switch stays, and
  the reason is recorded.

## Prediction (before any row)

Medians `a / p`:

| row | build | set-up | collect |
|:--|:--|:--|:--|
| k0n53 | [1.08, 1.25] | [1.00, 1.07] | [0.97, 1.03] |
| k0n61 | [1.08, 1.25] | [1.00, 1.04] | [0.97, 1.03] |

## Identity

Every process of a row must report the same certificates and `counts` as
that row's first `a` process. Any mismatch voids the session.

## Route

This session's container (Intel Xeon @ 2.1 GHz, AVX-512). The Iso Lab
bridge cannot submit while its CodeBuild quota reads 0. The result holds
for this host class only.
