# Validation review: `research/ic_gpu_rootfind_big_20261115/REPORT.md`

Validator (independent; did not write the code, the specs, the job or the report), 2026-11-15.
Standing of this review: `[pcc:unverified]` (no pcc claim is registered for it).

**Disposition: `validated-scoped`.**

The certificates hold, the identity claim holds, the four new curves are correct, and every table
cell holds to within one unit in the last digit. I recomputed all of it from the raw tarball and the
code with my own scripts. I also replayed 51 records on a CPU host of another hardware class with
the exact `ecbench` binary and a worker I built from the committed sources: all 51 reproduced.

What holds:

- **Certificates.** All 240 discrete-log certificates pass when re-verified with my own
  elliptic-curve code.
- **Identity.** All 136 IC-arm-vs-CPU-arm pairs match exactly: S, total GAE, recovered logarithm,
  status, relations, targets and relation-phase additions. A stricter check also matches every
  phase's charged counts, every counter, every non-`gpu_*` native and work counter, and the
  factor-base points hash.
- **Curves.** All four curves are proved correct: p and N are prime, N lies in the Hasse interval,
  and the order is N.
- **Fail-closed.** `backend=cuda` refuses to run on a host without a device, as stated: all 12 such
  records in smoke return status −2 on my CPU-only worker.

No measured number is refuted. The findings are:

- **Rounding.** Three table cells are rounded twice (finding 1). Two endpoints are off by one in the
  last digit (finding 5).
- **Statements not supported as written.**
  - The relation-phase range "13–33× at 33–35 bits" omits a 5.7× cell (finding 2).
  - The worker-create comparison "0.185 s against 0.780 s" pools unlike groups. Like for like it is
    0.044 s against 0.780 s (finding 3).
  - The "device-built against host-built table" comparison also changes the table type (finding 4).
  - "5.1–5.8 s, the same on every arm" holds only for the CPU arm (finding 6).
  - The multi-threaded arm's "about 5× one thread" and its thread-pool explanation (finding 7).
  - The binary provenance (finding 8).
- **Omissions.**
  - The uncharged device and emulation work is not named (finding 9).
  - ecbench's own measurement-quality verdict is not given (finding 10).
  - The §5a band is not given (finding 11).
- **Process gaps.** The raw data are not in S3, and the `backend=cuda` records cannot be replayed
  without a GPU, so a CPU-only cross-host audit is impossible for them.

## What was validated

| item | value |
|---|---|
| validated objects | `8fc1e54a5f695f6e08ad14c8c152215bf85a1542` (origin head at dispatch), read with `git archive` / `git show`. The working tree was not used. Origin has since moved to `be5e79543d`, which adds only lessons files. A local commit `3ba9f2034e` (round 4, in progress) changes `gpu/rootfind/*` and `rootfind_gpu.rs`. Neither touches `research/ic_gpu_rootfind_big_20261115/`. The code reviewed here is the code at `7417c99292` = `8fc1e54a5f` |
| measured revision (asserted) | `7417c99292` (`REV` in the tarball). `gpu/rootfind/` and `src/cryptanalysis/ic_framework/rootfind_gpu.rs` are byte-identical at `7417c99292` and `8fc1e54a5f`. The merge from `main` changed `ic_boundary.rs` (the rho walkers among others), `ic_framework/rootfind.rs` (a refactor of the `rootfind-mitm` and `rootfind-pairs` oracles) and `ecbench/generic.rs`. A binary built at `8fc1e54a5f` is therefore not the measured binary |
| raw tarball | `raw/out_rbig.tgz`: 223 485 B, sha256 `4144d0495ca2696a942b51f736e7195aff3cce3dbbe58c774cea643533a33f3c`. This matches `raw/out_rbig.tgz.sha256`, REPORT.md and the benchmark record. The tarball has 56 entries and no unsafe paths |
| tarball contents | `sessions/{smoke,rf3big,rf2mt}` hold 32, 160 and 48 records. Also present: `audit-*.{json,log}`, `run-*.log`, `table-*.md`, `compare-*.txt` (8), `progress.log`, `daemon.log` (240 `RF_SERVE_TIMING` lines), `daemon-start.txt`, `nproc.txt` (96), `lscpu.txt` (Xeon Gold 6240R, 2×24 cores, 96 threads), `nvidia-smi-q.txt` (A100 80GB PCIe), `nvcc.txt` (12.8), `test-serve.log` (ALL PASS), `build-serve.log` (rc=0), `serve-info.txt`, `sha256.txt` and `REV` |
| session bindings | For all three sessions, `records.jsonl`, `plan.json` and `host.json` hash to `records_sha256`, `plan_sha256` and `host_sha256` in `session.json`. The audit receipts' file hashes match the extracted files. Sequence numbers are contiguous, there are no warm-ups, and every record is L0 with the blocker "L1: no CPU reservation". `git_commit` and `git_dirty` are `null` in every session |
| specs | `sessions/{smoke,rf3big,rf2mt}/spec.json` are byte-identical to `specs/*.json` at `7417c99292` and at `8fc1e54a5f` |
| binary | `binary_sha256 = f01d5f91…` appears in all 240 records, the three audit receipts and the pod's `sha256.txt`. It also matches the local `target/release/ecbench` (mtime 2026-11-15 04:42:54Z), which I copied for the replay. The binary contains the three error strings introduced in `7417c99292`'s `rootfind_gpu.rs` and not the string they replaced |

Scratch work, not committed, is in
`/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/validator-rbig/`:

- scripts: `scripts/{load,ec,certs,curves,tables,try_seeds}.py`;
- outputs: `tables.json`, and the replay receipts in `replay/out/`;
- the 96-thread create/join timing: `bench/tpool.cpp`.

The replay's binary and daemon socket were in
`/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/vrb/`, because a Unix
socket path must be under 108 bytes.

## Requirement results

| # | requirement | result | evidence |
|--:|---|---|---|
| 1 | Raw tarball, sessions, audits, logs, host files | **pass** | Hash, contents and bindings are in the table above. `progress.log`: smoke 05:12:24–05:12:28; rf3big 05:12:28–06:01:12; rf2mt 06:01:12–06:10:10; done 06:10:11. Every run and audit has rc=0 and "0 problems" |
| 2a | All records verified | **pass** | 32 + 160 + 48 = 240 records have status `verified`, `matches_target` and `matches_planted` |
| 2b | Certificates (own code) | **pass, 240/240** | `scripts/ec.py` is my own Jacobian double-and-add, independent of the solver. For each record it checks `0 ≤ d < r`, `[d]G = Q` and `d = planted`. For the four `prime_explicit` curves, a and b are taken from the spec. For the `prime_search` curves (20, 28, 30, 32 bits), a and b are derived from G and one target, and the second target is checked to lie on the same curve. `[r]G = O` holds on all 8 curves |
| 2c | Replay audits reproduced | **pass (same host)**; **51/51 cross-host** | The pod receipts read `replays reproduced` 32/160/48, all "identical". Their auditor is the same binary, the same env class `ECBENV3hcbf732c8ca7b`, the same hardware class `ECBHW1hf5a1e3398259` and the same daemon. My replay ran on hardware class `ECBHW1he9e33daae9e7` with the exact binary and a host-only `rootfind_serve` built from the committed `gpu/rootfind` (its `make test`: ALL PASS). It covered smoke `--replay-all`, 22 rf3big seqs and 9 rf2mt seqs. Every rho, CPU, CPU A/A and `backend=emulate` record reproduced ("identical"), including 2 at 39.3 bits (CPU and multi-threaded), 4 multi-threaded records at \|F_x\| = 4096 (each with 4 batch splits) and 1 at 31.3 bits. The 12 `backend=cuda` records in smoke failed with `rootfind worker: init status -2` |
| 2d | 136 pair checks | **pass, 0 mismatches** | My `scripts/load.py` and an inline pair check group the records by (session, curve, target index, round, \|F_x\|, m). Each IC arm (mt, gpud, gpuh, cpuaa) is compared with the CPU arm. The pairs number 96 in rf3big, 24 in rf2mt and 16 in smoke, 136 in all. S, total GAE, recovered, status, relations, targets and relation-phase adds are equal. The stricter check is also clean: every phase's adds, doubles, scalar mults and GAE; all counters; LA native counters; factor-base `points_sha256`; the unpriced name list; and every native and work counter. The only differences are the `gpu_*` diagnostics present only on device and emulate arms |
| 3 | Curves | **pass (proved)** | `scripts/curves.py` (own code). p and N are proved prime by trial division to √n (n < 2⁴⁰). The curves are non-singular, a = −3, and \|p+1−N\| ≤ 2√p. G is on the curve and `[N]G = O`. Since N is prime and G ≠ O, ord G = N, so N divides #E. The only multiple of N in the Hasse interval is N itself, so #E = N. As further checks, `[N]Q = O` holds for 20 random points per curve, and the cubic has no root in F_p (no 2-torsion, consistent with odd N). `make_curves.py 34,36,38,40 20261115` reproduces `specs/curves_34_40.json` exactly; the seed is recorded nowhere in the study, and I found it by trying candidates. Traces: 124 571, −325 323, −68 843, −106 485 |
| 4 | Every number in the REPORT | **pass with corrections** | The confirmed numbers are in the next section. The errors are findings 1–7 |
| 5 | Code (device table, `backend`, `probe_build`, fail-closed) | **pass with notes** | See "Code review" |
| 6 | Protocol | **pass with notes** | `7417c99292` was committed at 04:32:27Z. GitHub's activity log records it on origin at **04:32:29Z** (branch creation). The pod job started at 04:43:43Z (`nvidia-smi -q` timestamp and `REV` mtime). The first session started at 05:12:25Z. The bench record `20261115T0432Z-ic-gpu-rootfind-big` was initialised at 04:32:41Z on `7417c99292` with `commit_dirty: false`. The `job.sh` CUDA check passed: no ABORT line, `daemon.log` reads "daemon backend: cuda: NVIDIA A100 …", and `serve-info.txt` reads "cuda: …". The process gaps are listed below |
| 7 | Wording | **8 statements unsupported or imprecise as written** | See "Unsupported or imprecise statements" |

## Confirmed numbers

Medians of `time.solve_wall_ns` and of the phase `wall_ns` over the 4 records per cell (2 targets ×
2 rounds). S / rho is the mean IC S over the mean S of the 4 `rho-neg` records, at w = 0.

`m = 3` (rf3big):

| log2 r | \|F_x\| | S / rho | CPU solve | mt solve | GPU solve | GPU / CPU | GPU / mt | relation phase CPU → GPU |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 33.45 | 2048 | 132.52 | 2.7447 | 2.7513 | 2.5578 | 1.073 | 1.076 | 0.5861 → 0.0437 (13.4×) |
| 35.18 | 2048 | 151.77 | 4.2184 | 3.4149 | 2.8131 | 1.500 | 1.214 | 1.7670 → 0.0539 (32.8×) |
| 37.39 | 2048 | 188.24 | 10.455 | 4.4901 | 2.9149 | 3.587 | 1.540 | 7.8345 → 0.1129 (69.4×) |
| 39.34 | 2048 | 521.37 | 32.639 | 7.7409 | 2.9291 | 11.143 | 2.643 | 30.217 → 0.2450 (123.3×) |
| 33.45 | 4096 | 230.38 | 8.6621 | 11.561 | 8.5565 | 1.012 | 1.351 | 0.3977 → 0.0703 (**5.7×**) |
| 35.18 | 4096 | 129.47 | 15.461 | 17.582 | 15.988 | 0.967 | 1.100 | 1.2628 → 0.0770 (16.4×) |
| 37.39 | 4096 | 106.86 | 25.316 | 19.466 | 17.252 | 1.467 | 1.128 | 8.0910 → 0.0886 (91.3×) |
| 39.34 | 4096 | 274.12 | 48.025 | 21.252 | 17.647 | 2.721 | 1.204 | 31.549 → 0.1516 (208.1×) |

`m = 2` (rf2mt, \|F_x\| = 4096): every cell holds.

| log2 r | S / rho | CPU | mt | GPU | mt vs CPU | GPU vs mt |
|--:|--:|--:|--:|--:|--:|--:|
| 27.48 | 8 467.8 | 3.0680 | 0.6229 | 0.1531 | 4.925 | 4.069 |
| 29.99 | 10 858.4 | 14.441 | 2.8780 | 0.2021 | 5.018 | 14.243 |
| 31.26 | 22 784.0 | 39.898 | 7.8092 | 0.3228 | 5.109 | 24.196 |

The ranges quoted in the REPORT are confirmed as follows:

- **Relation phase at 37–39 bits.** On the CPU it is 7.83–31.5 s ("8–32 s"); on the device it is
  0.0886–0.245 s ("0.09–0.25 s"); the speedup is 69.4–208.1× ("69–208×").
- **Solve at \|F_x\| = 2048.** At 37.4 and 39.3 bits the GPU is 3.59× and 11.14× faster than one CPU
  thread, and 1.54× and 2.64× faster than the multi-threaded arm.
- **Solve at \|F_x\| = 4096.** The gain is 0.967–2.721×.
- **The pair table at \|F_x\| = 4096.** It takes 16 781 312 additions (= 4096 · 4097) on every arm.
  The probe sets hold 16 752 884–16 776 850 abscissae.
- **Linear algebra at \|F_x\| = 4096.** Medians over all arms are 2.76–11.16 s ("2.8–11 s").
- **Set-up at \|F_x\| = 2048, device-built against host-built.** gpuh is 1.917–2.058 s and gpud is
  1.282–1.457 s. The per-curve gap is 0.460, 0.708, 0.654 and 0.773 s ("0.46–0.77 s"). The per-run
  ranges are disjoint in all four cells.
- **GPU set-up above the CPU arm's.** At 2048: 0.380, 0.180, 0.095 and 0.229 s. At 4096: 0.661,
  0.735, 0.798 and 0.163 s.
- **The previous round** (`out_rm3.tgz`, my recomputation). The daemon set-up gap was 0.124–0.145 s
  at 1024 and 0.498–0.566 s at 2048 ("0.12–0.57 s").
- **CPU A/A.** The per-curve solve medians differ by +6.1%, +8.7%, +16.3% and −5.2%. ecbench's
  pooled A/A is 1.036 [0.915, 1.173]. "Up to 16%" holds.
- **S / rho.** 106.86–521.37 ("107–521×").
- **Wall against rho.** The best GPU arm in each cell is gpud. Against rho's 4-run median solve it is
  69.6, 44.7, 12.9 and 7.02× at 2048, and 232.9, 253.8, 76.1 and 42.3× at 4096 ("7–254×"). Rho's
  medians are 36.74, 62.99, 226.61 and 417.37 ms.
- **The daemon.** It started once, with the context ready at 0.382 s and listening at 0.383 s. The
  job shell timed 0.399 s from launch to the socket.
- **The benchmark record's metric values.** 32.639 → 2.929 s against rho 0.41737; 30.217 → 0.245;
  2.058 → 1.285; 7.809 → 0.323 against rho 0.01449; S 567.3 against 1.089. All match the raw
  records.

## Unsupported or imprecise statements

1. **Table cells 2.75 s, 3.42 s and 2.92 s** (CPU at 33.4 bits / 2048, multi-thread at 35.2 bits /
   2048, GPU at 37.4 bits / 2048). The medians are 2.7447, 3.4149 and 2.9149 s, which round to
   **2.74, 3.41 and 2.91**. RESULTS.md prints 2.745, 3.415 and 2.915, and the REPORT rounded those
   again. `fig_big.svg` shows the correct 2.74, 3.41 and 2.91. The ratios in those rows are
   unaffected. REPORT.pdf carries the same cells.

2. **"(69–208×; 13–33× at 33–35 bits)".** The relation-phase speedups at 33.4 and 35.2 bits are 13.4×
   and 32.8× at \|F_x\| = 2048, but **5.7×** and 16.4× at 4096, so the range is **5.7–33×**. The
   sentence's other ranges (8–32 s, 0.09–0.25 s, 69–208×) cover both table sizes; this one drops the
   4096 cell at 33.4 bits.
   - Every run in that cell splits its batches 4 times: `gpu_batch_splits = 4` on both the gpud and
     mt arms, 0 everywhere else in rf3big and rf2mt.
   - The multi-threaded arm there is slower than one CPU thread: relation phase 0.733 against
     0.398 s, solve 11.56 against 8.66 s.
   - The REPORT mentions neither.

3. **"creating the session takes a median 0.185 s with the device build against 0.780 s with the host
   build (daemon timing lines)".** I mapped the 240 `RF_SERVE_TIMING` lines one to one onto the 120
   daemon-arm records, run then replay. Order, `n_probe` and lookup code agree on every line.
   - **The 0.185 s pools unlike groups.** It is the median of all 144 lines with `lookup=3`. That
     group mixes:
     - the CUDA device build at \|F_x\| = 2048;
     - the CUDA device build at \|F_x\| = 4096;
     - the `backend=emulate` arms, whose `lookup=3` table is built **on the host**
       (`rootfind_api.h`: "Emulation builds it on the host"), at both sizes;
     - the audit replays.
   - **Like for like** (CUDA, \|F_x\| = 2048, 32 lines each), the create step takes:
     - device build: **0.044 s** median (0.028–0.066 s);
     - host build: **0.780 s** (0.574–0.842 s).
   - The other groups: emulate at 2048 is 0.333 s; CUDA device build at 4096 is 0.172 s; emulate at
     4096 is 1.352 s.
   - So the report **understates** the device build's effect on the create step: about 18×, not
     about 4×.
   - The benchmark record's metric note repeats the pooled 0.185.

4. **"Device-built against host-built table".** The two arms do not differ only in where the table is
   built.
   - `probe_build=host` sends `lookup=0` (AUTO). `rf::resolve_lookup` resolves AUTO to **SORTED**
     (an Eytzinger table) whenever `hash_capacity_for(n) > RF_SHARED_HASH_CAP` (4096), that is, for
     every probe set over 2048 keys. The daemon lines confirm `lookup=1` on all 32 rf3big gpuh INITs.
   - The "host-built" arm therefore builds and probes a **sorted** table. The device arm builds and
     probes a **packed hash** table.
   - The set-up difference (0.46–0.77 s) is new path against old path, as "What was built" says.
     It is not build location alone.
   - The relation phase also differs between the two arms: 0.054–0.364 s against 0.044–0.245 s.
   - A host-built hash at this size is not available through the Rust parameters.

5. **Two endpoints off by one in the last digit.**
   - "The GPU set-up is now **0.10**–0.80 s above the CPU arm's": the smallest gap is 0.0947 s, so
     **0.09–0.80 s**.
   - "rho's median solve (**36**–417 ms)": the smallest median is 36.74 ms, so **37–417 ms**.

6. **"the pair table (≈ 16.8M sums built on the CPU, 5.1–5.8 s, the same on every arm)".**
   - What is the same on every arm is the counted work: 16 781 312 additions.
   - 5.1–5.8 s is the **CPU arm's** whole oracle set-up (5.108–5.762 s), which there is the pair table
     alone.
   - On the other arms the set-up phase is longer: GPU 5.769–6.417 s, multi-threaded 7.162–7.540 s.
     The multi-threaded arm builds a 16.8M-key hash single-threaded on the host, with a worker create
     median of 1.35 s.
   - The pair-table share is not timed separately on those arms.

7. **"It is the worker's host emulation: the kernel's per-thread function on 96 host threads, with a
   thread pool created for every batch, and it reaches only about 5× one thread."**
   - **The facts hold.** `rf::emulate` (`rootfind_host.hpp`) creates and joins
     `hardware_concurrency()` threads (96 on the pod) on every call, that is, per batch. At m = 2 the
     solve is 4.93–5.11× and the relation phase 5.2–7.0× the CPU arm.
   - **"About 5×" is m = 2 only.** At m = 3 the multi-threaded arm's solve is 0.75–4.22× the CPU arm,
     and its relation phase 0.54–10.2×.
   - **"One thread" is the Rust `rootfind-subtract` oracle**, a different implementation from the
     emulated kernel, so 5× is not a parallel-efficiency figure.
   - **The juxtaposition suggests the thread pool is the reason; the data do not support that as the
     main cause.**
     - Creating and joining 96 threads costs 3.4–4.4 ms per batch, measured on this 4-CPU host
       (`bench/tpool.cpp`).
     - A multi-threaded batch at m = 2 takes 35.6–47.4 ms (7–172 batches per run), so thread
       creation is at most about 10% of it.
     - From the records' own counters, the kernel sweeps all 2\|F_x\| candidates of every swept
       target. The emulation sweeps 1.8–2.5M candidates/s per logical thread (3.7–4.9M per physical
       core).
     - The single-thread Rust arm charges 31–33M candidate additions/s.
   - So the per-core throughput of the emulation code is about 1/7–1/9 of the Rust oracle's. That,
     not thread creation, is the larger factor. The cause of the low throughput is not established.
   - "A tuned parallel CPU sweep would close part of the 4–24× gap" is labelled as unmeasured; it is
     an interpretation.
   - "Not done: … (persistent thread pool)" would, on these numbers, recover at most about 10% by
     itself.

8. **"`ecbench` built locally from `7417c99292` on a clean tree (the benchmark record was initialised
   on that commit), `rootfind_serve_cuda` built on the pod from the same tree".**
   - **The binary hash is bound.** It matches all 240 records, the receipts, the pod's `sha256.txt` and
     the local `target/release/ecbench`. That binary contains the code strings new in `7417c99292`.
   - **The clean-tree statement is not verifiable from the artifacts.**
     - Every session has `git_commit: null` and `git_dirty: null`, and `host.json` reads "commit ?".
     - The record's `commit_dirty: false` was taken at 04:32:41Z. The binary's mtime is 04:42:54Z,
       ten minutes later.
   - **The pod-built worker's sources are not hashed** in the tarball. I could not reproduce
     `rootfind_serve` (`bcda4b3b…` on the pod) bit for bit, since compiler and paths differ.

## Code review (claim 5)

Reviewed at `7417c99292`: `gpu/rootfind/rootfind_cuda.cu` (`k_slots_fill`, `k_slots_insert`,
`rf_cuda_session64_create`), `rootfind.cuh` (`HashTable` packed probe, probe-mode emission),
`rootfind_host.hpp` (`build_hash`, `hash_capacity_for`, `resolve_lookup`, `emulate`),
`rootfind_api.cpp` (`rf64_session_create`, `resolve_backend`), `rootfind_serve.cpp` (INIT) and
`src/cryptanalysis/ic_framework/rootfind_gpu.rs` (`prepare`, `sweep_batch`, `decompose`), plus
`plugins.rs` `RootfindSubtractOracle::{decompose, charge_candidate, lookup}`.

**Can a device-built table change a returned relation or a charge?** Only by omitting a key (a false
negative), and the construction prevents that.

- **Completeness.**
  - `hash_capacity_for(n)` is the next power of two at or above 2n, so the load factor is at most
    1/2 and every insert loop terminates.
  - `k_slots_insert` inserts each key `j` by linear probing from `rf_hash64(key)`. It uses one
    `atomicCAS` against the empty value `(u64)RF_NO_INDEX`, the same empty encoding the host builder
    leaves.
  - There are no deletions. Every slot a key skipped was occupied and stays occupied, so a probe
    from `rf_hash64(key)` that stops only at an empty slot reaches the key. This holds whatever the
    insertion order.
  - Errors are checked after both launches: `CUS(cudaGetLastError())` and
    `CUS(cudaDeviceSynchronize())`.
  - Duplicate keys cannot arise, because the pair table's map keys are unique. If one did, the Rust
    side ignores a second hit at the same position.
- **No false positives reach a relation.**
  - The packed probe matches on a 32-bit tag. In probe mode the kernel then compares the full
    abscissa (`F::eq(x3, probe_x[j])`) before emitting.
  - The Rust side uses only the hit **position**. It ignores the device's `j`, recomputes the
    candidate on the CPU, and returns the CPU pair table's own `probe` result. Positions are visited
    in the CPU's order after `sort_unstable`, so slot order is irrelevant.
  - A reported position that the CPU probe rejects is charged `charge_candidate` plus the CPU probe.
    That is exactly what `rootfind-subtract` charges at that position. Unreported positions are
    charged by `charge_misses`, again as the CPU charges them.
- **Observed.**
  - In every device and emulate record, the hits examined on the CPU (`gpu_hits_confirmed_on_cpu`)
    equal the relations found, apart from relations taken by the CPU-fallback targets. No false
    positive was ever examined.
  - All of these match the CPU arm exactly:
    - the 36 pairs with a CUDA device-built table (smoke 4, rf3big 32);
    - the 36 m = 3 emulate pairs (smoke 4, rf3big 32);
    - the 20 CUDA host-built pairs (rf3big 16, sorted table; smoke 4, host hash, since 1 021 keys
      fit the hash cap).
- **Residual risk.** An incomplete device table would change relations and charges. Within a session
  it would show up as a pair mismatch. It cannot be caught by a CPU-only replay, because those
  records cannot be replayed without a device (below).

**Does `backend=cuda` fail closed?** Yes, in the committed worker.

- `resolve_backend` returns the requested backend unchanged when it is not AUTO.
- A build without CUDA returns `RF_ERR_NO_CUDA` (−2). In a CUDA build without a device, the first
  failing CUDA call inside `rf_cuda_session64_create` makes `CUS` return `RF_ERR_CUDA`. I read the
  second path but did not exercise it.
- `RootfindWorker::init` turns any non-zero status into an error, so `prepare` fails. A batch error
  panics in `decompose`. There is no CPU fallback.
- The old INIT protocol already passed `backend` and `lookup` in `w[0..1]`. An older worker would
  honour `backend=2`, and would reject `lookup=3` with `RF_ERR_BAD_ARG`.
- **Reproduced.** All 12 `backend=cuda` records in smoke fail on my host-only daemon with
  `rootfind worker: init status -2`. The same worker reproduces the `backend=emulate` and CPU records.

Notes:

- `RootfindWorker::init` stores the reply's `backend_used` but never compares it with the request,
  so fail-closed rests on the worker alone. Hardening would mean checking `w(2) == backend` when
  `backend != 0`.
- The comment in `prepare` ("a replay on a host without a GPU must read the same record") is now
  false for `backend=cuda` records. 72 of the 240 records (smoke 12, rf3big 48, rf2mt 12) cannot be
  replayed on a CPU-only host.
- `job.sh`'s comment "The backend is not in the records" is stale: the arm's method parameters now
  carry `backend=cuda`. The `job.sh` check itself tests device presence (`rf_backend_info`), not the
  session's resolved backend. The arms' `backend=cuda` enforces the latter.
- `probe_build=device` under `backend=emulate` builds the packed hash on the host. The
  multi-threaded arm's "device" table is a host table.

## Process and reporting-contract gaps (not errors in the numbers)

- **The uncharged device and emulation work is not named.** "Counted S is unchanged by design" is
  true. But the device and emulate arms sweep every candidate of every swept target, beyond the
  first hit, and that work is outside S and outside the §5a band:
  - `gpu_targets_unused` is 18–986 per m = 3 run in rf3big, 76–807 per m = 2 run in rf2mt, and
    44–897 in smoke;
  - 4 batch splits per run occur at 33.4 bits / 4096;
  - the daemon's 96-thread or device CPU time is not in the records' process accounting. The CPU
    arms are single-threaded: threads 1 → 1, CPU/wall 1.00.

  The previous review asked for these counts to be named. This report does not name them.
- **ecbench's wall verdict is omitted.**
  - Every `compare-*.txt` reads "measurement quality: 0 / 100 … admitted 0". Its pooled wall ratios
    agree in direction with the medians:
    - gpud/cpu 0.226 [0.138, 0.373] at 2048 and 0.609 [0.471, 0.787] at 4096;
    - gpud/gpuh 0.779 [0.742, 0.818];
    - mt/cpu 0.372 and 0.722 (m = 3);
    - rf2 mt/cpu 0.1955 [0.193, 0.198];
    - rf2 gpud/cpu 0.0124.
  - There is no A/A copy of any GPU or multi-threaded arm, and none at \|F_x\| = 4096 or in rf2mt.
- **The §5a band is not stated.** I recomputed it with my own implementation of `band.rs`'s rule
  (`cost.unpriced` counts, plus `table_insert_attempts` behind its `counted:` marker, at
  w = 0, 0.1 and 1), with every arm priced, rho's `canonicalisations_uncharged` included:
  - m = 3: IC/rho is **106.9–946.1×**;
  - m = 2: IC/rho is **8 468–22 867×**;
  - rho is ahead at every weight in every cell.

  The REPORT's "at w = 0" is labelled correctly. The rho reference is a 4-run mean whose per-curve
  S is 0.73–1.53 against a floor of 0.886. The jump from 188× to 521× (2048, 37.4 → 39.3 bits) is
  IC S × 1.97 and rho mean S × 0.71.
- **The raw data are not in S3.**
  - The record has `uploaded: false`; `head-object` on the record's `s3_uri` gives HTTP 403 from
    here.
  - The 223 KB `.tgz` is committed to Git, with no `.artifacts.json` manifest. AGENTS.md §13 routes
    archives over 64 KiB to S3.
  - RESUME.md asserts "same waiver as earlier rounds". I found no evidence of a waiver.
  - The sessions exist only inside the tarball, so CI's cross-host re-audit (§12) cannot see them.
- **The pod run was not launched through `benchrec run`.** The record's own summary discloses this;
  the REPORT does not.
- **`reports.pdf` in the record names `REPORT.md`**, not `REPORT.pdf`.
- **The curve generator's seed is not recorded** in the spec or the study. It is 20261115.
- **Leaderboard and gate are respected.** `5897026a2f` adds no leaderboard, `benchmarks/README.md` or
  `VALIDATION.json` row. That is consistent with the validation gate.

## Not checked or not checkable

- The pod id, cost (about $3.10) and deletion. The window is supported only at its edges: the first
  artifact is 04:43:43Z and the last log line 06:10:11Z.
- "checked by a brute-force count at 20 bits and by [N]Q = O for 20 random points at each size". No
  log of these checks is committed. My own proof above supersedes them.
- "Tests (host worker): m = 2, m = 3, daemon, `backend=emulate` and both probe builds equal
  `rootfind-subtract`". I did not build or run the crate's tests. The test code exists and matches
  the description. None of the tests can exercise the CUDA `k_slots_insert` path without a GPU.
  The REPORT says the first CUDA run of it is the smoke session, and it matches there.
- Whether the A100 wall times reproduce. I have no GPU: my replay checks counts, not wall, and
  cannot run the 72 `backend=cuda` records.
- Whether the build at 04:42:54Z was from a clean `7417c99292` tree. I did not rebuild bit for bit:
  that needs the original path and the repository's target directory, which this review was not
  to touch, and about 2 GB of the 4 GB free.
- S3 contents (access denied).
