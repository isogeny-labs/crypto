# N83 native AWS route amendment — 2026-11-14 UTC

The owner approved moving the single two-instance `g5g.4xlarge` Spot panel
from `us-east-1d` to `us-east-1a` and replacing the unavailable security group
with `sg-030ffadf6a62d2ce2`. This amendment binds both route changes together;
the earlier approval has not funded a second allocation.

| Frozen field | Amended value |
| --- | --- |
| AWS account and region | `590183823895`, `us-east-1` |
| Availability zone and zone ID | `us-east-1a`, `use1-az1` |
| VPC and subnet | `vpc-0da45a1bf46c94fa9`, `subnet-04e6765fae0326ef3` |
| Security group | `sg-030ffadf6a62d2ce2`, zero inbound permissions, outbound allowed |
| Instance count and type | Two distinct `g5g.4xlarge` Spot instances in one zone |
| Placement and bid gate | Score at least 3 for two units; latest quote at most $0.27 per instance-hour |
| Runtime guards | 21,600-second host shutdown and independent local EC2 termination guard |
| Budget | $0.27 per instance-hour maximum bid; $3.37 six-hour bid projection with encrypted 50 GB gp3 root disks and $0.05 S3 allowance, under the $4.20 cap |

The 2026-11-14 UTC read-only provider observation found the subnet in the
specified VPC with public-IP assignment and sufficient free addresses. The
replacement security group had zero inbound rules. `us-east-1a` scored 9 for
two units, but its latest Spot quote was $0.3968 per instance-hour, above the
frozen bid. This observation does not admit allocation. Recheck all live
provider, account, source-bundle, quota, price, placement, guard, and budget
gates immediately before the single allocation.

The source commit and verified bundle for this amendment are created only
after these route changes are committed. The old
`e084a0b8f33ad9f797d1a766b53ac66f77846c08` source bundle remains the
unaltered record of the prior `us-east-1d` route. The amended controller and
both hosts must use the same new commit and its matching verified bundle.
All sealed points, candidate manifests, paired IC/rho schedule, selector,
failure retention, and independent replay remain governed by the original
protocol.
