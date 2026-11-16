# Where the folded pair-table build spends its time: step probe (plan)

Written on 2026-11-13, **before any run**. This is a diagnostic: it prices
the steps of a build, and it decides nothing by itself and claims no
speed-up.

## Why

The A/B in
[`../build-pairing-lookahead-20261112/`](../build-pairing-lookahead-20261112/README.md)
found the build's additions to be a small part of its time: about 27 ns per
stored pair is unexplained. The next build lever should target the step
that holds that time, so the steps are measured first.

## Instrument

- **Code:** `build_probe` in `src/cryptanalysis/koblitz_index_calculus.rs`.
  - Its timing points are compiled only under `--features scan-probes`
    (time-stamp counter). A default build compiles none of them.
  - It charges the build's steps:
    - `setup`: rows, base order, canonical tables, lifted points,
      coordinate slices;
    - `row_add`: each row's additions;
    - `row_key`: each row's canonical keys;
    - `stream_push`: hashing each key and pushing it into its partition's
      streams;
    - `sort_fill`: the counting sort into buckets plus the filter fill;
    - `filter_merge`: ORing the runs' filters;
    - `dups`: the duplicate harvest, off by default;
    - `assemble`: assembling the table;
    - `whole`: the build's wall time, the check on the others.
- **Report:** `ic price` writes the totals under `scan_probes.build`, in
  ticks, with `tsc_hz` to convert.

## Runs

- **Rows:** suite `M1-T01` at `k0n53` (2^44.3) and `k0n61` (2^47.2).
- **Processes:** 5 each, under `isolated_bench run --wait --cpus 2` with
  `RAYON_NUM_THREADS=1`. With one thread, the steps should sum to `whole`.
- **Read-out:** per process, each step's ns per stored pair, summed over
  the process's builds and divided by builds × stored pairs. Per row, the
  median over processes and the min–max.
- **Check:** the steps must sum to `whole` within 2%; otherwise the
  instrument is wrong and nothing is read from it. `whole` is also set
  against the report's `setup_phases_ns.build`.

## Host

This session's container (Intel Xeon, AVX-512). Iso Lab is blocked while
the CodeBuild quota reads 0.
