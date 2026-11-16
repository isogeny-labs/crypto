# Publication checkpoint — 2026-11-15 02:56 UTC

PR: https://github.com/isogeny-labs/crypto/pull/4214

The implementation/report commit is
6e83f79b9760dcc61de1b698cfda6b9042febc88. The branch is published and the PR
is open with ordinary merge auto-merge enabled. Initial hosted review and
artifact migration checks succeeded; the required current-head merge checks
have not all run. No merge is asserted. [pcc:unverified]

GitHub reported DIRTY after PR creation. Per AGENTS.md's Copilot-first rule,
one PR comment requested a merge of main into this branch, preserving scope,
both ordered session logs, and immutable PCC history:
https://github.com/isogeny-labs/crypto/pull/4214#issuecomment-6888228246
Do not edit conflicted files while that request is pending. At this checkpoint
the current main ledger blob was 59b61fe35233973198750ff6b1b11209bc1e3f74,
exactly the parent ledger blob used by this PR. That observation is not a
guarantee about a later main. Never re-chain or select a side of divergent
histories. [pcc:unverified]

Next publication action: inspect PR #4214 for Copilot's response/current head,
review its merge diff against both parents and rerun affected checks. Merge
only when current-head gate/size/artifacts, actual protection and reviews
permit. If Copilot cannot act, use the documented fallback after recording
its failure; never treat the pending request as an accepted resolution.

The shared session log was not edited after the conflict request, to avoid
editing a potentially conflicted file while the designated resolver works.
Append this publication status there when integration completes. This file is
the durable task-specific continuation record until then.

Software checks: see VALIDATION.md and checks/. Mathematical next step: sharp
D.11 taper and D.14–15 enumerator refinement; this PR's coarse envelope does
not replace either. PCC-20261115-907a94 remains unverified.

Publishing note: the first shallow/blobless thin push stalled retrieving
unneeded blobs. Stopped that local process and pushed successfully with
GIT_NO_LAZY_FETCH=1, pack.window=0, pack.depth=0, pack.useSparse=true,
push.negotiate=true and --no-thin. The first premature PR-create call saw no
published branch and failed; the successful call created only PR #4214.
Changed-file artifact and large-file checks pass using the hydrated existing
.artifacts/config.json (initial default-policy check lacked that sparse file).
No artifact-policy, PCC-history or gate changes were made. [pcc:unverified]
