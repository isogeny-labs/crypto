# Active generated auxiliary setup continuation

Protocol frozen before source/native calls. Sources and exact scope are in
PROTOCOL.md, following the parent SETUP-REVIEW.md. Candidate uses fixed-degree
point halving and actual point checks, with independent OpenSSL preimage and
setup/cold replay; no generated result has yet been observed. [pcc:unverified]

The independent worktree is codex/auxiliary-halving-setup-20261115. Its initial
--no-checkout index was empty; sparse set alone did not populate it and a
merge saw tracked files as deleted. Initializing the owned empty worktree at
HEAD repaired it. The lingering failed merge process was terminated before
finishing the new merge. No new source or native call occurred during repair.
Merge15cad060ae preserves all186 main journal entries and all3 own entries.
The subsequent artifact-only parent merge preserves its trusted-base manifest.

Parent PR4300 is open, auto-merge enabled, with one Copilot request at
https://github.com/isogeny-labs/crypto/pull/4300#issuecomment-6893259365 to merge
main without rebasing/force push. Its shared journal is frozen while that
request is pending; continuation decisions are recorded here meanwhile.
Artifact-autofix run43704942686 reports one uploaded and complete-byte replayed
remote object, manifest commit379576ac63, whole indexed005688992 bytes and
SHA2568b8cb99d428ef8825d4522d2ed3583673605bfb5c8948b83b417e5ac6b6b5f38.
This does not promote PCC or execution standing. [pcc:unverified]

Parent root Rust suite completed at source2fd16de7ca:6406 passed,48 failed,
139 ignored. Failure output names omitted sparse checkout data/device sources
in untouched tests; the focused device case reproduces its missing source.
Its exact complete log is retained for the next evidence commit; no root-suite
pass is claimed. The original partial snapshot remains unchanged. All native
and local Lean controls retain their previous scope and source bindings.
[pcc:unverified]
