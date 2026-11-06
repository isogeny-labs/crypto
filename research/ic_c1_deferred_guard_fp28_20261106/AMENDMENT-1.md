# C1 route A: AMENDMENT-1 (procedural, pre-data), revision 2

**Status: FILED 2026-11-06, as signed off by the Coordinator (rulings addendum B1); it is frozen when its pull request merges.** With the
`protocol.json` change in section 5 it is the version-2 freeze. Revision 2 applies the Coordinator's edits R1-R4 (rulings
addendum A1) to revision 1 (`amendment/AMENDMENT-1-DRAFT.rev1.md`, sha256 `c745bc0a4c3f530ae3ee33bce9f0508734b4a53c85b56fbc03fe91ea2b145e84`).

Protocol `IC-C1-DEFERRED-GUARD-FP28-20261106`, frozen version 1: `protocol_sha256`
`43b1ca918c034433f3584f37db439613d11a618c0eec13d3d7831a1fa96d092f`, commit `339292abed16c2474e404e25f3b72d31b58bd9d8`,
`frozen_at` `2026-11-06T10:11:25Z` (crypto PR #3558).

Filed before any C1 session was launched, before any C1 record exists and before any C1 wall or counted field was read:
`filed_before_affected_data: true`. It changes no N, seed, curve, target count, arm, weight, threshold, margin, accounting rule,
decision mapping, budget gate or deadline. **`PROTOCOL.md` is not edited**: its bytes stay as frozen at `339292ab`; the pointers to this
amendment are this file, `links.amendments` and `extensions.c1_amendment_1` of `protocol.json`.

## 1. What is amended

Frozen text, `PROTOCOL.md` section 18 (revision 6), the `S01-probe` bullet: the step "confirms the crypto checkout HEAD is the pinned
source" and "recomputes every bound request sha256". The first sentence is replaced by section 3 (a); the second is restated, not weakened,
in section 3 (a). Nothing else in section 18, section 9 or any other section is changed. Where this file and that bullet disagree, this
file governs.

## 2. Reason (measured; full hashes)

The HEAD check cannot be satisfied as written. Hashes of `taskq/deploy/eks-karpenter/remote/<file>`, at the pinned source commit
`ea8e9ef71c3c82027dd9e01535302725519c3fca` (read with `git show ea8e9ef7:<path> | sha256sum` in the pinned clone) and on `main` (the
values the v6 launch card pinned, equal to origin/main on 2026-11-06):

| script | at `ea8e9ef71c3c82027dd9e01535302725519c3fca` | on main / used by v6 |
|---|---|---|
| `launch-remote` | `076ee44fac6ee5948d0836d9b8dccb53e8107eba02debd0f64e84d01a5166f45` | `de27df3830fb54589e5d030e416d2e6887f7d590bc03db2e50d7456449fc7362` |
| `validate-request` | `dc6b59d32e937db5c049704102fe0d5ca0db7af9ac2bd8de67270bcafc230426` | `dc6b59d32e937db5c049704102fe0d5ca0db7af9ac2bd8de67270bcafc230426` |
| `run-request` | `e9706cc618afed952d5d073866bb30f8b31302b1410f1d77a78784aac1fa2192` | `c4f75c585ca6c689bdbe3fa4840459df5347696db8901a3e23296239d21e6819` |
| `current-run-request-buildspec.yml` | absent (the directory has an older `buildspec.yml`) | `35504896fd8e9eae15690bda0ddd11c2a453ec1c463b8cbeb3c1224f286f335e` |

The launcher runs from `main` (as for v6); only the task image is pinned to `ea8e9ef7`. A checkout whose HEAD is `ea8e9ef7` would lack the
buildspec and carry different `launch-remote` and `run-request`, so the frozen check would stop the session for a reason that has no
bearing on what the experiment measures. A checkout of `main` fails the frozen check by construction.

## 3. Replacement text

> (a) `S01-probe` runs one bound script, `implementation/probe_s01.sh`. In order, and stopping at the first mismatch, it: prints
> `aws sts get-caller-identity`; checks the sha256 of the local pinned ecbench against `73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01`;
> recomputes the sha256 of every request the runbook binds, each of which must be present, against `protocol.json` `extensions.requests`
> (a bound request that is absent, or unknown to `protocol.json`, stops the session; nothing is skipped because it is absent; a `-r2` session
> binds and presents its `-r2` requests); checks the four launcher scripts under `taskq/deploy/eks-karpenter/remote` against
> `implementation/remote-pins.sha256`; and prints `git rev-parse HEAD` of the crypto checkout, which is recorded and not compared with a
> pinned commit. `launch_eks.sh` re-checks the four launcher scripts against the same pin file before any AWS call.
>
> (b) Build-source identity is enforced by the frozen requests' `job.build` (`source_revision` `ea8e9ef71c3c82027dd9e01535302725519c3fca`,
> `image_uri` `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:3c4ad8bf8ee1bd1fdc327cb4c0e2597a1fc5ef1cdd889aa21e1b1294bdd22914`,
> ecbench sha256 `73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01`), restated in the runbook `spec_bindings`, and verified after
> each job by `check_job.py` (`build-info.json` `source_revision` and the launcher `result.json` `build.image_uri`).

## 4. What this does not change

- The pins in `remote-pins.sha256` are the values the v6 launch card used, unchanged (sha256 of the pin file
  `7b7531895ea935bf996fb983d837c57fe5a14439e93bb38917d9ee5a7468ee0f`). A change to any of the four scripts stops the session in S01 and again
  in `launch_eks.sh`.
- Replay of trials, the exit-code acceptance set {0, 42, 43}, the duplicate guard, the rerun rule (`-r2` is a new session after a
  Coordinator decision), the section 10 gate and the 3,600 s deadline are as frozen.
- The implementation fixes ruled the same day (wrapper layout, S01 and S03 steps, step timeout 9,600 s, claim TTL 165 minutes,
  `memory_mb` 4,096, undeclared large artifacts, outcome classification) are edits to unfrozen files and are not part of this amendment.

## 5. How it is filed (format supported by `tools/experiment_protocol.py`)

`experiment_protocol.py` has no amendment subcommand. The supported mechanism, used by v6 (`V6-AMENDMENT-1`), is a version bump: the schema's
`amendments[]` entry (`id`, `filed`, `filed_before_affected_data`, `reason`, `changes[]`, optional `supersedes_fields[]`), the lint that flags any
amendment filed after the data it affects, and `freeze`'s refusal to re-freeze a frozen file ("bump version and amend instead").

1. Merge `AMENDMENT-1.md` (this text, completed) to `main`; record its sha256 in `extensions.c1_amendment_1.file_sha256`.
2. `protocol.json` becomes version 2: `version` 2, `links.amendments` (the schema admits it: `validate` 0 findings), the `amendments` entry
   below, `extensions.c1_amendment_1`, and `freeze` set back to the draft block.
3. `validate`, then `freeze ... --filed-before-affected-data`, then merge. The runbook re-binds `inputs/protocol.json` and updates the
   `/freeze/protocol_sha256` binding to the v2 value; every packet reference that means "the protocol in force" moves to v2 (grep the packet files for
   `43b1ca91`), and only provenance keeps v1. The approval DEC cites the `AMENDMENT-1.md` sha256 and both `protocol_sha256` values.

A validated candidate is `next-candidate/amendment/protocol-v2-candidate.json` (built by `amendment/build_candidate.py`).

`amendments[0]` (full text in the candidate file):

```json
{
  "id": "C1-AMENDMENT-1",
  "filed": "2026-11-06",
  "filed_before_affected_data": true,
  "reason": "PROTOCOL.md section 18 (revision 6) requires step S01-probe to confirm that the crypto checkout HEAD is the pinned source ea8e9ef71c3c82027dd9e01535302725519c3fca. That check cannot be met: ... launch-remote is 076ee44fac6ee5948d0836d9b8dccb53e8107eba02debd0f64e84d01a5166f45 there and de27df3830fb54589e5d030e416d2e6887f7d590bc03db2e50d7456449fc7362 on main; run-request is e9706cc618afed952d5d073866bb30f8b31302b1410f1d77a78784aac1fa2192 there and c4f75c585ca6c689bdbe3fa4840459df5347696db8901a3e23296239d21e6819 on main; current-run-request-buildspec.yml (35504896fd8e9eae15690bda0ddd11c2a453ec1c463b8cbeb3c1224f286f335e on main) does not exist at ea8e9ef7...; validate-request is identical (dc6b59d3...0426). Filed before any C1 session was launched or any C1 record read.",
  "changes": ["S01-probe checks the four launcher scripts ... against implementation/remote-pins.sha256 (full hashes) ...",
              "S01-probe recomputes the sha256 of every request the runbook binds, each of which must be present, ...",
              "Build-source identity is unchanged in substance: ... (full hashes) ...",
              "No N, seed, curve, arm, weight, threshold, margin, accounting rule, decision mapping, budget or deadline changes. PROTOCOL.md is not edited."],
  "supersedes_fields": ["PROTOCOL.md section 18, step S01-probe: 'confirms the crypto checkout HEAD is the pinned source'"]
}
```
(the elisions above are for this page only; the candidate file holds the complete strings, with every hash in full.)

## 6. Receipts to archive in the amendment PR (R4), with the exact command lines

Run from the root of a crypto checkout containing the amendment PR's files (`research/ic_c1_deferred_guard_fp28_20261106/`), with
`TOOLS=<crypto-autoresearcher checkout>/tools`. Outputs are archived verbatim as `amendment-receipts.txt` beside `AMENDMENT-1.md`.

1. Validate the v2 file (expected `0 finding(s)`, exit 0):
   `python3 $TOOLS/experiment_protocol.py validate research/ic_c1_deferred_guard_fp28_20261106/protocol.json`
2. Reproduce the v1 hash (expected `43b1ca918c034433f3584f37db439613d11a618c0eec13d3d7831a1fa96d092f`): write a copy with `version` 1,
   `amendments` `[]`, `links.amendments` and `extensions.c1_amendment_1` removed, then
   `python3 $TOOLS/experiment_protocol.py hash <that copy>`
   (the copy is made with `jq '.version=1 | .amendments=[] | del(.links.amendments) | del(.extensions.c1_amendment_1)' protocol.json`; `hash` ignores the `freeze` block).
3. Freeze (after the merge of `AMENDMENT-1.md`, with `file_sha256` filled): `python3 $TOOLS/experiment_protocol.py freeze research/ic_c1_deferred_guard_fp28_20261106/protocol.json --commit auto --filed-before-affected-data`, then
   `python3 $TOOLS/experiment_protocol.py hash research/ic_c1_deferred_guard_fp28_20261106/protocol.json` and `jq -r .freeze.protocol_sha256 research/ic_c1_deferred_guard_fp28_20261106/protocol.json`: the v2 `protocol_sha256` (the two must agree).
4. `sha256sum research/ic_c1_deferred_guard_fp28_20261106/AMENDMENT-1.md` and `git diff --stat 339292abed16c2474e404e25f3b72d31b58bd9d8 -- research/ic_c1_deferred_guard_fp28_20261106/PROTOCOL.md` (expected: empty, R3).

Receipts 1 and 2 were run on the candidate on 2026-11-06 (`next-candidate/amendment/receipts.txt`): validate `0 finding(s)`, exit 0; reproduced hash
`43b1ca918c034433f3584f37db439613d11a618c0eec13d3d7831a1fa96d092f` equal to the frozen file's.

## 7. Open points

- `PROTOCOL.md` section 18 is narrative, not a `protocol.json` field; `supersedes_fields` names the sentence (the v6 amendment did the same).
- The approval DEC cites the `AMENDMENT-1.md` sha256 and both `protocol_sha256` values (v1 `43b1ca91...`, v2).
