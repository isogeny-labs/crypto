# Multi-target IC target phase: compact, batched, prefetched table (preregistration draft, 2026-11-14)

Status: **draft, not frozen**. It freezes when committed together with the implementation and before any measured
run.

## Hypothesis

The multi-target IC's wall-time disadvantage against rho at 2^32 (1.11–1.29×, `research/mt_timed_20261112/`) is
mostly memory latency on target-phase table lookups. A table that is smaller per entry and probed with overlapping
misses removes most of it.

## Arms (same binary, same instances, same seeds)

- **A (baseline):** `mt_fieldops` arm `ic5` as in `research/mt_fieldops_v5_20261112/` (std `HashMap`, one probe per
  trial, targets one at a time).
- **B (candidate):** the same arm with the target-phase table replaced by a flat open-addressed array of
  `(fingerprint: u32, slot: u32)`. A fingerprint match is confirmed against the full x before it counts. Trials are
  issued in lock-step batches of B targets (B ∈ {8, 32}, fixed before data), with each batch's probe addresses
  prefetched before use. Recovered logs must equal arm A's for every target (correctness gate).
- **Reference:** the `rhodpu` and `rho2xks` arms of the same harness, unchanged.

## Cells

Group orders ~2^24, 2^28, 2^32; T ∈ {1024, 4096, 16384}; the v5 instance set; interleaved order, pinned to one CPU,
3 repeats, as in `research/mt_timed_runpod_20261112/` (RunPod `cpu5c`, the host of the latest measurement).

## Gates

- **Correctness:** B recovers the same log as A for every target, and every target passes its check. Any mismatch stops
  the reading.
- **Primary gate:** B/A target-phase wall time ≤ 0.8 at 2^32 for T = 4096 and 16384 (median of
  configurations, 4-of-4 sign rule as in the timed studies).
- **Against rho (secondary):** B/rho ≤ 1 at 2^32 in at least 2 of the 3 T cells. That would reverse the size trend
  at the largest size measured. It is still not a BSGS result.
- **Null:** B/A ≥ 0.95 at 2^32: the layout is not the bottleneck.

## Not claimed

No change to the single-target picture or to the BSGS gap. Wall time on a shared pod is diagnostic, read as a
paired ratio.
