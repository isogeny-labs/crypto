# Partition-local presence filter (top-bits layout): same-session A/B with its own A/A (protocol)

Written on 2026-11-13, **before any row**. The owner chose a CPU-only
change behind a layout tag. The motivation is the build-step diagnostic
([`../../build-steps-20261113/`](../../build-steps-20261113/README.md)):
- placing pairs is about three quarters of the folded build;
- a micro-benchmark cut placement from 8.5–12.6 to 5.0–5.7 ns per pair by
  taking the filter word from the hash's top bits.

## The change

- **`FilterLayout`:** `Low` is today's layout; `Top` takes the word from the
  top bits and the in-word bits from hash bits 16–33.
- **Recorded per table:** every table records its layout in `FoldedParts`
  and `FoldedStorage`, and every probe follows it. The probe is branch-free:
  `(h >> shift) & words`, with three fixed bit shifts.
- **Default:** this module's folded CPU builds default to `Top`;
  `KIC_FILTER_LAYOUT=low` restores today's.
- **Unchanged:** non-folded tables, GPU table files and the two
  GPU-parity tools (`examples/dump_fold_vectors.rs`,
  `examples/load_fold_table.rs`) stay `Low`.
- **Answers:** unchanged by construction. A filter has no false negatives,
  and every admitted key is checked against the entries. Tests:
  - `both_filter_layouts_give_the_same_answers`: the same answers on 3
    curves, the same buckets, a different filter;
  - `the_low_filter_layout_is_filter_slot`: the low layout is today's
    arithmetic exactly.

## Arms: one binary, three labels

- **Binary:** `ic` built at this protocol's commit with `--features
  scan-probes`, so that each arm's build steps are also recorded. Its
  SHA-256 goes in `runs/arms.sha256` before the first row.
- **Labels:**
  - **a**: `KIC_FILTER_LAYOUT=low`, today's layout, the baseline;
  - **q**: the same as `a`, its A/A twin;
  - **t**: the default, `Top`.

## Workload

- `ic price --params research/ic_tool_program/suite/v1/params/S/<row>/M1-T01.json
  --json --single-target --rho-seed <suite seed>` for `k0n53` (2^44.3) and
  `k0n61` (2^47.2).
- Every process runs under `isolated_bench run --wait --cpus 2` with
  `RAYON_NUM_THREADS=1`. Busy-gate refusals are retried and counted.

## Schedule

42 rounds. Round `r` runs `k0n53` then `k0n61`, each in permutation
`(r − 1) mod 6` of `[a q t, a t q, q a t, q t a, t a q, t q a]`.

## Metrics and statistics

- **Per process:** the median over repetitions of
  `setup_phases_ns.build`, `setup_phases_ns.collect` and `setup_ns`.
- **Per round:** A/B `a / t` and A/A `a / q`.
- **Per row and metric:** the median of per-round ratios, with a 95%
  percentile bootstrap over rounds. The bootstrap is lever 2's: 2,000
  resamples, an exact LCG step, a high-bit index and seed 20261113.
- **Dropped rounds:** a round with a failed or contended process is dropped
  and counted.

## Decision rule (applied once, to all rounds)

- **Established:** a gain on a metric and row is established if the A/B
  lower end is above 1 and above the A/A upper end.
- **Regression:** a collect regression is established if the A/B upper end
  is below 1 and below the A/A lower end.
- **Kept:** the layout stays the default if the build gain is established
  on at least one row and no collect regression is established on any row.
- **Otherwise:** the default reverts to `Low`. The code and switch stay as
  an option, and the reason is recorded.

## Prediction (before any row)

Medians `a / t`:

| row | build | set-up | collect |
|:--|:--|:--|:--|
| k0n53 | [1.05, 1.25] | [1.01, 1.07] | [0.97, 1.03] |
| k0n61 | [1.05, 1.25] | [1.00, 1.04] | [0.97, 1.03] |

## Identity

Every process of a row must report the same certificates and `counts` as
that row's first `a` process. Any mismatch voids the session.

## Route

This session's container (Intel Xeon, AVX-512), because the Iso Lab bridge
cannot submit while its CodeBuild quota reads 0. The result holds for this
host class only.
