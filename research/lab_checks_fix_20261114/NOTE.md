# Lab checks on main: leaderboard `--check` and `benchrec check` (2026-11-14)

Tree: origin/main 224624a91a. Both claims below were re-run locally on that tree
in this session. All are `[pcc:unverified]` assertions with the command
output quoted.

## 1. `python3 scripts/build_ic_leaderboard.py --check`

### Symptom
- 2026-11-13 (reported): validated cards (packed filter, aimed gather) were
  generated as "not validated", and `--check` called the committed files stale.
- 2026-11-14 (this tree): exit 1 with a single `SystemExit` line:
  `research/ml_cryptanalysis_pt_lgrow_k1_20261114/fb-pt-lgrow-k1: not validated; the K1
  pair-table rows cannot be shown: validator report raw_data sessions/n17-j0/session/records.jsonl:
  ... is not a file` (19 such paths).
- In CI, the `ic-leaderboard` workflow fails on every main push since at least
  2026-11-13 (runs 43377605874 to 43641377976) and never reaches `--check`. It stops at the
  pull step: `artifacts: download store/sha256/06/06b915ab...: (404) ... does not exist`.

### Root cause
1. The gate inputs were moved to the artifact store (`tools/artifacts`). Git keeps
   only a `.artifacts.json` pointer, and the bytes are gitignored:
   - `research/ic_packed_filter_20261029/validation/.artifacts.json` lists
     `validator-report.json` (moved in #3777, db7f7d3e10, 2026-11-09).
   - `research/ic_aimed_gather_20261023/aa2_20261029/runs/.artifacts.json` lists
     `isolation.jsonl`, the aimed-gather validator report's raw data (same commit).
   - The K1 session `records.jsonl` files were moved by 5a21f228a4 ("Move PR data
     files to the artifact store", 2026-11-14).

   In a checkout without `artifacts.py pull`, `validation_gate.check_evidence_dir`
   returns "is not a file" errors. Most builder sections then render a row as
   NOT_VALIDATED, which is a silent demotion and gives the 11-13 diff.
   `k1_pt_lgrow_component` raises `SystemExit` instead, which gives the 11-14 abort.
   The builder's guard `require_materialized_gate_inputs`
   (scripts/build_ic_leaderboard.py, formerly lines 522-560) caught only
   sparse-checkout exclusion (`git ls-files -t` "S" entries). It did not catch
   unpulled store pointers. The guard also never covered dirs that are checked
   outside the board and COMPONENT_EVIDENCE_DIRS, such as packed filter.

   The CI workflow pulls this data first, so this cause alone is a local and
   agent-checkout problem.
2. The CI-only cause: `research/ecbench_e2e_frontier_20261031/figures/.artifacts.json`
   names `boundary-map.png` sha256 `06b915ab...` (169645 B). That object is not in the
   store (404). Commit 3681cd325a ("research: re-establish current ECBench frontier
   plan", 2026-10-31) changed the pointer from `159fb889...` to `06b915ab...` and never
   pushed the bytes. `artifacts.py pull research/ecbench_e2e_frontier_20261031` exits 2, so
   the job fails before the builder runs. The builder does not read that PNG. It
   reads only `.../establishment/certificate.json` from that study.
3. Latent: `scripts/check_ic_leaderboard_paths.py` reported that the workflow's
   `paths:` did not cover 9 files the builder reads: `benchmarks/2026111{1,2}-mt-fieldops-*`
   (VALIDATION.json and validator-report.json) and
   `research/quartic_dl1_cd_sqrt_20261113/combo/validation/validator-report.json`. CI would
   hit this after the pull step was fixed.

### Fix (this change)
- `scripts/build_ic_leaderboard.py`:
  - Added `gate_inputs(evidence)`, the paths the gate reads. It was factored out of
    the old guard with no change in behaviour.
  - Added `refuse_unpulled_store_inputs(paths)`. It raises `SystemExit` with the
    path list and the `artifacts.py pull` remedy for any needed file that is absent
    but listed in its directory's `.artifacts.json`.
  - Added `checked_evidence_dir(...)`, which wraps `gate.check_evidence_dir`. All 18
    call sites in the builder now go through it, so no row is demoted because of
    unpulled data. In a hydrated checkout, the new code reads no extra file:
    `is_file()` short-circuits.
  - `require_materialized_gate_inputs` also calls the new refusal.

  No check is weakened, and no row's standing changes. Missing data now fails
  loudly instead of demoting the row.
- `scripts/test_leaderboard_sparse_component_inputs.py`: added two tests, one for an
  unpulled store input (refused) and one for a pulled store input (accepted).
- `.github/workflows/ic-leaderboard.yml`:
  - Added the 5 missing `paths:` entries.
  - Re-ran `tools/artifacts/wire_workflows.py` on the file. The pull step now also
    pulls the four mt-fieldops benchmark dirs.
- NOT fixed (needs store credentials or an owner decision): the missing
  `06b915ab...` object, see "Open" below.

### Before / after (local, no store access: AWS identity here gets 403 on the bucket)
Before:
```
$ python3 scripts/build_ic_leaderboard.py --check; echo $?
research/ml_cryptanalysis_pt_lgrow_k1_20261114/fb-pt-lgrow-k1: not validated; the K1 pair-table rows cannot be shown: validator report raw_data sessions/n17-j0/session/records.jsonl: ... is not a file; ... (19 paths)
1
$ python3 scripts/check_ic_leaderboard_paths.py
.github/workflows/ic-leaderboard.yml [push] does not cover 9 of 264 files the builder reads; ...
```
After:
```
$ python3 scripts/build_ic_leaderboard.py --check; echo $?
leaderboard validation inputs live in the artifact store and are not pulled here; run `python3 tools/artifacts/artifacts.py pull` on these paths (as .github/workflows/ic-leaderboard.yml does) before generating:
research/ic_aimed_gather_20261023/aa2_20261029/runs/isolation.jsonl
research/ml_cryptanalysis_pt_lgrow_k1_20261114/.../records.jsonl   (19 more)
1
$ python3 scripts/check_ic_leaderboard_paths.py
.github/workflows/ic-leaderboard.yml [push] covers all 264 files the builder reads
$ python3 tools/artifacts/wire_workflows.py --check .github/workflows/ic-leaderboard.yml; echo $?
0
```
With the board-level guard bypassed, the per-call guard also names
`research/ic_packed_filter_20261029/validation/validator-report.json`, which is the
original 11-13 demotion. Tests:
- `test_leaderboard_sparse_component_inputs.py`: 6 passed.
- `test_check_ic_leaderboard_paths.py`: OK.
- `tools/test_benchrec.py`: OK.
- `test_build_ic_leaderboard.py`: 2 errors both before and after, because
  `build()` needs the pulled data. Those errors now carry the store message.

The committed leaderboard outputs were NOT regenerated. They cannot be built
correctly here without the store bytes, and nothing shows that they are stale
once the data is pulled.

### Open (CI `ic-leaderboard` stays red until one is done)
Choose one of these options for `boundary-map.png` `06b915ab...`:
- (a) The author of 3681cd325a pushes the original bytes with
  `artifacts.py push`, if they still have them.
- (b) Re-render the figure with `research/ecbench_e2e_frontier_20261031/render_figures.py`,
  push it, and update the pointer. The bytes will differ, so this is a new
  pointer and a reviewed change.
- (c) Restore the pointer to `159fb889...`, if that object is in the store.
  This was not checkable here because the bucket returns 403.
- (d) Narrow the workflow's pull to the study's `establishment/` directory,
  the only thing the builder reads there. `wire_workflows --check` derives the
  pull list from the `paths:` glob `research/ecbench_e2e_frontier_20261031/**`,
  so this also needs that glob narrowed. Then the check would no longer run
  on frontier changes outside `establishment/`.

## 2. `python3 tools/benchrec.py check`

### Symptom
```
FAIL: benchmarks/records/20261105T1826Z-ic-aimed-gather-registry-binding-20261105.json: raw docs/curves/registry.json differs from its recorded sha256
```
This still reproduces offline on this tree.

### Root cause
`docs/curves/registry.json` is a living catalog. The closed record pins the
version at its commit 19b5e74b9a (sha256 `1edbd0d7...`, 22383109 B). The checkout
now has `33a16da0...`. `tools/benchrec.py`:
- verifies a closed record whose path changed through `archived_raw_matches`;
  see the call site near line 699,
- reads S3 under `--verify-s3`, or else `.benchrec/raw-archive-cache/<sha256>`.

Plain `check` without the cache fails by design (docs/benchmark-records.md,
"Closed raw data when a checkout path is reused"). CI runs
`check --verify-s3`, and `bench-records` passes on main, for example run
43648253722 for #4168. So this is not a repository defect. Declaring
registry.json in `living_paths` would change nothing, because the docs say it
"never waives byte verification".

### Remedy (local, documented, nothing committed)
```
$ git show 19b5e74b9a:docs/curves/registry.json > .benchrec/raw-archive-cache/1edbd0d7...c2bc8   # gitignored
$ python3 tools/benchrec.py check
benchrec: ...registry-binding-20261105.json: original raw bytes verified from local archive cache for docs/curves/registry.json; differing checkout bytes are not validated here
benchrec: OK, 173 record(s), progress report current
```

## Decision (2026-11-15): option (d)

Main's `ic-leaderboard` job still failed on the missing object `06b915ab…`
(runs 43658606351, 43664870758, 43664888784). The owner asked for the
remaining items to be done, so option (d) was taken:
- The `paths:` glob `research/ecbench_e2e_frontier_20261031/**` became
  `research/ecbench_e2e_frontier_20261031/establishment/**`.
- `research/ecbench_e2e_frontier_20261031` was dropped from the pull step.
  The generator reads its own pull line as a path reference, so the old entry
  kept itself alive until it was removed by hand.
- `wire_workflows.py --check` agrees with the result.

The builder reads only `establishment/certificate.json` in that study, so no
leaderboard input is lost. The trade-off is that the job no longer runs on
frontier changes outside `establishment/`. Those changes cannot alter the
leaderboard. The missing figure itself is still missing: options (a)–(c)
remain open for repairing the study's `figures/`.

## Option (b) attempt (2026-11-15): re-render done, push refused

All statements here are `[pcc:unverified]` assertions from this session's
command output.

- Rendered only `boundary_map()` from
  `research/ecbench_e2e_frontier_20261031/render_figures.py`, with output in a
  scratch directory. The install was matplotlib 3.10.8 (the version named in the
  committed PDF), numpy 2.5.4 and freetype 2.6.1. The only input is the
  committed `frontier_snapshot.json`, so nothing was pulled from the store.
  Neither that file nor the script changed between db7f7d3e10 and the tree.
- New `boundary-map.png`: sha256
  `81594cf35637a60c23047b0f2cfc682bd0c2c9e770798356c7c2ffcdae22add6`,
  193639 B, 2307 x 1148 px. The lost original was 169645 B (`06b915ab...`),
  so the bytes differ. The original is gone, so the pixels cannot be compared.
- The companion `boundary-map.pdf` from the same render is byte-identical to
  the committed PDF except for `/CreationDate`. The plotted data and layout
  therefore match. The PNG size difference most likely comes from the raster
  stack (freetype or zlib build). This was not tested.
- `python3 tools/artifacts/artifacts.py push .../figures/boundary-map.png`
  exited 2: `` artifacts: `aws` not found; install the AWS CLI or set ARTIFACTS_AWS ``.
  The pointer `figures/.artifacts.json` was left unchanged, still at `06b915ab...`.
- Still open: an owner with store write credentials re-renders the figure (or
  uses the PNG above), runs
  `python3 tools/artifacts/artifacts.py push research/ecbench_e2e_frontier_20261031/figures/boundary-map.png`,
  and commits the rewritten pointer as a reviewed change. The new bytes differ
  from the lost original.
