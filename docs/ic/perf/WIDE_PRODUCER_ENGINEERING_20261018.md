# Wide producer engineering: probe path (T02) and index build (T01)

Tasks T01 and T02 of `RESEARCH_IC_LEADS_ASSESSMENT_20261018.md`, in
`examples/koblitz_orbit_dlp_fast.rs` (module `wide`, `64 < n <= 127`). Input:
the profile `docs/ic/perf/WIDE_PRODUCER_PROFILE_20261018.md`.

**Class: engineering** (same algorithm, same probes, same relations; only the
wall time per unit of work changes). No end-to-end `S` claim, no rho
comparison, no leaderboard change. All times are wall time from an
**unisolated, shared 4-vCPU KVM guest** (Xeon 2.80 GHz, L2 4 MiB per core,
L3 33 MiB shared, no PMU; other sessions were building and running on the same
guest, load average 3 to 6 during the runs). Records:
`benchmarks/20261018-wide-producer-engineering/`.

## What the design already did (read first)

- The table is keyed by the canonical form (the minimum over the `n`
  rotations of the normal-basis word) of each stored root, with a Bloom filter
  on the same canonical key. A probe therefore canonicalised its partner root
  **on every probe**, before the Bloom test, with the 72-step rotate/compare
  loop (`NormalBasis128::canonical`, profile: 41.3% of the scan, 1,397 Ir per
  probe). There is no rotation-invariant key and no table of all rotations in
  the file.
- An exact cheaper canonicaliser existed behind `KIC_SPARSE_CANONICAL=1`
  (`canonical_sparse`, zero-gap method, returns key and shift). It was not
  profiled and is untouched here.
- A Bloom filter (3 hashes, 8 bits per key, hashes spread over the whole
  filter) was already on by default.
- Correction to the task statement: the 18,398 s of the landed n = 73,
  K = 600 run is the **rank-stage wall** (profile report, "Instrumentation").
  The serial index build is **66 s** at n = 73, K = 600 on this guest (median
  of 2, range 61 to 72 s; table below), not 18,398 s. T01 is therefore a
  tens-of-seconds saving per process, not hours.

## Changes (each behind its own switch; same binary is the control)

| switch | default | what it does |
|:--|:--|:--|
| `KIC_LAZY_CANON` | on (`=0` restores the old path) | Probe computes only the canonical **key** with `canonical_key_lazy` (early-exit doubling ladder over zero runs, then refinement below the first empty level; about 4 rotations for a random word instead of 72 steps), consults Bloom and table, and computes the shift (`canonical_selected`, unchanged) only for a probe that hits. Result equals `canonical(v).0` for every word. |
| `KIC_PREFETCH` | on (`=0` restores the one-pass scan) | `scan_state128` runs in two passes per state: pass 1 computes each partner's key and Bloom hashes in the sequential probe order and issues `_mm_prefetch` (T0) of the Bloom line (the table slot when the filter is off); pass 2 does the lookups in the same order. `probes` at a hit is unchanged. |
| `KIC_BLOCKED_BLOOM` | off | Bloom bits of a key all fall in one 64-byte block (block from the first hash, 9 bits per hash from the second). Same bits per key; slightly higher false-positive rate. Filter storage is 64-byte aligned `BloomBlock`s in both modes (bit addressing of the plain mode is unchanged). |
| `KIC_INDEX_THREADS=N` | 1 (serial) | `build_index128`: states for each `left` row, and the canonical keys, computed on an `N`-thread rayon pool (rayon is already a dependency); results are concatenated in serial order and the table is filled serially in order (first witness wins), with a slot prefetch 8 states ahead. |
| `KIC_INDEX_BATCH_INV=1` | off | Root solves in chunks of 256 share one inversion (`RootBatch128::solve`, Montgomery's trick; equal to `S3Solver128::roots` pair for pair, including exceptional inputs, by the existing test). |
| `KIC_VERIFY_INDEX=1` | off | After a non-serial build, rebuilds serially and aborts unless states, table slots, table length and shift, Bloom words and shifted rows are identical (`index_difference`). |
| `KIC_INDEX_TIMING=1` | off | One stderr line with the phases of the build. |
| `KIC_PROF_MAX_STATES=S` | off | Exit after `S` scanned states with `PROF states probes hits relations index_states scan_s ns_per_probe`. A cleaned-up form of `WIDE_PRODUCER_PROFILE_COUNTER_PATCH_20261018.patch`; without the variable the cost is one relaxed atomic add per state. `scan_s` runs from the first scanned state to the exit. |

Defaults: `KIC_LAZY_CANON` and `KIC_PREFETCH` are turned on because they give
a measured gain with identical output; `KIC_BLOCKED_BLOOM` stays off (no gain
that survives the other two switches, below). The index switches stay off (the
default build is the serial loop) because the right thread count depends on
what else runs on the host.

## Commands

```
export CARGO_TARGET_DIR=/home/user/wt/target-t03 CARGO_PROFILE_RELEASE_DEBUG=1
cargo build --release --example koblitz_orbit_dlp_fast
B=$CARGO_TARGET_DIR/release/examples/koblitz_orbit_dlp_fast
cd benchmarks/20261018-wide-producer-engineering
# scan: 5 arms x 5 repeats, interleaved with a rotated arm order per repeat
python3 run_ab.py scan --binary $B --spec construct:73:1:40  --states 40000 --repeats 5 --json-out scan_k40.json
python3 run_ab.py scan --binary $B --spec construct:73:1:300 --states 20000 --repeats 5 --json-out scan_k300.json
python3 run_ab.py scan --binary $B --spec construct:73:1:300 --states 20000 --repeats 5 \
   --only control,prefetch_blocked,lazy_prefetch,all_three --json-out scan_k300_pairs.json
python3 run_ab.py scan --binary $B --spec construct:73:1:600 --states 20000 --repeats 5 \
   --only control,lazy_canon,prefetch,all_three \
   --extra-env '{"KIC_INDEX_THREADS":"4","KIC_INDEX_BATCH_INV":"1"}' --json-out scan_k600.json
# index build: 4 arms x 3 repeats (K=300), 2 arms x 2 repeats (K=600)
python3 run_ab.py index --binary $B --spec construct:73:1:300 --repeats 3 --threads 4 --json-out index_k300.json
python3 run_ab.py index --binary $B --spec construct:73:1:600 --repeats 2 --threads 4 \
   --only serial,threads_batch_inv --json-out index_k600.json
# equality and end-to-end
KIC_VERIFY_INDEX=1 KIC_INDEX_THREADS=4 KIC_INDEX_BATCH_INV=1 KIC_PROF_MAX_STATES=1 $B construct:73:1:150 empty.txt 1 out.jsonl
python3 e2e_compare.py $B construct:69:0:60
cargo test --release --example koblitz_orbit_dlp_fast
rustfmt --edition 2021 --check examples/koblitz_orbit_dlp_fast.rs
python3 make_graph.py
```

Each arm is one process: `KIC_PROF_MAX_STATES=S $B construct:73:1:K empty.txt 1
out.jsonl` (constructed base, a = 1, empty target file, seed 1; the rank
stage scans states in order, the process exits after `S`). Control arm =
`KIC_LAZY_CANON=0 KIC_PREFETCH=0` and no other switch (K = 40 and K = 300
runs were taken when these were still off by default, so their control had no
environment at all; the code path is the same). `ns per probe` is
`scan_s` / probes, so setup (the 67% `factorise` of the profile) and the index
build are excluded.

## Results: scan, wall ns per probe at n = 73

Median over 5 interleaved repeats, min-max in brackets, ratio to the control
median. **Hits are 0 in every timed scan** (11.68 M probes at K = 40, 5.84 M
at K = 300 and K = 600; 292 probes per state, identical across arms), so the
timed runs never exercised the hit path; the hit path is covered in the
correctness section.

| arm | K = 40 (index 116,800 states, table about 17 MiB, Bloom 256 KiB) | K = 300 (6.57 M states, table about 1 GiB, Bloom 16 MiB) | K = 600 (26.28 M states, table about 4.3 GiB, Bloom 64 MiB) |
|:--|--:|--:|--:|
| control | 417.0 [414.3-424.7] | 614.5 [559.3-684.6] | 621.6 [583.1-653.5] |
| `KIC_LAZY_CANON` | 209.9 [203.0-223.1] x0.503 | 367.0 [349.3-416.9] x0.597 | 382.7 [372.0-433.1] x0.616 |
| `KIC_BLOCKED_BLOOM` | 433.0 [417.7-483.9] x1.038 | 576.0 [525.8-598.7] x0.937 | not run |
| `KIC_PREFETCH` | 401.7 [375.0-456.5] x0.963 | 434.7 [386.8-485.8] x0.708 | 441.6 [422.2-459.5] x0.710 |
| all three | 217.0 [204.5-245.0] x0.520 | 225.6 [216.7-250.5] x0.367 | 235.0 [223.4-303.0] x0.378 |

Second K = 300 run, pair arms (separate interleaved set, 5 repeats; its own
control): control 573.6 [558.2-660.0]; lazy + prefetch 232.8 [225.9-272.3]
x0.406; blocked + prefetch 431.6 [407.5-550.7] x0.752; all three 226.7
[207.7-309.6] x0.395.

Reading, with the caveats below:

- **Lazy canonicalisation is the large, robust gain**: about 2x at K = 40 where
  the table is cache-resident (the profile's Ir share of the old loop was
  41.3%; the wall saving is 50%, a difference this host cannot attribute, no
  PMU), and 0.60 to 0.62 at K = 300 and 600, where the unchanged memory
  stalls are a larger share of what remains.
- **Prefetch**: no gain at K = 40 (0.963, ranges overlap), 0.71 at K = 300
  and 600, where the Bloom filter (16 and 64 MiB) and the table are larger
  than the L2 and mostly L3 or DRAM resident. Prefetch alone prefetches only
  the line of hash 0; the other two Bloom lines of the plain filter are not
  prefetched.
- **Lazy and prefetch compose**: 0.37 to 0.38 together, better than the
  product of the single gains (0.60 x 0.71 = 0.43); the cause is not isolated.
- **Blocked Bloom gives nothing that survives**: 1.04 at K = 40, 0.94 at
  K = 300 alone, and no measurable difference on top of lazy + prefetch
  (all three 226.7 against lazy + prefetch 232.8 in the same set; ranges
  overlap). It stays off by default. It was not run at K = 600.
- Control cost per probe is 417 ns at K = 40 and 574 to 622 ns at K = 300
  and 600: the extra 160 to 200 ns is the memory behaviour of the large
  filter and table that the profile (Ir only) could not see. The all-three
  arm removes it: 217 ns at K = 40 against 226 to 235 ns at K = 300 and 600.
  Per-probe cost is now nearly independent of K, so the remaining cost is
  arithmetic: root solve (39.1% of the old scan) and the two linear maps
  (13.7%), both untouched.

## Results: index build, wall seconds at n = 73

Phases from `KIC_INDEX_TIMING`. For the serial rows the `table` column is
table allocation only and canonicalisation is inside `fill`; for the parallel
rows `table` is allocation plus the parallel canonical keys.

K = 300 (6.57 M states; median of 3, range in brackets):

| arm | states | table + keys | fill | total | ratio |
|:--|--:|--:|--:|--:|--:|
| serial (control) | 4.77 | 0.57 | 9.33 | 14.60 [14.39-15.81] | 1.000 |
| batch inversion, 1 thread | 2.45 | 3.66 | 2.40 | 8.51 [8.21-12.45] | 0.583 |
| 4 threads | 1.96 | 1.64 | 2.27 | 5.83 [5.79-6.40] | 0.399 |
| 4 threads + batch inversion | 1.13 | 1.68 | 2.75 | 5.56 [4.85-5.86] | 0.381 |

K = 600 (26.28 M states; median of 2, range in brackets):

| arm | states | table + keys | fill | total | ratio |
|:--|--:|--:|--:|--:|--:|
| serial (control) | 19.58 | 9.21 | 37.59 | 66.38 [60.61-72.14] | 1.000 |
| 4 threads + batch inversion | 6.08 | 11.86 | 11.19 | 29.13 [28.59-29.66] | 0.439 |

Batch inversion alone halves the state generation (1.0 s to 0.56 s at K = 150
in a separate check) and, with the parallel keys, the build is bounded by the
serial in-order table fill and by the page-faulting of the 4.3 GiB table, not
by arithmetic. With 4 threads on a guest that other sessions were using, the
build is 2.3x (K = 600) to 2.6x (K = 300) faster; threads add little over
batch inversion alone once the fill dominates. A parallel fill (partitioned by
slot range) is the next step if the build matters at larger K; at 29 s of a
process that spends hours in the rank stage it does not move the total.

## Correctness

- **Index equality** (the requirement for T01): `KIC_VERIFY_INDEX=1` at
  n = 73, K = 150 with `KIC_INDEX_THREADS=4 KIC_INDEX_BATCH_INV=1`:
  `VERIFY_INDEX identical states=1642500 table_entries=1642050` (states, table
  slots, table length, Bloom words and shifted rows equal to the serial
  rebuild; 450 duplicate keys were resolved first-wins identically). Also
  identical for `KIC_INDEX_THREADS=3` alone and `KIC_INDEX_BATCH_INV=1`
  alone. In-process: `wide::engineering_tests::parallel_batched_index_equals_serial_build`
  (n = 73 and the symmetric n = 83 layout, K = 7, arms 1/3/4 threads with and
  without batching) asserts `index_difference == None`.
- **Unit tests**: `cargo test --release --example koblitz_orbit_dlp_fast`,
  28 passed, 0 failed (22 existing, 6 new). New: lazy key equals `canonical`
  on every word for n <= 13 and on 60,000 sampled, thinned and periodic words
  at n in {65, 71, 73, 83, 97, 127};
  Bloom no false negatives for all four profiles, plain and blocked, and
  prehashed calls agree; screened table lookup equals the plain lookup;
  the scan probe sequence (per-state probe counts) and results are identical
  across lazy/prefetch on/off for a crafted target that reaches the table-hit
  path (asserted through the hit counter). The example's own startup
  self-check (2048 normal-basis and S3 root checks) runs in every execution.
- **End-to-end at a small wide n** (the timed n = 73 runs have no hits):
  `python3 e2e_compare.py $B construct:69:0:60` (n = 69, a = 0, K = 60,
  seed 1, 8 known-answer targets) runs the full producer (rank stage to full
  rank, then targets) for six arms. All six are identical to the control in
  every non-timing field: `rank_relations` 60, `rank_failures`,
  `rank_attempts`, `rank_rows_without_gain`, `rank_probes_mean` 8034.2, `targets_solved` 8 (scalars recovered, group
  verified), per-target probes `[187, 2609, 883, 8912, 4321, 10828, 1460,
  7980]`, `point_indices`, `x_codes` and `pinned_intermediates` of every
  target. The last arm turns on every switch including the parallel
  batched index and `KIC_VERIFY_INDEX` (`identical`). Record:
  `e2e_n69_k60.txt`. This exercises the hit path (lift) of both scan variants.

## Limits

- Unisolated shared guest, load 3 to 6 on 4 vCPUs, no PMU: absolute ns and
  seconds are indicative; the ratios are same-binary, interleaved, and the
  spreads are reported. The K = 600 index rows are 2 repeats, the K = 300
  rows 3. A/A control not run (the arms of one set are interleaved in a
  rotated order so slow drift falls on all arms).
- Scan timings cover 20,000 to 40,000 states from the start of the state
  order, not the whole index; hits are 0, so relation-check (lift) time is not
  in them (the lift runs once per hit, about 1 per 5.7e7 probes at the landed
  shape).
- `scan_s` is wall time of the single scanning thread; the target-stage
  parallel scan (`extract128_parallel`) uses the same `scan_state128` and
  inherits both switches but was not timed.
- The timed scans use an index built by the same code in every arm. The K = 600
  scan arms built it with `KIC_INDEX_THREADS=4 KIC_INDEX_BATCH_INV=1` (index
  proven identical to the serial build at K = 150 and in the unit test).
- No leaderboard change, no `S`, no rho comparison: this changes the wall
  time of a stage (the probe loop of the rank and target scans) and of index
  setup; end-to-end `S` and counted-operation costs are unchanged (probe
  counts are identical by the e2e check).

## Next

1. The root solve (`RootBatch128::solve`, 39.1% of the old scan by Ir, 5.5
   field multiplications per probe) and the two byte-table linear maps
   (13.7%) are untouched and are now the larger part of the 217 to 235 ns
   per probe.
2. Re-profile with `callgrind` on the new default path to give the Ir split of
   the remaining scan.
