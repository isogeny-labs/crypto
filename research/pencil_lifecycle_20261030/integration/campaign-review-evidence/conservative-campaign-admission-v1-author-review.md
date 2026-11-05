# Conservative campaign admission v1 — private unapplied software proposal

Patch: `conservative-campaign-admission-v1.patch`, SHA256 `ee05a0225db3da704fbfccb3d3b4992da665c8292cf010e8856b9348f85ef3df`. Private before/proposed copies and exact hashes are in `campaign-admission-proposal/PROPOSAL-RECEIPT.json`. This four-file patch adds campaign.rs, registers it in lib.rs, and extends contract.rs/cloud.rs incrementally after the private optional transport proposal. It is not applied, compiled, tested, deployed, or admitted for research execution.

## Rules and existing work checked

Read root AGENTS.md and the latest2026-10-30 session log; private-only assignment takes precedence over shared-source edits, session-log writes, commits and publication by this agent. Inspected retained controller Request/State, Stored resourceVersion CAS, computation provision/restore/fresh lifecycle, resume/rbac.yaml, private owner observation and known-C1 amendment v2. Relevant existing budget design in src/ml_dataset/budget.rs uses durable reservations but its counters are S3 resources rather than worker seconds; this proposal does not import it or redefine those counters.

Kubernetes documents that PUT updates carrying stale resourceVersion are rejected with409, providing the existing controller's lost-update protection: [Kubernetes API concepts](https://kubernetes.io/docs/reference/using-api/api-concepts/). The proposal uses the existing `kubectl replace` transport, preserves the full retrieved object, and does not switch to unconditional apply. A failed/ambiguous CAS response prevents the subsequent worker effect; the next reconciliation must reload.

The retained Role isolab-recovery-controller grants ConfigMap get/list/create/update/delete in isolab-recovery-system with no resourceNames restriction. It supports this source-level reuse without changing IAM/RBAC. This was a local source check, not verification of the deployed role. The research job-submitter role is not used as a substitute for this controller identity.

## Explicit configuration; no fresh balance by default

Request.worker_campaign is optional and skipped when absent. Legacy requests retain prior bytes and behavior; the existing legacy-layout fixture remains applicable. Enabling this path requires a complete Binding containing an explicit Policy, existing ledger ConfigMap UID, computation/fresh allocation ceilings and shutdown reserve.

Policy requires the original authorization SHA201df07743540ac247e2ae0ec79c82c7b284aa16dd07d468924c87cf2a4b7a2b, exactly25200 cumulative admission seconds, exactly1793398120 (2026-10-30 22:08:40 UTC), scientific cutoff at or before1793396320 (21:38:40 UTC), eight distinct compact Request.spec JSON SHA256 values, explicit prior charge, reconciliation receipt SHA and owner enforcement review SHA. These are mandatory config fields, not defaults. No populated production policy or empty ledger was generated. The eight compact spec digests are distinct from expanded plan hashes and ecbench identity-view digests; the reviewed configuration must bind the correct representation.

Every route derives the same campaign ConfigMap name from the authorization SHA, checks the same Policy hash and pins the same existing ConfigMap UID. A missing ledger fails; it is never initialized to zero by a controller. Deleting/recreating a same-name ConfigMap changes its UID and is rejected. A changed policy on the same campaign is rejected. Existing consumption must therefore be reconciled and explicitly represented before usable configuration exists. Receipt hashes are external bindings, not self-authenticating proof of that reconciliation.

## Admission behavior

The pure ledger has a256-KiB parse cap and256-reservation cap, checked arithmetic, strict serde fields, duplicate reservation checks, policy equality and no release/refund API. A reservation is identified by request SHA, attempt and computation/fresh epoch. All new reservations permanently debit their full configured conservative ceiling; failed, interrupted, ambiguous and never-started attempts are deliberately retained. A later request or restore cannot regain those seconds by changing its name, retrying, or providing a terminal status. Reconciliation of the same reservation is idempotent and retains its original cutoff; expired reservations cannot be extended.

Cloud provision reserves before ConfigMap/PVC/Pod creation, including restoration attempts, and rechecks before start/restore/status processing. Fresh benchmark reserves before manifest generation or Job creation. All eight requests debit the same ConfigMap through the existing CAS. No work proceeds after a failed CAS. The returned update must retain object UID and exact serialized ledger bytes and supply a changed nonempty resourceVersion. Retained manifests/observed resources bind the exact reservation digest. Existing computation/fresh creation guards continue to prevent duplicate effects.

The proposal caps each newly generated Pod/Job activeDeadlineSeconds by the reservation's remaining payload allowance after its explicit shutdown reserve. Request deadlines must remain at or before the scientific stop. Actual Kubernetes scheduling/termination and the supervisor's existing deadline paths remain the runtime enforcement layer. Both overlapping computation/fresh allocations consume separate reservations, rather than treating one mathematical payload as one allocation. This proposal does not add concurrency permission or change the one-scientific-payload rule.

## Exact claim boundary and remaining runtime work

This is a conservative admission balance, not measured allocated-worker time, CPU time, EC2 billing time, solver cost, or a cost ratio. Never copy charged_seconds into total_active_worker_ns. The configured owner-enforcement receipt is an external review binding; its presence does not prove a physical hard stop. Kubernetes start-time semantics, scheduler delay, API latency, termination grace, orphaned Pod/Job tails, replay/collection, and actual allocation-release observations still require the separately identified runtime enforcement/closure review. In particular, the proposal does not establish a finite hard bound for physical EC2 teardown, or add a new fresh-Job cleanup permission. Unknown actual consumption and missing release evidence remain unknown. If the intended allocation unit cannot be bounded by the reviewed runtime, this software ledger alone must not be used to admit execution.

There is deliberately no automatic refund or observed-duration settlement in v1. That makes accounting conservative and can exhaust the campaign before all targets complete; it must not be hidden by a new grant or reset. Future settlement would need independently bound complete allocation evidence and separate review. No authorization is inferred from a fixture's initial charge or its dummy receipt hashes.

## Software verification scope

Four fixture sources were added: eight route requests share one balance and serialized restart is idempotent; ambiguous restore/fresh attempts remain separate charges and exhausted admission leaves state unchanged; policy/deadline/duplicate-history rejection; and CAS response UID/revision/exact-byte binding. They use only synthetic values and no cloud, solver, target generation or experiment. Existing absent-option serialization tests should also run on integration.

Only rustfmt parsing/formatting ran on private files. No fixture or build ran. Independent review must examine the exact patch and later named software fixtures on the final integration tree. Production policy/ledger reconciliation, UID binding, actual deployed-role verification and runtime stop review remain outstanding inputs; no cloud action or new approval request was made.
