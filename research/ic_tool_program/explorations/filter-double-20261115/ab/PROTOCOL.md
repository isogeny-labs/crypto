# A doubled presence filter for CPU (Top-layout) pair tables: same-session A/B with its own A/A (protocol)

Written on 2026-11-15, **before any row**. The owner asked to keep
optimizing the index calculus until nothing is left, and approved
pursuing the doubled folded filter, a table-format change.

## Why

- The admitted stage costs 3.8–4.3 cycles a summand at k0n53, after huge
  pages.
- Admitted keys are 2.8% of summands at k0n53 and 2.0% at k0n61. Nearly
  all are false positives, and each costs a bucket and run read.
- A filter one bit wider (16 bits a key in place of 8) admits about a
  fifth as many absent keys.

## The change

- **Width:** `FilterLayout::extra_filter_bits` adds one bit to the width
  of a `Top`-layout folded table, which doubles the filter. Both folded
  builders apply it: the partitioned one and the two-pass reference.
  - The reference was added after a first draft of this protocol, when
    the tests comparing the two builders caught the gap. No row had been
    taken; the binary is built at the commit that freezes this text.
- **`Low` is unchanged:** `Low` tables keep today's width. That covers
  the table files, the GPU builder and `ecbench`'s methods, so their
  counted bucket reads stay as they were.
- **Control:** `KIC_FILTER_DOUBLE=0` keeps the old width.
- **Unchanged:** the stored pairs, answers, and every count `ic price`
  reports. The table's filter memory doubles.
- **Exploration** ([`../explore/`](../explore/), three isolated rounds an
  arm):

  | row | admitted (cycles a summand) | filter | admitted rate | collect |
  |:--|:--|:--|:--|:--|
  | k0n53 | 3.8–4.3 → 1.6–1.9 | +0.1 | 0.028 → 0.006 | 172–196 → 158–168 ms (one 229 outlier) |
  | k0n61 | 3.2–3.3 → 1.2–1.3 | −1.1 to +0.3 | 0.020 → 0.004 | 960–1229 → 884–1060 ms |

  - Certificates and the 160 and 172 counted and answer fields were
    identical between the arms.

## Arms: one binary, three labels

- **Binary:** `ic`, release, built at this protocol's commit with
  `--features scan-probes`. Its SHA-256 goes in `runs/arms.sha256` before
  the first row.
- **Labels:**
  - **a**: `KIC_FILTER_DOUBLE=0`, the baseline;
  - **q**: the same as `a`, its A/A twin;
  - **d**: the default (doubled).

## Workload, schedule, metrics and statistics

- **Rows:** `M1-T01` at `k0n53` and `k0n61`.
- **Isolation:** `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1`.
- **Schedule:** **24 rounds**, cycled over the six orders of `a q d` (four
  cycles).
- **Metrics, per process:**
  - median over repetitions of **collect** (primary);
  - filter-plus-admitted and admitted-stage cycles a summand (scan
    probes);
  - medians of build and set-up.
- **Statistics:** per-round A/B `a / d` and A/A `a / q`; the median with a
  95% percentile bootstrap over rounds (2,000 resamples, exact LCG step,
  high-bit index, seed 20261121).
- **Dropped rounds:** a contended or failed round is dropped and counted.

## Decision rule (applied once, to all rounds)

- **Established:** a gain is established if the A/B lower end is above 1
  and above the A/A upper end.
- **Regression:** a regression is established if the A/B upper end is
  below 1 and below the A/A lower end.
- **Kept:** the doubled filter stays the default if the **collect** gain
  is established on at least one row, and no **build or set-up**
  regression is established on either row.
- **Otherwise:** the default returns to the single width, the switch
  stays, and the reason is recorded.

## Prediction (before any row)

Medians `a / d`:

| row | collect | filter + admitted | admitted | build | set-up |
|:--|:--|:--|:--|:--|:--|
| k0n53 | [1.04, 1.12] | [1.15, 1.30] | [2.0, 2.8] | [0.95, 1.03] | [1.02, 1.08] |
| k0n61 | [1.04, 1.12] | [1.12, 1.30] | [2.2, 3.0] | [0.95, 1.03] | [1.03, 1.10] |

## Identity

- **Certificates:** every process of a row must report the same
  certificates and `counts` as that row's first `a` process. Any
  mismatch voids the session.
- **What may differ:** only memory and the scan-probe tallies (admitted
  keys).

## Route

This session's container (Intel Xeon @ 2.1 GHz, AVX-512, THP `madvise`).
The Iso Lab bridge cannot submit while its CodeBuild quota reads 0. The
result holds for this host class only.
