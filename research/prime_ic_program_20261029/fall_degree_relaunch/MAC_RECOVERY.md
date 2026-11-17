# Recovering the prime fall-degree files stranded on the owner's Mac

**Status: steps (plan).** Only the owner can carry them out: on the Mac itself, or by answering
a trust card in person. No agent session in this container can reach the Mac (measured,
`docs/session-logs/2026-10-29.md`):

- `get_session` for `session_01TCbLH7A3qhgVw5cSWZKhtH` reports `computer_unreachable`;
- `list_computer_folders` lists `git`, `Downloads` and `Volumes` only under `cannot_be_used`;
- `send_message` was refused with `elevated_session_untrusted_device`.

Worktree (from the session log): `/Volumes/SSD990-2/crypto-worktrees/prime-fall-degree-20261027`,
branch `research/prime-fall-degree-20261027`. The pushed head was `f1b7dfc03`. That branch was later
merged as PR #2793 (2026-10-28 20:05 UTC).

## What to recover, in priority order

1. `research/prime_fall_degree_20261027/runs/` (every `*.jsonl` and `*.log`). This includes partial
   rows from the chains in flight when the session stopped: stage A at `t >= 11`, stage C, `d-A`, and the
   `--engine f4` attempt at `m = 7`, `t = 2` (RESTART.md).
2. `research/prime_fall_degree_20261027/gates/` (`g3.jsonl`: the 70 capped-F4 runs of Amendment 3).
3. `research/prime_fall_degree_20261027/derived/`. DERIVED.md cites `ns_degree`, `spec_chain`,
   `spec_chain_synthetic`, `spec_1d_ladder` and `spec_1d_rows`. RESTART.md's list of lost files omits this directory.
4. `progress.txt`, `TABLE.md`, `score.json`, the draft `RESULTS.md` and the report draft.
5. The uncommitted edits to `examples/prime_fall_degree_tools.rs` since `f1b7dfc03`, and any other
   uncommitted source change.
6. The identity of the binary that produced the rows: `git rev-parse HEAD` at run time, and the
   SHA-256 of `target/release/examples/pkm_tower_pilot`.

## Steps on the Mac (owner)

```sh
cd /Volumes/SSD990-2/crypto-worktrees/prime-fall-degree-20261027
d=research/prime_fall_degree_20261027
git status --porcelain > /tmp/pfd-status.txt
git rev-parse HEAD > /tmp/pfd-head.txt
git log --oneline origin/research/prime-fall-degree-20261027..HEAD > /tmp/pfd-unpushed.txt
git diff f1b7dfc03 -- examples src > /tmp/pfd-source.patch
shasum -a 256 target/release/examples/pkm_tower_pilot target/release/examples/prime_fall_degree_tools \
  > /tmp/pfd-binaries.sha256 2>/dev/null
find "$d" -type f \( -path "$d/runs/*" -o -path "$d/gates/*" -o -path "$d/derived/*" -o -name progress.txt \
  -o -name TABLE.md -o -name score.json -o -name RESULTS.md -o -path "$d/report*" \) -print0 \
  | xargs -0 shasum -a 256 > /tmp/pfd-manifest.sha256
tar -czf /tmp/pfd-mac-20261027.tgz -C . $(cut -c67- /tmp/pfd-manifest.sha256) \
  -C /tmp pfd-status.txt pfd-head.txt pfd-unpushed.txt pfd-source.patch pfd-binaries.sha256 pfd-manifest.sha256
shasum -a 256 /tmp/pfd-mac-20261027.tgz
```

Then upload by **one** of these routes, and record the tarball's SHA-256 in the session log:

- **S3 (preferred for large logs):** copy it next to the study's checkpoint,
  `s3://setec-astronomy-artifacts/checkpoints/crypto/research__prime-fall-degree-20261027/mac-20261027/`
  (that checkpoint prefix is recorded in `docs/session-logs/2026-10-28.md`, 17:15Z).
- **Git (for small text):** push the `runs/*.jsonl`, `gates/`, `derived/`, `progress.txt` and the patch to a
  data branch, for example `research/prime-fall-degree-20261027-mac-data`, and open a PR ready for review.
  Large `.log` files go to S3 only, so that the repository's size check is respected.
- **Remote Control:** add the worktree folder itself, not `/Volumes`, to the Remote Control folder list in the
  Claude desktop app. A cloud session can then request it. A Git *worktree* root has a `.git` file rather than
  a `.git` folder, and the folder rule asks for "the folder that has the .git folder", so this route may be
  refused. Then use one of the routes above.

## How the recovered files are used (plan)

- **Rows:** copy the first-run rows into the relaunch's `runs/` as `A-mac-*.jsonl` (and `B-mac-`, `C-mac-`,
  `D*-mac-`). The scorer's repeat check (same key including `x_r`; equal `last_fall_degree`, `first_fall_degree`
  and `falls`) then applies Amendment 2 at row level. Any mismatch is a reproducibility failure.
- **`t >= 11` rows, if any exist:** they were produced after Amendment 6 was registered, so they are valid
  observations. Amendment 7 must list them. A relaunch row of the same system is a repeat, not a replacement.
- **G3 evidence:** `gates/g3.jsonl` restores the 70-run record. The relaunch reruns G3 anyway (README §2.2).
- **Scorer edits:** compare them with the patch Amendment 7 §D specifies. If the lost edits already
  implemented the Amendment 3 reading, prefer them, reviewed, over a fresh patch.
- **If nothing is recovered:** RESULTS states that the first run's rows, gate records and derived checks are
  lost. The transcript medians are then the only reference for the `t <= 10` reproduction (README §2.5).
