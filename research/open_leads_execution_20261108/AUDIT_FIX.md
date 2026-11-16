# AUDIT_FIX (slug r2audit): ecbench audit fails on curve25519_f1_walk_20261107 sessions

## Goal
Make the `evidence_audit_pr` shards pass for `research/curve25519_f1_walk_20261107/sessions/*` without editing evidence. All numbers `[pcc:unverified]`.

## What was done
- Read `.github/workflows/ecbench.yml`, `.github/scripts/ecbench-audit-sessions.sh`, `tools/artifacts/artifacts.py`, `docs/artifacts.md`, `.artifacts/config.json`, and the commit e57a00a17d.
- Findings (measured/derived from files): the workflow hydrates by running `artifacts.py pull` on each session directory only. The study's records were packed in `data/sessions-k{20,24}-records.tar.gz`, which the artifact bot moved to the store. Those tarballs are not under any session dir, so no step ever extracts them. Nothing else in the repo consumes them (grep). So no session has `records.jsonl`.
- Store is readable with this session's identity: `AWS_DEFAULT_REGION=us-east-2 artifacts.py pull` of both tarballs succeeded; sha256 188e090f... and ca22cd0d... match `data/.artifacts.json`. Each tarball holds `sessions/<id>/records.jsonl` for 8 sessions (16 files, 3.27 MB each).
- Reproduced before (real tree, the exact CI script on the 16 sessions, empty pin TSVs, `target/release/ecbench`): all fail with `records.jsonl: No such file or directory (os error 2)`.
- Applied fix (a) in a scratch copy (git archive, file:// store): extracted the members, `artifacts.py push` per file, deleted them, `pull` from the store, ran the script again: `16 of 16 sessions pass integrity ... 0 unbound prefixes`. `records_bound` was true for all, so the session.json records digests match the tarball bytes.
- Staged the generated pointers in `research/open_leads_execution_20261108/r2audit_stage/` (16 x `.artifacts.json` + `.gitignore`, `records.sha256`, `apply.sh`). The scratch tree and store were deleted.

## Results
- Before: 16 of 16 sessions fail (measured). After (a): 16 of 16 pass, `--replay 0 --allow-interrupted` (measured, scratch tree, file:// store; the binary is the existing `target/release/ecbench` built Nov 7, not rebuilt for main f13a690).
- Not shown: a run against the real S3 store, because I did not upload 16 objects to the shared write-once store.

## Decisions made (and why)
- Chose (a): 550 session dirs already use per-session `.artifacts.json` for `records.jsonl` (e.g. research/bitop_ladder_20261019/sessions/holdout-P16), so the existing `pull` step in both audit jobs hydrates them with no workflow or script change. New files are additions, which the "sessions are never edited" gate (`--diff-filter=MDRT`) allows.
- Rejected (b): the pull step never fetches `data/`, so (b) would need script plus two workflow edits (audit and replay jobs) plus a sparse-checkout of `data/`, and a convention that is specific to this packing. More invasive and a shared-CI risk.
- Rejected (c): the records exist and verify; a defective pin would hide a working session.
- The tarballs stay (they are the study's committed data and the README/REPORT cite the study as is).

## Open threads
- The 16 objects are not in the real store. Either the PR bot's migrate step uploads them (commit the raw extracted `records.jsonl` files, as the study did with the tarballs, and the bot rewrites them to pointers), or a committer with write credentials runs the `push` line below.
- The wider CI (`large-files`, `artifacts check`) was not run on the staged change.

## Errors
- First scratch run used `rm -rf *` after `cd`, which the harness refused; no deletion happened. Re-ran without it. A first `push` on the directory said "nothing to move" (policy only sees tracked files); per-file push worked.
- The script cannot run with a subset list against the real pin TSVs ("names no committed session"); I used empty TSVs for the local run.

## Ready-to-commit
Files (all new, none edited): for each of the 16 sessions `research/curve25519_f1_walk_20261107/sessions/<k20|k24>-s<0-7>/.artifacts.json` and `.gitignore` (copies in `r2audit_stage/<id>/`). Run from the repo root:
```
bash research/open_leads_execution_20261108/r2audit_stage/apply.sh
python3 tools/artifacts/artifacts.py push research/curve25519_f1_walk_20261107/sessions/*/records.jsonl   # uploads 16 objects; rewrites the same manifests
git status   # expect only the 32 new pointer/ignore files; records.jsonl stays untracked/ignored
python3 tools/artifacts/artifacts.py verify --remote research/curve25519_f1_walk_20261107/sessions
rm -f research/curve25519_f1_walk_20261107/sessions/*/records.jsonl
python3 tools/artifacts/artifacts.py pull research/curve25519_f1_walk_20261107/sessions/*
ls research/curve25519_f1_walk_20261107/sessions | sed 's#^#research/curve25519_f1_walk_20261107/sessions/#' > /tmp/list.txt
printf '# none\n' > /tmp/none.tsv
bash .github/scripts/ecbench-audit-sessions.sh /tmp/list.txt target/release/ecbench /tmp/none.tsv /tmp/none.tsv   # expect 16 of 16 pass
```
CI exercises it in `evidence_audit_pr` (pull step, then the audit script) on any PR that triggers the full session audit.

## Shared files edited
None. New files only under research/open_leads_execution_20261108/ (AUDIT_FIX.md, r2audit_stage/). Scratch list file: scratchpad/r2audit-list.txt.
