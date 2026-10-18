# AWS deployment

The `spot-experiments` stack in `us-east-2` has the configuration in
`aws-config.json` applied. The managed compute lifecycle is enabled separately
from IP cutover. Spot-only placement, 32 vCPUs/128 GiB, price-capacity-optimized
allocation, capacity rebalancing and the 1,800-second idle timeout remain set.

The managed workspace is `/srv/experiments/spot-worker`, its initial `jobs.json`
is empty, and new node data goes under its `nodes/` directory. The original
node database, project and five running experiments are separate. No legacy
experiment was registered, restarted or declared recoverable by this deployment.
Workers require the existing manifest and EFS mount; bootstrap does not create
an empty replacement manifest if durable state is missing.

`LifecycleEnabled=true` enables prompt wake/idle control. `Cutover=false` keeps
`3.134.144.201` on the original host. Lifecycle status reports `compute_ready`
separately from `ssh_endpoint_ready`, so a healthy managed worker cannot be
mistaken for a migrated SSH endpoint. `--wait` still waits for the SSH endpoint.
Do not enable cutover until the original jobs finish or support tested recovery
and workspace/dependency migration is complete.

## Capacity and cold start

The applied Standard Spot quota is now **96 vCPUs**, checked directly with
`get-service-quota`. This meets the requested minimum of 64 and permits the
original 32-vCPU host plus two 32-vCPU managed workers during rebalance.
Earlier launches failed while only 32 was applied; that history remains in
the earlier release and scaling activity records. The minimum-64 request's
case can remain open even after the separate 96 limit is applied. No
On-Demand fallback was introduced, and quota does not guarantee Spot capacity.

The first managed worker needed a bootstrap repair: AL2023 supplies
`/usr/bin/aws`, but has no `awscli2` RPM. The corrected script verifies the
existing CLI and installs EFS/PostgreSQL packages. A subsequent
`m7a.8xlarge` completed unattended cold boot, mounted EFS, passed the packaged
kill/corrupt/restore self-test, started all four service units, and reported
idle with no unknown probes. AWS then issued a real rebalance recommendation
and launched an `m8i.8xlarge` replacement. The original host and IP stayed put.

The template no longer specifies ASG `DesiredCapacity`. Creation defaults to
`MinSize=0`; wake/idle owns demand thereafter. The old `Capacity=0` property
reset a demand-driven group during updates and replaced an idle canary. It
has been removed so stack updates do not reapply a zero desired capacity.
Do not add a rolling replacement policy or instance refresh over active work
without verified checkpoint support and a deliberate drain.

AWS Support's case API required a Premium Support subscription, so the closure
reason could not be read through that API. A closed quota case is not evidence
of approval: check `get-service-quota` for the actual applied value.

```bash
aws service-quotas get-service-quota --region us-east-2 \
  --service-code ec2 --quota-code L-34B43A08
aws autoscaling describe-scaling-activities --region us-east-2 \
  --auto-scaling-group-name GROUP_FROM_STACK_OUTPUTS --max-items 3
```

Spot requests and running instances share the regional vCPU quota. See
[AWS Spot quotas](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-spot-limits.html).
New-worker cold boot and the packaged recovery gate have passed. Actual
application checkpoints, cross-worker recovery, and idle shutdown have separate
evidence entries; bootstrap success alone does not establish them.

## Bootstrap recovery gate

The runtime includes `bin/checkpoint-selftest`, built from the native recovery
test target with `target-cpu=x86-64`. Before starting the worker services or
setting `SpotReady=true`, bootstrap runs its kill/corrupt/restore test in a
private temporary directory under `WorkRoot`. It uses the packaged production
supervisor and scanner, then removes its synthetic state. Failure blocks
readiness. Output goes to `/var/log/spot-checkpoint-selftest.log`.

The exact packaged test passed on mounted EFS on both original and managed
hosts, including AMD and Intel workers. The existing factor-base processes
still run their older executable and have no native checkpoint interface.

Build/install both native executables before `package_runtime.py`:

```bash
RUSTFLAGS='-C target-cpu=x86-64' cargo build --release --locked \
  --manifest-path infra/spot-system/checkpoint-discovery/Cargo.toml
RUSTFLAGS='-C target-cpu=x86-64' cargo test --release --locked --no-run \
  --test recovery --message-format=json \
  --manifest-path infra/spot-system/checkpoint-discovery/Cargo.toml > /tmp/checkpoint-build.jsonl
```

Install `target/release/checkpoint-discovery` as
`~/.local/bin/checkpoint-discovery`. The Cargo JSON `compiler-artifact` with
`target.name=recovery` supplies the recovery target's `executable` path;
install it as `~/.local/bin/checkpoint-selftest`. This keeps the runtime
package independent of Cargo's generated executable suffix.

## Reapply configuration

Build the template, upload its digest-pinned runtime, and create a CloudFormation
change set with `aws-config.json`'s parameters plus `RuntimeDigest` from
`runtime.tar.sha256`. Preserve other current stack parameters. Review resource
replacements before execution; retain the original instance, EFS and S3 data.
Use a separate workspace for validation and check for active managed work before
an update that changes desired capacity. Demand-driven desired capacity belongs
to the lifecycle controller and is intentionally absent from the template.

The deployed wake function is the stack's `LifecycleFunctionArn` output.
Its `wake`, `status` and `release` actions require IAM `lambda:InvokeFunction`.
Workstation hook installation/trust remains a client-side step; hooks installed
only on a stopped worker cannot wake that worker.
