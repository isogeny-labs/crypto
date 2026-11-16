# Lazy point-index map in the collector and the descent solver: same-session A/B with its own A/A (protocol)

Written on 2026-11-15, **before any row**. The owner asked to keep
optimizing the index calculus.

## Why

Sub-step timers on current main (temporary, not committed) priced
`RelationCollector` construction at 8–13 ms of collect-setup's 10–17 ms:
- `FrobeniusFactorBase::shared_index_map` took 8.1 ms at k0n53, a
  `HashMap<(BigUint, BigUint), usize>` over every point;
- `m_can_decompose` took 2.0 ms.

The collector and `IndividualLogSolver` read that map only on their
decomposition paths without a pair table (`decompose_once`). The aimed
pair-table collection and descent never read it.

## The change

- **`LazyIndexMap`:** the map is built on first use behind a `OnceLock`.
  The same map is used wherever it is read.
- **Control:** `KIC_LAZY_INDEX=0` builds it at construction, as before.
- **Answers:** unchanged; nothing reads the map before its first use. The
  exploration ([`../explore/`](../explore/), 3 rounds an arm) had identical
  certificates and counts.
- **Exploration figures (ms):** collect-setup 9.8–13.2 → 3.7–5.0 at k0n53
  and 15.5–19.8 → 5.1–5.8 at k0n61; descent-setup 1.5–1.6 → 1.0–1.2 and
  2.1 → 1.3–1.4.

## Arms: one binary, three labels

- **Binary:** `ic` built at this protocol's commit with `--features
  scan-probes`. Its SHA-256 goes in `runs/arms.sha256` before the first
  row.
- **Labels:**
  - **a**: `KIC_LAZY_INDEX=0`, the baseline;
  - **q**: the same as `a`, its A/A twin;
  - **l**: the default.

## Workload, schedule, metrics and statistics

- **Rows:** `M1-T01` at `k0n53` and `k0n61`.
- **Isolation:** `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1`.
- **Schedule:** **24 rounds**, cycled over the six orders of `a q l` (four
  cycles). The effect is large next to the A/A seen so far.
- **Metrics:** per-process medians of **collect-setup**
  (`setup_phases_ns.collect_setup`), descent-setup and set-up.
- **Statistics:** per-round A/B `a / l` and A/A `a / q`; the median with a
  95% percentile bootstrap over rounds (2,000 resamples, exact LCG step,
  high-bit index, seed 20261117).
- **Dropped rounds:** a contended or failed round is dropped and counted.

## Decision rule (applied once, to all rounds)

- **Established:** a gain is established if the A/B lower end is above 1
  and above the A/A upper end.
- **Regression:** a set-up regression is established if the A/B upper end
  is below 1 and below the A/A lower end.
- **Kept:** lazy stays the default if the **collect-setup** gain is
  established on at least one row and no set-up regression is
  established.
- **Otherwise:** the default reverts to eager, the switch stays, and the
  reason is recorded.

## Prediction (before any row)

Medians `a / l`:

| row | collect-setup | descent-setup | set-up |
|:--|:--|:--|:--|
| k0n53 | [2.0, 3.5] | [1.1, 1.8] | [1.00, 1.05] |
| k0n61 | [2.5, 4.0] | [1.2, 1.9] | [1.00, 1.03] |

## Identity

Every process of a row must report the same certificates and `counts` as
that row's first `a` process. Any mismatch voids the session.

## Route

This session's container (Intel Xeon @ 2.1 GHz, AVX-512). The Iso Lab
bridge cannot submit while its CodeBuild quota reads 0. The result holds
for this host class only.
