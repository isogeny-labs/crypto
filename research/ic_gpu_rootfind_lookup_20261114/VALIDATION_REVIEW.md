# Validation review: rootfind table-lookup variants (`research/ic_gpu_rootfind_lookup_20261114`)

Validator: independent session (`validator` role, Opus 5.5), 2026-11-14. Producer: another session.
Every number below is my own recomputation from the raw tarball or my own host code. I did not
use the producer's `analyze.py` or `make_figure.py` (I read them only to see what they claim to compute).

**Disposition: `validated`, scoped.** The validated claim is a kernel-time speedup of the rootfind
decomposition kernel on one A100 80GB PCIe (sm_80, CUDA 12.8), for Fp64 (61-bit p) at the two
measured workloads. Relation identity rests on device selftests at |F| = 1024 and 4096 (Fp64) and
|F| = 256 (P-256), plus my independent host equivalence check. Every headline number recomputes
exactly. No number is unsupported by the artifacts. Four prose statements are imprecise or
over-general (section 4). The claim is not end-to-end, makes no counted-cost or rho statement, and
covers no other GPU. The report does not claim those either.

## 0. What was validated (frozen bindings)

| item | value |
|---|---|
| validated from | `main` at `a95274095fec4846dd1b8b0bb26525d778398fd3`, committed objects only |
| measured tree (raw `REV`) | `c308625f570fbf90cf5d5b25a0622ba542bda6f5` |
| code commit named in report | `1e757ab5511292beed907122b1f308c9ce798750` (`git diff 1e757ab551 c308625f57 -- gpu` is empty; c308625f57 adds only `RESUME.md` and the open bench record) |
| default switch | `e28940f72a`: `gpu/rootfind/Makefile` and `rootfind.cuh` only; changes defaults and comments, not code paths |
| `raw/out_lk.tgz` | sha256 `9ad094d30ea977fe367b8933dde9b613b82381b736c2fbacad36e83d302772ef`, matching `raw/SHA256SUMS` and the bench record |
| `REPORT.md` | sha256 `9692c2bb9748f17b2152480a492314ab669702291bea16298fd8cd0461a648a0` |
| `RESULTS.md` / `results.json` | `0a8f2a91…f609d7` / `aec8c289…916184` |
| `benchmarks/records/20261114T1855Z-ic-gpu-rootfind-lookup.json` | `c996e8cf…b67d37c` |

The evidence files listed above match their blobs at HEAD. During this review, another session
left **uncommitted** edits in the shared working tree:
- modified: `gpu/rootfind/Makefile`, `rootfind_api.cpp`, `rootfind_api.h`, `rootfind_cuda.cu`, `.gitignore`
- untracked: `rootfind_serve.cpp`

Those edits are not part of this validation. All code reading, host tests and harness builds used
`git archive a95274095f gpu/rootfind` (and `c308625f57` for the measured tree), not the working tree.

## 1. Raw data, builds, tests, rounds

| requirement | result | evidence (inside `raw/out_lk.tgz` unless noted) |
|---|---|---|
| tarball hash | pass | `sha256sum raw/out_lk.tgz` = the recorded value |
| `REV` | pass, with a naming note | `out_lk/REV` = c308625f57, which is code-identical to 1e757ab551 (see section 0) |
| -D flags match arm definitions | pass | every `nvcc` line in `build-<arm>.log` (2 per arm) carries exactly M/M2 000, E 100, H 010, P 001, EP 101, HP 011, EHP 111 (EYT PACKED PAIR); all builds `rc=0`, `code=sm_80` |
| M and M2 are the same code | pass | `build-M.log` and `build-M2.log` differ only in ptxas compile times (same registers, spills, stack). The binary sha256s differ (`sha256.txt`), plausibly from build paths embedded by `-lineinfo`; cannot check without nvcc |
| host tests per arm | pass | every `test-<arm>.log`: `ALL PASS`, `rc=0`, compiled with the arm's -D flags; `lookup primitives (EYT=a PACKED=b PAIR=c): 47618 checks` with the arm's flags |
| three device selftests per arm | pass | every `selftest-<arm>.log`: 3 blocks passed, 16/16 configurations OK; device relation counts equal the emulation's (1024, 1024, 1024) |
| selftests before timing | pass | `progress.log`: selftests 19:18:31–19:18:45, before `timed arms` and round 1 |
| all arms timed in all 5 rounds | pass | 80 files (`tp-*`, `tpsmall-*`), each with 6 `cuda <lookup> chunk=<n>` rows and the right workload header (61-bit, 65536/65536 and 1024/262144); every file names the A100 |
| order rotated each round | pass | tar member mtimes give the actual execution order: round r = `M E H P EP HP EHP M2` cyclically shifted by r−1, exactly as `job.sh` and `progress.log` state |
| environment | pass | nvcc 12.8.93; driver 595.91.07; A100 80GB PCIe; MIG off; no other processes; persistence on; ECC on (`nvcc.txt`, `nvidia-smi-q.txt`) |
| preregistration | pass | `RESUME.md` at c308625f57 (18:55:42Z, before the 18:56:24Z launch) fixes the arms, selftests-first, 5 rotated rounds, both workloads, the success rule ("outside the M/M2 spread and at the best configuration per workload") and the exclusion rule. `job.sh` was committed in 1e757ab551 and is unchanged since |

**A caveat on the selftest reference (it strengthens the claim).** At the measured tree,
`rootfind_api_cuda.o` is compiled without `$(RF_DEFS)`: Makefile lines 64–65 at a95274095f, and
line 1 of every `build-*.log`. The host emulation used as the selftest reference therefore ran
with the header defaults of c308625f57, which are 0/0/0 (the old lookups), in every arm. Each arm's
device kernels were compared against main's old-path emulation. That is direct evidence that the
relation sets equal main's at the selftest sizes. The default `make bench` at a95274095f compares
against the new paths instead (header defaults are now 1/1/1).

## 2. Recomputed results (medians of 5 rounds; ratio = arm median ÷ M median)

|F| = 65536, 65536 targets (M in ms; other columns are ratios to M):

| config | M ms | M2 | E | H | P | EP | HP | EHP |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| hash/C16 | 576.415 | 1.0001 | 1.0000 | 0.7128 | 1.0300 | 1.0299 | 0.7013 | 0.7014 |
| hash/C32 | 846.408 | 1.0000 | 1.0000 | 0.6788 | 1.0270 | 1.0269 | 0.6628 | 0.6628 |
| hash/C64 | 1219.469 | 1.0000 | 1.0000 | 0.6677 | 1.0238 | 1.0238 | 0.6456 | 0.6456 |
| sorted/C16 | 401.705 | 1.0000 | 0.9525 | 1.0001 | 1.0007 | 0.8961 | 1.0005 | 0.8967 |
| sorted/C32 | 625.285 | 1.0000 | 0.9498 | 1.0001 | 0.7951 | 0.7438 | 0.7952 | 0.7437 |
| sorted/C64 | 619.748 | 0.9999 | 0.9490 | 0.9998 | 0.8244 | 0.7658 | 0.8245 | 0.7661 |

|F| = 1024, 262144 targets:

| config | M ms | M2 | E | H | P | EP | HP | EHP |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| hash/C16 | 19.874 | 1.0004 | 0.9999 | 0.8995 | 0.9815 | 0.9812 | 0.8929 | 0.8923 |
| hash/C32 | 25.606 | 1.0000 | 1.0002 | 0.8545 | 0.9848 | 0.9846 | 0.8264 | 0.8265 |
| hash/C64 | 24.342 | 0.9998 | 1.0007 | 0.8916 | 0.9672 | 0.9670 | 0.8668 | 0.8667 |
| sorted/C16 | 21.738 | 1.0000 | 0.9715 | 1.0002 | 0.9800 | 0.9134 | 0.9797 | 0.9137 |
| sorted/C32 | 31.071 | 1.0000 | 0.9735 | 0.9999 | 0.8169 | 0.7822 | 0.8166 | 0.7824 |
| sorted/C64 | 30.719 | 0.9994 | 0.9717 | 0.9996 | 0.8516 | 0.8117 | 0.8515 | 0.8117 |

Best configuration per arm, as main's best ÷ the arm's best. At |F| = 65536 every arm's best is
sorted/C16; at |F| = 1024 every arm's best is hash/C16.

| arm | \|F\| = 65536 | \|F\| = 1024 |
|---|--:|--:|
| M | 401.705 ms (1.0000) | 19.874 ms (1.0000) |
| M2 | 1.0000 | 0.9996 |
| E | 1.0499 | 1.0001 |
| H | 0.9999 | 1.1118 |
| P | 0.9993 | 1.0189 |
| EP | 1.1160 | 1.0192 |
| HP | 0.9995 | 1.1199 |
| EHP | 1.1152 (360.195 ms) | 1.1207 (17.733 ms) |

Every cell of `RESULTS.md` matches my medians at 2 decimals and my ratios at 3 decimals (0
mismatches), and so do the figure labels and the numbers in `REPORT.pdf`.
In all 12 configurations, every EHP round is faster than every M round. The two arms do not
overlap at all.

## 3. Claim-by-claim

| claim (REPORT.md) | verdict | my numbers |
|---|---|---|
| Table 1: main sorted/C16 401.70 ms, hash/C16 19.87 ms; E 1.050/1.000; H 1.000/1.112; P 0.999/1.019; EP 1.116/1.019; HP 1.000/1.120; EHP 1.115 (360.19 ms)/1.121 (17.73 ms) | confirmed | section 2 |
| Table 2: EHP/M per configuration (12 values) | confirmed | section 2, EHP column |
| 1.115× at \|F\| = 65536, 1.121× at \|F\| = 1024 | confirmed | 1.1152, 1.1207 |
| all three faster than main at all 12 configurations | confirmed | 12/12; no round overlap |
| within 0.1% of the best arm at each configuration | confirmed | worst 0.065%: \|F\| = 65536 sorted/C16, where EP is best (359.962 vs 360.195 ms). That gap is inside the 0.12% round spread |
| A/A at most 0.06% | confirmed | 0.0619% (\|F\| = 1024 sorted/C64); 0.0132% at \|F\| = 65536 |
| one arm's five rounds within 0.36% | confirmed | 0.3598% (\|F\| = 1024 hash/C64, H); 0.1196% at \|F\| = 65536 |
| packed: 0.65–0.71× on the global hash at \|F\| = 65536 | confirmed, mixed attribution | packed alone (H) 0.668–0.713; packed + paired (HP/EHP) 0.646–0.701. The 0.65 end needs pairing |
| packed: 10–15% on the shared hash | confirmed | H alone at \|F\| = 1024: 10.1%, 14.5%, 10.8% |
| paired alone: 15–21% on sorted C32/C64 | confirmed within rounding | 14.8–20.5% (65536: 20.5%, 17.6%; 1024: 18.3%, 14.8%). The 21% upper end overstates the maximum by 0.5 pp. At C16 pairing alone gives −0.07% and 2.0% |
| paired alone slower on the split-array hash at \|F\| = 65536 (1.02–1.03×) | confirmed | 1.024–1.030 |
| "so they only belong together with packed slots" | not supported as stated | true at \|F\| = 65536. At \|F\| = 1024, pairing alone speeds the split-array shared hash by 1.5–3.3% (0.967–0.985) |
| Eytzinger + paired: 23–26% (65536), 19–22% (1024) | confirmed, at C32/C64 | 25.6%, 23.4%; 21.8%, 18.8%. At C16, the best sorted chunk: 10.4% and 8.7% |
| Eytzinger alone: 5% (65536), 2.6–3% (1024) | confirmed | 4.8–5.1%; 2.6–2.9% |
| Eytzinger on top of pairing: 6–10% at \|F\| = 65536 | confirmed | EP vs P: 10.45%, 6.5%, 7.1% |
| hash best 404 vs sorted best 360 ms at \|F\| = 65536 (C16 both), ratio 1.12× | confirmed | 404.270 vs 360.195 ms, 1.1224 |
| every arm passed host tests and the 3 device selftests before timing | confirmed | section 1 |
| five rounds, arm order rotated each round | confirmed | section 1 (mtimes) |
| CUDA 12.8; 18:55Z–19:23Z | confirmed | nvcc 12.8.93; artifact mtimes 18:56:25–19:22:37 |
| `test_lookup_primitives`: 47 618 checks per flag set, passing under every flag set | confirmed | raw logs for the 8 arm flag sets. My runs of `make test` from `git archive a95274095f` under all 8 combinations: ALL PASS, 47618 checks each |
| positive control: the new test fails with 3 610 mismatches on the buggy version | confirmed (reconstructed) | the buggy code was never committed. I rebuilt it from the report's description (`while (a <= n)` advancing both chains): exactly 3 610 `sorted lookup` mismatches |
| bug gave "11 failures in the existing oracle test" | cannot check | the reconstructed bug reads past the array (undefined behaviour); my runs show 15 and 20 non-primitive failures, varying between runs |
| oracle suite passes with the new defaults | confirmed | `make test` with 1/1/1 |
| Rust wrapper crate: 6 tests pass with the new defaults | cannot check | no cargo build (disk constraint). `rs/build.rs` compiles `rootfind_api.cpp` without RF_* defines, so it now uses the new paths through the header defaults |
| pod `n68o04pli1i8oq`, host `id20nfc1q14a`, about $1.00, deleted | cannot check | not in the raw data |
| per-arm binary identity (`sha256.txt`) | cannot check | needs nvcc and the pod's tree; the pod's dirty-tree state is not recorded |
| "41–85% of sampled cycles in table probes" (motivation) | out of scope | belongs to `research/ic_gpu_replay_20261114` |

## 4. Code-level review (a95274095f; the code paths are identical at c308625f57)

Every variant returns either the same candidates in the same order, or those same candidates plus
extra ones that the full-x check discards. The tagged-hash extras are discarded by
`rootfind.cuh:453–454` (`j < i`, then `F::eq(f, x3, a.fb_x[j])`) before `rf_emit`, which has no
side effects before that point. Confirmed by reading and by tests:

- **`SortedTable::begin`, Eytzinger** (`rootfind.cuh:166–186`). Standard branch-free Eytzinger
  lower_bound, with `k >>= ffs(~k)` and `k == 0 → n`. The scan over equal keys still uses the sorted
  `keys[]`, so the candidate order is unchanged. Duplicate keys are handled because the in-order
  traversal is non-decreasing.
- **`begin2`** (`:188–217`). Eytzinger: each chain advances only while `≤ n`, which fixes the
  partial-bottom-level case. Binary search: two independent lower_bounds.
- **`eytzinger_fill`** (`rootfind_host.hpp:54–68`). An in-order walk assigns sorted positions; I
  checked that `eyt[k] == keys[rank[k]]` and that `rank` is a permutation, for every size tested.
- **`HashTable`, packed** (`rootfind.cuh:242–251, 262–274`). Same home slot, same linear probe and
  same stop rule (empty slot) as the split arrays. It returns every slot whose 32-bit tag matches,
  a superset of the exact-key slots, in the same order. `begin2` loads both first slots, then
  falls back to `skip_mismatch(s+1)`, which is equivalent to `begin`.
- **Slot packing** (`rootfind_host.hpp:103–105`). `tag(key) << 32 | vals[k]`. Empty slots keep
  `RF_NO_INDEX` in the low word, so emptiness is tested first; indices are never `RF_NO_INDEX`.
- **Sweep under `RF_PROBE_PAIR`** (`rootfind.cuh:423–460`). `x3_m` and `x3_p` are computed by the
  same formula as the unpaired path, and the walks still run ε = +1 first. The flags-off sweep is
  byte-for-byte the pre-change code; the diff against the parent `fa75167c45` only adds `#if` blocks.
- **Device side** (`rootfind_cuda.cu`). `run_device` always allocates and copies `eyt`/`eyt_rank`
  (n+1 entries) and `slots` (cap entries), in every arm (lines 227–250). The packed shared staging
  copies only `slots`, using 8 B per slot (`:146–150`; `hash_smem = hcap*8` at `:258`); the split
  staging uses 12 B. The timed interval is `cudaEventRecord(e0)` … `e1` around the launch only
  (`:264–285`), so "kernel time" is accurate. Global hash is selected when cap > 4096 slots.

**Independent harness (my code, host).** Source:
`/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/validator-rf/scripts/harness.cpp`
(the scratchpad is not durable). I built it from the committed headers under all 8 flag
combinations. Results:
- **Sorted primitives.** 9 652 990 checks, 0 failures. Sizes n = 0…2100 plus 4095–4097, 8191, 65535
  and 65536, with and without duplicates, querying keys below, inside and above the stored range.
  Checks: `begin` = `std::lower_bound`; `begin2` = two lower_bounds; the candidate scan = the
  `equal_range` positions in order.
- **Hash primitives.** 304 692 checks, 0 failures. After filtering by the exact key, the packed and
  split candidate sequences equal my own split-array probe walk in order. Packed extras equal the
  tag-only slots on the walk. `begin2` = two `begin` calls. The forced distribution produced 35 110
  tag-only candidates.
- **Full sweep (`rf_decompose_thread`, single-threaded tid loop).** Fields: Fp64; Fp64 with a
  16-tag key (718 984 and 615 416 tag-only candidates on the probe paths); Fp64 with a coarse
  non-injective key (duplicate exact keys on 1.6 M queries); P-256 with low-limb keys. Instances:
  p = 1048571 (N = 700, T = 1500) and p = 67108859 (N = 1025, T = 1200); P-256 N = 200, T = 400.
  All covered sorted/hash × C16/32/64 (P-256: C16/32).
  - In all 40 (instance, field, lookup, chunk) cases, the **ordered** emission sequence was
    identical across the 8 builds.
  - Every Fp64 relation multiset (2 024 and 540 relations) equals an independent int128
    reference that does not use the producer's arithmetic or tables.
- **Mutation controls.** Swapping the walk order (ε = −1 first) changes my ordered digest and leaves
  the multiset unchanged, while the producer's `make test` still reports ALL PASS. The producer's
  suite compares only `std::set`s within one build, so it does not test emission order; the
  order claim rests on code reading and this harness.

What `test_lookup_primitives` covers (`test_cpu.cpp:443–500`). Sorted: sizes 1–400 with duplicates,
keys below, inside and above the range, `begin` and `begin2`; this is adequate. Hash: it asserts
only that the stored index is present (`:491`) and that `begin2` equals two `begin`s. It does not
check the exact-candidate order, and it has no duplicate full keys. Its tag-collision keys hash to
different home slots, so the tag-only path is reached only by chance (in a simulation of its
distribution: 992 tag-only candidates over 151 796 queries). My harness closes those gaps on the host.

**Device coverage limits.** The device selftests compare relation sets, not order, and only at
|F| ≤ 4096. In those instances the only relations are the 1024 positive controls, and the packed
tag-only discard path is essentially never reached on the device (61-bit keys). The timed
`throughput` runs record no relations (`bench.cu:168–181`). "Identical relations" at the timed
workloads therefore rests on code identity of the shared `__host__ __device__` header, plus the
selftests and host evidence above. It was not observed directly.

## 5. Scope and over-claim check

- The report's standing and scope are stated correctly: `[pcc:unverified]`, one A100, kernel
  component, "not an end-to-end IC solve, makes no counted-cost or rho claim, adds no scoreboard or
  leaderboard row". No entry exists in `docs/ic/*`, `docs/ic-leaderboard.html` or
  `benchmarks/README.md`; the `benchmarks/progress/` rows are the bench-record report. At
  a95274095f no IC pipeline calls rootfind, so "no relation-phase number exists" is accurate.
- "Emission order unchanged" holds per thread and in single-threaded emulation. On the GPU, the
  output order across threads is set by `atomicAdd` and is nondeterministic in every arm.
- The defaults were switched on for every target. That includes the Makefile's default `ARCH=sm_120`
  (RTX PRO 6000 Blackwell) and the Rust crate's CPU emulation, but the timing evidence is sm_80 only.
  The report and the code comments scope the number to the A100, so this is a decision without
  measured support on other GPUs, not a false statement.
- "The default build is the all-three arm" is true for the kernels. The host emulation object
  differs from the measured arms (old paths then, new paths now), which does not affect kernel time.
- Unpriced setup: every build, including flags-off, now builds `eyt`/`rank`/`slots` on the host and
  makes 2–3 extra `cudaMalloc`/`cudaMemcpy` calls per oracle call, outside the timed interval. This
  is small next to these kernels (well under 1 MB per call at |F| = 65536), but it is not measured,
  and it is not part of the claim.

## 6. Process notes (non-blocking)

- The report names commit `1e757ab551`, while the raw `REV` is `c308625f57` (code-identical). The
  bench record says `commit: 1e757ab551, commit_dirty: false` for the session, not for the pod.
- The bench record lists `uploaded: false` for the S3 copy of the raw data (the S3 waiver is in
  `RESUME.md`), so its `s3_uri` points to an object that was not uploaded. The tarball is in Git
  (14 334 B, under the §13 64 KiB archive threshold) and its hash matches.
- The record's `host` block describes the session VM, not the GPU pod, and `isolation` is
  `unstated`. Wall-clock trust here rests on the A/A arm and the rotation, both present. Metric unit
  `wall_s` is CUDA-event kernel time.
- Makefile at a95274095f: `rootfind_api_emul.o` and `rootfind_api_cuda.o` get no `$(RF_DEFS)`
  (lines 55–56, 64–65), and no object depends on the flag values. The host emulation therefore
  follows the header defaults rather than the `make` variables, and a flag change without
  `make clean` reuses stale objects. `job.sh` used fresh copies plus `make clean`, so the
  measurement is unaffected. The concurrent uncommitted edit adds `$(RF_DEFS)` to those rules; I
  did not validate it.

## 7. Reproduction (validator)

Scripts are in `/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/validator-rf/scripts/`,
which is not durable:
- `recompute.py`: reads the extracted tarball, the tar listing and `RESULTS.md`; writes `recompute.json`. Run as
  `python3 -I recompute.py OUT_LK_DIR TAR_LISTING RESULTS.md OUT.json`.
- `harness.cpp`: built with `g++ -O2 -std=c++17 -pthread -DRF_SORTED_EYT=a -DRF_HASH_PACKED=b -DRF_PROBE_PAIR=c -I <git archive a95274095f>/gpu/rootfind`.
- `cmp_oracle.py`: cross-build digest comparison.

Host tests: `make test` under the 8 flag combinations, run in a scratch copy byte-identical to
`a95274095f:gpu/rootfind`. Nothing in the repository was built, edited or cleaned.
