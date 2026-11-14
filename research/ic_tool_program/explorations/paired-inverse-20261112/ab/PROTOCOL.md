# Paired inverse (lever 2): a same-session A/B with its own A/A (protocol)

Written on 2026-11-12, **before any row of this measurement**. The
exploration in [`../README.md`](../README.md) motivates it. None of the
exploration's runs is pooled here.

## Question

Does sharing one inverse per ± twin pair in the aimed scan's folding kernel
make `ic price --single-target` faster on the collect phase and the whole
set-up? The change is `a31ae987a6`, with a clippy fix in `b06ee3ae7f`.
Outputs are unchanged by construction and by test.

## Arms: one binary, three labels

- **Binary:** `ic` built at this protocol's commit
  (`CARGO_TARGET_DIR=/dev/shm/tgt-prof cargo build --release --bin ic`). Its
  SHA-256 goes in `runs/arms.sha256` before the first row, and `run.sh`
  refuses to run any other binary.
- **Labels:**
  - **a** — unpaired, `KIC_SCAN_PAIRED=0`: today's path, the baseline;
  - **q** — unpaired again: the A/A twin of `a`;
  - **c** — paired: the default.
- **Why one binary:** both arms run the same code layout, and only the
  environment switch differs.

## Workload

`ic price --params research/ic_tool_program/suite/v1/params/S/<row>/M1-T01.json
--json --single-target --rho-seed <the suite's seed>` for two rows:
- `k0n53` (`icv1-f2m53-tm56619371-dac20a85`, 2^44.3);
- `k0n61` (`icv1-f2m61-t158598901-ab42b6c5`, 2^47.2).

Every process runs under `isolated_bench run --wait --cpus 2` with
`RAYON_NUM_THREADS=1`. This is the binary `rule/v9/binaries.sha256` names
(`2f56b833…`). A launch the busy gate refuses before it starts is retried
and counted. It is not a measurement.

## Schedule

42 rounds per row. Round `r` runs both rows, `k0n53` then `k0n61`, each in
label order permutation `(r − 1) mod 6` of
`[a q c, a c q, q a c, q c a, c a q, c q a]`. Over 7 cycles every label
runs in every position equally often. That is 252 processes in all.

## Metrics and statistics

- **Per process:** the median over its repetitions of
  `setup_phases_ns.collect`, and of `setup_ns`, the whole set-up.
- **Per round and row:** the A/B ratio `a / c` and the A/A ratio `a / q`,
  both within one round and one session.
- **Per row and metric:** the median of the 42 per-round ratios, with a 95%
  percentile bootstrap over rounds. The bootstrap uses 2,000 resamples and
  the LCG `s → (s·1103515245 + 12345) mod 2^31`, stepped exactly. Its index
  is `floor(s·n / 2^31)`, the high bits, as corrected in
  `research/ic_aimed_gather_20261023/aa2_20261029/CORRECTION.md`. The seed
  is 20261112.
- **Dropped rounds:** a round with a failed process, or one its isolation
  record marks contended, is dropped from both statistics for that row and
  counted.

## Decision rule

For each row and each metric separately, the paired gain is
**established** in this session only if the A/B interval's lower end is
above 1 **and** above the A/A interval's upper end. Otherwise it is
reported and not claimed. The rule is applied once, to all 42 rounds. There
is no early stop and no extension.

## Prediction (from the exploration, before any row)

A/B medians `a / c`:

| row | collect | set-up |
|:--|:--|:--|
| k0n53 | [1.02, 1.08] | [1.00, 1.05] |
| k0n61 | [1.05, 1.13] | [1.03, 1.10] |

## Identity

Every process of a row must report the same certificates (both arms'
scalars) and the same `counts` as that row's first `a` process. Any
mismatch voids the session.

## Route

This session's container, not Iso Lab (AGENTS.md §9a): Intel Xeon @ 2.1
GHz, 4 vCPU, AVX-512, VPCLMULQDQ and GFNI, recorded in `runs/host.txt`.
The Iso Lab bridge cannot submit any Job, because every CodeBuild
concurrency quota reads 0
([`../../scan-stages-main-epyc-20261111/README.md`](../../scan-stages-main-epyc-20261111/README.md)).
An A/A in the same session bounds the noise of this shared host. The result
is for this host class only.

## Not in scope

No board row, no rule re-pricing, and no claim against rho. This is a
component measurement of the collect phase. A Table A effect would need
rule v10.
