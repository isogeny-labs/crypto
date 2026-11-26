# Resume: registered RR5 B677 correctness gate

## Goal

Obtain the frozen B677 extractor outcomes needed for the ordinary prime-field
four-summand research gate. This is an extraction correctness and admission
step; no complete cold ECDLP cost or improvement is established by it.

## Lead / hypothesis

The published RR5 producer uses the original 509-point Dickson base plus the
168-point Y7 extension. The structured selector must be evaluated against the
registered positive before the fixed held-out/walk targets. The cofactor
reference is compulsory, including after an ordinary structured-positive stop.
Source review lives in the scaling-b677/payload-integration directory.

## Spec

The immutable experiment is
research/prime31_four_summand_20261122/scaling-b677/PROTOCOL.md.
The only sequence is positive/structured, positive/cofactor, then H0/structured
and W0/structured if the first call completes with independent row replay.
Keep the fixed degree/term/assignment/native resource limits, one scientific
worker, 2-GiB address-space ceiling and 180-second scientific outer limit.
The command reserves 2 CPUs/2 GiB with zero retries. Job provisioning/archive
deadline is separately bounded by the workflow request.

## State

This note is installed by the workflow before image publication or submission.
The exact source revision and workflow run/attempt are appended below.
The tested binary bundle contains build-info.json, source.sha256,
source-manifest.json, PROTOCOL.md and exact binary/script hashes.
No registered extraction outcome is asserted by the presence of this note.
The operator must already have established live capacity, a finite cost
ceiling and known remaining daily/monthly/lane budget before dispatch.

## Next steps

Run the existing checkpoint gate using trusted workflow tooling. On success,
publish the fixed bundle, prepare the immutable command request and call the
maintained remote/launch-remote-command once. Download launcher and payload
records through rr5-b677/collect, then replay rows through rr5-b677/replay in a
separate job without AWS credentials. These steps are encoded in
.github/workflows/isolab-experiment.yml; do not improvise targets or limits.

## Runs in flight

No scientific Job has been submitted when this checkpoint is first written.
After submission, retain the workflow run ID/attempt, CodeBuild ID, request
hash, Job UID, Pod UID, image digest and S3 receipt/result prefixes from
rr5-receipts and isolab-launch-output. Query the original attempt after any
ambiguous failure; a new workflow rerun is not recovery.

## Stop and success conditions

Stop before publication/submission on failed checkpoint, missing identity,
unknown/exhausted budget or insufficient live capacity. A failed or interrupted
scientific sequence remains failed; keep available evidence. A completed gate
requires both positive calls and both admitted followups, exact identity and
hash checks, complete source-bound traversal metadata and independent replay
of all rows. Even this does not prove independent completeness, solve the
complete ECDLP workload or beat rho. Report costs and timing only at their
measured scope; all unsigned execution assertions remain [pcc:unverified].

## Open questions

The registered B677 extraction outcomes and any downstream ECDLP value remain
open until actual admitted execution and evidence review. Confirm that the
existing publisher identity can write the checkpoint destination; do not
substitute the job-results bucket or change access policy if it cannot.
The workflow can use the repository's existing scoped checkpoint-upload token.
Full results readback is separately unresolved: the checked-in GitHub trigger
role has neither ListBucket nor the necessary raw-record GetObject grants.
Use only an already authorized transport; retain that boundary and do not
rename raw records to a permitted sanitized receipt filename.

## Local qualification checkpoint — 2026-11-23 16:28 PST

Published implementation: 8aa70e211f9a42e9823e859b1c7197c8677dceca, PR https://github.com/isogeny-labs/crypto/pull/5099. The exact published-source native bundle is in /workspace/scratch/cdbe94c88979/rr5-qualified-bundle. Qualification records are in research/prime31_four_summand_20261122/scaling-b677/payload-integration/evidence/. Native unit and mock contract observations are unsigned (PCC-20261124-21920c); required root-library compilation still fails in unchanged root code. No registered scientific call or cloud Job has been submitted.

The immediate action is scripts/checkpoint.sh gate from the current worktree. Do not publish or dispatch unless it succeeds and an already authorized identity can collect the complete raw results. Remaining daily/monthly/lane budget and live capacity are unknown; no paid build or scientific workload is admitted. The checked-in GitHub trigger role does not grant the necessary full raw-result readback. On an unchanged transport failure, retain the failure and stop cloud execution. Complete evidence publication and PR checks independently.
