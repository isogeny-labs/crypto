---
title: "rootfind: cheaper table lookups (packed hash slots, paired lookups, Eytzinger search)"
date: 2026-11-14
---

# rootfind: cheaper table lookups

User request (2026-11-14): "Yes, try the rootfind lookup improvements", after the phase profile of
`research/ic_gpu_replay_20261114` put 41–85% of rootfind's sampled cycles in table probes.

**Standing.** `[pcc:unverified]`: one A100 pod, measured by the session that wrote the code;
an independent validator recomputed every headline number from the raw tarball
(`VALIDATION_REVIEW.md`: validated, scoped; four wording corrections, applied here). This is a kernel-component speedup of the decomposition oracle with identical
relations; it is not an end-to-end IC solve, makes no counted-cost or rho claim, and adds no
scoreboard or leaderboard row.

![Speedup at each arm's best configuration, [pcc:unverified]](fig_lookup.svg)

## Changes (`gpu/rootfind`; measured at `c308625f57`, whose `gpu/` tree equals commit `1e757ab551`'s; defaults switched on after the measurement)

Every variant returns the same table candidates in the same order, or extra candidates that the
sweep's existing full-x comparison (`F::eq(x3, fb_x[j])`) discards, so the relation set and its
emission order are unchanged.

- **`RF_HASH_PACKED`**: one u64 per hash slot, `(low 32 bits of the key) << 32 | index`, instead of
  `keys[]` and `vals[]`: one load per probe step, same slots and probe order. A slot whose tag matches
  but whose key differs becomes a candidate and is dropped by the full-x check. The shared-memory
  staging for small factor bases holds 8 bytes per slot instead of 12.
- **`RF_PROBE_PAIR`**: the two ε lookups of a factor-base entry start together (`begin2`): for the
  sorted table one loop runs both binary searches as independent chains; for the packed hash both
  first-slot loads are issued before either is examined. The walks then run ε = +1 first, as before.
- **`RF_SORTED_EYT`**: the sorted search descends an Eytzinger (BFS-order) copy of the keys and maps the
  result back to the sorted position through a rank array; the scan over equal keys is unchanged.

**A bug found by the host tests.** The first paired Eytzinger search assumed both descents leave the
tree at the same level; with a partial bottom level (n ≠ 2^h − 1) they can differ by one, and it read
past the array. All flags on gave 11 failures in the existing oracle test; each chain now advances only
while inside the tree. A new test `test_lookup_primitives` (`test_cpu.cpp`) checks `begin` against
`std::lower_bound` and `begin2` against two `begin` calls on table sizes 1–400 and keys below, inside and
above the range, and that the packed hash returns every stored index (with deliberate tag collisions):
47 618 checks per flag set, passing under every flag set; it fails with 3 610 mismatches on the buggy
version (positive control). The oracle suite (`make test`) and the Rust wrapper crate's tests
(`gpu/rootfind/rs`, 6 tests against its CPU reference) pass with the new defaults.

## Run

One A100 80GB PCIe pod (`n68o04pli1i8oq`, Runpod host `id20nfc1q14a`), CUDA 12.8, 18:55Z–19:23Z
2026-11-14, about $1.00, deleted. Eight builds of the same tree (`c308625f57`; its `gpu/` is identical to `1e757ab551`) with the flags set per
arm: main's lookups twice (M and M2, the A/A control), each variant alone, Eytzinger + paired, packed +
paired, and all three. Every arm passed its host tests and the three device selftests (Fp64 at
\|F\| = 1024 and 4096, every (lookup, chunk) configuration against the host emulation; P-256 positive
controls) before it was timed. Five rounds of `./bench throughput` at two workloads, arm order rotated
each round, `CUDA_MODULE_LOADING=EAGER`. `job.sh`; raw `raw/out_lk.tgz`; full tables `RESULTS.md`.

## Results

A/A: main and its second build differ by at most 0.06% at any configuration; one arm's five rounds by
at most 0.36%.

Speedup at each workload's best configuration (main's best kernel time ÷ the arm's best) [pcc:unverified]:

| arm | \|F\| = 65536, 65536 targets | \|F\| = 1024, 262144 targets |
|---|--:|--:|
| main | sorted/C16 401.70 ms | hash/C16 19.87 ms |
| Eytzinger | 1.050× | 1.000× |
| packed hash | 1.000× | 1.112× |
| paired | 0.999× | 1.019× |
| Eytzinger + paired | 1.116× | 1.019× |
| packed + paired | 1.000× | 1.120× |
| **all three** | **1.115×** (sorted/C16 360.19 ms) | **1.121×** (hash/C16 17.73 ms) |

Per configuration, all three against main (ratio of kernel times, lower is better):

| \|F\| | hash C16 / C32 / C64 | sorted C16 / C32 / C64 |
|---|---|---|
| 65536 | 0.701 / 0.663 / 0.646 | 0.897 / 0.744 / 0.766 |
| 1024 | 0.892 / 0.826 / 0.867 | 0.914 / 0.782 / 0.812 |

All three is faster than main at all 12 configurations and within 0.1% of the best arm at each
[pcc:unverified]. What each change buys, as the profile predicted:

- **Packed slots** cut the global hash path by about a third (0.67–0.71× alone at \|F\| = 65536, 0.65 with pairing) and the
  shared-memory one by 10–15%.
- **Paired lookups** cut the sorted search by 15–20% at chunks 32 and 64 on their own, and together
  with Eytzinger by 23–26% at \|F\| = 65536 and 19–22% at \|F\| = 1024. On the split-array hash at \|F\| = 65536 they are slightly slower (1.02–1.03×); at
  \|F\| = 1024 they speed it up by 1.5–3.3%. They ship together with packed slots.
- **Eytzinger** takes 5% off the sorted search alone (2.6–3% at \|F\| = 1024) and 6–10% more on top
  of pairing at \|F\| = 65536.

The hash path at \|F\| = 65536 is still slower than the sorted one (404 against 360 ms at their
best chunks, C16 both), so the best configuration per workload stays as before (sorted at large
\|F\|, hash at small); the gain at that configuration is what counts.

## Defaults

`RF_SORTED_EYT`, `RF_HASH_PACKED` and `RF_PROBE_PAIR` now default to 1 (`rootfind.cuh`, `Makefile`);
the default build is the "all three" arm measured above. Setting a flag to 0 restores the old path.

## Not done / next

- The decomposition oracle's end-to-end effect: rootfind is not yet called from an IC run in this
  repository's measured pipelines, so no relation-phase number exists.
- Further: the hash path at large \|F\| (still 1.12× the sorted path at their best chunks); a sorted
  search with software prefetch of the next Eytzinger level.
