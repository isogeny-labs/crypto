# Trace-homomorphism matched experiment checkpoint access

- **Status:** open
- **Kind:** access/tool limit; separate wide-encoder implementation dependency
- **Affected work:** [trace integration](../research/ecc2k_trace_homomorphism_20261109/README.md), [pending comparison protocol](../research/ecc2k_trace_homomorphism_20261109/PROTOCOL.md)
- **Next action owner:** session environment administrator for access; research session for execution
- **Last updated:** 2026-11-09

## Problem

The requested matched trace-on/off solver measurement cannot start because
this session cannot push the required task-specific S3 checkpoint.
Local correctness development remains feasible. Degree-83 and ECC2K-130
full-DLP evaluation are not replaced by smaller correctness tests.

## Evidence and uncertainty

- Observed: `scripts/checkpoint.sh gate` rejected launch with no pushed task
  checkpoint. No CHECKPOINT_UPLOAD_TOKEN binding, token file or AWS identity
  is configured in the attached environment. Local AWS CLI is absent.
- Observed: GitHub CLI can read the source and EKS workflow; querying Actions
  variables returned HTTP 403. Visibility of that workflow is not confirmation
  of its deployed access or proof that the checkpoint gate has been met.
- Observed: tracked `.resume/RESUME.md` belongs to an unrelated workstream;
  it must not be uploaded as the trace task's checkpoint.
- Previous implementation dependency resolved for this panel: generic
  FieldStructure remains limited to degrees <=64, but the prepared native
  instrument uses the exact wide AND/XOR S3 backend and actual n83/131
  public fixtures. Its correctness controls and locked build are committed.
- Missing measurements do not establish an algorithmic impossibility or any
  timing conclusion. The configured trace-zero subgroup-base redundancy is a
  separate mathematical/source analysis [pcc:unverified].

## What would unblock it

- [ ] Configure the normal checkpoint upload transport through environment
  settings, without putting credentials into chat, Git or artifacts.
- [ ] Prepare this task's resume note and successfully push/gate its code,
  inputs, protocol and source identity through the normal checkpoint tool.
- [ ] Freeze an executable matched harness, exact field/subgroup/base identities,
  EKS source/image identity, finite dollar ceiling, CPU/RAM, deadline, retry
  allowance and admission controls before launch.
- [ ] For the degree-83/131 requirement, use a demonstrated exact wide-field
  solver path, then retain full-DLP costs and independent result replay.

## Prepared bounded local alternative (pending explicit exception)

The new [concrete protocol](../research/trace_zero_experiments_20261109/PROTOCOL.md)
defines33 cases, four arms and two repetitions (264 primary rows),16 separate
engineering pilot calls, one CPU,2GiB,600 seconds total, no retries and zero
cloud spend. Input, source and analysis are committed at3ae8b21241256517a9c867b4e78fb25e02ff502e.
The existing release executable is pinned by SHA256
15c4d1b193999153c6ddc6ec94959167c8f4d016be197b55b26d6993001b1126.
Later independent solver additions must not replace this instrument or be
silently added to its arms.

An explicit user exception to EKS and S3 for this exact local run has been
requested and is pending. No pilot or primary panel has run. The alternative
does not establish natural-target full ECDLP costs; that original evaluation
remains open. The latest actual checkpoint failure and workflow-read receipt
are retained in the prepared study's checks directory.

A successful checkpoint discharges the access dependency only; it is not a
measurement or a correctness certificate.

## Next discriminating step

With transport configured, rerun the ordinary checkpoint gate on this task's
checkpoint and qualify the source-pinned EKS command. On success, launch only
the frozen paired experiment under its admitted resources. On another access
failure, preserve the receipt and diagnose the named permission; do not change
pools or silently run a local timing substitute. Recheck when the environment
credentials or the wide-encoder adapter change.

## Resolution and history

2026-11-09: open. Native integration and local correctness work continue
independently. No performance experiment has been launched.
