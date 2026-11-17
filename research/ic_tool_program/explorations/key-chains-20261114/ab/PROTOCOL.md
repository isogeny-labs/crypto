# Split min chains in the least-rotation kernel: same-session A/B with its own A/A (protocol)

Written on 2026-11-14, **before any row**. The owner asked to keep
optimizing the index calculus. Collection is about 80% of set-up at
2^47.2, and its key stage is about 5 ns per summand.

## The change

- **What:** `least_rotation16_funnel_avx512` in
  `src/cryptanalysis/koblitz_fast.rs` folds the odd and the even
  rotations into two minima a vector and merges them at the end, instead
  of one chain of `n` dependent minima a vector.
- **Answers:** the words are identical. The existing kernel test now also
  compares the old single-chain kernel, kept as
  `least_rotation16_funnel_one_chain_avx512`, on every word class.
- **Control:** `KIC_KEY_CHAINS=1` restores one chain.
- **Micro-benchmark** ([`../microbench/lr.rs`](../microbench/lr.rs),
  random words, 16 a call, a quiet run): n = 61, 3.89 → 3.48 ns per key;
  n = 53, 3.46 → 3.07.

## Arms: one binary, three labels

- **Binary:** `ic` built at this protocol's commit with `--features
  scan-probes`. Its SHA-256 goes in `runs/arms.sha256` before the first
  row.
- **Labels:**
  - **a**: `KIC_KEY_CHAINS=1`, today's kernel, the baseline;
  - **q**: the same as `a`, its A/A twin;
  - **k**: the default, the split chains.

## Workload, schedule, metrics and statistics

The same as the fold-pack A/B
([`../../fold-pack-20261114/ab/PROTOCOL.md`](../../fold-pack-20261114/ab/PROTOCOL.md)):
- **Rows:** `M1-T01` at `k0n53` and `k0n61`.
- **Isolation:** `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1`.
- **Schedule:** 42 rounds cycled over the six orders of `a q k`.
- **Metrics:** per-process medians of build, collect and set-up.
- **Statistics:** per-round A/B `a / k` and A/A `a / q`; the median with a
  95% percentile bootstrap over rounds (2,000 resamples, exact LCG step,
  high-bit index, seed 20261115).
- **Dropped rounds:** a contended or failed round is dropped and counted.

## Decision rule (applied once, to all rounds)

- **Established:** a gain is established if the A/B lower end is above 1
  and above the A/A upper end.
- **Regression:** a build regression is established if the A/B upper end
  is below 1 and below the A/A lower end.
- **Kept:** the split stays the default if the **collect** gain is
  established on at least one row and no build regression is established.
- **Otherwise:** the default reverts to one chain, the switch stays, and
  the reason is recorded.

## Prediction (before any row)

Medians `a / k`:

| row | collect | build | set-up |
|:--|:--|:--|:--|
| k0n53 | [1.00, 1.05] | [1.00, 1.03] | [1.00, 1.04] |
| k0n61 | [1.00, 1.05] | [1.00, 1.03] | [1.00, 1.05] |

About 0.4 ns of a 15 ns scanned summand. This may not clear this host's
A/A, about ±2–3%, and a null result is a likely outcome.

## Identity

Every process of a row must report the same certificates and `counts` as
that row's first `a` process. Any mismatch voids the session.

## Route

This session's container (Intel Xeon @ 2.1 GHz, AVX-512). The Iso Lab
bridge cannot submit while its CodeBuild quota reads 0. The result holds
for this host class only.
