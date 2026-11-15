# Validation review: `research/ic_gpu_rootfind_pairs_20261115/REPORT.md`

Validator, 2026-11-15. I did not write the code, the specs, the job or the report.
Standing of this review: `[pcc:unverified]`. No pcc claim is registered for it.

**Disposition: `validated-scoped`.**

The certificates hold, the identity claim holds, and the device-built pair table is charged exactly
as the CPU build, phase by phase and counter by counter. Every table cell reproduces from the raw
records. The code does what the report says it does. Several prose statements are not supported as
written (findings 1–9 below). The report omits the §5a band (finding 10) and the uncharged device work
(finding 11). There are protocol gaps: the raw data are not in S3, the run was not under `benchrec`,
and the PR merged before this review (findings 12–14).

What holds:

- **Certificates.** All 260 discrete-log certificates pass when re-checked with my own
  elliptic-curve code.
- **Curves.** The orders of all eight curves are proved. [pcc:unverified]
- **Identity.** All 156 pairs of an IC arm against the CPU arm match exactly. That covers S, total
  GAE, every phase's charged counts, every counter, the unpriced list, and every native and work
  counter except the `gpu_*` diagnostics.
- **Replays.** The 30 worker-free records I replayed on another hardware class reproduced.
- **Local check.** On this CPU host, a run of `probe_build=pairs` through the host worker matched
  `rootfind-subtract:m=3` on 12 of 12 pairs. One of those pairs went through the degenerate-row path.

No measured number is refuted.

## What was validated

| item | value |
|---|---|
| validated objects | `d905fb89d9c2c0bcc2986a1c24c5fd8f11837f86`, read with `git archive` / `git show`. This was the PR head at dispatch. PR #4276 merged it at 2026-11-15T08:05:10Z (merge commit `77ab1f47d4`) and deleted the branch |
| measured revision | `5142615e5d` (`REV` in the tarball). It was pushed 07:05:09Z. Between it and `d905fb89d9`, the reviewed files `gpu/rootfind/*`, `rootfind_gpu.rs`, `plugins.rs`, `ic_boundary.rs` and `ecbench/` differ by one edit only: the clippy rewrite `rem % 2 == 0` → `rem.is_multiple_of(2)` in `pair_at` (`87923d96de`, pushed 07:13:01Z). The two are semantically identical. `438663feb1` → `5142615e5d` changes only a round-3 PDF |
| raw tarball | `raw/out_rpair.tgz`: 231 629 B, sha256 `852cbf70b3a131ea9c18066e365ccd8dcb74e29fd1e9a750dd2978f892dcd694`. This matches `.sha256`, the REPORT and the benchmark record. It has 60 entries, all regular files or directories, with no links and no absolute or `..` paths |
| sessions | `smoke` 36, `rf3pairs` 176 and `rf2mt2` 48 records, all `verified` [pcc:unverified]. `records.jsonl`, `plan.json` and `host.json` hash to the values in `session.json`. Sequence numbers are contiguous, there are no warm-ups, and every record is L0. All records and receipts carry one binary, `7d2a69d0…`. `git_commit`/`git_dirty` are `null` in every session |
| audits | The pod receipts are `ok`, with 0 problems and 36/176/48 replays `identical`. The auditor was the same binary on the same hardware class (`ECBHW1hf98eb466ccc1`), so these are same-host replays |
| specs | `sessions/*/spec.json` are byte-identical to `specs/{smoke,rf3pairs,rf2mt2}.json` at `3ba9f2034e` (pushed 06:28:27Z) and unchanged through `d905fb89d9` |
| binary | `binary_sha256 = 7d2a69d0…` equals the local `target/release/ecbench` (mtime 07:04Z), which I copied (not rebuilt). It contains the round-4 strings `probe_build=pairs needs both signs`, `the worker built an empty pair table` and `probe_build must be host or device (or pairs at m = 3)`. It does not contain round 3's `probe_build must be host or device, got` |
| pod environment | `lscpu`: AMD EPYC 7543, 2 sockets, 128 CPUs. `nproc` reports 128. `cgroup-cpu.txt`: cgroup v1, `cfs_quota_us` 1360000 and `cfs_period_us` 100000, a quota of 13.6 CPUs (no `cpu.max`). `nvcc` is 12.8 and the GPU an A100 80GB PCIe. `daemon.log` reads "emulation threads 14", "daemon backend: cuda: NVIDIA A100 80GB PCIe (sm_80, 108 SMs)" and "context ready, 0.280 s". `serve-info.txt` reads `cuda: …`. `build-serve.log` rc=0, and `test-serve.log` reads ALL PASS (host-only `test_cpu`) |

## Requirements

| # | requirement | result | evidence |
|---|---|---|---|
| 1a | Tarball hash and contents | **pass** | sha256 above. Present: sessions, three receipts, `progress.log` (07:24:15–07:53:32), `daemon.log` (280 `timing init` lines, which `RF_SERVE_TIMING=1` prints), `cgroup-cpu.txt`, `lscpu.txt`, `nproc.txt`, eleven `compare-*.txt` files, tables and build/test logs |
| 1b | Daemon timing lines aligned to records | **pass** | Aligned in sequence order (run, then replay, per session), 280 lines match the 140 daemon-worker records twice each. The lookup mode matches each arm's oracle on every line. `lookup=4` (pairs) appears only for `rf3-gpup` arms |
| 2a | Certificates re-verified independently [pcc:unverified] | **pass, 260/260** | Pure-Python affine arithmetic (`scripts/certs.py`) checked `recovered·G = Q`, `Q` on the curve, and recovered = planted mod r for every record. The smoke curve's `a, b` were solved from `G` and `Q` |
| 2b | Curve orders | **pass, 8/8** | p and r are prime (deterministic Miller–Rabin). `r·G = O`, r > 4√p, and t² ≤ 4p, so #E = r. The trace and the j-invariant match ICV1 fields 2 and 4 on all eight curves |
| 2c | Replays | **pass (same host) 260/260; cross-host 30/30 worker-free** | My replays used the copied measured binary on hardware class `ECBHW1he9e33daae9e7`: smoke seqs 0, 1, 5, 8; rf3pairs 20 seqs (rho, rho A/A, CPU at 2048 and 4096, CPU A/A, on every curve); rf2mt2 seqs 0, 1, 8, 9, 16, 17 (rho and CPU, including 31.3 bits). All are `identical`. Daemon-worker records were not replayed cross-host (see "Not checked") |
| 2d | 156 pair checks of every IC arm against the CPU arm | **pass, 0 mismatches** | The 156 pairs are smoke 20, rf3pairs 112 and rf2mt2 24, over arms mt, gpud, gpuh, gpup and cpuaa. All fields are equal: S, total GAE, recovered, planted, status, relations, targets, matrix rank and work, `cost.unpriced`, and every phase's adds, doubles, scalar mults and GAE. The top-level `field_ops`, the `bitop/v1` totals and the per-phase bitops are also equal. So are all phase `native` and `work_native` maps, including linear algebra's `gauss_entries_scanned`, `pivot_inversion_steps`, `row_build_words` and `row_key_mixes`, except the `gpu_*` diagnostic counters (`scripts/pairs.py`, `scripts/deepdiff.py`) |
| 2e | `gpup` oracle set-up charge equals the CPU build's | **pass, 36/36 records** | `oracle_setup` is identical on every arm. At \|F_x\| = 2048: adds 4 196 352 = a(a+1), doubles 0, `table_insert_attempts` 4 194 304 = a², `lookups` 4 096 and `montgomery_conversions` 4 096, `counted:table_insert_attempts` = 1. At 4096: 16 781 312 / 0 / 16 777 216 / 8 192 / 8 192. `lookups` = 2a is a from the table's index reads plus a from the sweep's |
| 3 | Code review | **pass, with notes** | See "Code" below |
| 4 | Every number | **pass for the tables; prose findings 1–9** | See "Confirmed numbers" |
| 5a | Specs pushed before the run | **pass** | Push activity (GitHub REST `activity`): `3ba9f2034e` at 06:28:27Z, `438663feb1` at 06:56:41Z and `5142615e5d` at 07:05:09Z. The job's first log line is 07:24:15. The spec bytes are equal to the committed specs |
| 5b | Record on a clean commit | **pass, with notes** | `benchmarks/records/20261115T0656Z-ic-gpu-rootfind-pairs.json`: `commit` `5142615e5d…`, `commit_dirty: false`. Its opening commit message says "initialised on 438663feb1" (the two are code-identical). Its one command is `analyze.py`, not the run (disclosed). `raw.uploaded: false`; `validation: null` |
| 5c | CUDA-backend check | **pass** | `job.sh` aborts unless `daemon.log` contains `cuda: ` and `serve-info.txt` starts with `cuda: `; both hold. Every GPU arm is `backend=cuda`, which fails closed without a device (`resolve_backend` keeps the request; `rf_cuda_session64_create*` errors) |
| 6 | Wording | **findings 1–9** | Below |

## Confirmed numbers

I recomputed these from `records.jsonl` and `daemon.log` with `scripts/tables.py`, `scripts/band.py` and
`scripts/daemon_timing.py`. Medians are over the 4 runs per cell (2 targets × 2 rounds); ratios are
ratios of medians.

- **`m = 3` table, all 80 cells hold to the printed digit.**
  - S/rho: 140, 166, 302, 569, 242, 142, 172, 298 (139.7–569.2).
  - CPU solve: 1.058, 1.863, 6.253, 21.099, 5.069, 5.407, 11.349, 24.809 s.
  - Multi-thread: 1.224, 1.288, 1.869, 4.001, 5.271, 6.348, 8.425, 7.383 s.
  - GPU with the CPU table: 0.788, 1.018, 0.916, 0.949, 4.754, 5.872, 5.050, 5.783 s.
  - GPU with the device table: 0.246, 0.260, 0.295, 0.368, 0.706, 1.362, 1.656, 1.488 s.
  - Device table against the CPU arm: 4.30, 7.18, 21.22, 57.33, 7.18, 3.97, 6.85, 16.67×.
  - Device table against the multi-threaded arm: 4.97, 4.97, 6.34, 10.87, 7.47, 4.66, 5.09, 4.96×.
  - Set-up, CPU → device table: 0.511→0.034, 0.496→0.028, 0.501→0.018, 0.505→0.021,
    4.205→0.124, 2.941→0.116, 3.805→0.109, 2.918→0.122 s.
  - Per pair, the device-table arm is faster than the CPU arm in every one of the 32 pairs (3.25–63.06×)
    and faster than the multi-threaded arm in all 32 (3.46–11.31×). [pcc:unverified]
- **`m = 2` table, all 18 cells hold.**
  - S/rho: 5 303, 19 501, 22 023.
  - CPU solve: 2.334, 12.894, 33.972 s.
  - Multi-thread: 0.313, 1.304, 3.238 s.
  - GPU: 0.101, 0.158, 0.282 s.
  - Multi-thread against CPU: 7.45, 9.88, 10.49× (per pair 6.36–10.79×).
  - GPU against multi-thread: 3.11, 8.27, 11.49×.
- **Prose that holds.**
  - CPU set-up 0.50–0.51 s at 2048 and 2.9–4.2 s at 4096; device-table set-up 0.018–0.034 s and
    0.11–0.12 s.
  - Device table against the GPU arm with the CPU table: 2.58–6.74×, printed "2.6–6.7×" (per pair
    2.17–8.58×).
  - Relation phase of the device-table arm at 2048: 0.017–0.158 s. Linear algebra at 2048: 0.184–0.217 s.
  - S 140–569× rho at w = 0.
  - Wall 3.39–98.9× rho's median solve (7.1–108.4 ms); 3.39× at 39.3 bits and 2048
    (0.368 / 0.1084 s).
  - Cgroup quota 13.6 CPUs; 14 threads; multi-thread 7.5–10.5×.
  - Device table against multi-thread 4.7–10.9×; GPU against multi-thread 3.1–11.5×.
  - CPU A/A per-cell median ratio 0.962–1.135 ("up to 14%").
  - Multi-threaded set-up 3.94–5.99 s at 4096.
  - Worker create median 0.0340 s over all 72 `lookup=4` lines.
  - Daemon context 0.280 s; daemon start to socket 0.292 s.
  - Identity counts 36/176/48 and 156.
- **Benchmark record.** Its metric values hold: 0.368 vs 21.099 with reference 0.10842; 0.122 vs
  2.918; 3.238 vs 33.972; and 565.7 with reference 0.9942. So do its summary ranges.

## Statements not supported as written

1. **"the worker creates the session in a median 0.034 s (daemon timing lines)".** The 0.034 s is
   the median of all 72 `lookup=4` lines. Those pool the smoke runs (8 lines), \|F_x\| = 2048 (32) and
   4096 (32), runs and replays together. The distribution is bimodal, so the pooled median describes
   neither size. Per size the `rf3-gpup` create time is 0.0166 s at 2048 (0.0095–0.145 s) and
   0.111 s at 4096 (0.0995–0.146 s).
2. **"its set-up builds a host hash of the 4–17M-entry probe set (3.9–6.0 s) on top of the CPU pair
   table".**
   - The 3.9–6.0 s is the multi-threaded arm's whole oracle set-up at \|F_x\| = 4096. That includes
     the CPU pair-table build (2.9–4.2 s on the CPU arm). The 4M-entry probe set belongs to \|F_x\| =
     2048, where that set-up is 0.79–0.89 s.
   - The daemon lines put the host hash itself at 0.17–0.21 s (4.2M keys) and 0.75–0.92 s (16.8M
     keys).
   - **The attribution fails at 33.4 bits, 4096.** There the multi-threaded set-up (3.99 s) is below
     the CPU arm's (4.21 s). The slowdown comes from the relation phase (0.685 s against 0.282 s), and
     every multi-threaded, GPU and device-table run in that cell splits its batches 4 times
     (`gpu_batch_splits`).
   - The CPU arm's own set-up varies 2.92–4.21 s across the four 4096 curves for identical counted
     work (16 781 312 additions). Set-up differences of a second or so are near that wall noise.
3. **"At m = 3 the multi-threaded arm is slower than one thread at the small sizes (0.85–0.96× at
   33–35 bits, 4096)".**
   - The 0.96× (33.4 bits, 4096) is inside the A/A spread. Per-cell A/A medians differ by up to
     13.5%, and the per-pair A/A ratio spans 0.73–1.38.
   - Per pair, multi-thread against CPU is 0.57–1.09× at 33.4/4096 and 0.71–1.04× at 35.2/4096. The
     A/A was measured only at 2048.
   - The cell 33.4/2048 (0.86×) also qualifies but is not mentioned.
   - Supported reading [pcc:unverified]: the multi-threaded arm is not faster than one thread at 33–35 bits, and it is
     unresolved whether it is slower.
4. **"Its relation phase is 2.5–9.6× one thread where that phase is large".** The excluded cell is
   33.4 bits, 4096, where it is 0.41× (slower). That cell is the one where the relation phase, not the
   set-up, makes the arm slower. "Large" is not defined. The full range is 0.41–9.61×.
5. **"at 4096, linear algebra (0.53–1.50 s, the same code on every arm)".** The device-table arm's
   linear algebra is 0.533–1.494 s. The 1.50 s is the multi-threaded arm's 1.504 s. Across arms the
   range is 0.523–1.504 s. The counted linear-algebra work is identical on every arm; the wall differs
   by up to 14% (1.458 against 1.278 s at 39.3/4096).
6. **"the closest any round has come" (3.4× rho in wall).** This is a cross-round wall comparison.
   The report's own Standing paragraph says wall times are compared within this session only, and
   the session log says "cross-round wall comparisons avoided".
   - Within the rootfind GPU rounds, the per-session minima of the GPU arm's wall against rho are:
     `ic_gpu_rootfind_ic_20261114` 115×, `ic_gpu_rootfind_m3_20261115` 28×,
     `ic_gpu_rootfind_big_20261115` 7× and this round 3.4×. So the statement is true of that
     sequence. But each ratio is from a different host, and "any round" is broader than that sequence.
   - The ratio is an A100-accelerated arm against single-thread CPU rho. It is not a matched-hardware
     comparison and says nothing about cost.
   - Supported reading: in this session's wall time, the device-table arm is 3.4× rho's median solve
     at 39.3 bits, 2048.
7. **"The one that changed this round's plan: … the multi-threaded arm's weakness had another cause
   (below)", with the 13.6-CPU quota and "round 3's emulation ran one thread per visible host
   thread".**
   - This presents the oversubscription hypothesis as the cause. The session log calls it "the
     leading explanation", and RESUME.md says "the run tests that". The run does not test it.
   - Round 3's quota was not recorded. This round has no same-host control, such as an
     `RF_EMU_THREADS=128` arm or a per-batch-thread arm.
   - The 5× → 7.5–10.5× change also changes the host CPU (Xeon 6240R → EPYC 7543), the thread count,
     the pooling and the quota at once.
   - The round-3 validator's finding stands: the cause of the low throughput is not established. Read
     the "(round 3, … about 5×)" comparison as cross-host and confounded.
8. **"With the pool sized to the 14 CPUs the pod may use".** The quota is 13.6 CPUs. The pool is
   ⌈13.6⌉ = 14 threads, slightly above it. The figure's legend says "CPU, all threads (pool)": that is
   14 of 128 visible threads. The figure's subtitle says "every arm reports identical relations". The
   records carry no relation digest; what is equal is the relation counts and every linear-algebra
   counter (see Code, (a)).
9. **Headline use of the wall ratio.**
   - The commit and PR title read "57× at 39 bits", and the table bolds **57.3×**. That is an L0 wall
     ratio of an A100 arm against one CPU thread, with identical counted S.
   - §6 keeps wall time out of the headline. ecbench's own verdict on these wall comparisons is
     "measurement quality: 0 / 100 … admitted 0" (`compare-*.txt`), which the report does not quote.
   - The Standing paragraph does label every record L0 and wall as the only difference.
   - In the round-4 terms of §3, this is engineering on wall time with S flat: no counted
     improvement over the index-calculus baseline.

## Omissions and protocol gaps

10. **The §5a band is not given.**
    - The rho comparison is stated at w = 0 only. My band uses `cost.unpriced` from counters and
      phase natives, plus `table_insert_attempts` behind its `counted:` marker, the same on both arms.
    - `m = 3` against rho: 140–569× at w = 0, 190–574× at w = 0.1 and 297–1 019× at w = 1.
    - `m = 2`: 5 303–22 023×, 5 314–22 037× and 5 364–22 101×.
    - Rho is ahead at all three weights in every row. No arm is missing a count.
11. **Uncharged work is not named.** By design the GPU and multi-threaded arms are charged the CPU
    oracle's work. The work they do beyond it is unpriced and not named in `cost.unpriced`:
    - the device or emulation sweeps every candidate of every target, with no early exit;
    - targets are swept and never used (`gpu_targets_unused`, e.g. 56 per 2048 run);
    - the device pair-table build, dedupe, compaction and probe hash;
    - batch splits;
    - the degenerate-row search `first_pair_for`, which makes 2a scratch additions and up to 2a index
      reads per degenerate row. It is charged nothing ("lookup work, not charged"), and the REPORT's
      "search the first pair by enumeration" does not say so. It occurred once (smoke) and never in
      rf3pairs or rf2mt2.

    Every GPU and multi-threaded record also carries `work.complete: false`, with the `gpu_*`
    counters in `work.unpriced`; every CPU record has `complete: true`. The report does not mention
    this. The round-3 validator raised the same omission (its finding 9).
12. **Raw data not in S3.**
    - The record has `raw.uploaded: false`.
    - The 231 629 B `.tgz` is committed to Git. Under §13 an archive over 64 KiB goes through the
      artifact store with an `.artifacts.json` manifest.
    - RESUME.md cites "the same waiver as earlier rounds". I found no waiver in the committed objects.
13. **Run not under `benchrec`.** This is disclosed in the record. The record's only command is the
    analysis. `job.sh` is not in the tarball, so I could not confirm that the committed `job.sh` is
    byte-for-byte the one that ran. Its outputs match its file layout and log lines exactly.
14. **Merged before validation.** PR #4276 merged at 08:05:10Z while "validator check of the round-4
    report" was still an open thread. The record's `validation` is `null`. No leaderboard or
    `benchmarks/README.md` row was added, so the validation gate itself was not breached.

## Code

The code reviewed is at `d905fb89d9`, which equals `5142615e5d` apart from the clippy rewrite. It
covers `gpu/rootfind/{rootfind_cuda.cu, rootfind.cuh, rootfind_api.cpp, rootfind_host.hpp,
rootfind_serve.cpp, fp64.cuh}`, `ic_framework/rootfind_gpu.rs`, `ic_framework/plugins.rs`
(`RootfindSubtractOracle`) and `ic_boundary.rs` (`PairTable::build_negation_folded`,
`PairTable::probe`, `PrimeCurve::add_raw`, `key`, `GroupOps`).

- **Can the result differ from `rootfind-subtract:m=3`?**
  - Given a correct device answer, the ledger and every counter except `gpu_*` are equal by
    construction, and the returned relation is the same:
    - per miss: `charge_misses` = one addition, 4 multiplications, 1 squaring and one lookup;
    - per hit: `charge_candidate`, one lookup, and the probe's one charged addition and compare;
    - per target: `charge_sweep_setup`;
    - for the degenerate row: one ledger `g.add` per position, no candidate charge, then the lookup.
  - The table key is `x + 1`, so a key hit always means `S = ±T`. Every returned relation is
    re-added on the ledger (`s == t` or `s == −t`), so a wrong device answer cannot produce a false
    relation.
  - A wrong device answer could, however, change the charge or the relation:
    - (a) A non-least build index on an abscissa that several pairs share leaves S, the ledger and
      the relation count unchanged, but returns different summands. That is visible only in the
      relation's content, which the records do not store. Indirect evidence that it did not happen:
      - The device tables' sizes in `daemon.log` equal the CPU tables' sizes on all 8 cells
        (4 192 870 … 16 776 850).
      - The cells hold a² − entries = 18–1 434 (2048) and 366–24 332 (4096) duplicate pairs, about
        N²/p as expected, so hits on shared abscissae occur (roughly 5 per run at 33.4/4096).
      - All linear-algebra counters are equal in all 156 pairs.
    - (b) A false positive adds one charged addition.
    - (c) A false negative changes the sweep.
    - Both (b) and (c) would show in the identity check.
- **Build-index decode.**
  - `rf_pair_offset(ai, n) = 2(ai·n − ai(ai−1)/2)` (unsigned wrap at ai = 0 gives 0) is the same
    function as `pair_at`'s `off`. `k_pair_keys` writes `keys[off(ai) + 2(bi−ai) + s]` for bi ≥ ai,
    with identities as `RF_KEY_EMPTY`.
  - `pair_at`'s binary search inverts it. My independent model (`scripts/pairmodel.py`) checks every
    index exhaustively for a = 32 and 128 on the smoke curve, and the four boundary indices of every
    ai-segment for a = 2048 and 4096.
  - Indices stay below 2³² (maximum 16 781 311). `rf64_session_create_pairs` refuses n(n+1) ≥ 2³².
- **First pair per abscissa.**
  - `k_pair_dedupe` claims a slot by `atomicCAS` on the key and then `atomicMin`s the index. Equal
    keys land in one slot, and slots are never removed, so the slot keeps the least index.
    Montgomery values are fully reduced (`fp64_reduce`, `add` and `sub` end in [0, p)), so one
    abscissa has one key.
  - The CPU build inserts in increasing build index, skips identities and keeps the first insert
    (`or_insert`). That is the least index.
  - The host build path uses `unordered_map::emplace` with increasing `b`: also first wins.
  - My model confirms device-least = CPU-first on every key, including all 427 collided keys at
    a = 128. It also confirms `first_pair_for` returns exactly that index for random keys and for
    every collided key, and `None` for non-keys. The model's a = 32 table has 1 021 entries, the
    smoke `n_probe` in `daemon.log`.
- **Per-case build charge.**
  - `PrimeCurve::add` increments `adds` and calls `add_raw`. Its field work depends only on the case:
    general (1 inversion, 2 multiplications, 1 squaring), `x` equal with `y` summing to non-zero
    (doubling via `double_raw`), or the identity (no field work).
  - The build's cases are exactly a(a−1) general, a doublings and a identities (a² insert attempts),
    provided every abscissa has both signs and no point has y = 0. `charge_pair_table_build` checks
    that and refuses otherwise.
  - It copies every `GroupOps` field: adds, doubles, scalar mults, field multiplications, squarings
    and inversions, and Montgomery reductions.
  - The deferred `table_insert_attempts` = a² and `lookups` = a reach `setup_work_native` with the
    `counted:` marker, as the CPU table's `work_counters` do. Matches the records (2e).
- **Degenerate rows.** `decompose_pairs` drops the device's row-d hits (also `tau = 0`) and processes
  2d and 2d+1 as the CPU does. `misses()` skips those two positions. The CUDA kernel substitutes 1 for
  the zero difference before the batched inversion, as the CPU does, so the other rows are unaffected.
  My local run hit one degenerate row and matched.
- **Build-index translation.** In both the CUDA and the host session paths of
  `rf64_session_decompose`, `out[k].j = pairb[out[k].j]` for `tau != 0`, after the `cap` clamp. There
  is no bounds check on the device's probe index, which is a robustness gap only: an index out of
  range would read out of bounds rather than fail closed.
- **`EmuPool` and `rf_cpu_budget`.**
  - The pool is correct as far as I can see:
    - one batch at a time (`run_mu_`);
    - a generation counter, with `busy_` set to the worker count, decremented once per worker per
      generation, and waited to 0 before `run` returns;
    - so the job lambda, which captures by reference, outlives every use;
    - wake-ups are protected by their predicates;
    - shutdown sets `stop_` and bumps the generation.
  - `rf_cpu_budget` returns `RF_EMU_THREADS` if set. Otherwise it returns
    `min(hardware_concurrency, ⌈quota/period⌉)` from cgroup v2 `cpu.max` or v1 `cfs_*`. On the pod
    that gave min(128, 14) = 14, as `daemon.log` prints. It ignores the affinity mask, which made no
    difference here (`nproc` 128).
- **Local functional check (not a measurement).** I built `rootfind_serve` host-only from the
  committed sources in scratch (`make test`: ALL PASS). I then ran the measured `ecbench` on a scratch
  spec: 20- and 22-bit curves, \|F_x\| = 32 and 128, 3 targets each, `rootfind-subtract:m=3`
  against `rootfind-subtract-gpu:m=3,worker=spawn,backend=emulate,probe_build=pairs`.
  - 24/24 verified. [pcc:unverified]
  - All 12 pairs are equal in every non-timing field except `stderr_sha256` (the worker's stderr) and
    the `gpu_*` counters.
  - One degenerate row occurred.
  - This exercises the Rust pairs logic and the host build path, not the CUDA kernels.

## Not checked

- **The CUDA path on a device.** The measured `rf3-gpup` arms are `backend=cuda`; there is no GPU
  here, so the 36 `backend=cuda` pairs records rest on the pod's same-host replay. The CUDA kernels
  are reviewed by reading. Their correctness on these inputs is evidenced by the 36 identity matches
  and the equal table sizes.
- **Cross-host replay of the daemon-worker records** (multi-threaded and GPU arms). A host daemon
  needs its socket beside the `ecbench` executable. The scratch path exceeds the 108-byte Unix
  socket limit, and I did not use `/tmp` outside scratch. The worker-free records, 30 of them,
  reproduced.
- **The "7 tests (host worker)" and the wrong-pair sabotage test ("7 299 targets against 42").** No
  test log is committed; the session log asserts it. Without a worker, these Rust tests skip and pass,
  so a green CI run is no evidence. I did not build the Rust crate: the disk had 3.7 GB free, and the
  repository's target directory was off limits.
- **The relations' content.** The records store no relation digest. See Code, (a).
- **Run facts outside the tarball.** The pod id (`y8dxu6qiiykumx`), the 07:05Z start, the cost (about
  $1.75) and the deletion are not in the tarball. `progress.log` covers 07:24:15–07:53:32Z.
- **Binary provenance.** The binary hash binds the records to the local `target/release/ecbench`
  (mtime 07:04Z). Its strings fit round-4 code. I did not rebuild it reproducibly, so "built from
  `5142615e5d`" rests on `REV` and timing.
- **Round-3 corrections.** I did not map each of the round-3 validator's eight corrections onto that
  report's edits. Its REPORT.md changed 48 lines and carries 6 "corrected" markers.

## Scratch

Scratch work, not committed, is in
`/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/validator-rpair/`:

- scripts: `scripts/{certs,pairs,deepdiff,tables,band,daemon_timing,align,pairmodel}.py`;
- extracted committed objects: `art/` and `code/`, and the tarball under `raw/`;
- cross-host replay receipts: `replay/audit-{smoke,rf3pairs,rf2mt2}.json`;
- the local functional check: `localcheck/` (spec, session and log).
