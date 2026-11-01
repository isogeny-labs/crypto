# Prime-IC line-layout EKS payload failure cause

- **Status:** `blocked`
- **Kind:** L2 isolation gate; payload-artifact access limit
- **Affected work:** [`research/ic_line_layout_tournament_20261031/`](../research/ic_line_layout_tournament_20261031/), workflow runs [41260778029](https://github.com/isogeny-labs/crypto/actions/runs/41260778029) and [41260778856](https://github.com/isogeny-labs/crypto/actions/runs/41260778856)
- **Next action owner:** lab infrastructure owner or authorized artifact operator
- **Last updated:** 2026-11-01

## Problem

The frozen fp20 generic-decomposition line-layout tournament cannot produce its
required end-to-end performance decision because both preregistered EKS Jobs
terminated at the L2 isolation gate. The retained Kubernetes status proves the
gate classification and exit code, but the existing workflow role cannot read
the payload artifacts needed to identify the exact failed L2 condition.

The affected source is merge commit
`b4d751706eb2244314ebc9b84ba9545485ba4ac7`; the native executable SHA-256 is
`0784fe47212d64dd2ee81510d3c28d2f604782086500135a3114c61738f854b7`.
The original acceptance rule still requires both independent sessions to pass
L2, replay, operation parity, A/A and the frozen confidence bounds.

## Evidence and uncertainty

- **Observed:** both workflows passed contracts and build, then returned remote-result exit 23 and wrapper exit 26. Their public receipts retain distinct Job UIDs and request hashes but no successful Pod UID or results URI. See the [terminal observation](../research/ic_line_layout_tournament_20261031/TERMINAL-OBSERVATION-20261101.md).
- **Observed:** read-only diagnosis run [41266993333](https://github.com/isogeny-labs/crypto/actions/runs/41266993333) authenticated with the main-branch OIDC role and read the four fixed launcher receipts for both trials. Payload-object `HeadObject` requests returned 403. The reader then failed because it incorrectly treated that copy failure as success; no diagnosis artifact was uploaded.
- **Observed:** corrected read-only diagnosis run [41267915424](https://github.com/isogeny-labs/crypto/actions/runs/41267915424) completed both fixed matrix jobs and uploaded one allowlisted diagnosis per trial. Each retained prefix contains 19 objects and one failed Pod. The identity-checked Kubernetes status reports exit 42, classified `l2_isolation_gate`, for Pod UIDs `458712fd-1251-4d62-b943-b35f44a51e44` and `bfe971ea-45d0-4f50-a851-8afbd61f61f6` respectively.
- **Derived or proved:** the exact retained binary completes all 360 executions and 320 replay-eligible checks in an excluded L0 diagnostic; this establishes bounded deterministic correctness, not EKS behavior or performance.
- **Derived or proved:** both frozen EKS failures occurred at the L2 isolation gate. This excludes later owner replay, A/A and timed comparison stages for these runs; it does not identify which L2 predicate failed.
- **Suspected explanation:** an isolation-host or isolation-evidence condition failed. The exact condition remains unknown because `l2-gate.json` and its retained manifest could not be read.
- **Missing capability:** the existing role cannot read payload objects. Both diagnoses therefore record `artifact_access: payload_read_unavailable`, `manifest_verified: false`, and null L2 detail. Status evidence is not a substitute for a hash-verified payload manifest.
- **What this evidence does not establish:** neither a candidate regression nor a speedup, and no universal EKS or algorithmic obstruction.

## What would unblock it

- [x] Validate each retained launcher request/result against its fixed Job, launcher, request, source and binary identities.
- [x] Identify the payload exit classification from identity-checked Kubernetes status; use hash-verified payload metadata only if the role can read it.
- [x] Retain the infrastructure diagnosis without changing the candidate or weakening the L2 gate.
- [ ] Obtain narrowly scoped, authorized access to each retained `SHA256SUMS` and `l2-gate.json`, or qualify the L2 infrastructure under a new frozen evidence version, before attributing the exact gate condition.

These conditions classify the two failures. They do not make the frozen evidence
version successful or authorize a retry. Further scientific trials require a new
frozen evidence version and preregistration.

## Next discriminating step

- **Action or proof obligation:** identify the exact failed L2 predicate from hash-verified retained evidence, or qualify the isolation environment under a new frozen evidence version.
- **Inputs and controls:** the two fixed workflow/Job/launcher/request tuples; exact source and native hashes; per-file `SHA256SUMS`; wrong-request and private-line nonpublication fixture controls.
- **Execution limits and stop condition:** no retry of either consumed trial; no broad payload access; no raw stderr or timing publication. A future trial requires a new frozen evidence version and preregistration.
- **Outcomes and next actions:** a hash-verified L2 detail may justify an infrastructure repair. Status-only exit 42 does not justify changing the line-layout candidate. After infrastructure qualification, any scientific comparison starts as a new evidence version with unchanged gates.
- **Recheck trigger:** an authorized operator provides the exact manifest-bound L2 files, or a separately reviewed infrastructure qualification is available.

## Resolution and history

2026-11-01: opened after both preregistered trials terminated failed. The first
fixed, read-only diagnosis ran under the reviewed main-branch OIDC boundary as
run 41266993333. Both jobs exposed payload-read 403 responses and the same
copy-status handling defect; no artifact was published. The correction records
the access state and falls back only to identity-checked Kubernetes status.
Corrected run 41267915424 then classified both retained Pods as
`l2_isolation_gate` with exit 42 and published no raw measurements or stderr.
The exact L2 predicate remains blocked on payload-artifact access; no candidate
repair or scientific retry was authorized.
