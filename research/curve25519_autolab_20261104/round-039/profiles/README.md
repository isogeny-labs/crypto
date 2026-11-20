# Round 39: where the wall time of `tw2l-x20-j16-n2` goes, and a walk-table change that removes most of it

Written 2026-11-19. Development evidence, not a validated result: every number below is [pcc:unverified], taken in this session on one host without a hardware PMU and without isolation, outside a frozen `ecbench` session.

## Question

Round 39 (`../README.md`) ratcheted `tw2l-x20-j16-n2` at k = 28 on counted GAE (sel / inc 0.944 [0.919, 0.970]) while its solve wall time in the same session was 1.2356x the incumbent's `wwalkGg-x09-sr-n2`. Where does that wall time go?

## Method

- Fixed workload: the first 160 non-warm-up holdout records of each arm (`ab/seqs.json`), exported with `ecbench profile-input` and run through the hidden measured child `ecbench exec`. The standalone `exec` reproduces the stored counted cost (job 642: `total_gae` 10664, as recorded).
- Profile: `tools/profile.sh cpu --call-graph dwarf` (perf `cpu-clock`, 999 Hz, software timer: where time goes, no cycle or cache attribution; `isolation: none`), 160 jobs x 10 repeats per arm. Outputs: `k28-sel-cpu/`, `k28-inc-cpu/`. Not committed: the 160 to 180 MB `perf.data` files, and the folded stacks (`folded.txt`, 1.6 MB each, sha256 `2d01ef87037789b0d01395ac27f1ae58001cab5720b9d559511314fe2ce91a72` for `sel` and `3c90811cf34fc3ddd2e4e1f73a314713dc1172bfdc50b7710a06391c4dde7a97` for `inc`) and `hotspots.tsv`, which the artifact policy sends to the store and this session has no store credentials for. `ab/analyze_folded.py` splits folded stacks by phase and leaf cost and `ab/folded_breakdown.jsonl` holds its output for these two runs; the tables below come from it.
- A/B: `ab/ab_bin.py` and `ab/ab_wall.py` run the arms interleaved (order alternating per job and round), read `report.solve_wall_ns` measured inside the child, and require every non-timing field of the report to be identical across variants and repeats.

## What the profile shows

Samples inside `solve_ic_prime` (`folded.txt`):

| | `sel` (tw2l) | `inc` (wwalkGg) |
|:--|--:|--:|
| samples inside the solve / all | 5753 / 11004 | 4214 / 9480 |
| `oracle_setup` (the walk table build) | 93% | 76% |
| `relations` | 1% | 16% |
| curve addition (`PrimeCurve::add_raw`) | 1787 samples (31%) | 1847 (44%) |
| page faults and memory charging (kernel) | 17% | 10% |
| hash-table insert (`hashbrown`) | 12% | 3% |

The solve of `sel` has 1.365x the samples of `inc`'s. Curve addition costs the same in both, so the extra time is the walk table: first-touch page faults on its memory and the inserts. The table was already pre-sized (`fast_map(t_max + 1)`), so it was not growth; each entry is a 48-byte tuple (`u128` forces 16-byte alignment) in a power-of-two bucket array, about 190 fresh pages per solve [pcc:unverified].

## Cheap test of the memory hypothesis (no source change)

`GLIBC_TUNABLES=glibc.malloc.hugetlb=1` (malloc asks for transparent huge pages), six interleaved rounds, with a no-op environment variable as the A/A control:

| | `sel` B/A | `inc` B/A | `sel`/`inc` solve wall |
|:--|--:|--:|--:|
| A/A control | 1.020 | 0.995 | 1.297 and 1.329 |
| hugetlb | 0.851 | 0.990 | 1.320 and 1.136 |

## The change

`src/cryptanalysis/ic_boundary.rs`: the walk table of `TargetWalk` becomes `TwTable`, an open-addressing table of 40-byte slots at load at most 3/4 in one zeroed allocation, with one `madvise(MADV_POPULATE_WRITE)` over tables of at least 256 KiB (any failure is ignored). It doubles if the size hint was too small. Insert-or-get order and the keys that collide are unchanged. Four new unit tests compare it with a `HashMap` (exact hint, hint far too small, above the prefault threshold, the keys 0 and `u64::MAX`); with the six existing `target_walk` and wwalk pipeline tests, `cargo test --release --lib -- tw_table target_walk` gives 10 passed.

## Result (`ab/ab_base_vs_new_5rounds.txt`)

Baseline: `ecbench` built from `main` at ad2a014c00 (sha256 1b86126f...). New: the same plus this change (sha256 38d7d1b0...). Five interleaved rounds, 800 runs per arm and binary:

| | baseline | new | new / baseline |
|:--|--:|--:|--:|
| `sel` mean solve wall | 3.150 ms | 2.329 ms | **0.739** |
| `inc` mean solve wall | 2.401 ms | 2.361 ms | 0.984 |
| `sel` / `inc` | 1.312 | 0.986 | |

The A/A control (the baseline against a copy of itself, five rounds): `sel` 0.9967, `inc` 1.0005. Non-timing report fields (counted GAE, phases, counters, recovered scalar) are identical across both binaries and all repeats for all 320 jobs, so the counted cost is unchanged.

## What this does and does not show

- It shows the wall-time cost of the `tw2l` arm at k = 28 is mostly its table memory, and that a smaller, prefaulted table removes most of it in this harness [pcc:unverified].
- It is not a frozen-protocol measurement: separate processes, no CPU pinning, no isolation, a host without a PMU, 160 workloads per arm. This harness gives `sel` / `inc` 1.31 for the baseline against 1.2356 in the round-039 session, so the absolute ratios do not transfer. The improvement needs a frozen, isolated `ecbench` session (an experiment, so an owner waiver or checkpoint token) before it is quoted as a speedup.
- The counted-GAE result of round 39 is untouched. This changes the implementation, not the method or its counted cost; it is no ratchet and no leaderboard row.
- Page-fault cost depends on the kernel and memory cgroup (this host is a VM). `MADV_POPULATE_WRITE` needs Linux 5.14 or later and is a no-op elsewhere.
