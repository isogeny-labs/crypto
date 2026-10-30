# Native quotient target calibration cannot launch before its S3 checkpoint

- **Status:** `open`
- **Kind:** configured access; upload authorization is recorded
- **Affected work:** [native target continuation](../research/quotient_calibration_20261025/target-native/README.md)
- **Next action owner:** user for the environment connection; then the repository operator
- **Last updated:** 2026-10-30

## Requirement and observed blocker

The parent protocol's fixed curve, kernel, frame and all 32 coordinate
challenges remain unchanged. Native target arithmetic and the three-phase
calibration/evaluation/reference path are implemented. Fixed component tests
pass, including all three historical target fixtures. The certified-anchor
frame and new challenge targets have not been computed.

AGENTS.md requires a successful remote resume checkpoint before an
experiment starts. The official `scripts/checkpoint.sh gate` exits 1:

> no way to upload: no AWS profile 'ecc-challenges-new', no
> CHECKPOINT_UPLOAD_TOKEN, no /home/agent/.config/checkpoint/upload-token

A proposed temporary raw-key process was then rejected before execution by
automatic approval review. Its stated reason was that the transcript did
not specifically authorize that credential use or the new checkpoint
payload/destination; it instructed against solicitation/reuse or an indirect
bypass. The action was not retried, the helper was removed, and the gate
remains unsatisfied.

The earlier explicit approval covered PR #2569 auto-merge and one exact
67,540-byte raw-log archive upload. That completed publication is preserved;
this record does not reopen it or ask to approve it again.

## Concrete prepared action

Upload standard resume snapshots for branch
`codex/quotient-target-calibration-20261026` through the repository's normal
checkpoint tool to:

`s3://setec-astronomy-artifacts/checkpoints/crypto/codex__quotient-target-calibration-20261026/`

The snapshots contain this branch's code, protocol/specification, resume
note and same-branch run state, plus the standard LATEST pointer. The local
review-only initial preview is 43,171 bytes with SHA-256
`ad0c0f06b1ab8223f5943abba62c73dc5d99e3656cb3da16272c10d80dd4f96e`.
It contains a branch bundle at `caec72812`, the protocol and the resume note;
no credentials, source/image lookup table or other-repository state.
The preview is not a successful S3 checkpoint.

On 2026-10-30 the user explicitly approved the ongoing same-branch uploads
described above. No further approval is needed for that scope. The resumed
environment (configuration revision34) reports current observations with no
secret, runtime-variable or outbound-identity bindings, and the normal gate
again exits1 with the same missing-transport error. The authorization part
is complete; configured access remains open. No raw-key helper was retried.

Current repository instructions route the pending target experiment to EKS
Spot. The draft record retains the same inputs, one-thread/2-GiB/3600-second
bounds and acceptance threshold. Its matching target image and supported
custom-command route still need qualification; the standard GitHub workflow
builds ecbench and does not execute this standalone target binary.

## Exit criteria and next discriminating step

1. Configure a supported AWS connection or checkpoint upload token through
   environment settings; upload authorization is already recorded.
2. Use the normal checkpoint gate and retain its actual successful receipt.
3. Record preflight diagnostics, instrument hashes and the analysis hash;
   freeze and commit the execution record before affected target data.
4. Run the declared nine-point calibration, freeze/checkpoint the matrix,
   and test every fixed challenge without refitting or replacement.

The [preparation report](../research/quotient_calibration_20261025/reports/preparation-20261026T1824Z/REPORT.md)
links the implemented work, fixed checks and proof record. A successful
checkpoint resolves this access blocker; it does not prove a calibration,
inverse transfer, global identity, speed improvement or vulnerability.
