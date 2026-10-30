# Native Koblitz comparison lacks S3 checkpoint admission

- **Status:** open
- **Kind:** access/tool limit
- **Affected work:** [native halftrace integration](../research/koblitz_halftrace_20261030/PROTOCOL.md)
- **Next action owner:** session operator / existing authenticated lab environment
- **Last updated:** 2026-10-30

## Problem

The requested native baseline/candidate measurement cannot launch from this
session because its mandatory pre-experiment S3 checkpoint has no uploader.
This is an access gap, not a mathematical or measured performance obstruction.

## Evidence and uncertainty

- **Observed:** `scripts/checkpoint.sh push` reports no AWS profile
  `ecc-challenges-new`, no `CHECKPOINT_UPLOAD_TOKEN`, and no configured upload-token
  file; nothing was checkpointed. `scripts/checkpoint.sh gate` was also executed
  and exits 1 with the same upload failure. The cloud runtime reports no configured secret bindings or outbound
  identities. GitHub read/write access is available and Rust 1.99.1 is installed.
- GitHub Actions has an OIDC experiment workflow, but it does not perform this
  source-resume checkpoint. Secret-name metadata lookup returned HTTP 403. That
  response does not establish whether an Actions upload secret exists; none was
  obtained or copied into this session. The checkpoint gate was not bypassed.
- **Derived:** AGENTS.md's S3 checkpoint rule prohibits scientific launch before
  this gate succeeds. An EKS image or Git commit alone does not satisfy that gate.
- **Suspected explanation:** this fresh workspace lacks the existing lab connection.
- **Missing capability:** an existing authorized checkpoint uploader or an existing
  lab environment that can perform and verify the checkpoint and EKS submission.
- **Limits:** no EKS native timing, L2 result, A/A admission or runtime speedup is
  established. Local Rust build/test elapsed times are not experiment evidence.

## What would unblock it

- [ ] Attach the existing approved lab identity or checkpoint uploader through
  environment configuration; do not put credentials in Git, issues or chat.
- [ ] Run `scripts/checkpoint.sh push` and `scripts/checkpoint.sh gate` successfully.
- [ ] Build the reviewed source into a pinned image, submit the frozen native
  spec on EKS Spot, retain failures, and verify L2, scalar replay and S3 receipts.

These conditions permit the requested comparison; they do not predetermine a
gain. Independent replay and the existing validation gate still govern claims.

## Next discriminating action

In the authenticated environment, use the resume record and frozen spec in
`research/koblitz_halftrace_20261030/`. Submit one 900-second, four-CPU, 4-GiB
run after checkpoint admission. Preserve the original workflow/job identity.
If checkpoint access remains unavailable, retain the implemented integration
and tests with measurement explicitly pending; do not substitute local timings.

## Resolution

Unresolved. Committing this record or merging its implementation PR does not
complete the requested measurement.
