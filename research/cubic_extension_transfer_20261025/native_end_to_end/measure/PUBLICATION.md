# Native measurement component publication, 2026-11-05

## What was done (commands, files, features)

Published implementation commit `464f7e2a` on
`codex/cubic-native-measurement-20261105` in regular
[PR #3505](https://github.com/isogeny-labs/crypto/pull/3505). The native component,
draft protocol, raw development receipts and four unsigned claim records are
committed. No scientific ECDLP benchmark has been submitted. [pcc:unverified]

## Decisions made (and why)

Leave the PR open while full-library validation remains incomplete. Preserve
the pinned draft when creating the later frozen protocol. The normal merge
analysis identifies only a session-journal conflict; current main's PCC ledger
equals the branch base ledger. Asked Copilot to merge main into the PR head,
preserve both log histories and leave immutable evidence unchanged, without
rebasing, force-pushing or merging the PR into main. [pcc:unverified]

## Open threads (unfinished, pick up next time)

Review Copilot's resolution against both parents, check the actual current
head's gates/reviews and complete full-library validation with adequate
storage. A PR follow-up is scheduled for 2026-11-05T23:59:16Z; it is not a
benchmark job. Scientific launch still needs the access and technical gates
listed in HANDOFF.md. [pcc:unverified]

## Errors (what went wrong, how it was handled)

Main advanced during publication. The PR conflicts only in
`docs/session-logs/2026-11-05.md`; that file is left untouched while Copilot's
request is pending, so this task-specific publication note records the status.
The original whole-library/disk, toolchain, compatibility and access failures
remain visible in the committed receipts. [pcc:unverified]
