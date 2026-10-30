# N83 cross-family Tier 1 west-region allocation revision

The two-instance same-point IC/rho schedule, strata, six-hour runtime cap,
$0.27 per-instance-hour Spot bid, 50 GB encrypted gp3 root volumes, $0.05 S3
allowance, and $4.20 total projected allocation cap remain as specified in
the [cross-family protocol](README.md). This revision moves the two
`g5g.4xlarge` ARM64 Spot instances to `us-west-2c` because the frozen
`us-east-1d` plan failed its placement-score gate before allocation. The
source archive and plan must be rebuilt for this exact commit; the old
east-region bundle and plan cannot authorize this revision.

| Binding | West-region value |
| --- | --- |
| EC2 region and zone | `us-west-2`, `us-west-2c` (`usw2-az3`) |
| Default VPC and public subnet | `vpc-0394e002d95674fe6`, `subnet-0fe2fa8824df57aea` |
| Outbound-only, zero-ingress security group | `sg-031c140ef9d078b3d` |
| Source/result S3 bucket region | `us-east-1` |
| Spot capacity and quota gate | score at least 3 for two units, G/VT Spot quota at least 32 vCPUs |
| Latest read-only observation | 2026-10-27: placement score 9, quota 32 vCPUs, quote $0.2592 per instance-hour |
| Six-hour projection | $3.25 at observed quote; $3.37 at bid, including rounded disk and S3 allowance |

The two instances receive independent EC2 identities and overlap in time.
They share an availability zone, which is permitted by the Tier 1 contract.
The local controller and each instance have separate termination guards.
The guard's EC2 region is bound to the allocation receipt; S3 custody uses
the bucket's actual region. Before any allocation, the controller refreshes
the exact-source plan and rechecks quota, image, subnet, placement, quote,
security group, IAM profile and absence of another live N83 allocation.
Observed capacity and prices are perishable. A plan with a failed gate has
no allocation authority. The $4.20 figure is a projection under the frozen
bid and duration, with residual billing risk if both deletion guards fail.
