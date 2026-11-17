# Exact final-source selection review

## Recomputed selection

Live exact-commit Git inventory matched the proposed frozen inventory byte-for-byte. Recomputed 5909 selected files, 289945217 bytes and 190081 omitted entries. Existing selected changes consist only of isolation.rs; additions are two retained benchlock.rs and two retained helper.rs files. No selected path was removed.

## Closure

All eight recursive local package manifest blobs remain unchanged, including the optional five-leads dependency. No local default build.rs is present. The include scan is byte-identical to the old reviewed scan: 141 expressions and the same retained runtime-string false positive. All source/config metadata selection rules remain; exact path patterns are checked. This is retained lexical closure, not arbitrary macro expansion proof.

## Prospective mutation path

The refresh script targets the fixed existing isolated checkout and target-cache owner lock. It requires a reviewed wrapper hash, admitted binding, exact final commit/tree, old/new selection hashes and matching patterns. Before mutation it obtains a nonblocking exclusive lock, checks scoped live processes, detached old HEAD, tracked cleanliness, all untracked files including ignored files, and exact old materialized blobs/modes. Only sparse pattern update, detached checkout and reapply mutate the isolated checkout. New commit/tree, cleanliness and complete materialization are then checked. No build, cleanup, cache fingerprint changes or automatic retry is present.

## Deadline and receipt boundary

Git commands have per-command timeouts within the shared allowance; verification checks a cooperative deadline. A previously observed unsealed source gap omitted TimeoutExpired command receipts. Final reviewed source writes a started record and preserves timeout output/error receipts. Filesystem IO can still block beyond a cooperative check; root must retain outer supervision and reconcile this same destination on interruption. This review does not turn that bound into a hard IO wall-time guarantee.

## Admission conditions

Root must supply the exact reviewed binding, hold exclusive source ownership, and reconcile live owners immediately before execution. The lock is released at script return; cached build admission must reacquire its own lock and recheck source/owner bindings. The script rechecks the exact external-config ledger and CARGO_HOME before mutation; its process scan includes CARGO_TARGET_DIR and nested source working directories. Read-only source verification does not provide protection against an unrelated concurrent writer.

## Evidence and limits

The selector completed read-only with exit0; its initial argument-shadowing failure is retained. This reviewer inspected source and recomputed metadata only: no checkout, build, container or scientific run. Proposed preparation remains unexecuted. Old b985 binary/receipts remain unchanged; this review concerns preparation for exact 3c592af and does not qualify a new binary.
