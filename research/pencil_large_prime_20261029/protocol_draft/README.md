# Pencil paired mathematical cohort protocol draft

`protocol.json` is a complete cross-repository `experiment.protocol/v1` draft.
It is not frozen and does not authorize a job. No cohort data were taken while
writing it. The actual prepared candidate arm is named `pencil`; use
`pencil rho rho_AA rho` with the paired exporter.

The design includes all 1,260 fixed public synthetic workloads, five predetermined
measured algorithm streams and one warmup per arm, three arms, and separate cold
and online calibration. All five rho_AA/rho ratios must fall in inclusive
[0.90,1.10] independently for each metric and workload. That tolerance is not a
confidence interval. The measured streams differ in algorithm seed; they are not
five identical-seed timing repeats. The primary clock encloses the full method
call. It excludes harness preparation, serialization and parent audit; those
boundaries are explicit. The five online phases are secondary. Partial event
counts cannot substantiate complete priced-cost comparisons.

The p739 60-workload gate precedes p186379 target0 and independent replay. The
upper gate retains 25 million coefficients, 8 GiB and 3,600 seconds. A capped or
invalid gate prevents extension. Complete plans contain 22,680 base executions;
gate rows count once and independent replay is separately identified.

## Prerequisites before freeze

- Pin final source commit, lockfile/toolchain/build flags, native binary and image.
- Emit and freeze exact native plans for the nine prepared specs; verify all
  workload IDs and paired stream seeds against the original archive.
- Resolve the EKS admission/job identity and reconcile retained work using the
  authorized connection; bind cumulative resource/time budget ; the first capped gate itself is the runtime preflight. The draft does not grant research-namespace permissions.
- Demonstrate L2 coverage of the common cold-call boundary, physical core/SMT
  placement and nuisance measurements, not merely successful pod execution.
- Bind the approved `isolab.eks.resumable/v1` CRIU+EBS runtime image and request: periodic 300-second checkpoint interval, at most one restore, and original absolute deadline. Confirm the draft 120-second interruption grace budget against the controller. Computation epochs are always timing-invalid; a separate fresh benchmark epoch must pass L2, replay, A/A and artifact checks. Runtime build remains pending.
- Complete and pin native experiment.export/v1 generation and independent review.
  The existing paired exporter explicitly leaves qualified ratios null; it does
  not verify sealed-record authenticity or certify L2/restore/protocol adherence.
- Run the canonical repository semantic checker and current Draft202012 validator,
  then freeze and commit before affected data. Structural validity is insufficient.

`schema-validation.json` records the bounded structural check actually performed:
the host has only jsonschema Draft7Validator. The schema uses compatible validation
keywords; its relative top-level `$id` was removed in memory to work around that
old resolver. The source schema file was untouched. Canonical validation remains
open. `sample_size.power.passes=false` intentionally declines a statistical power
claim; this descriptive design uses neither p-values nor confidence intervals.

Historical raw observations informed this design; no out-of-sample mathematical
or security claim is registered. Completing this clock study does not discharge
the overarching task's unresolved full counted-cost accounting requirement.

The authoritative repository validator subsequently passed with current jsonschema via `uv run --no-project --with jsonschema>=4 python .../tools/experiment_protocol.py validate protocol.json`. `execution` describes the fresh timing epoch (no checkpoints); `extensions.computation_epoch` describes mandatory CRIU+EBS computation (always timing-invalid). All computation, gate, control, checkpoint, restore and replay costs remain in the export; no first-gate work is erased.
