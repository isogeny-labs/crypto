#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false)
= Dataset pilot: observed launch failures
Protocol ML-ECDLP-DATASET-PILOT-20261027. Cutoff: 28 October 2026, 06:10 UTC.
This execution report covers pilot001 and pilot002. It is not an independent
review. Neither attempt established a completed cloud dataset cell or S3 roundtrip.

#image("attempt-flow.svg", width: 100%)

== What ran
Both EKS Spot attempts used the tested 4c94f1dfc9bf source binary, the frozen
720-cell plan, and at most 36 cells on shard 0 of 19. Budgets were 30 seconds per cell,
2048 MiB worker address space, 64 MiB worker output, and 2 CPU/4 GiB with 3600 seconds Job
TTL and zero retries. These are limits, not measured throughput. The fixed
executable SHA-256 is
`333da60b62312d33efb43ba6c4fb7826d8627c5c2fbf5bfcb7c645eed227222b`.

== Failures and retained evidence
Pilot001 exited 1 at 05:59:10. Cleanup raced with scheduling, so terminal status
was retained but workload stderr was lost. The same digest-pinned image,
tested locally without permitting a scientific worker to start, could not read
the owner-only plan file as UID 1000. The replacement image set input modes to 0644.

Pilot002 exited 1 at 06:09:39. Its retained stderr reports S3 HeadObject 403 during
publication. Read-only IAM inspection found the EKS role permits only the legacy
bucket. A policy scoped to this campaign's approved-bucket prefixes was prepared;
it had not been applied at this cutoff. No scalar recovery, instruction count,
PDP completion or exact fall degree is inferred from either failed Job.

== Provenance and next execution
The canonical records are the pilot-wave-001/002 launch and terminal JSON files
under `research/ml_dataset_20261028/`, plus the pilot002 submission receipt. Their
image digests, Job and Pod identities, full argv and explicit receipt routing
are retained. The 49 focused and 5,016 full-library tests for the launch source
passed with 122 ignored; the six logs/hash records are in the S3-backed
`validation-4c94f1d/.artifacts.json`. Those local tests are not cloud trial data.

The complete research export requires 144 arm-instance bundles from 720 method
cells. There is no complete-panel claim and no invented unattempted row. The
next bounded launch must demonstrate worker S3 access, fresh object readback,
native restore/replay and a leakage-checked training view. Ordinary EKS Job
success would still not establish CRIU recovery or comparative timing.
