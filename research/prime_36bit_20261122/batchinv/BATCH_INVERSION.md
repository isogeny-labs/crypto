# Batched inversions in the 36-bit MITM relation probe

**Date:** 2026-11-22. **Status:** measured, `[pcc:unverified]`.

## Change

In `find_one_relation_s4_via_mitm_counted`, the word-sized probe previously called `add_point_u64` twice per factor-base point, so every trial paid `2B` extended-Euclid inversions. `R + F_k` and `R - F_k` share the denominator `x(F_k) - x(R)`. The probe now batch-inverts all `B` denominators with Montgomery's trick (one inverse plus `3B` multiplications) and derives both candidate abscissae from the same inverse. The `x(F_k)=x(R)` case falls back to the original addition.

## Correctness

Every hit is still finalized and re-added in the `BigUint` group by `try_finalise_s4`, and the ladder verifies `[x]G = Q`. All four 36-bit seeds recover the archived truth scalar:

| seed | recovered | trials | ic_matches_truth |
|--:|--:|--:|:--|
| 20261121 | 65040628637 | 7220 | True |
| 20261122 | 22987853180 | 7235 | True |
| 20261123 | 7964943479 | 7256 | True |
| 20261124 | 58792401953 | 7326 | True |

## Timing (36-bit, B=3072, sparse LA, `--no-rho`)

Archived before (same four seeds, `research/prime_36bit_20261122/a3_36bit_u64_seeds_b3072.jsonl`): 13.77-16.09 s CPU. After:

| seed | user s | sys s | wall s | relations ms | table ms | LA ms |
|--:|--:|--:|--:|--:|--:|--:|
| 20261121 | 8.47 | 0.21 | 14.64 | 8762 | 2080 | 2974 |
| 20261122 | 8.12 | 0.16 | 13.17 | 7902 | 2304 | 2705 |
| 20261123 | 7.85 | 0.13 | 9.42 | 4589 | 1230 | 3351 |
| 20261124 | 8.13 | 0.14 | 9.64 | 4940 | 1221 | 2856 |

Median CPU drops from **14.0 s to 8.1 s**, a **1.73x** end-to-end CPU-time improvement at the same B, seed set and output. Wall time on this host is load-sensitive (observed 9.4-14.6 s) and is not the primary comparison.

## Scope

This is a constant-factor relation-probe win, not a regime change. The tuned `cryptanalysis` rho still answers the same 36-bit targets in 16-23 ms, so IC remains ~4x10^2 slower. No `vs_rho`, deployed-curve or asymptotic claim is made.

## Artifacts

- `before.json`, `before.time` — pre-change seed 20261121 run.
- `after*.json`, `after*.time` — four post-change seeds.
- Code: `src/cryptanalysis/ec_index_calculus.rs` (`batch_inv_prime_u64` and the probe loop).
