# 2026-10-28 top-quotient dataset

## What was done

Reconciled previous48 FFD and32 V2 certificates, current inputs and EKS inventory.
Inspected source equations and primary literature. Implemented explicit graded
quotient certificates for all three families and m2..9. Five native controls pass.

## Decisions made

Before choosing new bases, test whether the original top-form ideal has a
structural obstruction. Preserve every failed projection and every earlier run.
No factor base or original equation is changed. Root library is unchanged since
its preceding4966-pass validation; the new standalone crate is tested separately.
The shared dated session log remains outside this task's reserved scope.

## Open threads

Run frozen certificate batch, preserve/replay/export/report. Independent review,
matched candidate bases, and held-out direct regularity/solving measurements remain
required for the full elite-base goal. No universal FFD/LFD conclusion yet.

## Errors

No target-data errors before freeze. EKS shared-bucket access failure from earlier
stages remains recorded; use existing host without IAM changes or duplicate jobs.

## What was done — certificate batch complete

All48 certificates and fresh replays passed. Obtained34 finite intervals and14
positive-dimensional top-ideal certificates; all source hashes match prior
FFD/LFD systems. Raw60-file archive uploaded, downloaded, hash-checked and its
manifest compared. Export validates with zero findings. Source snapshot retained.

## Decisions made

Binary/Koblitz m2 now separates FFD=LFD2 from original top-form regularity>=28.
Prime m3..9 has no finite graded vanishing degree in this convention; this is
not a claim about affine dimension or Castelnuovo–Mumford regularity. Preserve
all old timeouts and failed projection candidates. Added committed progress
metrics only after a clear Conductor scope check.

## Open threads

Independent review, matched candidate-base generation and held-out comparisons
remain open. Do not rank base quality by unqualified original-generator regularity
or silently count preprocessing as a base improvement. Goal remains active.

## Errors

No failed producer/replay, source mismatch or export finding. No retries.

## What was done — publication

Merged the PR artifact-migration commit33c1bac without rewriting frozen history;
it replaced a prior metrics JSONL payload with a content-addressed S3 pointer.
Both histories and all bytes remain preserved. Pushed raw dataset via merge
commit59f676b2b78. The two-page report was rendered and visually inspected.
Committed-source metric forwarding produces the expected certificate fields.

## Decisions made

Keep ordinary migration changes and all previous artifacts. Refresh the existing
PR rather than create a second one. Report schema, hash checks and replay scope
explicitly; do not call producer replay independent scientific review.

## Open threads

Current shared session-log merge conflict remains reserved by Conductor T-698;
no edit to that file. CI and review remain separate from completed dataset work.
The full elite-base goal remains active.

## Errors

First push rejected because artifact automation advanced the remote branch.
Fetched, inspected and merged its three metadata/data-pointer changes; push
then succeeded. No force push, loss of source revisions or data deletion.

## Retention correction

Final diff inspection caught replacement of prior handoff entries during publication. Restored every prior entry from the parent commit and retained the publication entry. Experimental data and frozen source revisions were unaffected.
