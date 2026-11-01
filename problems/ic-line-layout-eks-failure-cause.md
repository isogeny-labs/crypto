# Prime-IC line-layout EKS payload failure cause

- **Status:** `investigating`
- **Kind:** unknown cause; access/tool limit
- **Affected work:** [`research/ic_line_layout_tournament_20261031/`](../research/ic_line_layout_tournament_20261031/), workflow runs [41260778029](https://github.com/isogeny-labs/crypto/actions/runs/41260778029) and [41260778856](https://github.com/isogeny-labs/crypto/actions/runs/41260778856)
- **Next action owner:** fixed-target GitHub diagnosis workflow
- **Last updated:** 2026-11-01

## Problem

The frozen fp20 generic-decomposition line-layout tournament cannot produce its
required end-to-end performance decision because both preregistered EKS Jobs
terminated failed before the public launcher receipt authenticated a successful
Pod UID and result prefix, while the causal artifacts remain private.

The affected source is merge commit
`b4d751706eb2244314ebc9b84ba9545485ba4ac7`; the native executable SHA-256 is
`0784fe47212d64dd2ee81510d3c28d2f604782086500135a3114c61738f854b7`.
The original acceptance rule still requires both independent sessions to pass
L2, replay, operation parity, A/A and the frozen confidence bounds.

## Evidence and uncertainty

- **Observed:** both workflows passed contracts and build, then returned remote-result exit 23 and wrapper exit 26. Their public receipts retain distinct Job UIDs and request hashes but no successful Pod UID or results URI. See the [terminal observation](../research/ic_line_layout_tournament_20261031/TERMINAL-OBSERVATION-20261101.md).
- **Derived or proved:** the exact retained binary completes all 360 executions and 320 replay-eligible checks in an excluded L0 diagnostic; this establishes bounded deterministic correctness, not EKS behavior or performance.
- **Suspected explanation:** unknown. Payload execution, owner instrumentation, native audit, L2, A/A and infrastructure remain live alternatives until the private artifacts are classified.
- **Missing capability:** an authenticated, privacy-preserving read of the two fixed launcher and payload prefixes.
- **What this evidence does not establish:** neither a candidate regression nor a speedup, and no universal EKS or algorithmic obstruction.

## What would unblock it

- [ ] Validate each retained launcher request/result against its fixed Job, launcher, request, source and binary identities.
- [ ] Identify the exact payload exit classification from hash-verified private artifacts without publishing raw records or stderr.
- [ ] If the cause is implementation-scoped, reproduce and repair it without weakening any scientific gate; otherwise retain the infrastructure diagnosis.

These conditions classify the two failures. They do not make the frozen evidence
version successful or authorize a retry. Further scientific trials require a new
frozen evidence version and preregistration.

## Next discriminating step

- **Action or proof obligation:** merge and dispatch `.github/workflows/isolab-failure-diagnosis.yml` once on `main`.
- **Inputs and controls:** the two fixed workflow/Job/launcher/request tuples; exact source and native hashes; per-file `SHA256SUMS`; wrong-request and private-line nonpublication fixture controls.
- **Execution limits and stop condition:** read-only S3 access, no Job submission or retry, one terminal diagnosis artifact per retained run.
- **Outcomes and next actions:** an implementation gate classification permits a scoped repair and new preregistration; an infrastructure classification is retained without changing the candidate; missing or inconsistent receipts keep the cause unresolved.
- **Recheck trigger:** terminal diagnosis workflow artifacts or an authorized operator-provided equivalent.

## Resolution and history

2026-11-01: opened after both preregistered trials terminated failed. A fixed,
read-only diagnosis path and privacy fixture were implemented but have not yet
run under the reviewed main-branch OIDC boundary. Unresolved.
