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
