# H113 run-admission receipt

Status: infrastructure-blocked; no candidate data generated.

Recorded at `2026-11-17T21:41:41Z` from crypto source commit
`b4f04f225fc531dc1042b6bbde4adfe671b79f98`. The source later changed only
in proof/documentation and compact rendering work; any admitted experiment
must bind and record its own final pushed commit.

## Requirement-to-evidence table

| Requested or frozen requirement | Status | Evidence or exact gap |
| --- | --- | --- |
| Test the frozen H113 producer on all 28 rows and 9,096 targets | blocked | Mandatory checkpoint upload failed before execution. |
| Preserve candidate, null-control, decoder and operation-count output | implemented but unverified | Producer serializes these fields; no raw TSV or result JSON exists. |
| Verify independently, including tamper rejection | implemented but unverified | Paired cryptanalysis verifier compiles and self-tests; it has no measured artifact to open. |
| Prove the structural and conditional decoder obligations | verified complete at the conditional scope | Pinned Lake build passes; proof gate reports 19 declarations, zero failing and zero trusted-native. |
| Establish an exact public digit oracle and subexponential prime-field ECDLP algorithm | not established | This requires an exact predicate on the frozen out-of-sample cohort and an applicability argument beyond tiny curves. |

## Admission command and observed output

The mandatory command was:

```text
scripts/checkpoint.sh gate
```

It terminated before experiment launch with:

```text
checkpoint: no way to upload: no AWS profile 'ecc-challenges-new', no CHECKPOINT_UPLOAD_TOKEN, no /home/agent/.config/checkpoint/upload-token
checkpoint: cannot upload, so nothing was checkpointed
```

The environment also had no `aws` executable, and `.resume/.last-push` was
absent. Two earlier attempts were externally interrupted; neither created a
receipt. The stable failure above is the operative admission result.

## Execution accounting

- Candidate quick runs: `0`.
- Candidate full runs: `0`.
- Measured targets: `0`.
- EKS jobs or pods submitted: `0`.
- Scientific result: none; infrastructure failure is neither acceptance nor
  falsification of the frozen predicate family.

Development-only checks were one-point arithmetic self-tests, compilation,
the Lean build, and the proof audit. They did not invoke producer `--quick` or
full modes and do not discharge the requested experiment.

## Resume condition

Resume the exact frozen run only after `scripts/checkpoint.sh gate` uploads a
current private checkpoint and writes its receipt. Then verify caller identity,
capacity, budget, pushed source revision and image digest before submitting the
single allowed deterministic run. Do not substitute a local run for that
source-bound experiment.
