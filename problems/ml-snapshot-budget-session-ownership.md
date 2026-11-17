# Snapshot helper ownership of the campaign budget session

- **Status:** implemented; whole-integration validation pending
- **Kind:** integration and process-ownership gap
- **Affected work:** PR2958, reviewed ML trace components, src/ml_dataset/live.rs, snapshot.rs, budget.rs, s3_budget.rs
- **Next action owner:** CI/main integration maintainer
- **Last updated:** 2026-10-30

## Problem

The older partial-snapshot helper reopens its worker budget journal in another process, while current main deliberately permits only one live local owner of a campaign worker session. Combining those APIs directly cannot preserve both snapshot behavior and the current ownership guard.

## Evidence and uncertainty

- **Observed:** Stack378fec1495f983120dfe0078ca9fa0160b5b7c83 uses mounted-journal BudgetPolicy/Limits, BudgetSession::open(policy,allocation,journal), per-command S3 guards, and a separate snapshot-worker process. Main da7a63b11b486861bbc0ee6ac7695a9c756486f8 uses owner-bound campaign/launch grants, monetary/output reservations, SQLite accounting, exclusive worker flock, immutable activation and native S3BudgetContext transport. Its BudgetSession::open requires path, typed policy, grant and owner; a second live owner is refused.
- **Observed:** The normal main merge has21initial conflicts. Unambiguous typed capture, native-root cap, strict remote-absence and full-journal resolutions preserve both parents' controls; the budget/live/snapshot integration is not yet compiled or qualified.
- **Derived:** A helper may not silently receive a second allowance or bypass the lease. Existing operation/byte limits do not replace the current monetary, launch or owner constraints.
- **Implemented:** Native snapshot publication/restore now use the current S3BudgetContext, exact expected-object registration and original Cell/Work/Finalization scopes. The bounded helper inherits the same owner flock description and resumes the same journal/activation nonce; no new grant or balance.
- **Verified component:**105 actual production budget/store tests passed, including all original current and offline-legacy budget controls and a real exec child-process lease/unknown-charge test. This is component qualification only; the whole library and CLI still require validation.
- **Scope:** No cloud worker, scientific trial or new allocation was launched. Old frozen protocols, traces and reports retain their original source attribution.

## What would unblock it

- [ ] Use the current campaign session and admitted native transport for every snapshot request, with a single owner/grant and cumulative charges. Carry the actual cell/work/finalization scope explicitly.
- [ ] Preserve independent solver deadlines, bounded request/output admission, partial output custody and refusal to certify unacknowledged completion. Cancellation must prevent later attempts; in-flight uncertainty remains charged and recorded.
- [ ] Retain and run the meaningful controls from both parents, including owner/lease/grant rollback, concurrent reservations, failed network/output paths, trace admission and scientific certificate checks.
- [ ] Complete current-source library, affected native/CLI checks and review. Preserve every frozen artifact; no data-side selection resolves this API conflict.

These conditions enable software integration only; they do not grant research launch authority or qualify historical performance measurements.

## Next discriminating step

Implement a shared owner-bound context adapter for the snapshot supervisor, using bounded native transport rather than opening another worker journal. Exercise fake/local transport with blocked/failing requests and deadline cancellation before any remote execution. If process isolation remains necessary, use an explicit accounted delegation design; do not reconstruct or clone a balance.

Use existing software test/resource limits. No budget or deadline extension and no new scientific trial is part of this step. A failed adapter remains unqualified and its original source/tests are retained.

## Resolution and history

2026-10-30: Recorded the incompatible ownership models during normal main integration after the unanswered Copilot request. Unresolved; this record does not replace the required implementation or authorize merging an unverified resolution.

2026-10-30 adapter implementation: retained every original legacy-budget source byte and all25 tests as an actual offline receipt utility, with no route to live S3 authority. Current owner-budget source retains all27 original controls plus4 new ownership controls. Real child-process handoff passes. The30 remaining main-module conflicts are resolved with stricter current replay/deadline admission and the live monitor retained. Whole integration compile, source-qualified full suite and deployment remain pending.
