# PR 3610 merge custody

The merge retains main `7ec6459ef1afa599735f9ce387c7d73fee7ea8b4` and reviewed
branch `795d9a52a57039b6c9523a622246c62985abd042` as parents. Their common base
is `1f3b1765637fee98eeff395dc610a8db7231b597`. Only the shared session log and
PCC ledger conflicted. Neither conflict was resolved by dropping a history.

Copilot request 6701797466 received no response, reaction or head change for
more than one hour. Authorized takeover was recorded in comment 6702408050
before the local merge. There was no rebase or force-push.

## Ledger and log reconciliation

Main has 401 ledger entries, retained as the exact byte prefix. The branch's
one divergent unsigned claim entry is retained byte-for-byte in
`ORIGINAL-BRANCH-SUFFIX.jsonl` and in the original Git parent. Its type, body
and signature field were appended unchanged using the existing canonical
`tools/pcc/pcc.py:append_entry`; sequence, predecessor, timestamp and envelope
hash necessarily belong to the new append. The claim file and proof digests
did not change. A final `note` entry links both parents and the original record
to its new position. `RECONCILIATION.json` records the exact mapping and hashes.
The resulting 403-entry chain passes the canonical checker against that main
parent. This is unsigned merge metadata, not a new verdict. [pcc:unverified]

Main's complete log text is retained, followed by all 5,859 bytes of the
branch's additions and the new handoff entry. The branch additions are also
retained separately. Main contained earlier log edits as well as appends, so
the branch's older log copy was not substituted for main's current text.

## Affected validation

The preparation manifest's 63 bindings and the seven native source/cohort
bindings check after the merge. No original native source, preparation file,
PCC claim, keyring or historical proof subject changed relative to the branch
parent. Against main, the only added claim file is PCC-20261107-65e160; no main
claim is deleted or modified. [pcc:unverified]

The canonical protocol checker still reports zero findings for the unchanged
draft. PCC standing remains `unverified`; its statement/attestation proof is
`not_covered` (exit 2), not failed or signed verification. Cargo remains absent
(exit 127), so no local Rust library pass is claimed. Native controls were not
rerun: neither native code nor its fixtures changed. New-head required CI and
independent review remain required before auto-merge. [pcc:unverified]

The first manifest check encountered a missing sparse-worktree path, although
the index still held the original blob. The study and claim paths were hydrated
from the unchanged index, then the checks passed. The older Git client rejected
`sparse-checkout add --no-cone`; the existing non-cone mode with `--stdin` worked.
No receipt was rewritten to conceal a failure. Raw check outputs are under
`review-evidence/`, bound by `MERGE-MANIFEST.sha256`.

A whole-log PCC prose scan returns six findings already present on the main
parent; the merged log has the identical six findings and introduces zero.
The new README and original-branch log archive scan returns zero findings.
The raw comparison is retained in `review-evidence/prose-delta.json`. Main's
existing prose is unchanged; no blanket lint pass is claimed. [pcc:unverified]

Sources checked (KB): no KB MCP was loaded and the repository KB wrapper still
returned 127. Used current main/head AGENTS.md, the newest shared log,
`docs/SABOTAGE_RESISTANCE.md`, canonical ledger functions and the original
preparation records instead. Conductor was also absent (127); no reservation
is claimed. This is repository reconciliation, not a new research mechanism.

No scientific job, paid build, point count, image publication, checkpoint upload
or credential/infrastructure change occurred. The pilot stays unfrozen/unrun.
