# Validation review: rotating sweep window for m = 3 rootfind-subtract

Validator review of `research/ic_m3_sweep_window_20261118` (PR #4739, branch
`claude/index-calculus-gpu-optimization-otouqv`), 2026-11-18.

- **Validated commit:** `7fc37ffed7a3a19ae0a002588e444e6f635e857d`, read from the object store with
  `git archive`, not from the working tree. The branch has since moved to `3f9d6cbb4b`
  ("record the replication rerun"). That commit adds 17 lines to the bench record and 5 lines to
  `REPORT.md`, and **this review does not cover it**.
- **Disposition: `validated`**, scoped to the counted-S claim. At w = 0, `sweep=1/16` brings mean S
  to 0.360 to 0.554 of the full sweep in the four (curve, |F_x|) cells. Every f < 1 arm is below its
  f = 1 arm on every (workload, round) pair, and that order holds at w = 0, 0.1 and 1 under both
  pricings. All recovered logarithms pass an independent check.
- **Findings F1 to F5 are wording and reporting corrections.** Make them before the numbers are cited
  outside the study, for example on a leaderboard row. F6 to F8 are process gaps. None of them
  changes a measured number or a verdict.
- **Claim status:** the claim is still `[pcc:unverified]`. This review is evidence for a pcc claim; it
  is not one.

## How the checks were done

- **Source checkout.** I first made a full worktree of 7fc37ffed7. It used 6.8 GB of disk and left
  the shared VM 98% full, so I removed it. In its place I extracted, from the same commit with
  `git archive`, the study directory, the bench record, `research/ic_gpu_rootfind_la_20261115`
  (round 5), `research/ic_m3_cost_leads_20261115` and the crate's build set. The build set is 3,986
  files (190 MB): `src`, `crates`, the path dependencies, and every `include_bytes!`, `include_str!`
  and `#[path]` target, resolved transitively.
- **Dispatch-time binary.** Before anything was rebuilt I hashed `target/release/ecbench`:
  `63c25518…f4f7`. That equals `binary_sha256` in both sessions and in all 204 records.
- **Independent build.** I built a release `ecbench` from the archived 7fc37ffed7 tree with
  `cargo build -j 2 --release --bin ecbench` and a separate `CARGO_TARGET_DIR`. It took 24 min 58 s,
  and the log shows `Compiling crypto v0.1.0 (<archive tree>)`. The result is `b760c5bb…6802`.
- **Untrusted data.** The tarballs were extracted into empty scratch directories. Every script ran
  with `python3 -I` from a separate directory.
- **Recomputation code.** All recomputation was done by my own scripts, which read record fields
  with `[]`. I did not use the producer's `analyze.py` or `replicate_check.py`; I ran them only
  afterwards, to check that they regenerate the committed outputs.

## Requirements

| # | requirement | result | evidence |
|--:|:--|:--|:--|
| 1 | Frozen commit, not the moving tree | pass | 7fc37ffed7 via `git archive`; branch head 3f9d6cbb4b not reviewed |
| 2 | Tarball hashes | pass | `raw/session.tgz` acba46e4…11c0 OK; `raw/replicate.tgz` a7f88191…2d OK; round-5 `research/ic_gpu_rootfind_la_20261115/raw/out_rla.tgz` OK (`sha256sum -c`); `verify.json` 672d597e…5a73 = the bench record's raw entry |
| 3 | Session bindings: spec, records, commit, dirty state, binary | pass | `session/spec.json` byte-equal to `specs/sweep.json` (a14caab7…); `records.jsonl` fdd267b6…c894 = `session.json` `records_sha256` = `verify.json`; `git_commit` 75b978e173, `git_dirty` false; no `src`/`Cargo.*` change between c7535aafb1, 75b978e173 and 7fc37ffed7; 144/144 records `deterministic`, no warmups, L0 |
| 4 | Binary bound to source | pass | `git_commit_source` is `runtime_worktree`, so the record alone does not bind the binary (F9). The independent build differs from `63c25518…` only in `.note.gnu.build-id`; `.text`, `.rodata`, `.data`, `.data.rel.ro`, `.eh_frame`, `.gcc_except_table` and every other allocated section are byte-identical |
| 5 | Certificates (independent of the solver) | pass | My own affine short-Weierstrass code checks [d]G = Q, re-derives the planted k from `uniform_scalar_sha256_v1` (SHA-256 of canonical `{"domain","parts":[model,r,seed,index]}`), and checks k = `planted`, d ≡ k mod r, [r]G = O and status `verified`. Main session 144/144; replicate session 60/60 |
| 6 | Per-cell mean S, S/S(1), S/rho, rows/a max, trials (REPORT.md table) | pass | All 20 rows reproduce at printed precision in every measured column (S, S/S(1), S/rho, rows/a max, trials) (table below). Max \|ratio − frozen prediction\| = 0.0141 (claim: within 0.015) |
| 7 | Paired order, f < 1 below f = 1 | pass | 6/6 in each of the 16 candidate cells (24/24 per fraction); all arms of a (workload, round) share one `algorithm_seed` |
| 8 | Rows unbiased (prediction 3) | pass | Per-record rows/a 0.905–0.937 for every arm (wording: F2) |
| 9 | Row yield (prediction 4) | pass | μ = −ln(1 − Σrel/Σtrials) on the f = 1 arm; \|relative error\| ≤ 1.77% in every cell (claim: within 1.8%) |
| 10 | §5a band (default and `--no-price-table-work`) | pass | (a) `ecbench band` on the extracted session, with the dispatch binary and with the independent binary, gives `band.json` and `band_no_table.json` byte-identical. (b) My own implementation of band.rs's rule (`unpriced_counts`: top-level counters first, then phase `native` as `X_uncharged` or `X`; counted `table_insert_attempts` only behind its marker; byte counters exempt; no calibration pins) matches both files to 4.3e-16 relative. Rho's `canonicalisations_uncharged` (a top-level counter) is priced. No arm has an uncounted unpriced counter. No unpriced counter is in band.rs's field-arithmetic class, so the w_cal reading is not triggered |
| 11 | Order across the band | pass | Every f < 1 arm is below its f = 1 arm at w = 0, 0.1 and 1, under both pricings, in all 16 cells |
| 12 | Falsifiers (PROTOCOL.md) | pass (none triggered) | f = 1/8 at \|F_x\| 4096: mean ratio 0.5710 and 0.4354 (paired range 0.552–0.581 and 0.428–0.448) against the 0.75 threshold; control 0.5623 and 0.4333. Max rows/a in any candidate record is 0.9365 (≤ 1.05). Verification failures: 0/144 and 0/60 |
| 13 | Replication, 36/36 identical to round 5 | pass | I compared every deterministic field: method id and params, workload sha256, target, planted value, algorithm seed, outcome, the whole `cost` block, every phase (adds, doubles, scalar_mults, gae, `native`, `work_native`; `wall_ns` excluded), counters, factor base, boundaries, `detail`. The only difference is `detail.counted_field_ops`, a block the round-5 binary (`0742b939…`) did not emit. 36/36 identical (rho-neg 12, rf3-f1-F2048 12, rf3-f1-F4096 12). Replicate verified 60/60. The deviation note's example also holds: main session seq 1 S 299.4 against round 5's 280.3, algorithm seeds 5321629819566045629 and 40544001460283621 |
| 14 | A/A copy equals the reference | pass | `rho-neg-aa` = `rho-neg` on 12/12 (workload, round) pairs, comparing method, cost, counters, phases, outcome, algorithm seed, boundaries and field ops |
| 15 | Replay certificate and own replay | pass | Producer receipt: 144/144 reproduced, same host and same binary (F10). Mine, main session: dispatch binary, `--replay 12 --replay-seed 1795030000 --replay-seq 41 --replay-seq 115`, 14/14 identical (seqs 12 17 28 32 39 41 45 47 49 57 63 67 112 115; covers rho, rho-aa and every fraction f = 1 to 1/16, across both curves); independent binary, `--replay 10 --replay-seed 20261118 --replay-seq 126`, 11/11 identical (seqs 3 6 7 18 42 55 56 63 65 119 126). Replicate session, which the producer never audited: dispatch binary, seed 1795030001, 8/8; independent binary, seed 20261119, 6/6. All integrity checks true in all four receipts |
| 16 | The window's work is charged | pass | See "Code change" below; the identities hold in all 120 IC records |
| 17 | `sweep=1` is the old path | pass | By inspection of the diff, and by row 13 on the real workloads |
| 18 | Seed deviation handled honestly | pass | See "Seed deviation" below |
| 19 | Producer scripts regenerate their outputs | pass | `analyze.py` → `results.json`, `replicate_check.py` → `replicate_check.txt` and `make_figure.py` → `fig_sweep.svg` all regenerate byte-identically. `analyze.py` reads record fields with `[]` (its two `.get` calls touch only its own constant table and its own row dicts). It computes no band of its own (L-546d37ca7750 not triggered). It was committed at 1c5ca1a230 (17:55:48), before results, and is unchanged since |
| 20 | Reporting contract | partial | One unit (S), ratio columns, rho priced in the band, distance to rho stated (91x and 107x at w = 0; 119x and 129x at w = 1), and the m = 3 arm explicitly not called the program's best IC baseline: all pass. Gaps: F1 (headline factor wording), F3, F4 (§5a per-arm unpriced list), F5 (§3 class and §3a chain). m = 83 and ECC2K-130 disclosure: not applicable. Wall time: no claim made |
| 21 | Bench record and artifact policy | incomplete (process) | F7 |
| 22 | REPORT.pdf matches REPORT.md | minor fail | F6 |

## Recomputed table (main session, mean of 3 targets × 2 rounds)

S/S(1) is the ratio to the f = 1 arm at the same |F_x|, under default pricing (table work priced).
Rho means: 37.4 bits 1.0016 / 1.1016 / 2.0022 and 39.3 bits 1.1203 / 1.2323 / 2.2401 at
w = 0 / 0.1 / 1.

| log2 r | \|F_x\| | sweep | S (w=0) | S/S(1) w=0 / 0.1 / 1 | S/rho w=0 / 1 | rows/a min–max | trials / trials(f=1) |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 37.4 | 2048 | 1 | 289.599 | 1 | 289.1 / 296.5 | 0.917–0.925 | 1 |
| 37.4 | 2048 | 1/2 | 170.884 | 0.5901 / 0.5922 / 0.6016 | 170.6 / 178.4 | 0.910–0.918 | 1.13 |
| 37.4 | 2048 | 1/4 | 132.079 | 0.4561 / 0.4588 / 0.4708 | 131.9 / 139.6 | 0.909–0.928 | 1.70 |
| 37.4 | 2048 | 1/8 | 116.567 | 0.4025 / 0.4052 / 0.4173 | 116.4 / 123.7 | 0.918–0.928 | 2.93 |
| 37.4 | 2048 | 1/16 | 112.159 | 0.3873 / 0.3900 / 0.4017 | 112.0 / 119.1 | 0.915–0.925 | 5.58 |
| 37.4 | 4096 | 1 | 163.789 | 1 | 163.5 / 220.7 | 0.909–0.926 | 1 |
| 37.4 | 4096 | 1/2 | 117.346 | 0.7164 / 0.7338 / 0.7919 | 117.2 / 174.8 | 0.912–0.924 | 1.12 |
| 37.4 | 4096 | 1/4 | 101.284 | 0.6184 / 0.6430 / 0.7252 | 101.1 / 160.1 | 0.912–0.924 | 1.59 |
| 37.4 | 4096 | 1/8 | 93.531 | 0.5710 / 0.5949 / 0.6745 | 93.4 / 148.9 | 0.915–0.922 | 2.60 |
| 37.4 | 4096 | 1/16 | 90.732 | 0.5540 / 0.5769 / 0.6535 | 90.6 / 144.2 | 0.912–0.927 | 4.72 |
| 39.3 | 2048 | 1 | 576.538 | 1 | 514.6 / 517.9 | 0.917–0.931 | 1 |
| 39.3 | 2048 | 1/2 | 329.779 | 0.5720 / 0.5724 / 0.5744 | 294.4 / 297.5 | 0.916–0.928 | 1.13 |
| 39.3 | 2048 | 1/4 | 248.537 | 0.4311 / 0.4318 / 0.4351 | 221.9 / 225.4 | 0.915–0.924 | 1.69 |
| 39.3 | 2048 | 1/8 | 216.747 | 0.3759 / 0.3767 / 0.3801 | 193.5 / 196.8 | 0.905–0.919 | 2.93 |
| 39.3 | 2048 | 1/16 | 207.625 | 0.3601 / 0.3611 / 0.3654 | 185.3 / 189.2 | 0.908–0.937 | 5.59 |
| 39.3 | 4096 | 1 | 291.359 | 1 | 260.1 / 285.6 | 0.914–0.928 | 1 |
| 39.3 | 4096 | 1/2 | 179.527 | 0.6162 / 0.6230 / 0.6512 | 160.3 / 186.0 | 0.918–0.928 | 1.14 |
| 39.3 | 4096 | 1/4 | 141.226 | 0.4847 / 0.4929 / 0.5263 | 126.1 / 150.3 | 0.916–0.927 | 1.69 |
| 39.3 | 4096 | 1/8 | 126.856 | 0.4354 / 0.4449 / 0.4838 | 113.2 / 138.2 | 0.912–0.925 | 2.91 |
| 39.3 | 4096 | 1/16 | 119.966 | 0.4117 / 0.4198 / 0.4528 | 107.1 / 129.3 | 0.913–0.922 | 5.39 |

## §5a: unpriced work per arm (mean count per record)

This is the list §5a requires for every arm. REPORT.md does not carry it (F4). Every counter is
counted (`uncounted` is empty for all 24 band rows). Every arm's S is a lower bound
(`cost.lower_bound: true`).

- **rho-neg and rho-neg-aa:** `canonicalisations_uncharged` 424,267 (37.4 bits) and 935,230 (39.3 bits).
- **IC arms, `cost.unpriced`:** `inversions`, `legendre_symbols`, `lookups`, `row_ops`, `sqrt_solves`
  and `target_guard_probes`. The band also adds the counted `table_insert_attempts` (default
  pricing): 4,194,304 at |F_x| 2048 and 16,777,216 at 4096.
- **Example, 37.4 bits, |F_x| 4096:** f = 1 has inversions 9,321, lookups 52.5 M and row_ops 48.6 M;
  f = 1/16 has inversions 44,003, lookups 21.5 M and row_ops 45.6 M. In every cell `inversions` and
  `target_guard_probes` both rise in proportion to trials. `lookups` falls with the window.
  `legendre_symbols` and `sqrt_solves` are factor-base constants: 4,108 and 2,048 at 37.4 bits,
  |F_x| 2048.

## Code change (`git diff origin/main..7fc37ffed7 -- src/cryptanalysis/ic_framework/plugins.rs`)

- **`decompose` with `window == 0` or `window >= k` (sweep = 1) is the old path.** It sets
  `(n, offset) = (k, 0)`, `at(j) = j`, and runs `batch_inv` on `diffs[..k]`, which is the whole
  vector the old code inverted. The charges are `muls += 3k`, `sweep_field_muls += 2 + 3k` and
  `sweep_field_adds += k`, as before. `self.trial` does not advance. The f = 1 spec arms do not name
  `sweep`, so their method identity is round 5's; `method_id` is equal in row 13.
- **With the window, each candidate is charged exactly as before.** Each `T = R − εF_i` is one
  `ops.adds`, 4 muls and 1 sqr. Each candidate's lookup is counted (`lookups`, priced in the band).
  There is one batched inversion per swept trial (`inversions`, priced in the band). The per-trial
  target comes from the walk and is charged as additions. In all 120 IC records:
  - `sweep_field_adds = n · inversions` and `sweep_field_muls = (2 + 3n) · inversions`, with
    n = ceil(k/N);
  - 2n(T − R) ≤ `lookups` ≤ 2nT, where T is trials and R is relations;
  - (relation-phase adds − lookups − walk steps) per relation is constant across N, at about 21 for
    |F_x| 2048 and about 11 for 4096.

  In four records (seqs 41, 114, 115 and 126) `trials` exceeds `inversions` by
  `repeated_targets_skipped` (1 or 2), because a target the guard skips is never swept. That is
  consistent with the code.
- **Rotation.** The window offset is `(t · w) mod k`, so the windows tile the base when N divides k,
  as here.
- **GPU path.** `rootfind-subtract-gpu` refuses `sweep` because its whitelist does not contain the key.
- **Not re-run.** I did not re-run the unit test `rootfind_subtract_sweep_window`. It would need a
  test-profile build. The sweep = 1 equivalence rests on the inspection above and on row 13.

## Seed deviation

- **What happened.** `specs/sweep.json` froze measurement seed 20261182 rather than rf3la's
  20261128. The spec is unchanged since the freeze at c7535aafb1 (17:38:22).
- **When it was recorded.** The main session started at 17:54:31 and finished at 18:16:39. The
  deviation was appended to PROTOCOL.md at 1c5ca1a230 (17:55:48), about one record into the run.
  Its PROTOCOL diff is append-only.
- **What changed.** Only where prediction 1 is read: a separate control (`specs/replicate.json`,
  seed 20261128), committed at 1c5ca1a230 before it ran (18:16:52 to 18:27:47). The main spec,
  predictions 2–5 and the falsifiers were left as frozen.
- **Prediction origin.** The predictions were committed in LEADS.md at 13be685cac (01:34) from a
  fitted model, before the code (17:38) and before the run.
- **Verdict.** The deviation was disclosed, not hidden. It was the minimum change needed and was
  made before results could steer it. The control reproduces round 5 exactly (row 13).

## Findings

1. **F1 (wording, should fix).** The REPORT "Reading" says the window lowers the counted cost "by
   1.8x to 2.8x … across the §5a band".
   - The 1.81–2.78x range is the f = 1/16 reading at w = 0.
   - At w = 1, f = 1/16 gives 1.53–2.74x (ratios 0.365–0.653); at w = 0.1 it gives 1.73–2.77x.
   - At f = 1/2, w = 0, the gain is only 1.40–1.75x.
   - What holds across the band is the order, not the factor range.
   - The dispatch summary's "0.36–0.55x at N = 16" is likewise the w = 0 reading.
   - State the fraction and the weight with the range, and give the w = 1 range beside it.
2. **F2 (wording).** "Rows at the stop are 0.918–0.937 a" is the range of the per-cell maxima. The
   per-record range is 0.905–0.937 a. Prediction 3 and the falsifier are unaffected.
3. **F3 (completeness).** Two prediction-5 points are reported loosely.
   - The skeptic's dilution estimate ("ratio at w = 1 closer to about 0.67 than to the w = 0 ratio")
     held only at 37.4 bits, |F_x| 4096. At 39.3 bits, |F_x| 4096, f = 1/8, the w = 1 ratio 0.484
     stays near the w = 0 ratio 0.435, and the same holds at |F_x| 2048. REPORT.md cites only the
     cell where it held.
   - Inversions were predicted to rise "about 4x" at 1/16. They rose 4.7–5.6x, which REPORT.md
     calls "as pre-registered".
4. **F4 (§5a list).** REPORT.md does not list every unpriced counter with its count for every arm,
   rho included. It names only inversions, and `band.json` carries totals only. The list is in the
   §5a section above. Also state in REPORT.md that S is a lower bound on every arm.
5. **F5 (§3 / §3a).** No §3 class label and no §3a cost chain. The data support class
   *engineering*: S fell, the floor did not move, and the change is a constant factor ("not to the
   exponent", as the REPORT says). The chain is one replace line:
   - C(rf3-f1) = Σ phases (unpriced work listed above);
   - → replace the oracle's per-trial sweep (all k abscissae) with a rotating window of ceil(k/N);
   - measured at the ratios above, ordered across the band;
   - = C(rf3-f1/N);
   - same targets, curves (ICV1 slugs), success probability (all verified) and unit.
6. **F6 (minor).** `REPORT.pdf` (created 18:29:12) was rendered from c0329ce691's REPORT.md. It
   lacks the final audit sentence ("144 records, 144 verified, 144 replays reproduced…") that
   7fc37ffed7 added. Re-render with the next REPORT edit.
7. **F7 (process: benchmark record and store).** At 7fc37ffed7,
   `benchmarks/records/20261118T1754Z-ic-m3-sweep-window.json` (f17f89d5…) has these gaps:
   - `status: open`: `benchrec close` is blocked by the unrelated malformed record on main.
   - All three raw entries are `uploaded: false`: raw data is not in S3.
   - `commands` lists the main run and the `--replay-all` audit, but not the replicate session's
     run or the two `ecbench band` runs.
   - `raw/session.tgz` (138 KB) and `raw/replicate.tgz` (71 KB) are archives over the 64 KiB
     data-file threshold of AGENTS.md §13, committed to Git without an `.artifacts.json` manifest.

   None of this affects the measurement. It does leave the "Benchmark records" rule unmet for the
   PR. 3f9d6cbb4b edits the record after this review.
8. **F8 (informational, binding).** The replicate session ran from a dirty tree (`git_commit`
   cb12c742e4, `git_dirty: true`), and the producer did not audit it. It binds through its binary
   hash, which is the same `63c25518…` as the main session, whose allocated sections match the
   independent build. I replayed 14 of its records identical with two binaries.
9. **F9 (informational).** `git_commit_source` is `runtime_worktree` in both sessions. The recorded
   commit is the worktree's at run time, not a commit embedded at build. Requirement 4 closes this
   for this study. A build with `CRYPTO_BUILD_GIT_COMMIT` set would make the binding
   self-contained.
10. **F10 (informational).** The producer's `verify.json` is a same-host, same-binary self-audit.
    My replays used an independent build but ran on the same VM and hardware class. The
    cross-host-class re-audit that §12 assigns to CI has not been done here.
11. **F11 (informational).** The `--replay-all` audit (18:17–18:39) ran concurrently with the
    replicate session (18:16–18:27) on the 4-core VM. Counts are deterministic and no wall time is
    claimed, so this has no effect.

## Not checked

- PR #4739's description: `gh` is not usable in this container.
- research-kb: not reachable, the same gap the producer recorded.
- A cross-host-class replay.
- The unit test `rootfind_subtract_sweep_window`.

## Validated artifact hashes (at 7fc37ffed7)

```
4e6ec95b1346afbc041b543fb8830604e5c9faac217671fb364b4a993e6ca355  PROTOCOL.md
4293c35e0123e1cc6dd7d6b5a49378ac14be22511ac890070e2f73efdc838fbe  REPORT.md
05e7187e14ca29e81370db7b84beb715abd83aa638d719ea903cbf45c72dc5c8  REPORT.pdf
fddbb6d3e1d5f526e63a5fe14141c6dd552ed399722ca3c23c5d40263aa79567  analyze.py
3b330a947d62d83956a983d1cd9067319f68481bc8443e50159f84c5b486169c  results.json
a58940ff16a8e1d03bd2bff4aaa49a6f9144952109b830b7438cfbca885e4e35  band.json
0475f26fa0264ca3da2d04f2f859dd720ef2d4def08774f67df94dca98faf386  band_no_table.json
ddcb48015614e60e9cbb00f13537a621de6061c5b2e292bb5ba186f80b48b9e9  replicate_check.py
0a76fcbcaf697e6f6f6186c07f73690e772baab90ab298ee7024a53eb6483d69  replicate_check.txt
672d597e32172ed9ffed6aae9aafadf72269505b238de6f2543057e99ada5a73  verify.json
a83ded6873765e509ed1ac002f2a69e0a6976e9474a6e84aaa944d84a53e9eec  fig_sweep.svg
a067858b0096b8b64b643f46675416ca59af2712c411d9b267346c8089f197a0  make_figure.py
a14caab7f450dd8d7803f630a6e787699415e7f6ce02cc93660a15186c01e2be  specs/sweep.json
3c4dc93594aa1bf206490139327460695f8cff11d0c3f24a236b94bdb600c43c  specs/replicate.json
acba46e4735c46d70bccfe90b9c05c28a47a1f0978e6ddf5ac7f281f595f11c0  raw/session.tgz
a7f88191af46b71f3236ad6dab9b4ac5ee481fb1063f90a59c371d8c5c2ce22d  raw/replicate.tgz
fdd267b6edb73db5fcb6e5dbd0bc28354a10fcaf0c674321a910c6ca5261c894  session/records.jsonl (in session.tgz)
de8799b11640dbcc0110aefcfb2a44a9728ae131a78b328bd2e0ca918a321717  replicate/records.jsonl (in replicate.tgz)
f17f89d51c491e1e1b4b97168405cc3e1e576742a616a7937270a0f3d8e6c392  benchmarks/records/20261118T1754Z-ic-m3-sweep-window.json
```

The binaries were `63c255181b82146266d7833c783c18b84f490f8cd62e89e0d71dee357a0af4f7` (producer, and
dispatch-time `target/release/ecbench`) and `b760c5bb8ea458d74b2fcae6b3ac4b0aa234bbcdcef8380be9a0e52ec27d6802`
(independent build of 7fc37ffed7). The validator's own replay receipts are not committed; their
seeds and sequence numbers are given in requirement 15, so the same draws can be rerun.

## Disposition

**`validated`** for the counted-S measurement at 7fc37ffed7, which consists of:

- the per-cell means, ratios and S/rho in REPORT.md;
- the order f < 1 below f = 1 across the §5a band under both pricings;
- the replication of round 5 by `sweep=1`;
- the falsifiers, none triggered.

Fix F1–F5 before the numbers are cited outside the study. Because a validation pin may name the
earlier REPORT.md bytes, correct them in a new commit or an `ERRATA.md`, and close F7 for the
benchmark-record rule. The distance to rho stays large: the lowest arm is 90.6x matched
`rho.negation` at w = 0 and 119x at w = 1. This is an improvement of the m = 3 rootfind-subtract
arm only, not of the program's best index-calculus baseline.
