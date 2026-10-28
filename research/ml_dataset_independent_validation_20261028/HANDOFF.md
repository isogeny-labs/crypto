# Consumer handoff — 2026-10-28 UTC

## What was done (commands, files, features)

Added the isolated dataset audit, S3 fetch, and static feature export CLI;
19 adversarial software tests passed. A real S3 software fixture was uploaded,
downloaded and rehashed: 12 references, 10 unique objects, 3,692 declared bytes.
The completion receipt was published last and read back. Pinned development
code audits were archived to S3 with download/hash verification. Small evidence
records are in this directory. No native producer or existing E12 worker was
changed by this companion work, and no new compute worker was launched.

## Decisions made (and why)

Conductor T-693 reserves this separate directory. T-684 owns the native
producer in another worktree. Keep witnesses, manifest seeds and terminal
solver traces out of model features, and preserve unsupported/censored rows.
Use an isolated restored tree for native replay and reject changed manifests
or verifier binaries. The shared daily log is reserved by another owner,
so this handoff stays with this scoped PR.

## Open threads (unfinished, pick up next time)

Complete native-dataset-to-S3-to-independent-consumer validation once an
actual completed native dataset is available. Reconcile static audit findings
with the producer's final revision. Full solver/curve/twist coverage, counts,
interruption retention and campaign launch remain with the native producer.
The total $200 ceiling and conservative budget record are in merged PR #2751;
no account-wide enforcement is claimed. Avoid duplicate launches.

## Errors (what went wrong, how it was handled)

A no-checkout worktree initially had an empty index. Initialized it from HEAD
and restricted sparse checkout before staging; no repository deletions are
included. Independent consumer review found a new factor-base rule was
rejected and native replay could certify stale cached rows. Added both the
new exact rule and pre/post input/binary checks, with regression tests.
The initial producer test log had failures; those are the other worktree's
responsibility, not evidence that this consumer validates native datasets.

# Phase measurement continuation — 2026-10-28 UTC

## What was done (commands, files, features)

Added hash-bound phase telemetry labels to the independent training view.
The suite now passes 25 tests, covering exact/raw/scaled distinctions, absent
PMU readings, unfinished phases, inconsistent timing/scaling and feature
separation. No new generation, PMU experiment or compute worker was launched.

## Decisions made (and why)

Keep phase counts and exclusions intact instead of presenting them as total
pipeline instruction cost. Preserve older S3 and software validation receipts;
the new validation record identifies the changed consumer and tests separately.

## Open threads (unfinished, pick up next time)

At the current read-only check, producer HEAD was
3f1c7822976d29b15a01fd94e0f29c417760c6b5, its native builds remained active,
its pilot protocol remained draft, and the documented native S3 catalog
prefixes contained no completed cells. The $200 budget admission and durable
publication of retained interrupted attempts were not integrated in the
inspected producer. Source audits still identify prime F4 modulus limits and
public/config and outcome consistency defects; the consumer rejects several
of these but does not repair the separately owned producer. Recheck the final
revision before describing any of these as current or resolved.

## Errors (what went wrong, how it was handled)

The repository's restricted fetch refspec did not create a remote-tracking
ref for the consumer branch. Fast-forwarded using the explicitly fetched
FETCH_HEAD instead; no rebasing, force pushing or producer edits occurred.

Independent review found unchecked cycle scaling and unavailable-counter estimates.
Both counter tuples now use the same producer-compatible validator, including
zero-running boundaries and aggregate multiplexing. Regression tests pass.
The original consumer merged in PR #2754; T-700 owns this follow-up.

# Native consumer and positive arithmetic — 2026-10-28 UTC

## What was done (commands, files, features)

Replaced the supported execution path with the independent Rust crate in
`native/`: audit, static export, phase-label validation, bounded S3 restore,
optional producer replay and public positive-certificate arithmetic. The crate
links no producer arithmetic. The full release suite passed 79 tests (72 library,
7 integration), with formatting, strict Clippy and release build also passing.
Native integration includes rehashed false certificates and provenance, corrupted
objects, incomplete attempts, duplicate keys, symlinks/FIFOs and label isolation.

Read-only AWS inspection at 06:52 UTC found E12a and E12c still running; both
06:51:57 snapshot archives had zero completed target draws. E12b's completed
mixed result was merged in PR #2758 at 06:21:29 UTC. No new worker was launched
and no IAM permissions were changed in this continuation.

## Decisions made (and why)

The repository forbids Python field arithmetic, certificate verifiers and replay.
The prior Python consumer is historical; its files and old test receipts remain
unchanged. The unpublished arithmetic draft was archived to S3 and read back
with matching SHA-256 5457479f629afc34b19baf573f69b2a4e5b6ccc53878f8df70dd19c538cf0a15.
The native replacement has separate validation and S3 evidence. Public scalar
or PDP equations do not establish order, solver independence, UNSAT, cost or
fall-degree claims. Both broad independently-verified flags remain false.

Conductor T-706 reserves only this companion directory. The shared daily log
and native producer are separately owned, so this scoped handoff preserves the
work without changing either owner’s files. The total budget ceiling remains
$200 through tomorrow, with the existing conservative $188.3888 reservation;
there is no new daily allowance or account-wide spending guarantee.

## Open threads (unfinished, pick up next time)

Pilot `ml-dataset-4c94f1d-pilot-002-20261028` failed with S3 HeadObject 403 before
its first publication. The dated canonical catalog remained empty at 06:47 UTC.
No recoverable native cell was found in the available job/receipt interfaces.
Producer HEAD was eb7e1c203a5dfc98eae88134f6b43e775ee45721, with substantial
uncommitted native coverage work in its separate worktree. Reconcile that owner
before edits, retries or source freezes.

A prepared prefix-scoped `isolab-job` inline policy is awaiting the explicit
administrative approval already requested. It permits no deletion or IAM
management. After approval, check the actual worker's fresh-key HEAD/conditional
PUT/GET/hash behavior: prefix-limited ListBucket may still leave a missing-key
HEAD returning 403. Fix that narrow publication path if needed; do not silently
widen listing or reuse the old unapproved bucket-root store. Run no new pilot
until preflight and source/budget gates pass. Separately requested permission
to message the other dataset chat remains pending.

The broad campaign remains unfinished: solver/factor-base/family/twist coverage,
all intermediate traces, instruction accounting, real native S3 replay, extensive
production, linked-component splits and scientific acceptance of labels.

## Errors (what went wrong, how it was handled)

The first unpublished arithmetic extension mistakenly used Python. It was
superseded before publication with native Rust, preserving the draft as historical
evidence. Independent review found FIFO reads could block before file-type checks;
regular-file checks, no-follow/nonblocking Unix opens and regressions now cover it.
A release-only corruption-fixture failure was fixed by constructing equal-length
corrupted bytes directly in Rust, ensuring the hash rejection is what is tested.
The first strict Clippy pass found three style issues; they were fixed and the
complete release suite rerun. None of these software fixtures are campaign draws.

At 07:02 UTC the native live S3 software test completed: 13 references,
11 unique objects, 4,153 declared bytes, complete ETag-bound download and hash
verification, and one independently checked positive certificate after restore.
The receipt SHA-256 is bb4afa59b8e2b68b9e5f59ff547db60b6c7688bac6ae5c4c6ab79449d3124122.
`evidence/native-20261028/validation.json` pins the source hashes, binary,
reports, raw native test output and the durable receipt. This used the controller
role and a software fixture; it does not validate worker permissions or replace
the first real native dataset replay.
