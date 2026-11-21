# Validation report: runpod_capped revision 4 (scope check of a1c9a4b79c -> c895a6fd38)

Validator, independent. Every number is `[pcc:unverified]` (no pcc claim ids exist for these artifacts).

**Frozen artifact set.** Commit `c895a6fd384a5844f69e0024c7c0080ac8063c99` (HEAD of /home/user/crypto, parent `ba8821ef02`).
`git status --porcelain` was empty before and after every command (checked at the end: 0 lines, `git diff --quiet` clean).
Reviewed state: `a1c9a4b79c` (an ancestor of c895a6fd38). I edited no repository file, made no commit, no pod, no paid call,
and did not read `rpc_state`. Scratch work is in `.../scratchpad/val4/` (archives of both commits, probe scripts, outputs).
Disposition vocabulary: validated / invalid_measurement / insufficient_evidence / refused. This is a scope-and-test
validation of a draft (not a measurement), so the disposition below is on the five requested checks only.

## 1. Diff scope (PASS for scripts; two items outside the literal list, both non-script)

Merges with main in between: `git log a1c9a4b79c..c895a6fd38 -- research/pkm_falldegree_unblock_20261107/runpod_capped/`
lists exactly one commit, c895a6fd38 itself. The range holds 31 commits (12 merges, 19 non-merge); the other 30, which
arrive from main (817 files overall), touch nothing under `runpod_capped/`, and `git diff --stat` of the whole
`research/pkm_falldegree_unblock_20261107/` directory equals the `runpod_capped/` one (145 files). Confirmed.

Script blobs, `git rev-parse <commit>:<path>` compared:

| script | status |
|:--|:--|
| `run.sh` | **byte-identical** (blob equal; sha256 `c6a191f5...`; also unchanged since the smoke2 launch commit `fd5a92da08`) |
| `rp_pinned.sh`, `make_requests_m112.sh`, `tests/curl_stub.sh`, `receipts_dev/.../boot-template.sh` | byte-identical |
| `common.sh` | changed, 1 line |
| `fetch.sh` | changed, 2 hunks |
| `reap.sh` | changed, 4 hunks (comment + counter; S1 block + direct-DELETE wrap, one hunk; end message) |
| `tests/run_tests.sh` | changed (tests) |

Hunk classification:

| file / hunk | where | class |
|:--|:--|:--|
| common.sh:27 | `CLASSIFIER_VERSION=a18-r3` -> `a18-r4` | the classifier version string (allowed) |
| fetch.sh header, old 52-60 | comment block above `manifest()` (no code) | text, comment only |
| fetch.sh old 175-188 / new 176-205 | inside `decide()` (the function spans lines 158-249): replaces the host-side-evidence block, adds `$loss0 $alive $lossck $sawdone $lossuser`, new `$rl` branches | **inside decide() (allowed)** |
| reap.sh header comment (hunk 1) | two comment lines | text (lock) |
| reap.sh `left=0 errs=0 locked=0` | counter | locking |
| reap.sh new 41-48 | per-name `flock -n` on fd 8, skip + JSONL note + `continue`, release before `fetch.sh` | S1 lock (allowed) |
| reap.sh new 54-61 | direct DELETE wrapped in `flock -w 60`; `if ! flock ... elif list_pkm ...` | S1 lock (allowed) |
| reap.sh end | `[ "$locked" = 0 ] \|\| echo ... >&2` | S1 lock (stderr message only) |

No code line of fetch.sh lies outside `decide()`; no code line of reap.sh lies outside the lock handling; no change to
`run.sh`, `facts()`, `delete_phase()`, `checkin()`, the collect decision or the pins. The one new output key in decide(),
`relaunch_note`, is not read by any script (`grep`; `lostlike` reads only `.relaunch == "auto"`).

**Hunks outside the allowed scope (none in scripts), for completeness:**
- `ledger.tsv` (+23 lines, 16 EKS rows and one open US$40.00 `pfd-b-rp` row, and UNKNOWN comments) and the new
  `ledger_SOURCES.md`. These are data, not "text": `run.sh` reads `ledger.tsv` for the US$400 study ceiling
  (`run.sh:99-126`) and `ledger_has`. They change what `run.sh` computes without changing its bytes. Effects: ledger sum
  0.0679 -> 59.4003; `complete_as_of` is still `never`, so `run.sh` still refuses a study launch until the session
  sets it. No new row name collides with a `pkm-x-` name. Recomputed: 18 rows, sum 59.4003 (closed 19.4003, open
  40.0000); for rows 3-17 (bound formula (pod s/3600 + 0.25) x 0.9708) I recomputed every figure and all match to
  4 decimals; rows 1-2 are derived 1.04 x s; row 7 recorded; row 18 is the US$40 ceiling on an open pod and I did
  not verify the pod from sources. I did not check the 16 cited source lines beyond three spot checks (RESULTS.md:36,
  RESULTS_STAGE_C.md:43, SPOT_LOSSES.md:10-12, durations agree).
- Outside `runpod_capped/`: the commit also edits `research/open_leads_execution_20261108/DECISIONS.md` (row 16,
  "owner decision 16", "This session (Recommended)", about 02:30Z). Text only, but it is an owner attestation I cannot
  verify from any artifact (rows 13-15 are the same style, same limit). The dispatch's path filter did not cover it.
- Text/data that the allowed list covers: `AMENDMENT_R2_H`, `DECISIONS_ROUTE.md`, `SMOKE.md`, `tests/README.md`,
  `tests/results.tsv`, `tests/out/*` (regenerated logs: timestamps, mktemp names, hashes; two of them predate the ledger
  fill, e.g. `out/C02.txt` shows "ledger sum US$0.0679", so `tests/out/` is not byte-reproducible from the commit),
  the copied review. `tests/run_tests.sh`: C02 version string, six new cases (D43, D43b, D43c, D44, D44b, F08), README
  generator text.
- Amendment text diff read in full: edits are R1/R2 rule text, R3 corrections, S1 lock, S3/S4/S7/S8/S10 wording, §9.5, §10
  hashes, status line. S8 adds a rule sentence ("partial trees of a LOST run are never read for any rule"); it is
  consistent with the code (memory branches are reached only under `$secured`). No registered parameter, stage string,
  budget or flavor changed.

## 2. Suite, plain (PASS)

`bash tests/run_tests.sh` (no `--write`) from `runpod_capped/`, layer `r2g_dl/layer.tgz` sha256 `a0f7e28e...` (equals
`image.json` digest) and `r2g_layer/usr/local/bin`: "image layer available: 1", **passed 169, failed 0,
SKIP-NEEDS-LAYER 0** (46 of 169 are layer-dependent). 169 `PASS` lines, 0 `FAIL`, 0 `SKIP`; ran 1 min 37 s;
`git status` unchanged afterwards. Output: `.../scratchpad/val4/suite_plain.txt`.
Without the layer (copy of the committed content, `RPC_LAYER_SRC=/nonexistent`): 123 passed, 46 SKIP-NEEDS-LAYER,
0 failed, equal to the README's claim.
Earlier cases that must stay `auto` still do: C05, C13, D07b, D24 PASS; D24c, D38, D39 (owner) PASS; F01, F02, F07 PASS.

## 3. Reviewer's probes against the revised fetch.sh (PASS)

Method: `git archive c895a6fd38` into my own dir; the reviewer's `probes.sh` with only `A` and `W` repointed
(`val4/probes_scripts/probes_val4.sh`). Control: the same script on a `git archive a1c9a4b79c` reproduces the reviewer's
defects.

| probe | original a1c9a4b79c | revised c895a6fd38 | expected |
|:--|:--|:--|:--|
| P1 (earlier benign change, then restart naming the user) | LOST P3, relaunch **auto** (host_side cites check-in 1) | LOST P3, relaunch **owner**, host_side null | owner: met |
| P2 (done seen, then GET 404, before the deadline) | LOST P3, **auto** | LOST P3, relaunch **owner** | owner: met |
| P3 (done seen, then GET 404, after the deadline) | LOST P6, **auto** | LOST P6, relaunch **owner** | owner: met |
| P4 ("Exited: stopped via API") | LOST P3, auto | LOST P3, auto (unchanged) | not part of the correction; now disclosed in §4.3 (deny-list text) |
| P5 (verified tree, then 404) | ADMITTED P2d | ADMITTED P2d | unchanged |

Tests encode the cases (read from `tests/run_tests.sh` and run):
- D43 = P1 (running + "Migrated: host maintenance", then idle + "Restarted by User: x"): expects owner. PASS.
- D44 = P2 (done under s0, then 404 at S0+82000 s < deadline S0+90000): expects LOST P3, owner. PASS.
- D44b = P3 (404 at deadline + 3600 s): expects LOST P6, owner. PASS.
- D43b (benign change after the last alive check-in, before the loss): auto (positive control that the rule still grants a relaunch). PASS.
- D43c (the only change was seen while the runner was alive): owner. PASS.
- F08 = S1: a due 27 h pod whose per-name lock is held by `flock ... sleep 6`: exit 0, pod still listed, "lock held" in the
  JSONL, no DELETE call in the stub log. PASS.

**Mutation check (do the new tests bite?).** Revised suite with a1c9a4b79c's `fetch.sh` and `reap.sh` swapped in:
164 passed, **5 failed: D43, D43c, D44, D44b, F08** (each shows `relaunch=auto` or the pod deleted; D43b passes on both,
as designed). Output: `val4/suite_mutated.txt`.

Five extra boundary probes of my own (`val4/probes_extra.txt`), all as the stated rule says: alive, benign change with the runner
unreachable, then a user-named restart at the loss check-in -> owner; same with an unreadable loss REST record -> auto
(change is after the last alive check-in); done seen then idle restart -> owner; change seen while running then 404 -> auto
via form 2 (404) only; loss from the collection only with no evidence -> NO_EVIDENCE P7.

## 4. Smoke2 reclassified offline (PASS)

`cd runpod_capped && ./fetch.sh --classify-dir receipts_dev/pkm-x-dev-smoke2 results/pkm-x-dev-smoke2` (exit 0):
**ADMITTED, rule P2d**, "completed; admission rules 1-7 hold", secured true, relaunch null, deadline 2026-11-16T02:54:16Z,
classifier `a18-r4`. `jq -S .facts` is byte-identical to the recorded `outcome.json` facts; the only differences from the
recorded `outcome.json` are the classifier version (a18-r3 -> a18-r4) and the two script hashes. The git-excluded
`results/pkm-x-dev-smoke2/collect-1` manifest equals `receipts_dev/pkm-x-dev-smoke2/local-sha256-1.txt`. I also checked the
R3 numbers in the revised text against the receipts: smoke 2 `AMD EPYC 9575F 64-Core Processor`, `MemTotal` 527,605,596 kB,
container `nproc` 16, runner `nproc` 128, `cgroup2fs`; smoke 1 `AMD Ryzen Threadripper 7960X 24-Cores`, `MemTotal`
263,400,384 kB, runner `nproc` 48. All as the text states.

## 5. §10 hashes (PASS)

sha256 of the blobs at c895a6fd38 (and of the working tree) against the §10 table:
`common.sh` 5c29eb2a...f68f24, `run.sh` c6a191f5...b7bfb, `fetch.sh` cec7c7b7...933a19, `reap.sh` 686307f3...e77d3a,
`make_requests_m112.sh` 1459e3d3...d58b, `rp_pinned.sh` b4b60199...6c5d (= `runpod/rp.sh` at `d2a16c723d`),
`runpod/server.py` f56fd08e...6550: **7 of 7 equal**. These are also the hashes the offline run records in `outcome.json`.

## Observations that do not change the scope verdict (for the freezer)

1. **reap.sh exit code with a skipped pod (documentation defect).** A skipped (lock-held) due pod is not counted in
   `left`, so `reap.sh` exits 0 while a due pod is still listed (F08 asserts rc 0). The header still reads "Exit: 0 no due
   pod remains listed", and §3 of the amendment does not say so. The signals are the stderr line and the JSONL
   `skipped: lock held` row. The main session's scheduled check-in should read them.
2. Lock-path equality: `reap.sh` builds the file as `tr -c 'a-z0-9-' '_'` of the name, `run.sh`/`fetch.sh` use `$NAME.lock`;
   identical for every allow-listed and `pkm-x-dev-*` name (lowercase, digits, hyphens). F08 locks that exact path.
3. The S1 lock code in `reap.sh` and the a18-r4 classifier have never run on a pod: the smoke2 pod ran `a18-r3`
   `fetch.sh`/`reap.sh`. `run.sh` (the pod-facing code) is byte-identical to what smoke2 ran. §9.5 says smoke2 used `a18-r3`.
4. Review SHOULD items not addressed in this revision, as informational: S2 (30 h worst-case life; only the wording "at
   intervals of at most 4 h" changed, `WORST_HOURS=30` unchanged), S5, S6 partly (ledger filled, other routes still do not
   consult it), S9, S11, S4(a) (no GET after delete in `delete_phase`). The reviewer ranked these SHOULD, not MUST.

## Verdict

**Scope ok for the scripts.** The script diff is confined to the three allowed places: `fetch.sh` `decide()` (plus one
comment block above `manifest()`), `reap.sh`'s per-name lock (S1), and the `CLASSIFIER_VERSION` string in `common.sh`;
`run.sh` and every other script are byte-identical. The offline suite passes 169/169 with the layer (123 + 46 skips
without), the reviewer's P1, P2 and P3 give relaunch `owner` on the revised `fetch.sh` (they gave `auto` on a1c9a4b79c), the
new tests D43, D43b, D43c, D44, D44b and F08 encode them and fail on the old scripts (5 failures, D43b as designed), smoke2
reclassifies as ADMITTED P2d with facts identical to the record, and all 7 §10 hashes equal the files at c895a6fd38.
**Scope exceeded only outside the stated list and outside the scripts:** (i) `ledger.tsv` and the new `ledger_SOURCES.md`, data that
`run.sh` reads for the US$400 ceiling (recomputed: 18 rows, US$59.4003; `complete_as_of` still `never`); (ii)
`research/open_leads_execution_20261108/DECISIONS.md` row 16, an owner-decision record outside `runpod_capped/` that I cannot
verify. Neither alters a script byte or the classifier. Disposition: **validated** for the five requested checks, with
observation 1 (reaper exit-0-with-skip is undocumented) to be written into the text before the hashes are entered if the
freezer wants the text exact; editing the header comment of `reap.sh` would change its §10 hash, so it should go in the amendment text instead.
