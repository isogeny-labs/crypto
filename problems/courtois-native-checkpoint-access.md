# Native Courtois measurement admission is incomplete

- **Status:** open
- **Kind:** configured access and implementation/admission gap
- **Affected work:** [native Rust study](../research/courtois_native_20261115/README.md)
- **Next action owner:** environment operator for checkpoint access; implementation session for native runner/pricing
- **Last updated:** 2026-11-15

## Problem

The native implementation cannot yet supply an admitted cold performance
comparison: the required S3 checkpoint has no configured transport, and the
existing ecbench image workflow does not build this standalone executable.

## Evidence and uncertainty

The actual gate output is retained at
`research/courtois_native_20261115/evidence/checkpoint-gate.txt`. It reports no
AWS profile `ecc-challenges-new`, no `CHECKPOINT_UPLOAD_TOKEN`, and no configured
token file. This is an access failure, not a rejected authorization or a
mathematical conclusion. [pcc:unverified]

The same underlying checkpoint access condition is recorded for other work in
[the quotient-calibration record](quotient-target-calibration-checkpoint-gate.md).
That record's separate historical approvals do not authorize a credential
workaround for this task. No raw keys were requested or used.

The maintained Iso Lab submission skill and `.github/workflows/isolab-experiment.yml`
were read. A source SHA does not by itself put this executable in the image;
the build bundle currently names other binaries. No remote job, paid resource,
or performance measurement was launched. [pcc:unverified]

The native library tests and cross-repository replay are development producer
checks. The unpublished local claim is retained in the study's evidence
directory; canonical PCC ledger integration and independent signed standing
remain open. They do not satisfy measurement admission. [pcc:unverified]

## What would unblock it

- Attach the existing supported checkpoint upload token or AWS connection to
  the runtime, then retain a successful official `scripts/checkpoint.sh gate` receipt.
- Add the existing-native-arithmetic splitter as an ecbench oracle, or qualify
  a tested digest-pinned custom command image through the supported runner.
- Freeze the same-point cold protocol, resource envelope, finite funding,
  complete field/bit pricing, strong rho reference and independent checks.

A checkpoint resolves only the access item. It does not establish the image,
cost accounting, independent standing, comparative performance or a full-ECDLP theorem.

## Next discriminating action and limits

Re-run the unchanged checkpoint gate only when its configured access changes.
On success, record the receipt and continue the native runner/pricing work;
on another transport failure preserve the output and keep admission open.
The proposed envelope is in the study protocol; nothing has been spent and
no automatic retry is authorized. Do not replace the requested measurement
with local timings or the runner's default integration fixture.

## Resolution

Unresolved. Code publication and producer unit checks do not close this record.
