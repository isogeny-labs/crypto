# Attempt 2: alternate AWS account Spot host

Frozen before instance launch on 2026-11-16 UTC. This is a separate capacity
attempt following the [account-blocked attempt 1](../attempt-1/RESULT.md).
It retains the parent [source, fixture, Rust image, cgroup and release-suite
contract](../PROTOCOL.md), with only the host account, subnet, security group,
run tag and output path changed. Do not rewrite or pool attempt 1.

## Frozen host difference

Use the separately configured `ecc2k130` AWS profile, IAM user `adam` in
account `590183823895`. In `us-east-1f`, request one `c7g.2xlarge` Spot
instance on AMI `ami-061bb79b9c35a0b94`, subnet
`subnet-07f637a5d1c6e15d2`, security group
`sg-09d3d2dc37e3354a7`, and key `meow34`. The group currently permits
TCP/22 only from `99.100.36.89/32`; stop rather than widening ingress if
the source IP changes. Use the same 150 GiB `gp3` root with
`DeleteOnTermination=true`, one-time Spot/terminate behavior,
`MaxPrice=0.18` USD/hour, instance-initiated termination and the committed
225-minute [`user-data.sh`](../user-data.sh) shutdown backstop. The current
Spot price observation for that AZ/type was `0.123300` USD/hour at
`2026-11-16T10:00:00Z`; the cap controls the request, and neither number
is a measured charge. Use fixed client token
`n83-full-fixture-20261116-attempt2` to avoid duplicate allocation after an
ambiguous CLI response. No instance role or new security-group rule is
authorized for this attempt.

The exact alternate-account `RunInstances --dry-run` returned
`DryRunOperation` before this packet was frozen. That is authorization-shape
evidence only; real account acceptance and capacity remain untested. Run the
committed [`launch.sh`](launch.sh) once into a fresh local output directory.
Retain its full response or error, status, UTC endpoints and returned IDs.
If the API gives an unclear or timed-out response, inspect the client token
and tagged instance inventory before any further call. Do not restart by
assuming no instance. If it returns `Blocked`, insufficient capacity or a
policy denial, preserve the stop and do not launch a second candidate under
this packet.

If allocated, check the exact instance ID, Spot request, root volume,
instance type, architecture, AMI, tags, public IP, shutdown behavior and
user-data execution. Verify the source-IP rule still matches, then use the
local `meow34` key to reach `ec2-user` over SSH. Install Docker on the fresh
AL2023 host, clone the public repository at exact merged source
`a91b8b66d0f1ffc175fdce984d1059144ce232f6`, and set
`core.sparseCheckout=false`. Download the pinned 2,293,097-byte archived
fixture locally through `ecc-challenges-new`, verify SHA-256 and byte count,
then SCP only that public fixture and the committed Dockerfile and host
runner. No AWS credential, private archive or Docker socket enters the host
or container. Build the pinned Rust-image Dockerfile and run the committed
[`run-host.sh`](../run-host.sh) under the parent 12 GiB/no-swap/four-CPU/512-PID
and 7,200-second child cap. Retain setup failures as such; a failed clone,
package, fixture, image or cgroup gate is not a library test verdict.

The instance must terminate within four hours of launch. After the attempt,
retrieve raw output, inspect Docker `OOMKilled` and the source/fixture/test
receipts, terminate by exact instance ID if still live, and verify the
instance state and root volume deletion. The automatic shutdown is a backup,
not a substitute for explicit cleanup. Keep local copies of partial output
if SSH or Spot capacity is interrupted. No index-calculus factor-base arm
may run on this validation host under this packet.
