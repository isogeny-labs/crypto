# Amendment 3: pushed-Git checkpoint transport

Date frozen: 2026-11-27 UTC, before any admitted depth census.

## Reason

The repository checkpoint wrapper requires S3 transport, but the owner has
confirmed that S3 is not available for this task. Three executor restarts
showed no upload token, token file, AWS profile, or bound Vault credential.
Treating unavailable transport as a scientific prerequisite would prevent the
already bounded deterministic experiment without improving its semantics.

No candidate census output, verifier transcript, analysis row, or promotion
classification existed when this amendment was written.

## Replacement checkpoint gate

For this study only, the one-shot runner uses a pushed-Git checkpoint by
default. Before release build or execution it must establish all of the
following:

1. `GIT_CHECKPOINT.md` exists and is tracked;
2. the Git index and visible worktree are clean, including untracked files;
3. the current branch has a unique remote branch of the same name on `origin`;
4. local `HEAD` exactly equals that remote head; and
5. every active source-manifest SHA-256 passes.

The tracked checkpoint contains the complete goal, state, launch command,
artifacts, and stop conditions. The source, protocol, verifier, analyzer,
runner, and checkpoint are therefore recoverable from the pushed branch before
the run. The successful evidence directory is committed and pushed immediately
after verification and inspection. A failed stage preserves its uniquely named
temporary directory and is not silently retried.

`PPLUS_CHECKPOINT_MODE=s3` remains available if transport later returns, but it
is not required for this run.

## Unchanged scientific contract

This amendment changes only durable checkpoint transport. The candidate,
curve, fixed targets, independent replay, cost model, subquadratic tests,
1.6-million gates, no-cross-target rule, and stop-before-m=7 rule are unchanged.
It adds no result-dependent choice and mints no PCC claim. [pcc:unverified]
