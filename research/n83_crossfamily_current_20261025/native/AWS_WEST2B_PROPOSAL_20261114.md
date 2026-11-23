# N83 native AWS west-2b route amendment — 2026-11-14 UTC

The owner approved `us-west-2b` for the same single two-instance allocation
on 2026-11-14, superseding the earlier `us-east-1a` route. This amendment
changes the region, zone, VPC, subnet, security group, and maximum Spot bid;
it does not authorize another allocation.

| Field | Approved binding |
| --- | --- |
| AWS account, region, zone and zone ID | `590183823895`, `us-west-2`, `us-west-2b`, `usw2-az2` |
| VPC and public subnet | `vpc-0394e002d95674fe6`, `subnet-0ffc4f0562edc244f` |
| Security group | `sg-031c140ef9d078b3d`, zero inbound rules, one all-IPv4 outbound rule |
| Instance allocation | Two distinct `g5g.4xlarge` Spot instances, 50 GB encrypted gp3 root disk each |
| Placement and price gate | Live score at least 3 for two units; latest `Linux/UNIX` quote at most $0.31 per instance-hour |
| Runtime guards | 21,600-second host shutdown and an independent detached local EC2 termination guard |
| Bid-based projection | `2 × 6 × $0.31 + $0.08` disks `+ $0.05` S3 `+ $0.30` cross-region transfer = **$4.15**, below the frozen $4.20 cap |
| S3 bucket | Existing `aburan28-n83-tier1-20261016-590183823895` in `us-east-1`; S3 calls and presigned URLs bind that bucket region |

The west-2 hosts transfer data to and from the east-1 bucket. The controller
caps each source archive at 64 MiB; the host caps each raw result archive at
3 GiB and each final status archive at 256 MiB before upload. The combined
two-host ceiling is 6.625 GiB, or less than 7.12 decimal GB. The $0.30
allowance covers that ceiling at a conservative $0.04 per decimal GB. The
controller refuses a larger source archive. A host that exceeds either output
cap uploads a bounded status archive recording `ARCHIVE_CAP`; collection
retains a versioned status-only failure and requests early termination after
both final statuses arrive. The $0.05 S3 allowance separately covers storage
and API requests. This transfer allowance is a protocol budget bound, not an
observed AWS invoice. Recheck live provider costs before allocation.

At the 2026-11-14 UTC read-only observation, the two-unit placement score was
7, the latest zone quote was $0.290500 per instance-hour, the subnet was
available with public-IP assignment and 4,090 free addresses, and the west-2
G/VT Spot quota was 32 vCPUs. The original security group is in this west-2
VPC with the rules above. These observations expire; the controller must
repeat account, source, quota, subnet, image, placement, and quote gates, and
the operator must verify security group, S3, and absence of another active
N83 allocation immediately before mutation.

The west-2b controller must be committed and bundled from its exact clean
source commit. The source bundle, read-only plan, approved route amendment,
and complete two-curve measurement protocol must be bound before
the single allocation. The controller checks for active N83-tagged instances
in both `us-west-2` and `us-east-1`. The paired points, candidate identities,
alternating same-point IC/rho schedule, independent replay, raw failure
retention, and selector remain frozen. The `a=0` N83 curve requires its
separate exact solver panel and cannot be inferred from this `a=1` panel.

The 2026-11-14 launch attempt returned `Client.Blocked` and created zero
instances; its [failure receipt](verification/aws-west2b-attempt-20261114/RESULT.md)
retains the versioned S3 inputs. A retry requires AWS to confirm account
launch access, followed by a fresh exact source bundle, read-only plan, and
all live gates. The approved $0.31 per-instance-hour bid and $0.30 transfer
allowance remain within the same one-pair, $4.20 cap.
