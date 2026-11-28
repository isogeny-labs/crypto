# Validation review: `research/ic_gpu_rootfind_setup_20261115/REPORT.md` (round 6)

Validator, 2026-11-15. I did not write the code, the protocol, the specs, the job or the report.
Standing of this review: `[pcc:unverified]`. No pcc claim is registered for it.

**Disposition: `validated-scoped`.**

Every certificate holds, the identity claim holds, and every number in REPORT.md and RESULTS.md
recomputes from the raw records with my own code. The §5a band values match both `ecbench band` and my
own implementation of the `band.rs` rule. The 64 worker-free records I re-executed on another host
reproduce identically. `rf_pair_run` is correct by code review, by the producer's host test (which does
compare every cell it covers) and by a harness of mine over 1.07 million cells. No measured number is
refuted. [pcc:unverified]

The scope is narrower than the report's wording in three places. The "7% lower" set-up figure compares
two sessions that ran on different host CPUs and different physical GPUs, which the report does not say
(finding 1). The 67 MB host copy is the largest timed stage, but 36% of session creation is outside every
timed stage, and the code shows a second host copy there (finding 2). The round-5 session-creation
baseline is a pooled figure, not a like-for-like one (finding 3). Prediction 3's failure is not affected
by any of these. The GPU records cannot be replayed without a CUDA device, so their walls and the kernel
timing rest on the pod's own same-host audit and the pod's `--bench-pairs` output. [pcc:unverified]

## What was validated

| item | value |
|---|---|
| validated objects | `23c5295b3b47fdac5f043e19394a2c82808f1e77` (HEAD at dispatch). The study directory and the record `benchmarks/records/20261115T2225Z-ic-gpu-rootfind-setup.json` are byte-identical to commit `2d7dbac206` (empty `git diff`). REPORT.md is blob `28cb90b06d`, sha256 `61235f3d2ee3…`. The working tree holds unrelated edits by other agents (`linalg.rs`, an untracked `ic_m3_cost_leads_20261115/`); neither is an input here |
| measured revision | `0c8c431185` (`REV` in the tarball). HEAD's `gpu/rootfind` differs from it (round 7's commit `0132f05c93` changed `rootfind_cuda.cu` and `rootfind_api.cpp`), so I read the measured sources with `git show 0c8c431185:gpu/rootfind/…`, not from the working tree |
| raw tarball | `raw/out_rsu.tgz`: 120 837 B, sha256 `2967f6cc2d8eee1b18f8c61b7118ec72a639d2214ed6070f49fa2726006ce142`. Equal to `raw/out_rsu.tgz.sha256` and to the record's `raw` entry. 42 entries (36 files, 6 directories), no links, no absolute or `..` paths. Extracted into a fresh scratch directory |
| sessions | `smoke` (`ECBS1hd4926a44a330`, 36 records) and `rf3setup` (`ECBS1hbd69b299e3b6`, 84 records); every record has status `verified`, level L0, no warm-ups, contiguous `seq`, one binary (`0e228c222d2b…`) and one environment class. `records.jsonl`, `plan.json` and `host.json` hash to the values in `session.json`. `git_commit` and `git_dirty` are `null` in both |
| pod | one A100 80GB PCIe (sm_80), AMD EPYC 9374F, 128 logical CPUs, cgroup v1 quota 13.6 CPUs, driver 595.91.07, nvcc 12.8.93; `daemon.log`: backend `cuda`, context ready 0.285 s |
| my tools | the dispatched `ecbench`, copied before use to the scratch directory, sha256 `0e228c222d2b41c0397087caf113408cdaef053c515af0534a3eb1ffdd21a9fc`; my own Python (`certs.py`, `identity.py`, `band_check.py`, `results_check.py`, `pair_stages.py`, `pair_split.py`) and C++ (`own_pair_test.cpp`) under the scratch directory `validator-r6/` |

## Requirements

| # | requirement | result | evidence |
|---|---|---|---|
| 1a | Protocol and specs pushed before the run | **pass** | GitHub activity API: `0c8c431185` pushed 22:19:12Z (protocol, specs, `job.sh`, code), the record-open commit 22:26:13Z, `ecbench.sha256` at 22:26:21Z. `run_pod.sh` started 22:25:58Z; the pod's first session started 22:39:21Z (`session.json`). Protocol, specs, `job.sh`, `run_pod.sh` and `RESUME.md` are unchanged from `0c8c431185` to HEAD |
| 1b | Session specs equal the committed specs | **pass** | `sessions/*/spec.json` sha256 `47a2807dae41…` (smoke) and `21f24185c60e…` (rf3setup) equal `specs/*.json`. `ecbench plan` on the committed specs gives `ECS1h60e81602bdce` and `ECS1h1dd5793e0877`, the ids in `session.json` |
| 1c | Tarball hash | **pass** | above |
| 1d | Dispatched `ecbench` hash equals the hash in the tarball | **pass** | `ecbench.sha256`, `sha256.txt` in the tarball, `binary_sha256` of all 120 records and the file `target/release/ecbench` at my dispatch all read `0e228c222d2b…`; the file's mtime is 22:25, before the launch |
| 1e | Binary bound to a commit | **not verifiable** | Finding 10 |
| 2a | Certificates, all 120 records, with my own EC arithmetic | **pass, 120/120** | `certs.py`. p and r prime; a, b read from the construction string (explicit curves) or solved from G and the first target and checked against the ICV1 j-invariant (the searched smoke curve); nonsingular; trace, order and ICV1 fields consistent; G and Q on the curve; r·G = O; r > 4√p so #E = r; recovered·G = Q; recovered = planted mod r; flags consistent. Tampered copies (recovered + 1, Q moved, planted + 1) all fail |
| 2b | Planted scalars | **pass, 120/120** | Re-derived from `uniform_scalar_sha256_v1` (model hash, r, seed, index) with my own SHA-256 code, read from `canonical.rs::derive_u64`, not from the producer |
| 2c | Replays | **same host 120/120; cross-host, worker-free: 64/64 `identical`** | My replays: `ecbench verify --replay-seq` of every rho, rho A/A and CPU IC record (smoke 16, rf3setup 48), same binary, hardware class `ECBHW1hd21b654a34b7` (pod: `ECBHW1h2bf7d41a6ddc`). Both receipts `ok`, 0 problems. The 36 GPU-arm records of rf3setup (and 20 daemon-arm records of smoke) cannot be replayed without a device |
| 3a | Identity: GPU arms against `rf3-cpu-echelon`, every charged field | **pass, 36 comparisons, 0 mismatches** | `identity.py`: a full recursive diff of every record pair (not only the producer's fields). Phase adds, doubles, scalar mults and GAE (720 phase-field checks), trials, relations, rank, rows, recovered, status, S, total GAE, factor-base hashes, every `native` and `work_native` map and the band inputs are equal. The only differences are the four `gpu_*` diagnostics, the label and decomposition name, and `work.complete` / `work.unpriced` (bitop/v1 lists the `gpu_*` counters as unpriced) |
| 3b | Band (§5a) | **pass** | Finding 9: `ecbench band`, default and `--no-price-table-work`, and my own implementation of the rule, agree to the last digit on all 14 rows |
| 4 | Every number in REPORT.md | **pass for every number; findings 1–5 on how some are read** | "Confirmed numbers" |
| 5 | Code review of `rf_pair_run`; host test coverage | **pass, with minor notes** | Findings 6 and 7 |
| 6 | Wording: failed prediction and cause | **prediction 3 stated plainly; cause stated more firmly than the data allow** | Findings 1 and 2 |

## Confirmed numbers

Recomputed from `records.jsonl`, `daemon.log` and `bench-pairs.txt` with my own code. Medians are over 6
runs per (curve, arm). [pcc:unverified]

- **`--bench-pairs` (device; not replayable here).** The file reads n = 1024: 1 049 600 keys, 0.0004 s
  against 0.0001 s, 4.0×; n = 2048: 4 196 352 keys, 0.0012 against 0.0002, 5.1×; n = 4096: 16 781 312
  keys, 0.0047 against 0.0008, 5.7×; mismatches 0 at all three. Prediction 1 (0 mismatches; at least 5×
  at 4096) holds as printed. [pcc:unverified]
- **Table, all cells hold to the printed digit.** GPU `gpup` solve: 0.115 and 0.220 s at 2048, 0.255 and
  0.292 s at 4096. CPU echelon solve: 4.251, 15.659, 5.488 and 15.512 s. Relations / set-up / LA of
  `gpup` at 4096: 0.035 / 0.116 / 0.096 s (37.4 bits) and 0.075 / 0.109 / 0.104 s (39.3 bits). Rho median
  solve 0.0325 and 0.0690 s. `analyze.py` run on my extraction reproduces `RESULTS.md`, `results.json`
  and `fig_stages.svg` byte for byte. [pcc:unverified]
- **S / rho at w = 0 / 0.1 / 1** (own band rule, equal to `ecbench band`): 389.9 / 391.7 / 399.8
  (37.4 bits, 2048), 220.3 / 233.3 / 291.8 (37.4 bits, 4096), 684.5 / 685.4 / 689.1 (39.3 bits, 2048),
  351.8 / 357.2 / 381.8 (39.3 bits, 4096). The report's ranges 220–685 / 233–685 / 292–689 hold. Rho's S
  (mean, w = 0 / 0.1 / 1) is 0.7435 / 0.8177 / 1.4859 and 0.8274 / 0.9101 / 1.6542; the top-level counter
  `canonicalisations_uncharged` is priced. Rho stays ahead at every weight under both readings
  (`--no-price-table-work` lowers the IC rows at w = 1 only, for example 433.6 to 394.1). The default
  band JSON from my run is byte-equal to the pod's `band-rf3setup.json`. [pcc:unverified]
- **A/A.** GPU solve per pair 0.868–1.131 (n = 12), as reported. Not in the report: GPU set-up per pair
  0.766–1.440 (n = 12) and rho solve per pair 0.998–1.007 with identical counts (n = 12). [pcc:unverified]
- **Stage medians** (`pair_stages.py`, 4096, n = 48): keys 1.9, dedupe 6.9, compact 5.1, probe 2.9,
  copy 47.3 ms (my median of the 48 values prints 47.2 or 47.3 depending on rounding). At 2048 (n = 24):
  1.3 / 2.0 / 2.0 / 1.6 / 15.5 ms. Session creation at 16.8M keys: median 0.1050 s (n = 24) and 0.1052 s
  (n = 24); at 4.19M keys 0.0341 and 0.0375 s (n = 12 each). [pcc:unverified]
- **Set-up ranges.** `gpup` at 4096: 0.1156 s (37.4 bits) and 0.1095 s (39.3 bits), so "0.109–0.116 s"
  holds; round 5's same cells are 0.1215 and 0.1212 s, so "0.121–0.122 s" holds. [pcc:unverified]
- **Provenance counts.** Pod audits: 36/36 and 84/84 records with replays reproduced, 0 problems. The
  daemon log has 112 `timing init` lines and 80 `timing pairs` lines. [pcc:unverified]

## Findings

1. **The "7% lower" set-up figure is a cross-session comparison across different hardware; the report
   does not say so.** Round 5's pod had an AMD EPYC 7543 and an A100 with serial 1323222022766 (VBIOS
   92.00.90.00.0F). Round 6's pod had an AMD EPYC 9374F and an A100 with serial 1650823006223 (VBIOS
   92.00.A0.00.05). Driver (595.91.07), nvcc (12.8.93), cgroup quota (13.6 CPUs) and PCIe generation (4)
   are the same. The report calls the comparison "cross-session" and "indicative", but the title says
   "set-up only 7% lower". By the `gpup` arm's medians the change is −4.9% (0.1215 to 0.1156 s) and
   −9.7% (0.1212 to 0.1095 s); the report's 7% is their mean. Pooling the A/A copy (12 runs per curve)
   gives −10.4% and −11.5% (11 of 12 and 12 of 12 round-5 runs lie above round 6's median), and the
   record's own baseline (0.121 against 0.109) gives 9.9%, while the progress report prints 10.66%. The
   in-process benchmark predicts a saving of about 3.9 ms (4.7 to 0.8 ms), roughly 3% of 0.121 s, so
   most of the cross-session change is not explained by the kernel. The host-side stages are the ones a
   change of host CPU would move. There is no same-session arm with the old kernel in the set-up path.
   Within one round, A/A medians of 6 differ by +7.0%, +1.7% (round 5) and −3.0%, −0.5% (round 6).
   Reading: the direction is consistent, the magnitude cannot be attributed to the kernel, and the
   failure of prediction 3 (at most half of 0.12 s) is robust, since no more than 12% was observed.
   [pcc:unverified]

2. **The 67 MB copy is the largest timed stage, and the report names it as the cause; the stage accounting
   is incomplete.** At 4096 the per-session median of (creation − sum of the five stages) is 37.4 ms
   (range 34.5–53.8): stage-sum median 67.0 ms against creation median 105.2 ms, so 36% of creation is
   in no stage; at 2048, 13.3 of 36.5 ms. The figure (`fig_stages.svg`) stacks the stage medians to 64.1
   ms and shows no remainder, although its title asks where the build spends its time. At the measured
   commit, `rf64_session_create_pairs` (`rootfind_api.cpp:350,354`) also does `s->pairb.assign(pb, pb +
   n_entries)` and `std::free(pb)`: a second pass over the same 67 MB into a fresh `std::vector`, outside
   the five marks, inside "create". It is a candidate for part of the gap; I did not measure it. Also:
   the stage called "keys" starts before six session-buffer allocations, two host-to-device copies and the
   134 MB key-buffer allocation, so it is not kernel time (1.9 ms against 0.8 ms in the bench); and the
   stage marks add five `cudaDeviceSynchronize` calls to every daemon session (the daemon runs with
   `RF_SERVE_TIMING=1`), which round 5's sessions did not have. By arithmetic on the medians (an extrapolation, not a
   measurement), removing the whole 47 ms copy would leave 0.0683 s and 0.0622 s, still above prediction
   3's 0.06 s. Reading: "the hypothesis that the key kernel bounded set-up is refuted" holds (the old
   kernel is about 4.7 of about 105 ms). "The stage timings name the 67 MB host copy" holds as the largest
   timed stage (47 of 67 ms staged, 45% of creation). Calling it the cause of the remaining set-up is a
   hypothesis that round 7 (which removes both host copies at HEAD) tests; the report's commit title,
   "bound by the 67 MB host copy", reads stronger than that. [pcc:unverified]

3. **The round-5 session-creation baseline is not like for like.** The report's "(round 5: 0.112 s)"
   equals the median over all 128 pairs-mode sessions in round 5's daemon log (0.1124 s), a mixture of
   sizes. Round 5's 16.8M-key sessions alone (72, over both curves) have a median of 0.1186 s. Round
   6's like-for-like figure is 0.1051 s (n = 48), 11% lower, not 6%. The direction is unchanged.
   [pcc:unverified]

4. **The stage and creation medians pool the timed run with the audit replays.** Each GPU record created
   two daemon sessions (the run and its replay in the pod's audit), so "median of 48" is 24 run-phase and
   24 replay-phase sessions. By log order, run-phase only (24): creation 108.1 ms, copy 51.9 ms,
   stage-sum 69.8 ms; replay-phase only (24): creation 98.2 ms, copy 43.2 ms, stage-sum 61.0 ms. The
   records' set-up walls come from the run phase. A 9% difference in creation between two phases of one
   session is of the same size as the cross-session change in finding 1. [pcc:unverified]

5. **`--bench-pairs` is one sample per size.** Each size runs both kernels twice and keeps the second
   time, with no repeat and no spread (the benchrec metric has `n = 1`, `spread = null`). The device
   times print to four decimals; the 5.7× is the harness's own ratio of unrounded times, while 0.0047
   over 0.0008 gives 5.9×. The printed ratios of the rounded times lie in [5.5, 6.3]; the harness's 5.7× is
   14% above prediction 1's 5×, on one sample. The comparison is the kernel against itself on a random base
   over the 40-bit field, not against a round-5 in-daemon timing. The mismatch count compares the two
   device kernels with each other; the batched kernel against `rf_pair_x` was checked on the host (finding
   7). [pcc:unverified]

6. **Code review of `rf_pair_run` (`rootfind.cuh:231–280`, `rootfind_cuda.cu:537–549`): correct.** Index
   arithmetic: `rf_pair_offset(ai, n) = 2 (ai n − ai (ai − 1)/2)` is twice the sum of row lengths before
   row `ai`, and the cell is `base + 2 (bi − ai) + sg`, matching the documented layout; my harness checks
   `rf_pair_offset(ai, n) + 2 (bi − ai) + sg` against a running counter for every cell, and the layout
   covers `n (n + 1)` cells exactly once. Run boundaries: the kernel launches `n × ceil(n/16)` threads,
   each computing `b0 = ai + r·16` and returning when `b0 ≥ n`; row 0 needs `ceil(n/16)` runs, so no run is
   dropped, and runs of a row are disjoint. The last run has `m = n − b0` entries. Batched inversion:
   after the forward pass `inv = (Π d)⁻¹`; the backward pass takes `1/d[k] = inv · pre[k−1]` and then
   `inv ← inv · d[k]`, reading `d[k]` before overwriting it. Diagonal: the denominator is `2 y`, the
   numerator `3 x² + a`, `x(2P) = λ² − 2x`, and sign 1 (P − P) is `RF_PAIR_EMPTY`, as in `rf_pair_x`.
   Off-diagonal: both signs share the denominator `x_b − x_a`, with `y₂` negated for sign 1. Zero
   denominator: the bad cell's denominator is replaced by 1 so the product stays invertible, and both of
   its signs are recomputed with `rf_pair_x`, so the keys equal the old kernel's even for degenerate
   input; a base of distinct abscissae with `y ≠ 0` never reaches it (`rf64_session_create_pairs`
   rejects `y = 0`; distinct abscissae come from the factor-base construction). `RF_KEY_EMPTY` (`rootfind_cuda.cu`) and `RF_PAIR_EMPTY` are both `~0ull`. Minor: the
   comment above `k_pair_keys` ("one thread per (ai, bi) cell") describes the old kernel; nothing asserts
   `RF_PAIR_CHUNK ≤ 32`, which the `1u << k` bit mask needs; the two 16-word arrays go to local memory on
   the device (performance, not correctness). [pcc:unverified]

7. **The host test compares every cell it covers; the coverage is narrow.** `test_pair_runs`
   (`test_cpu.cpp:505`) runs n ∈ {1, 15, 16, 17, 37, 50} over p61, p40 and p20, builds the table with
   `rf_pair_run` over every `(ai, b0 = ai, ai + 16, …)` and compares each of the 4 776 cells per prime
   (14 328 in all) with `rf_pair_x`, counting `b` against `n (n + 1)`. Row lengths 1–50 cover every
   remainder mod 16. I rebuilt the host test from the measured sources with `g++` (no CUDA): ALL PASS.
   Seven mutations of mine (backward-pass index, run length 15, diagonal denominator, no zero-denominator
   fallback, same sign for both, P − P written as 0, shifted write index) fail 12 996, 528, 408, 30,
   6 741, 408 and 240 checks (the last also corrupts the heap), so the test is sensitive. The producer's
   "1 230 checks" cannot be tied to a mutation (none is given). Weaknesses: the key buffer starts at 0, not a sentinel, so an unwritten
   cell would pass only where the expected key is 0 (probability about 1/p per cell); no guard against
   writes past the end; n stops at 50; no `y = 0` diagonal (unreachable in production). My harness
   (`own_pair_test.cpp`: sentinel fill, guard words, n = 1…70, three primes, a duplicated abscissa and a
   duplicated point, an independent `__int128` affine reference) compared 1 073 520 cells with 0
   mismatches against `rf_pair_x` and 1 050 633 against the affine reference with 0 mismatches; the first
   five mutations (the ones I ran against it) fail it. The Rust pairs tests the report mentions were not run here. [pcc:unverified]

8. **Identity is wider than the producer's check, and holds.** `analyze.py` compares trials, relations,
   recovered, status, S, total GAE and each phase's four charged fields, indexing with `[]`. My full diff
   (requirement 3a) shows nothing else differs apart from the `gpu_*` diagnostics, labels and the bitop/v1
   `work.complete` flag (false for GPU arms, because the `gpu_*` counters are listed unpriced). The
   report's S/rho and GAE-based readings do not use bitop/v1, so the flag does not touch them, but a
   reader should know the GPU arms' bitop/v1 total is marked incomplete. [pcc:unverified]

9. **The band is read from `ecbench band`, not recomputed by the script (lesson L-546d37ca7750).**
   `analyze.py` reads `mean_s` from the pod's `band-rf3setup.json`; I regenerated that file with the same
   binary (byte-equal) and recomputed every `mean_s` from the records with the rule of `band.rs`
   (`total_gae + w · U`, `U` from `cost.unpriced` in the top-level counters then the phase `native` maps,
   plus counted `table_insert_attempts`), default and without table work: maximum relative difference 0
   on all 14 rows. Rho's `canonicalisations_uncharged` is priced. [pcc:unverified]

10. **The binary is not bound to a commit.** `git_commit` and `git_dirty` are `null`; the binary embeds no
    commit string; its mtime (22:25) is after `0c8c431185` (22:18:43). The dispatch hash chain is intact
    (recorded 22:25:58–22:26:21Z, equal in the tarball, the records and my copy), and `0c8c431185` touched
    no Rust file, but the Rust sources the binary was built from are unknown. The kernel-side claims
    depend on the `gpu/rootfind` sources built on the pod, which `REV` and `git archive` bind to
    `0c8c431185`; the counted S, the relations and the oracle's charge also depend on the binary's Rust
    side. The replays I ran used this same binary, so they bind records to the binary, not the binary to
    a commit. [pcc:unverified]

11. **Raw data and sessions are in Git, not S3.** `out_rsu.tgz` (120 837 B) is an archive over the 64 KiB
    data-file threshold; the record has `uploaded: false`; the report states the waiver (AWS keys rejected).
    Sessions are inside the tarball, not under a `sessions/` directory. [pcc:unverified]

12. **Record-keeping and wording notes.** (a) The report omits rho A/A (0.998–1.007, identical counts),
    which the protocol lists. (b) The record's `host` block describes the launcher VM (Intel Xeon, 4
    CPUs, isolation unstated), not the pod. (c) `reports.pdf` in the record points to `REPORT.md`, not
    `REPORT.pdf`. (d) `run_pod.sh` started 22:25:58Z with a wall of 1 331 s, so it ended near 22:48Z; the
    report says 22:49Z. (e) PROTOCOL.md says creation is "linear in the number of pairs"; the data give 3.1×
    for 4× the keys (0.0341 to 0.1050 s). (f) The report's per-row S/rho table exists only in RESULTS.md,
    which has a ratio column but no correctness column (the verified counts are a status line above the
    table), and no floor or boundary ratio; REPORT.md states ranges. None changes a number.
    [pcc:unverified]

## Reporting contract

One unit (S in GAE with the §5a band), the matched rho priced alike (its `canonicalisations_uncharged`
at w), the baseline CPU arm and the A/A copies present as rows, ratios against rho in RESULTS.md,
phases separated (set-up, relations, linear algebra, solve), and every record carries a certificate that
I re-checked (RESULTS.md shows the counts as a status line, not a column, finding 12f). The m = 83 and ECC2K-130 disclosure rules do not apply (prime curves of 38 and 40 bits). The
report adds no leaderboard row, as it states. Wall figures are L0 on a shared pod and are descriptive,
as the report says. [pcc:unverified]

## Not checked

- GPU records were not replayed (no CUDA device here). The 36 GPU rf3setup records and 20 daemon-arm
  smoke records rest on the pod's own same-host audit (all identical) and on identity with the CPU arm.
- The CUDA kernels were not compiled or run; the shared header `rootfind.cuh` was compiled for the host
  only. Device-versus-host agreement rests on the pod's `--bench-pairs` (0 mismatches) and on identity.
- The Rust pairs tests ("pass on it") were not run (no cargo build, disk).
- Which Rust sources `ecbench` was built from (finding 10).
- That the pod was deleted, its cost, and the pod-side clock times beyond the logs.
- `REPORT.pdf` against `REPORT.md`; the regenerated `benchmarks/progress/` files beyond the one row read.
- Round 7 (`research/ic_gpu_rootfind_devmap_20261115`): only its diff to `rootfind_api.cpp` was read; its
  record says the first launch found no A100 and ran nothing.
- The lessons ledger (`tools/lessons.py outcome`, `test`) was not updated: the dispatch forbids creating
  repository files other than this review, so the main session should record the outcome. Lessons applied:
  L-546d37ca7750 (band recomputed), L-99ee952350b2 (keys indexed with `[]`; my REQ list and the producer's
  `analyze.py` both raise on a missing key), L-3f39e1d8b3a9 (binary hashed and copied at dispatch; binding
  reported unverifiable), L-26895912385b (ratios recomputed from unrounded values where they exist; the
  bench ratio exists only rounded, finding 5).
