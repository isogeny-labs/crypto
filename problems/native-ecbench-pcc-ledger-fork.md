# Concurrent PCC ledger branches block native ECBench integration

- **Status:** open
- **Kind:** assistance restriction / missing approved reconciliation workflow
- **Affected work:** [crypto PR2984](https://github.com/isogeny-labs/crypto/pull/2984), [native study](../research/hashed_ecdl_ecbench_20261030/README.md)
- **Next action owner:** repository owner for the canonical-ledger exception; author for validation
- **Last updated:** 2026-11-04

## Problem

The native branch cannot merge current main while preserving both divergent
append-only chains in the single canonical ledger under the existing no-rewrite
rule. Copilot is requested; the canonical ledger and conflicted primary journal
remain untouched. This is a repository-integration obstruction, separate from
the algorithm's general audit and the scientific execution prerequisites.

## Evidence and uncertainty

The [custody bundle and concrete proposal](../research/hashed_ecdl_ecbench_20261030/resume-20261104/ledger-fork/PROPOSAL.md)
retain complete parent ledgers/journals, source/hash identities and an unexecuted
canonical draft. The read-only merge preview identifies both conflicts; the
current-main prefix command reports a mismatch. The five branch-only claim
events concern unsigned producer statements. These are unregistered operational
observations, without an independent signed verdict. [pcc:unverified]

No documented PCC fork-import command was found. The official append primitive
exists; no key, verifier or check change is needed by the proposed procedure.
Permission to replace the PR canonical path is missing. This says nothing about
algorithm runtime, challenge scalars or general correctness.

## What would unblock it

- [ ] Owner approves the archived-fork/canonical-replay exception, or supplies an
  existing approved reconciliation mechanism that preserves both histories.
- [ ] Every original claim/snapshot remains byte-identical; the resulting ledger
  has current main as a prefix and passes the unmodified chain checker.
- [ ] Copilot coordination, exact combined-source qualification and ordinary
  current-head protection/review/CI gates finish.

These conditions address integration. Budget headroom, fresh S3 readback and
native-image admission still precede the bounded scientific run.

## Next discriminating step

Obtain the narrow exception for the inspectable draft. If approved, coordinate
with Copilot and use the canonical primitive; if denied or unresolved, leave
protected files and auto-merge untouched. Refresh immutable snapshots if refs
advance. No scientific job or cloud build is part of this step.

## Resolution and history

Unresolved. The proposal is not an executed migration or research result.
