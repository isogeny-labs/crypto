# Assessment: CARD-fp31-phase4-rhostep-20261112-1

```yaml
card_id: CARD-fp31-phase4-rhostep-20261112-1
card_sha256: 58e524daf5732a2b021ad0183de63a808d17c7319200bda2e0afa4b4dded2343
relay_verbatim: not_checked   # no executor report file was supplied; the quoted values match the receipt (see 1)
outcome: impeded              # executor ran the card's argv as written; 3 of 48 classes absent because the host was busy
stopped_at_step: null         # no command failed; run-phase4 exited 0 with 3 FAILED lines
recommendation: rerun_after_infrastructure_fix   # c046, c047, c048 only; PROTOCOL 4 says not replaced, so the main session must decide (see 8)
reasons:
  - 3 classes refused at start by ecbench: "host is not quiet (PSI cpu some avg10 > 5.00)" (runs/c04[678].partial/run.log); the main session reports it was running cargo build then.
  - c052 ran with every row graded L0 (not a failure, a result to report).
interpretation: not_assessed
```

## 1. Receipt and digests

- receipt sha256 `c5879f9415375c954e96f7079b06ed0c4b7cb59922f87f3a00f3fcb5d366ae4e`: matches the report. status `completed`, commands_total 2.
- card sha256 `58e524da...d2343`: matches the receipt's `card_sha256`.
- `run-phase4`: exit 0, 3526.949 s, attempts 1, detached; stdout sha256 `699a9860bec10412c6d3c27f830b5b1fe3366beafbd20d577de7866145b7a740` (file on disk matches); stderr empty (e3b0c442...).
- `rho-timing`: exit 0, 6.673 s, attempts 1; stdout and stderr empty. `analysis.json` sha256 `1d6a6472d95f31ec08cf252c843e3c8edc1e15a985dc2d4baa5d26d4c43b863a` (file on disk matches the receipt output digest).
- The two commands ran in card order, one attempt each, no retry.
- `python3 tools/task_card.py check-relay` was not run: no saved executor text was given to me. The values quoted in the dispatch match the receipt.

## 2. The 7 binds (phase4/PROTOCOL.sha256, card binds)

`sha256sum` on each path from the repository root equals the bound value, all 7:
PROTOCOL.md `cd2984ea...72b1`; run_phase4.sh `6b35b4c9...a591`; phase3/classes.json `f5e04adc...a5d10`; each equals the card's bind; tool/src/phase4.rs `340e7fb9...de1`; tool/src/stats.rs `df661a28...c0468`; target/release/ecbench `86a9421d...254a`; fp31-class-census `632d1b77...9db`.
- phase4.rs current hash `340e7fb924ff64539dd9b8bbf99967830f95824b1f15dc49843bbc0dd0e7fde1` equals the bound hash. Its mtime is 03:48:06 UTC; `git status` does not list it as modified.
- All 45 session.json files carry `binary_sha256` 86a9421d...254a (the bound ecbench).
- Not verified: the hash of phase4.rs during the run (before 03:48:06 and after). Only the current hash is known. The run read the binary (not the source), whose hash is unchanged (mtime 01:01:51, fp31-class-census 03:00:36).

## 3. stdout

48 lines: 45 `ok`, 3 `FAILED` (c046, c047, c048, lines 39-41), 0 `skip`. Runs directory: 45 completed dirs c008..c045, c049..c055 and `c046.partial`, `c047.partial`, `c048.partial`.

## 4. FAILED classes

Each `runs/<id>.partial` holds spec.json, spec.log and run.log (no session, no verify.log, no audit.json):

| class | run.log time (UTC) | message |
|---|---|---|
| c046 | 03:48:52.9 | `ecbench: host is not quiet (PSI cpu some avg10 5.69 > 5.00); wait, or pass --allow-busy to record runs below L2` |
| c047 | 03:48:55.6 | same, avg10 5.39 |
| c048 | 03:48:58.3 | same, avg10 5.32 |

Neighbours: c045's last record is at 03:48:47; c049's first record is at 03:49:01. The three refusals fall in the gap between them. The files contain nothing about the cause of the load; the cargo build comes from the main session's report, and the timing fits. phase4.rs was written 03:48:06 and commit b07c887 is dated 03:48:40.

## 5. The 45 ok classes

For every ok class: `done`, `seconds`, `spec.json`, `audit.json`, `session/` exist (no MISSING). Per class:
- audit.json: `ok: true`, `session_status: complete`, `problems: []`.
- replays: 4 per class, all `reproduced: true` (180 of 180).
- records.jsonl: every record `outcome.status` = `verified` (225216 rows, 0 not verified); analysis.json `records_not_solved_correctly` 0.
- spec: `targets_per_curve` 64, `target_kind` planted_shared, rounds 2, warmup 1, order shuffle, `isolation_required` L2, arms in order plain (rho.plain, reference), plain-aa (rho.plain, control), control (rho.negation, candidate). All 45 match PROTOCOL 3-4.

## 6. analysis.json

- `missing` = [c046, c047, c048]; `classes` = 45; `records_measured` 150144; `records_not_L2` 12283; `records_not_solved_correctly` 0.
- Recomputed from records.jsonl (rounds that are not warmup): 150144 rows, 12283 below L2. Equal.

## 7. Rows below L2 (per row `isolation.level`)

All rows, warmup included: 225216 rows, 18516 below L2. Measured rows only (what analysis.json counts): 150144, 12283.

Measured rows below L2 per class (count / rows, percent):

c008 155/2304 6.73; c009 86/1536 5.60; c010 69/1152 5.99; c011 112/2304 4.86; c012 134/2304 5.82; c013 116/2304 5.03; c014 138/2304 5.99; c015 195/3072 6.35; c016 215/3840 5.60; c017 124/2304 5.38; c018 62/1152 5.38; c019 71/1536 4.62; c020 133/2304 5.77; c021 119/2304 5.16; c022 47/1152 4.08; c023 117/2304 5.08; c024 218/3840 5.68; c025 135/2304 5.86; c026 90/1536 5.86; c027 188/3072 6.12; c028 137/2304 5.95; c029 173/3072 5.63; c030 232/3840 6.04; c031 218/3840 5.68; c032 141/2688 5.25; c033 213/3840 5.55; c034 220/3840 5.73; c035 203/3840 5.29; c036 260/3840 6.77; c037 244/3840 6.35; c038 199/3840 5.18; c039 224/3840 5.83; c040 220/3840 5.73; c041 295/4608 6.40; c042 219/3840 5.70; c043 304/5376 5.65; c044 341/5376 6.34; **c045 345/5376 6.42**; c049 302/5376 5.62; c050 317/5376 5.90; c051 228/3840 5.94; **c052 3840/3840 100.00**; c053 269/4992 5.39; c054 335/5376 6.23; c055 280/5376 5.21.

- Level mix: every class other than c052 has only L2 and L1 rows (no L0). Excluding c052: 8443 of 146304 measured rows below L2 (5.77%); per-class range 4.08 to 6.77%, median 5.72%.
- A sample of the L1 rows' recorded blockers (c008): L2 criteria missed (2-3 involuntary context switches during a 8-18 ms solve against at most 1; run-queue delay above threshold; solve too short to admit wall time).
- **c045** (running 03:46:43 to 03:48:47, i.e. through the 03:47-03:48 build and edit window): 6.42% below L2. The adjacent classes: c044 6.34%, c049 5.62%. c045 is not outside the range of the other classes (4.08 to 6.77%). By minute (all rows incl. warmup): 03:47 248/3987 (6.2%), 03:48 182/2987 (6.1%), against 5-7% for other minutes. No excess in the c045 window.
- **c052** (03:54:36 to 03:56:00): all 5760 rows (3840 measured) graded L0, from the start of the class to its end. Per-minute: 03:54 1682/3645 (the class starts mid-minute), 03:55 4149/4149. In c052 5432 rows carry the blocker `L1: 2 user threads other than the runner's could run on the reserved CPU`; sampled row at its midpoint lists those threads as `21503/21504 bash` and `python3`, and `foreign_busy_ticks -1`. Session preflight for c052 was `quiet: true` (PSI avg10 2.45 before, 4.18 after; `allow_busy` false), run CPU 3, `runner_cpus` [0,1,2]. The cause of the extra user threads is not recorded in the session; I did not investigate beyond these fields.
- So below-L2 rows cluster in time only at c052; c045 has no excess.

## 8. Scope and unlisted writes

- `git status --porcelain`: untracked `phase4/analysis.json` (card output, 04:02:10) and `phase4/diag_consistency.json` (04:04:19); modified `tool/src/main.rs` and `tool/src/phase5.rs` (04:04:08).
- The receipt's `dirty_after` lists only `analysis.json`; the receipt (run ended about 04:02) predates the other three files. They are not outputs of the card's argv. `diag_consistency.json` is inside `write_scope` (its text says "exploratory diagnostic run after the frozen phase-4 analysis"); `main.rs` and `phase5.rs` are outside the card's `write_scope` but belong to later phase-5 work and were modified after the card finished. I attribute none of them to the executor; I cannot establish the author from the files.
- The repository HEAD is now `b07c887` (card commit `4339290`); phase 4's binds are unchanged.

## 9. Classification

- Executor conduct: the card's two argv ran, in order, once each, with the frozen env; both exited 0; no write by the card outside `write_scope`; no bind mismatch. Executed as written.
- Experiment: not complete against PROTOCOL 3 (48 classes). 45 classes complete and audited, 3 classes (c046, c047, c048) absent, refused at start because the host was not quiet during the main session's `cargo build`. The failures were caused by that concurrent build, not by the executor or the card. PROTOCOL 4 says a failed class is recorded as missing and not replaced; analysis.json does so.
- Outcome `impeded`; recommendation `rerun_after_infrastructure_fix` applies only to c046-c048 (run with a quiet host). Because PROTOCOL 4 and the card's notes forbid retry or replacement, a rerun changes the frozen procedure: the main session decides (amendment) and I did not rerun anything.
- c052's all-L0 rows are a result to report (PROTOCOL 7: rows below L2 are counted and reported); they are not a reason to rerun.
- Not assessed: interpretation of any result, any ranking, pricing band.
