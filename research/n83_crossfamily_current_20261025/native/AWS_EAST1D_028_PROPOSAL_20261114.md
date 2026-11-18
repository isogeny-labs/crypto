# Proposed N83 native AWS route amendment — 2026-11-14 UTC

The currently approved `us-east-1a` route remains price blocked: the latest
Linux/UNIX `g5g.4xlarge` Spot quote at 11:00 UTC was $0.3969 per instance-hour,
above its $0.27 bid. This proposal changes the single allocation to
`us-east-1d` with the already approved replacement security group and raises
the maximum bid to $0.28 per instance-hour. Both the zone and bid require
separate owner approval before allocation. The existing approval funds only
one pair of instances; this proposal does not add an allocation.

| Proposed field | Value |
| --- | --- |
| AWS account and region | `590183823895`, `us-east-1` |
| Availability zone and zone ID | `us-east-1d`, `use1-az6` |
| VPC and subnet | `vpc-0da45a1bf46c94fa9`, `subnet-0561480728d2b1ffd` |
| Security group | `sg-030ffadf6a62d2ce2`, zero inbound permissions, all outbound |
| Allocation | One pair of distinct `g5g.4xlarge` Spot instances in one zone |
| Admission | Placement score at least 3 for two units; latest quote at most $0.28 per instance-hour |
| Runtime guards | 21,600-second host shutdown and independent local EC2 termination guard |
| Budget | $0.28 maximum bid per instance-hour; $3.49 six-hour bid projection with two encrypted 50 GB gp3 roots and $0.05 S3 allowance, under $4.20 |

At 13:36 UTC, the live placement score for `use1-az6` was 9, the latest
`us-east-1d` Spot quote was $0.2735 per instance-hour (10:00 UTC), and the
available public-IP subnet had 4,090 free addresses. The security group was
in the same VPC with no inbound rules. These values can change; the
controller must recheck account, source bundle, quota, subnet, group, price,
placement, guards, and budget immediately before the sole allocation.

This proposal changes zone, subnet, and bid from the previously approved
`us-east-1a` source. It retains the group, one-pair limit, runtime cap, sealed
points, candidate manifests, paired IC/rho schedule, selector, failure
retention, and independent replay. The controller and both hosts must use
one clean source commit and its hash-checked matching bundle. No allocation
is authorized by this proposal alone.
