# Lazy point-index map in the collector and the descent solver

- **Status:** measured and kept, 2026-11-15, on this session's container.

## Why

Temporary sub-step timers priced `RelationCollector` construction at
8–13 ms of collect-setup's 10–17 ms:
- `FrobeniusFactorBase::shared_index_map` took 8.1 ms at k0n53: a
  `HashMap<(BigUint, BigUint), usize>` over every point;
- `m_can_decompose` took 2.0 ms;
- `FieldStructure::new` took 0.3 ms.

The collector and `IndividualLogSolver` read the map only on their
decomposition paths without a pair table.

## The change (`src/cryptanalysis/koblitz_index_calculus.rs`)

- **`LazyIndexMap`:** the map is built on first use behind a `OnceLock`,
  the same map wherever it is read.
- **Control:** `KIC_LAZY_INDEX=0`.
- **Tests:** 217 Koblitz tests pass, and the index-calculus tests also
  pass with `KIC_LAZY_INDEX=0`. Clippy is clean.

## A/B ([`ab/PROTOCOL.md`](ab/PROTOCOL.md), frozen before any row; [`ab/report/report.pdf`](ab/report/report.pdf))

24 rounds, one binary, a same-session A/A. Ratios are eager over lazy.

| row | collect-setup | descent-setup | set-up |
|:--|:--|:--|:--|
| k0n53 | **2.80** [2.70, 2.90], established (10.4 → 3.7 ms) | **1.49** [1.45, 1.55], established | 1.034, not established |
| k0n61 | **3.03** [2.83, 3.09], established (15.6 → 5.2 ms) | **1.60** [1.55, 1.62], established | 1.014, not established |

- **Identity:** zero mismatches.
- **Exploration:** 3 rounds an arm, in [`explore/`](explore/).
- **Scope:** a component result on this host class. It makes no claim
  against rho and adds no board row.
