# Prepared-table producer registration and canonical ledger storage

- **Status:** open
- **Kind:** conflicting publication requirements
- **Affected work:** producer artifact registration for prepared point tables
- **Next action owner:** canonical PCC/artifact-maintenance lane
- **Last updated:** 2026-11-15

## Problem

The mandatory append-only PCC ledger is already over the artifact policy's
64 KiB JSONL threshold. Adding one producer artifact statement therefore makes
the artifact gate select the entire existing ledger for storage migration.
The PCC workflow reads that canonical path directly and has no hydration step;
automatic migration would lose the required checked Git prefix on checkout.
These are source/preflight observations, not signed verdicts. [pcc:unverified]

## Evidence and uncertainty

The original ledger remains unchanged on the implementation PR. Full proposal
commit `04f4d6734d588015617c1e9957afdc4dc2e9061d`, on branch
`codex/reusable-point-tables-20261115`, preserves statement
`PCC-20261115-3bef1a`, its exact new hash-linked entry and complete base prefix.
Its actual artifact preflight selected `pcc/ledger.jsonl` at 696,074 bytes.
No registered/verified standing is claimed in the implementation PR. The source
and producer test artifacts are fully retained. [pcc:unverified]

## What would unblock it

Review a canonical control-plane storage contract that preserves both the
append-only prefix and actual artifact checks, or an explicitly approved policy
exception with the migration workflow addressed as well as the gate. Do not
silently move, rewrite or rechain the ledger, apply an author-approved waiver,
or represent the proposal as a signed execution/performance result.

## Next discriminating step

Validate the reviewed contract against the exact proposal/base ledger bytes and
all PCC/artifact workflow readers. The native source PR can be reviewed with
its existing producer-only qualification; canonical registration remains a
separate open obligation. No scientific execution or new cloud job is needed.

## Resolution and history

Unresolved. Original implementation, proposal commit and both ledger bodies
remain in Git. No canonical ledger change is included in the code PR.
