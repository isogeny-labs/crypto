# Transparent huge pages for the pair table's scanned arrays: same-session A/B with its own A/A (protocol)

Written on 2026-11-15, **before any row**. The owner asked to keep
optimizing the index calculus until nothing is left.

## Why

Collection is now 70–85% of set-up. Scan-probe cycles a summand at k0n53
are: subtract 4.7, key 9.9, filter 9.8, admitted 5.0, trial 1.3.

The filter stage and the admitted stage read the pair table at random:
- the presence filter `present`;
- `bucket_start`;
- the run words `rests`.

These are tens of MB on 4 KiB pages, while this host runs transparent
huge pages in `madvise` mode and nothing asked for them. So every random
read can also miss the TLB. The build's counting-sort scatter writes the
same arrays at random.

## The change

- **Advice:** `advise_huge_pages` calls `madvise(MADV_HUGEPAGE)` on the
  2 MiB-aligned span of `bucket_start`, `rests` and each `present` in
  `PairSumTable::fold_partitioned`. It does so right after allocation,
  before the first write.
- **Control:** `KIC_HUGE_PAGES=0` turns the advice off.
- **Unchanged:** the arrays, answers and counted work. Only the page size
  behind the arrays changes.
- **Exploration** ([`../explore/`](../explore/), `isolated_bench`, three
  rounds an arm):
  - the host's noise was as large as the effect: the key stage alone
    moved about 15% between processes;
  - filter over key went from 0.92–1.00 to 0.83–0.94 at k0n61;
  - at k0n53 there was no visible change.

## Arms: one binary, three labels

- **Binary:** `ic`, release, built at this protocol's commit with
  `--features scan-probes`. Its SHA-256 goes in `runs/arms.sha256` before
  the first row.
- **Labels:**
  - **a**: `KIC_HUGE_PAGES=0`, the baseline;
  - **q**: the same as `a`, its A/A twin;
  - **h**: the default (advice on).

## Workload, schedule, metrics and statistics

- **Rows:** `M1-T01` at `k0n53` and `k0n61`.
- **Isolation:** `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1`.
- **Schedule:** **24 rounds**, cycled over the six orders of `a q h` (four
  cycles).
- **Metrics, per process:**
  - **filter-stage cycles a summand** (primary);
  - admitted-stage cycles a summand (both over the process's scan
    probes);
  - medians over repetitions of collect, build and set-up.
- **Statistics:** per-round A/B `a / h` and A/A `a / q`; the median with a
  95% percentile bootstrap over rounds (2,000 resamples, exact LCG step,
  high-bit index, seed 20261120).
- **Dropped rounds:** a contended or failed round is dropped and counted.

## Decision rule (applied once, to all rounds)

- **Established:** a gain is established if the A/B lower end is above 1
  and above the A/A upper end.
- **Regression:** a regression is established if the A/B upper end is
  below 1 and below the A/A lower end.
- **Kept:** the advice stays on by default if a **filter or admitted**
  gain is established on at least one row, and no **build or set-up**
  regression is established on either row.
- **Otherwise:** the default turns the advice off, the switch stays, and
  the reason is recorded.

## Prediction (before any row)

Medians `a / h`:

| row | filter | admitted | collect | build | set-up |
|:--|:--|:--|:--|:--|:--|
| k0n53 | [0.97, 1.08] | [0.97, 1.08] | [0.98, 1.04] | [0.90, 1.05] | [0.97, 1.03] |
| k0n61 | [1.03, 1.15] | [1.02, 1.12] | [1.00, 1.06] | [0.90, 1.05] | [0.99, 1.05] |

## Identity

Every process of a row must report the same certificates and `counts` as
that row's first `a` process. Any mismatch voids the session.

## Route

This session's container (Intel Xeon @ 2.1 GHz, AVX-512, THP `madvise`,
defrag `madvise`). The Iso Lab bridge cannot submit while its CodeBuild
quota reads 0. The result holds for this host class and THP setting only.
