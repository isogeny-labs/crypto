# Applied AWS deployment

`spot-experiments` in `us-east-2` is deployed with `LifecycleEnabled=true`,
`Cutover=true`, `IdleSeconds=1800`, and protected source `i-01f85a6c905181498`.
The IP controller is armed, but explicitly refuses to move `3.134.144.201` while
that source is running. Live invocation returns `protected-source-active`.
The original host is outside the managed ASG and cannot be scaled down by its
idle controller. Its 2 TiB root volume has `DeleteOnTermination=false`.

The applied Standard Spot quota (`L-34B43A08`) is **96 vCPUs**, exceeding the
requested 64. A quota request's case status can remain open after another
increase applies; the applied value is the operational check. There is no
On-Demand fallback. Spot gaps remain possible.

## Placement, durable state and readiness

The ASG requests one 32-vCPU/128-GiB x86_64 host, min zero/max two for rebalance,
with price-capacity-optimized Spot placement. It ordinarily admits m6i, m6a, m7i,
m7a, m8i and m8a families. While any registered legacy memory image is unfinished,
the controller restricts new placement to m8i, without refreshing existing workers.
The wider pool is restored only after all legacy completion receipts commit.
The 60% price protection compares to AWS's optimal matching On-Demand price;
it does not guarantee the globally cheapest host or move a healthy worker for
every quote change.

Encrypted retained EFS `fs-001b57936c90979ed` is mounted with TLS/IAM. Managed
work lives at `/srv/experiments/spot-worker`, project `.../work`, nodes `.../nodes`.
Its current `jobs.json` is empty. Legacy source workspaces/node data remain
separate. Native checkpoints and managed outputs go to EFS. Legacy CRIU memory
backups use the private/versioned/AES256 S3 bucket and independent namespaces in
[CRIU_LOCAL.md](CRIU_LOCAL.md). Memory images are excluded from Git/software bundles.

Bootstrap verifies its content-addressed runtime, mounts EFS, prepares a sparse
pinned Git workspace and Rust toolchain, and runs generic and actual solver
kill/corrupt/restore gates before `SpotReady=true`. Root recovery separately
verifies/restores legacy images. The IP moves only when EC2 checks, bootstrap,
and every unfinished legacy ownership/readiness record agree on one replacement.
After client installation, new SSH connections wake compute and pin its public host key through authenticated
SSM/Lambda. Private host keys are never copied into EFS or the client package.

Cairn, Conductor and PostgreSQL are supervised by `spot-nodes.service`; node state
survives on EFS and a lifetime writer lock serializes normal replacements. This
is a private node deployment, not a funded/public executor network. General
network-partition-safe database fencing is not established by an NFS lock.

## Wake and idle

Laptop hook events invoke only the lifecycle Lambda, without prompt/transcript
contents. A prompt wakes desired capacity one; release leaves the 30-minute
idle timer in charge. Running/pending jobs, registered recovered processes,
Cairn/Conductor work and unknown/stale activity block shutdown. Node daemons alone
do not keep the worker awake. The ASG's desired capacity is intentionally absent
from CloudFormation, so stack updates preserve live demand.

Laptops need authenticated AWS CLI access and client installation. The package
is ready; this EC2 session has no laptop execution access. Codex requires review
and trust of the new definitions through `/hooks`.

## Executed checks

Earlier AMD/Intel cold starts, actual rebalance, native cross-worker recovery and
30-minute scale-to-zero passed. Later validation proved guarded source IP,
automatic legacy recovery, new-owner backups, and a second successive recovery
from a time-namespace image. The newest managed m8i cold boot passed all recovery
bootstrap gates; the independent Diem service/runtime was then installed on that
idle standby, and a copy restore passed. The original source was never terminated
or migrated to demonstrate this. See [VALIDATION.md](VALIDATION.md).

Earlier failures are retained: quota 32, nonexistent `awscli2` RPM, missing build
assets/OpenSSL, packaged CRIU restore failure, and namespace experiments. AMI
AL2023 already supplies `/usr/bin/aws`; bootstrap checks it rather than installing
that nonexistent RPM. EFS-backed Cargo checkouts include required assets and
build dependencies. In-place updates refuse an active managed manifest.

## Reapply and rollback

Build/upload the runtime first, verify full download SHA-256, then regenerate
`template.json` and update `RuntimeDigest`, preserving other stack parameters.
`aws-config.json` records the currently applied settings. Run tests and cfn-lint.
Review replacements before any stack change; do not add a rolling refresh over
active experiments. Old runtime/source bundles remain available for rollback.

```bash
python3 infra/spot-system/package_runtime.py
python3 infra/spot-system/build_template.py > infra/spot-system/template.json
python3 -m unittest discover -s infra/spot-system
cfn-lint infra/spot-system/template.json
aws service-quotas get-service-quota --region us-east-2 \
  --service-code ec2 --quota-code L-34B43A08
```

To stop IP reconciliation during an operational rollback, set `Cutover=false`
first. Never move the address or terminate the protected source as a validation
shortcut. Retained volumes, EFS, snapshots, S3 and IPv4 continue to incur charges
when compute is off; no retention expiry policy was added.
