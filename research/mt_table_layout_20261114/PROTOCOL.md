# Multi-target IC target phase: compact, batched, prefetched table (2026-11-14)

Frozen before any measured run. Draft and motivation: [`../relation_phase_survey_20261114/`](../relation_phase_survey_20261114/README.md).
In wall time the multi-target IC is faster than the best multi-target rho at 2^24–2^28 and slower at 2^32
(1.01–1.29×, `../mt_timed_20261112/`, `../mt_timed_runpod_20261112/`), while counted ratios are flat with size.
The timed study attributes the climb to cache-missing hash-table lookups. A cheaper hash was tried; the table
layout and access pattern were not.

## Instrument

`examples/mt_fieldops.rs`, new arm `ic5b` (this PR): `ic5` (k = 4, g = 1.2, μ = 1.0) with
- table mode `std` (the std `HashMap<u64, (u32, u16, i8)>` of `ic5`) or `flat` (an open-addressed array of
  8-byte slots: u32 x key and u32 index into an entry array; load factor at most 1/2; multiplicative hash;
  linear probing), and
- batch size B: targets processed in lock-step batches of B; each batch computes all its trial x-pairs, prefetches
  their probe slots (`flat` only), then probes in `ic5`'s order.

Every target's trial sequence is `ic5`'s. **Recovered logs and all counted operations (I, M, S, H, L, N) must equal
`ic5`'s exactly**; only the order of memory accesses differs. New output field: `ms_targets` (target-phase wall
time). One build (std hasher, default features) for every arm. The binary's SHA-256 is recorded on the pod.

## Arms (per configuration)

| key | arm | role |
|---|---|---|
| A | `ic5b std 1` | baseline (`ic5` itself) |
| Bs | `ic5b std 32` | batching alone (no layout change, no prefetch) |
| Bf1 | `ic5b flat 1` | layout alone |
| Bf8 | `ic5b flat 8` | layout + batch 8 + prefetch |
| **B** | `ic5b flat 32` | **the candidate (fixed before data)** |
| rho | `rho2xks` κ ∈ {0.05, 0.1, 0.25, 0.5}; `rhodpu` κ ∈ {0, 0.25} × offset ∈ {−1, 0, 1} | reference, as `../mt_timed_runpod_20261112/` |

## Configurations and order

Group orders about 2^24, 2^28, 2^32 (instances W{24,28,32}-{0..3} of `../mt_fieldops_v5_20261112/instances/`), T ∈ {1024,
4096, 16384}, seed 43: 36 configurations × 15 keys × 3 repeats = 1,620 executions. One process at a time, pinned to
one CPU, every key's repeats interleaved over the session in a fixed-seed shuffled order (`run.py`, as in the timed
studies). Host: one RunPod `cpu5c` pod (the host family of `../mt_timed_runpod_20261112/`); the pod is deleted when
the run ends and in any case 10 h after creation.

## Gates (time of a key = median of its 3 repeats; cell value = median over its 4 configurations; a cell decision is a
4-of-4 sign rule)

- **Correctness (stops the reading on failure):** for every configuration, each `ic5b` key recovers every target
  (`ok = targets`) and its counted totals I, M, S, H, L, N equal key A's.
- **Primary:** `ms_targets(B) / ms_targets(A)` ≤ 0.8 at 2^32 for T = 4096 and T = 16384.
- **Null:** `ms_targets(B) / ms_targets(A)` ≥ 0.95 at 2^32 for T = 4096 and 16384: the layout and access pattern are
  not the bottleneck.
- **Secondary (against rho):** `ms_arm(B) / min_rho ms_arm` ≤ 1 at 2^32 in at least 2 of the 3 T cells (min over the
  10 rho keys per configuration). This would reverse the size trend at the largest size measured; it is not a BSGS
  result.
- Reported, not gated: the same ratios at 2^24 and 2^28; Bs, Bf1 and Bf8 against A (decomposition into layout,
  batching and prefetch).

Prior (stated before data): primary met, 0.55; secondary met, 0.3; null, 0.15.

## Not claimed

No `S`, no single-target change, no BSGS result, no leaderboard or `benchmarks/` row. Wall time on a shared cloud
pod is read only as paired ratios within one binary.
