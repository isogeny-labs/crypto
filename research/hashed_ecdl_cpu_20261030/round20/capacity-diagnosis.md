# Post-report capacity diagnosis, 2026-11-01

The control-plane audit records identify the blocking limit. At
03:50:19, 03:50:59 and 03:51:09 UTC, cpu-spot had limits of 16 vCPUs,
128 GiB and **one node**. Its status reported one node and eight vCPUs already
provisioned. These timestamped observations are retained in
nodepool-limit-observations.json with CloudWatch and audit identities.

The Karpenter FailedScheduling event for Pod UID
991e9449-9982-46f8-a9ce-2db9bdfa58bc states that cpu-spot node limits were
exhausted; it also lists exhausted GitHub-runner pools and the untolerated
L2 measurement taint. The exact Event object and log identity are retained in
karpenter-failed-scheduling-event.json. The scheduler independently reported
anti-affinity against the occupied worker and selector mismatch on the other
two nodes. Thus spare nominal CPU quota did not make another eligible worker
available: the node-count ceiling and anti-affinity were binding.

These are selected records from bounded, filtered CloudWatch reads of
/aws/eks/isolab/cluster for 03:49:30 through 03:56:00 UTC, not a complete
account history. The MCP overview's advertised 32-vCPU per-pool limit was not
the authoritative live limit during this attempt. The records above support
the observed-time limit only, not its permanence or who changed it.

The initial report published at 4f90e8f3936a correctly limits its conclusion
to the original scheduler/terminal receipts. This later addendum supplies the
additional autoscaler and NodePool evidence without rewriting that report.

No pool caps, taints, anti-affinity rules or other jobs were changed. Normal
autoscaling authorization did not authorize raising the one-node cap. Before
another admitted panel, wait for the occupied worker to become eligible and
recheck both scheduler eligibility and current limits; configured CPU limits
alone are insufficient. Round20 is terminal and will not be retried.
