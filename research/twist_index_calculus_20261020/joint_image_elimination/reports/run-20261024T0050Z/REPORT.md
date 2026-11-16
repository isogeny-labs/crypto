# Research EKS capability probe

Job `twist-joint-capability-20261024-v1` has UID
`ee711fcc-27b1-47cd-be6b-5c78b068ccc1`. Its accepted submission starts
2026-10-24 00:50:10 UTC, using the existing scoped job-submitter role,
two requested CPUs, 2 GiB and the 60 s job deadline.

![Submission and scientific-result boundary](../../figures/joint-mechanism.svg)

The pod remains unscheduled because the available nodes do not match its
affinity. The job reaches `DeadlineExceeded`; no benchmark command or
scientific computation executes. An earlier one-CPU request is rejected by
validation before any job is created. Saved submit, pending and terminal
status JSON are in `runs/cloud-capability-v1/`.

The probe is terminal and was not repeated after migration reconciliation.
No role, IAM permission or node-pool policy was expanded. An unscheduled
job supplies no mathematical, runtime or solver evidence. With no
comparable executed measurements, the visual is an explanatory diagram.
