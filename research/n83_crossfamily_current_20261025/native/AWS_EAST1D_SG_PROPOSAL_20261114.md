# Proposed N83 native AWS route amendment — 2026-11-14 UTC

This proposal pairs the original `us-east-1d` zone with the owner's approved
replacement security group `sg-030ffadf6a62d2ce2`. The owner's existing
approval binds `us-east-1a` and that group together; this combination needs
separate approval. No allocation is authorized by this proposal alone.

| Proposed field | Value |
| --- | --- |
| AWS account and region | `590183823895`, `us-east-1` |
| Availability zone and zone ID | `us-east-1d`, `use1-az6` |
| VPC and subnet | `vpc-0da45a1bf46c94fa9`, `subnet-0561480728d2b1ffd` |
| Security group | `sg-030ffadf6a62d2ce2`, zero inbound permissions, all outbound |
| Allocation | One pair of distinct `g5g.4xlarge` Spot instances in one zone |
| Admission | Placement score at least 3 for two units; latest quote at most $0.27 per instance-hour |
| Runtime guards | 21,600-second host shutdown and independent local EC2 termination guard |
| Budget | $0.27 maximum bid per instance-hour; $3.37 six-hour bid projection with two encrypted 50 GB gp3 roots and $0.05 S3 allowance, under $4.20 |

A read-only observation on 2026-11-14 UTC found score 9 for `use1-az6`,
latest Linux/UNIX Spot quote $0.2689 per instance-hour at 02:00 UTC, and
4,090 free addresses in the available public-IP subnet. The security group
belongs to the same VPC and had no inbound rules. These values can change;
the controller must recheck every provider, account, source, quota, price,
placement, guard, and budget gate immediately before any allocation.

The proposal changes only zone and subnet from the previously approved
`us-east-1a` amended source. It retains the approved group, one-pair limit,
bid, caps, sealed points, candidate manifests, paired IC/rho schedule,
selector, failure retention, and independent replay. A clean source commit
and hash-checked matching bundle must bind the route before allocation.
