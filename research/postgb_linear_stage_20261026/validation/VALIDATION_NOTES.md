# Validation notes: postgb_linear_stage_20261026

Independent validation of snapshot `e9bf8737a38fa4446acec23812eaaf085d8426a6`
(branch `feat/sleepy-tesla-1ucug8`), 2026-10-27. The validator did not produce
the study. Every input was read from the commit with `git show` / `git archive`,
not from the working tree. `scripts/summary.py` was neither read nor run before
the recomputation agreed with `analysis/summary.json`.

Files:

- `recompute.py` writes `validator-report.json`. It is the validator's own
  recomputation from the raw `runs/` files and the card receipts.
- `replay/sample.py` draws the nonce sample (`replay/sample.json`).
  `replay/run_replay.py` runs the pinned binaries and writes `replay/journal.json`
  and `replay/out/`. `replay/check_replay.py` writes `replay-certificate.json`.

## Status

| file | status |
|:--|:--|
| `validator-report.json` | `pass`, `independent: true`. 273 checks, 110 headline numbers at rel_tol 1e-9, 0 mismatches, 0 underivable numbers. The gate's own `check_validator_report` returns no errors with `validation/` as base. |
| `replay-certificate.json` | `verified`, `independent: true`, `scalars_verified: false`. The gate's `check_replay_certificate` reports exactly one error, "scalars_verified is not true" (issue 1). |

## Replay

- **Nonce.** `82fe12d03e751ddca56c8de9e43ae80569404e92a7c2ea63e3db3068f79ca68f`,
  256 bits from `/dev/urandom`. It was drawn at 2026-10-27T05:44:53Z, after
  the snapshot commit (05:39:32Z).
- **Sample.** 14 systems across all five families:
  - jv-nagao: p101-0, p1009-11c, p53-8c
  - gaudry-s4: p271-6, p271-3, p8191-7
  - gaudry-d4: p97-4, p97-3, p1033-0
  - gaudry-s5: p13-0
  - dense: n4-d5-p31-0, n4-d3-p31-1, n8-d2-p32-2, n2-d32-p16-0
- **How they were reached.** 12 runs of the pinned `postgb_la_bench`
  (c5612e2e…), one per (family, p[, shape]) group, with the card's `--per 8`
  and `--reps 1`, cwd `/home/user/crypto`.
- **Bench result.** All 82 systems in those groups (the 14 sampled plus 68
  others) equal their recorded rows: n, p, delta, the arms present, every
  arm's solution count, agreement, planted-point finding, fast give-up and
  fallback.
- **Counted muls.** These are exact on the 17 systems the replay placed at the
  same index `si` as their card: jv-nagao p53 ×12, gaudry-s5 p13 ×2 and dense
  n2d32 p16 ×3. Elsewhere the randomized choices differ by construction
  (convention 5).
- **The give-up on p97-4.** It reproduced at a different index (si 76 → 4),
  so it does not depend on one random functional.
- **DLP.** Pinned `jv_cover` (afa830fc…) with
  `--exp dlp --sizes 53 --seeds 1 --rho-runs 4 --check-every 64`:
  - Under `JV_LIN=fast`, all 54 non-timing fields equal R2-D2's p=53 seed=1
    row: `lin_muls` 4593464878, `relations` 31, `solved` and `correct` true,
    and all 4 rho runs correct.
  - The extra run under `JV_LIN=sparse` equals R2-D1's row on all 54
    (`lin_muls` 4032563426, `relations` 31).
- **Timing not replayed.** The `f4_ms`, `lin_ms` and `wall_ms` fields were not
  compared. The replay host is host-B hardware (`replay/replay-host.lscpu.stdout`),
  but a concurrent `cargo test --release` build kept CPU pressure high, so the
  replay's summed `lin_ms` was about 2× the recorded value.

## Mismatches

None. All 110 headline numbers and every other numeric or list leaf of
`analysis/summary.json` match the recomputation from the R2 raw records at
rel_tol 1e-9, most of them exactly. The other leaves are meta, skipped, gates,
the 253-entry `systems` list, the predictions, the DLP rows and the host-A
blocks. `recompute.py` found no leaf it could not derive.

## Coverage, hosts, receipts (all hold)

- **Coverage.** R2-L1..L6 contain exactly the protocol corpus: 253 systems,
  each once, none dropped, none extra. That is jv-nagao 48, gaudry-s4 24,
  gaudry-d4 24, gaudry-s5 4 and dense 153 (17 shapes × 3 primes × 3).
  R2-A1 + R2-A2 cover the same 253. Every `skipped` list is empty.
- **Arms and rounds.** The dense arm is present exactly where δ ≤ 1024. Every
  arm has 7 rounds (3 in the ablation), and every `median_ms` equals
  `sorted(ms)[len//2]` of its raw `ms`.
- **Receipts.** All ten R2 receipts are `completed`, and every command has exit
  status 0, a single attempt and no timeout. Each `card_sha256` equals the
  committed card. Every card binds `postgb_la_bench` c5612e2e… and `jv_cover`
  afa830fc…, the pins, which also match the copies used for the replay.
  `tools/task_card.py run` refuses a card whose binds do not match, so the
  receipt plus the card hash binds the binary. Output, stdout, stderr and
  `lscpu` hashes in the receipts equal the committed files. The launcher logs
  name the committed receipt hashes.
- **Ordering and timeouts.** Each card started at the commit that added the
  previous card's receipt, so the cards ran one at a time. The run timeouts
  follow amendment 2: 1700 s for R2-L1 and R2-L2, 7000 s for the rest.
- **Source stability.** No file under `src/`, `examples/` or `Cargo.*` changed
  between the commit of the first R2 card (e370e78) and that of the last
  (d544b78).
- **Hosts.** The ten R2 `lscpu` records are byte-identical: Xeon @ 2.80 GHz,
  L2 4 MiB, L3 33 MiB (host B). L1–L4 ran on host A (Xeon @ 2.10 GHz, L2 8 MiB)
  with the pre-fix bench binary 10e79226…. No third host appeared. Host A is
  only in the `host_a_*` blocks, which recompute from L1–L4 alone, and every
  host-B number recomputes from R2 rows alone. Nothing is pooled. Host A lacks
  exactly the 11 systems amendment 1 names (`n3-d10-p32-1`, `n3-d10-p32-2` and
  the nine `n6-d3`), plus the four gaudry-s5 systems, which were never run
  there.
- **Gates.** All arms agree on all 253 systems. Every dense-family planted
  point is found. The fast arm gave up on one system (R2-L1 `gaudry-d4 p97-4`,
  "minimal polynomial short of delta"), so P3 = 1/253 = 0.00395. The sparse arm
  fell back on that same system. The ablation agrees and matches the main
  runs' solution counts on every system.

## Conventions identified (not errors; stated so that a reader can recompute)

1. **Direction of `counted_ratio`.** In each block it is the block's own
   direction: sparse.muls/fast.muls in `sparse_over_fast` and
   dense.muls/fast.muls in `dense_over_fast`. P4 is worded the other way
   ("fast's counted multiplications within ±25% of sparse's"). The P4 verdict
   is the same in both directions. Fast/sparse spans [0.528, 1.311] and
   sparse/fast spans [0.763, 1.895], so the counted-cost sub-prediction fails
   either way. `summary.json` records `P4.held: false`, and the recomputation
   agrees.
2. **Family `median_ms`.** For an even system count it is the upper median,
   `v[n//2]`, not the mean of the two middle values. For example, jv-nagao
   sparse is 3.054797 and not 3.044692.
3. **`portable_over_simd_fast`.** The geometric mean over systems of
   (R2-A fast median) / (R2-L fast median), paired by family and label.
4. **Bootstrap CI.** The validator's own stdlib bootstrap reproduced every
   claimed `ci95` bit for bit, so the CIs are in the 1e-9 headline. Its
   conventions are `random.Random(20261027)`, 4000 resamples of the per-system
   log-ratios by `choice` in card order, and nearest-rank 2.5%/97.5%. The
   protocol fixes only the resample count and the seed, so this agreement
   depends on those conventions.
5. **Counted multiplications depend on the invocation.** The arms are seeded
   with `0x5EED ^ si`, where `si` is the system's index in that invocation's
   filtered list. A system's `muls` is therefore fixed only for a given
   (invocation, position). Across R2-L and R2-A, the 98 systems at the same
   `si` have identical sparse and fast `muls`, and the other 155 differ by
   construction. This explains the small differences between the main-run and
   ablation `counted_ratio` values, for example gaudry-s5 max 0.8787 against
   0.8745. Solution sets do not depend on `si`.

## Issues and discrepancies (none changes a recorded number)

1. **Scalars not independently verified (gate blocker).** The pinned
   `jv_cover` prints and writes neither the solved logarithm nor the instance
   (α, G, Q). The only check available is the binary's own: `dd·G == Q` on
   E(F_{p^6}), then `correct = (dd == planted d)`. The replay reproduced
   `solved`/`correct` = true, so `replay-certificate.json` has
   `scalars_verified: false`. `docs/VALIDATION_GATE.md` requires `true`, so a
   `VALIDATION.json` naming this certificate will not pass the gate. Closing
   this needs a path that emits dd, G, Q and α from the pinned source, checked
   by independent F_{p^6} curve arithmetic. That would be a new card, not an
   edit of these records.
2. **Dense-family F4 budget.** PROTOCOL.md says "a 900 s budget for the others
   [than jv-nagao]". The bench has given the dense family 600 s
   (`dense_opts`) since the protocol was frozen (544eb84), and amendment 1's
   table also says 600 s. No R2 system hit an F4 timeout, so this had no
   effect on what was measured. The protocol text and the code still differ.
3. **Planted points outside the dense family.** The bench sets
   `planted: None` for the constructed jv-nagao systems (label suffix `c`).
   The gate "every planted point is found" is therefore enforced only on the
   dense family. The validator checked the weaker property that every `c`
   system has at least one solution in every arm; this holds for all 16. The
   planted point itself is not verifiable from the records.
4. **Activity during R2-A2.** R2-A2's measured run started about 04:33:25Z
   (launch 04:31:39Z, then a 105 s quiet wait). The producer committed afa857a
   (analysis scripts) at 04:34:09Z, during that run. The quiet-CPU rule is
   checked only before a run. This can affect A2's timings, not its counts.
   Other concurrent activity during any card cannot be ruled out from the
   records.
5. **Snapshot HEAD and the binaries' source.** The snapshot includes a merge of
   main (6dd803e, after R2-D2) that changes `src/cryptanalysis/f4_fp.rs`,
   `jv_cover.rs`, `gaudry_cubic.rs`, `gaudry_quartic.rs` and `Cargo.toml`. The
   binaries correspond to the run commits (e370e78..d544b78), not to HEAD, so a
   rebuild from HEAD would not reproduce them. The correspondence between the
   binaries and their source was not checked by a rebuild; the binaries are
   bound by sha256 only. Their mtimes (jv_cover 2026-10-26T23:30:41Z,
   postgb_la_bench 2026-10-27T00:31:51Z) are a few minutes earlier than the
   commits that record their source (544eb84, 4d55b73).
6. **The DLP `lin_ms` is not single-threaded wall time.** `jv_cover` runs each
   64-residual batch on a rayon pool (4 threads on 4 vCPU), and `lin_ms` sums
   each residual's linear-stage wall time inside that pool. The bench arms are
   single-threaded. Facts from the records, not an assessment:
   - The instance and relation counters are identical across `JV_LIN` arms:
     residuals, decompositions, relations, unknowns, filtered, la_ops,
     incomplete and cross_checked.
   - The counted S under `fast` is 2.3–3.0% higher than under `sparse`
     (sparse/fast = 0.970, 0.971, 0.976, 0.976), so S/rho is higher under
     `fast` on all four rows.
   - `wall_ms` is about the same for p = 53 (248.5 → 247.5 s and
     135.7 → 136.4 s) and lower for p = 101 (121.0 → 75.7 s and
     164.3 → 119.3 s).
7. **The working tree changed during validation.** The producer had
   uncommitted edits to `scripts/plots.py` and `src/cryptanalysis/jv_cover.rs`
   (the `JV_LIN` default changed to `fast`). `README.md` and `figures/` were
   untracked, and a `cargo test --release` build was running in the shared
   `target/`. None of this is in the snapshot, and none of it was validated.
   At check time every committed run, card, receipt and `summary.json` in the
   working tree was byte-identical to the snapshot. The README that amendments
   1 and 2 refer to (the host-A/host-B comparison) is not in the snapshot.

8. **A producer commit landed during this validation.** Commit `e6ff48ea72`
   (05:49:33Z) came after the snapshot, while the replay was running.
   - **It swept validator files.** It added a partial copy of this directory's
     `replay/` files under the producer's commit: 7 of 14 runs. Its
     `journal.json` and `run_replay.log` were later superseded by the complete
     ones here.
   - **It made gate-relevant changes before validation finished.** It changes
     `docs/ic/leaderboard.json`, `docs/ic/LEADERBOARD.md`,
     `docs/ic-leaderboard.html` and `scripts/build_ic_leaderboard.py`. It adds
     `benchmarks/20261027-postgb-linear-stage/` and makes `fast` the `JV_LIN`
     default. No `VALIDATION.json` is named.
   - **It does not touch the validated records.** `runs/`, `cards/`,
     `analysis/` and the protocol files are unchanged.
   - **Not validated here.** Whether those rows pass the gate as committed was
     not checked; that is for the Coordinator.

## Not assessed

- Whether the speed-up matters.
- The rho reference that the validation gate needs (kind, strongest available,
  §5a band). The DLP S/rho uses the same-process `rho_e` with 4 runs, as
  recorded.
- Any leaderboard or benchmarks row.
