# Build-side pairing and filter lookahead: a same-session A/B with its own A/A (protocol)

Written on 2026-11-12, **before any row of this measurement**. The owner
asked for a proper A/B on both levers after the exploration in
[`../README.md`](../README.md). None of the exploration's runs is pooled
here.

## Questions

1. **Build pairing.** Does forming each ± twin pair of a fold-build row from
   one inverse make the build phase, and the whole set-up, of
   `ic price --single-target` faster?
2. **Lookahead 64.** Does prefetching the aimed scan's filter words 64 keys
   ahead instead of 32 make the collect phase, and the whole set-up, faster?

Outputs are unchanged by construction. The fold-build test checks the
tables word for word with pairing on and off.

## Arms: one binary, four labels

- **Binary:** `ic` built at this protocol's commit with `--features
  scan-probes`, the same features as the exploration. Its SHA-256 goes in
  `runs/arms.sha256` before the first row, and `run.sh` refuses any other
  binary.
- **Labels:**
  - **a** — `KIC_BUILD_PAIRED=0`, lookahead 32: today's path, the baseline;
  - **q** — the same as `a`: its A/A twin;
  - **b** — build pairing on, lookahead 32;
  - **l** — `KIC_BUILD_PAIRED=0`, `KIC_LOOKAHEAD=64`.
- **Choice of 64:** made from the exploration's sweep before this
  protocol. It had the lowest median collect at `k0n61` and the
  second-lowest at `k0n53`. No other value is tested.

## Workload

`ic price --params research/ic_tool_program/suite/v1/params/S/<row>/M1-T01.json
--json --single-target --rho-seed <the suite's seed>`, for `k0n53` (2^44.3)
and `k0n61` (2^47.2).

- Every process runs under `isolated_bench run --wait --cpus 2` with
  `RAYON_NUM_THREADS=1`.
- A launch the busy gate refuses before it starts is retried and counted.
  It is not a measurement.

## Schedule

- 44 rounds a row.
- Round `r` runs `k0n53` then `k0n61`, each in label order `(r − 1) mod 4`
  of the cyclic Latin square `[a q b l, q b l a, b l a q, l a q b]`.
- Over 11 cycles every label runs in every position equally often.
- 352 processes in all.

## Metrics and statistics

- **Per process:** the median over its repetitions of
  `setup_phases_ns.build`, `setup_phases_ns.collect` and `setup_ns`.
- **Per round and row:** the A/B ratios `a / b` for build and set-up,
  `a / l` for collect and set-up, and the A/A ratio `a / q` for each of
  the three metrics.
- **Per row and ratio:** the median of the per-round ratios, with a 95%
  percentile bootstrap over rounds. The bootstrap is lever 2's: 2,000
  resamples, the LCG `s → (s·1103515245 + 12345) mod 2^31` stepped exactly,
  the index `floor(s·n / 2^31)`, and seed 20261112.
- **Dropped rounds:** a round with a failed or contended process is dropped
  from that row's statistics and counted.

## Decision rule

A lever's gain on a metric and row is **established** in this session only
if its A/B interval's lower end is above 1 **and** above the upper end of
the A/A interval for the same metric. Otherwise the gain is reported and
not claimed.

- The rule is applied once, to all rounds. There is no early stop and no
  extension.
- A lever whose primary metric is established on at least one row is kept
  in code: build for `b`, collect for `l`.
- Otherwise the lever is reverted, and its switch with it.

## Prediction (before any row)

Medians from the exploration and the arithmetic:

| lever | row | primary metric (median) | set-up (median) |
|:--|:--|:--|:--|
| `a / b` | k0n53 | build in [1.00, 1.03] | [1.00, 1.01] |
| `a / b` | k0n61 | build in [1.00, 1.04] | [1.00, 1.01] |
| `a / l` | k0n53 | collect in [0.98, 1.04] | [0.98, 1.03] |
| `a / l` | k0n61 | collect in [0.98, 1.05] | [0.98, 1.04] |

The likely outcome is "not established". The A/A floor in lever 2's
session was about ±3–6%.

## Identity

Every process of a row must report the same certificates and `counts` as
that row's first `a` process. Any mismatch voids the session.

## Route

This session's container (Intel Xeon @ 2.1 GHz, 4 vCPU, AVX-512), as for
lever 2. The Iso Lab bridge cannot submit, because the CodeBuild quota
reads 0. The result holds for this host class only.

## Not in scope

No board row and no claim against rho.
