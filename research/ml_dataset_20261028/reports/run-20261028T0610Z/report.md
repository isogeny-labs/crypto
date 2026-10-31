# Dataset pilot: two failed launch attempts

Observation cutoff: 2026-10-28T06:10Z. Protocol
`ML-ECDLP-DATASET-PILOT-20261027`, frozen before the attempts. This report covers
only pilot 001 and pilot 002. It is an execution record, not an independent review
or a mathematical verdict. No completed cloud dataset cell or S3 roundtrip was
established by either attempt.

| Attempt | Submitted UTC | Container outcome | Observed limitation |
|---|---|---|---|
| pilot 001 | 05:58:19 | exit 1 at 05:59:10 | Same-image local reproduction found UID 1000 could not read the 0600 plan; workload stderr was lost when cleanup raced with scheduling |
| pilot 002 | 06:08:41 | exit 1 at 06:09:39 | Retained stderr: S3 HeadObject 403 during publication; worker role only permits legacy bucket |

Both attempts used source `4c94f1dfc9bf2df2290a0645a2de6dada7b3c024`,
binary SHA-256 `333da60b62312d33efb43ba6c4fb7826d8627c5c2fbf5bfcb7c645eed227222b`,
the frozen 720-cell pilot and an explicit 36-cell maximum on shard 0/19. Limits
were 30 seconds per worker cell, 2048 MiB worker address space, 64 MiB worker output,
and a 2 CPU/4 GiB EKS Spot Job with 3,600 seconds TTL and no retries. Diagnostic clocks
are not comparative timing evidence. Job completion would not establish CRIU.

The second image fixed public-input modes to 0644 while retaining the same native
binary and plan. Its command also explicitly replaced the service's legacy
results URI with a unique prefix in `setec-astronomy-artifacts`. The default
dataset store was the approved bucket's `store` prefix. The actual EKS role,
`isolab-job`, lacked access to that bucket. A scoped additive policy was prepared
for user approval; it had not been applied at this report's cutoff.

The 720 method cells represent 72 parameter instances with ten methods each.
The standard complete-panel export would have 144 arm-instance bundles. Neither
failed attempt supplies that complete panel, and no rows are fabricated for
unattempted cells. Known-answer construction, independent scalar recovery,
instruction counters and exact fall degrees are not inferred from a failed Job.

Supporting immutable records are `../../pilot-wave-001-launch.json`,
`../../pilot-wave-001-terminal.json`, `../../pilot-wave-002-launch.json`,
`../../pilot-wave-002-submission.json` and `../../pilot-wave-002-terminal.json`.
The launch source's software verification is recorded by
`../../validation-4c94f1d/.artifacts.json`: 49 focused tests and 5,016 library tests
passed, with 122 ignored and zero failures. Those are local software tests,
not completed cloud trials. The report diagram encodes observed states only.

Next execution requires verified worker access to the scoped approved prefixes,
then a separately recorded bounded launch, fresh S3 download/hash checks,
native restore/replay, leakage-checked training export and actual coverage counts.
