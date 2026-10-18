# Spot experiment host

Source and tests live here. Download the complete private deployment bundle,
compiled runtime and workstation installer using [ARTIFACTS.md](ARTIFACTS.md)
and its SHA-256 manifest. Publication checks are in [VALIDATION.md](VALIDATION.md).

[Checkpoint discovery](CHECKPOINT_DISCOVERY.md) inventories live experiments and
checkpoint candidates. New manifest-contract jobs validate generation checksums
and automatically select the newest valid state before resuming. A read-only
timer writes the durable inventory 15 minutes after its previous run finishes.
[Native factor-base checkpoints](NATIVE_CHECKPOINTS.md) save and restore the
actual sparse-elimination state for new runs; existing old processes are separate.

The AWS compute lifecycle is now enabled in an isolated managed workspace;
IP cutover remains disabled to protect the original jobs. The applied Standard
Spot quota is now 96 vCPUs and managed workers have booted successfully.
See [AWS deployment](AWS_DEPLOYMENT.md) and `validation/aws-deployment.json`.

This deployment targets one logical x86_64 host with 32 vCPUs and 128 GiB RAM
in us-east-2. It uses Spot only, with capacity gaps accepted. It preserves
the existing SSH endpoint `3.134.144.201` by moving its existing Elastic IP
only after a replacement finishes bootstrap and EC2 status checks pass.
Moving the IP breaks existing SSH connections; reconnect after recovery.

## Prompt wake and 30-minute idle shutdown

The lifecycle Lambda accepts IAM-authenticated `wake`, `release`, and `status`
requests. A submitted prompt requests desired capacity one. `SessionStart` also
wakes it. A completed turn/session sends `release`, which leaves the idle timer
in charge. Ending a chat never immediately stops an experiment.

Every minute the controller checks fresh worker reports. It scales to zero only
after 1,800 seconds of confirmed inactivity, and no earlier than 30 minutes after
the most recent wake. Running or pending manifest jobs, Cairn queue/running spool
entries, and nonterminal Conductor tasks prevent shutdown. Missing reports,
failed probes, clock skew, and booting replacement instances prevent shutdown.
Node daemons alone do not keep compute awake. `IdleSeconds` changes the timeout.
Worker reports arrive every 30 seconds and become unusable after 90 seconds.
Reports retain the last busy timestamp, including admission of short jobs that
finish between timer ticks, so a brief experiment still restarts the idle window.

Wake, idle decisions, activity reports and managed job admission share a DynamoDB
lease. Before starting a job, the runner publishes busy state under that lease
and refuses admission on a draining/non-InService instance. A prompt racing
shutdown renews demand or requests a replacement afterward. Pending demand is
stored before the scaling API call so the next timer tick can retry it.
Register experiments in the manifest/managed spool or wake via the hooks before
submitting work through Conductor. A conservative process scan also blocks
shutdown on unrecognized processes owned by the worker user; the current
factor-base workers have an additional explicit guard. Unregistered root/remote
jobs are outside that scan. Tracking remains necessary for checkpoint recovery
and coordinated job admission.

`LifecycleEnabled` controls managed startup/shutdown independently of `Cutover`,
which controls IP movement. The deployed settings enable the former and leave
the latter false until the migration gates below are met.
The original host is outside the new Auto Scaling group and is not a shutdown
target. The new Lambda role can change desired capacity only for its worker group.
Stopping compute keeps EFS, backups, snapshots and the Elastic IP allocation;
their charges continue. Cold startup may wait for Spot capacity and node recovery.

## Cairn and Conductor nodes

`spot-nodes.service` is installed and running on the current host. It manages
PostgreSQL 17, Conductor, and Cairn. The private endpoints are Conductor on
127.0.0.1:8080 and Cairn on 127.0.0.1:9180 (P2P on 127.0.0.1:9100). Both HTTP
endpoints passed health checks, and a service restart preserved PostgreSQL's
cluster identity. It uses no Docker and does not run model inference.

PostgreSQL data/WAL, Conductor's state/future encryption key, and Cairn state/new at-rest key live under the encrypted EFS
path `/srv/experiments/nodes`, mode 0700. PostgreSQL accepts peer-authenticated
local Unix-socket connections only. The node bundle holds an EFS single-writer
lock for its whole lifetime, so overlapping replacements wait before opening
the database. Network-partition fencing still needs the cross-host recovery test;
the file lock is not a proof of partition-safe database failover.

Conductor is built from source revision
`d99a2bbb23d5e59862d7614886e414c95c3dfe13`, using Go checksum verification.
Cairn is release v12.1.0, checked against its published SHA-256. The Cairn node
is currently a private relay: no public seeds, funded verifier, or executor host
agent was configured. Existing research code and live experiments are unchanged.
The node is available locally/through an SSH tunnel; joining an existing network
requires its actual peer/bootstrap configuration.

New workers fetch a deterministic runtime archive from a private, encrypted,
versioned S3 bucket and verify its SHA-256 before installation. The archive
includes the node binaries and supervisor/idle services. `NodeProject` selects
the durable project directory. Scientific dependencies still need migration.
Build the native checkpoint-discovery binary as described in its runbook and
the bootstrap self-test as described in `AWS_DEPLOYMENT.md`; install both beside
the node executables before packaging. After a runtime change,
run `python3 package_runtime.py`, upload it as
`runtime/<digest>.tar.gz` in the stack bucket, and explicitly update the
`RuntimeDigest` stack parameter. Keep old versions for rollback.

## Install the wake hooks on each coding-client machine

`spot-client.tar.gz` contains a ready-to-install client package and a least-privilege
IAM policy allowing only invocation of this stack's lifecycle Lambda. It contains
no credentials, prompt text or private host keys. Download/extract it on the
machine where Claude/Codex/OpenCode run, then run `python3 setup.py` inside its
`spot-client` directory (`--profile NAME` selects an existing AWS CLI profile).
The client needs Python 3, AWS CLI v2, and authenticated InvokeFunction access.

The installer preserves existing settings, backs up changed files, and merges:

- Claude: `~/.claude/settings.json`, SessionStart/UserPromptSubmit wake and Stop/SessionEnd release.
- Codex: `~/.codex/hooks.json`, the same events.
- OpenCode: `~/.config/opencode/plugins/spot-system.js`, session creation and
  user-message events wake; idle/deletion events release.

Codex requires reviewing/trusting the exact new hook definitions through `/hooks`.
The installer does not bypass this trust check. Hooks are best-effort: missing
AWS access warns but does not block unrelated coding. Prompt/transcript input
is neither uploaded nor saved. An interactive readiness wait is available with
`python3 ~/.local/share/spot-system/wake.py --wait 600`; the function reports
ready only after a ready group instance holds the expected Elastic IP.

These client hooks have been prepared and tested in temporary homes. They have
not been installed on the user's workstation: this session controls the EC2
host. Installing only on the Spot host cannot wake that same host while it is off.

Hook schema references: [Codex hooks and trust](https://learn.chatgpt.com/docs/hooks),
[Claude hook events](https://code.claude.com/docs/en/hooks),
[OpenCode plugin events](https://opencode.ai/docs/plugins/).

## Observed current host (2026-10-18)

- Instance: `i-01f85a6c905181498`, m8i.8xlarge, Spot, us-east-2c.
- Existing address allocation: `eipalloc-00c2a8a867901bf7d`.
- Root volume: `vol-0f3f8dc899a8f30c2`, 2 TiB, about 106 GiB used.
- **Fixed and verified:** root `DeleteOnTermination` changed from true to false.
  A retained volume remains billable and remains in its original availability zone.
- Recovery snapshot requested: `snap-064b6480704e5ced9`. This is a live,
  crash-consistent disk snapshot, not an application checkpoint or RAM image.
- No existing Auto Scaling membership was present.
- Five running `factor_base_shape_ladder` jobs were found. The launcher has
  no native checkpoint, resume, or signal handling implementation. Approved
  leave-running CRIU snapshots now exist locally and are refreshed by a timer;
  actual legacy restore remains unverified. Do not terminate this host to test recovery.

## Architecture and limits

CloudFormation creates encrypted, retained EFS with IAM-authenticated TLS mounts in all three
default-VPC AZs, a Spot-only Auto Scaling group, a scheduled Lambda IP
controller, a restricted worker role, and a controller error alarm. An optional
SNS ARN enables alarm notifications; without it the alarm has no delivery target.
EFS automatic backups are enabled; account backup policy/restore must be tested.
EFS storage, throughput, backup, EC2 roots, IPv4, and snapshot charges are additional
to compute. No snapshot expiry policy has been installed.

The six currently matching types are m6a, m6i, m7a, m7i, m8a and m8i `.8xlarge`.
Auto Scaling uses price-capacity-optimized placement and Capacity Rebalancing.
Price protection defaults to 60% of AWS's optimal matching On-Demand price;
this is not a fixed USD ceiling. It chooses capacity at launch/replacement;
it does not migrate a healthy host every time another pool becomes cheaper.
Equivalent RAM/vCPU counts do not imply equal experimental throughput.
Compile portable x86 binaries and check CPU-specific instructions before allowing
cross-family recovery. `-C target-cpu=native` builds from m8i are not generally safe
to move to m6a/m6i without rebuilding.

Roots on NEW workers are disposable 100 GiB volumes. All source, inputs,
outputs and checkpoints needed for recovery must be under `/srv/experiments`
or otherwise durably reproducible. Never treat a replacement root as persistent.
The original retained 2 TiB disk is not automatically mounted by new workers.

The supervisor runs jobs sequentially, allowing each job the full machine.
It uses an NFSv4 file lock to prevent normal overlapping replacements from running
the same workload. Distributed network partitions and external side effects need
application fencing/idempotency; this is not an exactly-once execution guarantee.
The Lambda runs outside the Spot host every minute and uses a conditional
DynamoDB lease to prevent overlapping invocations from moving the IP concurrently.
No public SSH probes or private SSH client keys are required by the controller.

## Checkpoint contract

Each job must periodically save a coherent, durable checkpoint and restore it
at startup. Save to `$SPOT_CHECKPOINT_DIR` by writing a temp file, fsyncing it,
atomically renaming it and fsyncing the containing directory. A multi-file
checkpoint needs a committed manifest/generation scheme. Keep a previous
verified generation so corrupted checkpoints can be rejected and rolled back.
Write results to `$SPOT_OUTPUT_DIR`; `$SPOT_RESUMED=1` signals existing state.
Handle SIGTERM by finishing a checkpoint and exiting within 75 seconds.
The supervisor polls the IMDSv2 interruption endpoint every five seconds.
Checkpoint on a timer too: an interruption warning is not guaranteed.
Maximum lost work depends on the application's last durable checkpoint,
not the supervisor polling interval. It does not suspend/restore arbitrary RAM.

An explicit contract acknowledgment in the manifest prevents accidental admission
of jobs with no recovery support; it cannot prove the application implements it.
Require an actual kill-and-resume test for each workload before admitting it.
The supervisor skips successfully completed jobs and rejects reusing an id with
a changed command/spec. Foreground commands only; do not daemonize workers.

Example `/srv/experiments/jobs.json` (an application implementing these flags
must exist before using this example):

```json
[
  {
    "id": "search-001",
    "cwd": "work/search",
    "argv": ["./search", "--resume-from-env"],
    "checkpoint_contract": "atomic-files-and-sigterm-v1"
  }
]
```

## Staging and migration

Build and check:

```bash
cd infra/spot-system
python3 build_template.py > template.json
python3 -m unittest discover -s . -p 'test_*.py' -v
cfn-lint template.json
aws cloudformation validate-template --region us-east-2 --template-body file://template.json
```

`spot-experiments` was initially prepared with capacity zero and cutover false,
with an initial minimum of zero. The current deployment enables wake/idle control
in `/srv/experiments/spot-worker` while preserving the original IP. See the AWS
deployment record for current runtime/parameters, cold-boot and recovery tests.
Current resource ids, parameters, snapshot progress and remaining work are saved
in the private release bundle's `status.json` (download instructions are in `ARTIFACTS.md`). Inspect its current state with:

```bash
aws cloudformation describe-stacks --region us-east-2 --stack-name spot-experiments
```

Before activation:

1. Finish or checkpoint the five existing non-resumable experiments. Adding
   checkpoint code to a new binary cannot retrofit running processes.
2. Mount the stack's EFS output on the current host at `/srv/experiments`
   using amazon-efs-utils with `tls,iam`. Copy required workspaces and dependencies;
   verify hashes. Do not copy an actively changing multi-file checkpoint as
   though it were a coherent generation. Keep the original disk as rollback.
3. Choose an SSH host identity policy. `PreserveHostKeys=false` is the default:
   workers have distinct keys, so verify each new fingerprint over SSM before
   updating the client's known_hosts entry. Do not disable host verification.
   Automatic approval review rejected copying the original private host keys
   into shared EFS because this changes their credential storage boundary.
   No keys have been copied. Only with explicit user approval, place original
   keys in a root-owned, mode-700 `ssh-host-keys` directory on encrypted EFS
   and set PreserveHostKeys=true. Bootstrap then imports that identity.
4. Provide a validated jobs.json. Install workload dependencies via a tested
   x86_64 AL2023 custom AMI, or a tested extension of bootstrap. The supplied
   bootstrap installs node services, PostgreSQL, EFS utilities, AWS CLI and Python;
   research tools still require migration.
   Validate the image on every admitted CPU family. Keep the root device name
   `/dev/xvda` and the bootstrap-compatible AL2023 environment.
5. Stop only the original `spot-nodes.service` before handing node state to the
   replacement; this releases its EFS writer lock and does not stop the existing
   experiment processes. Update the stack to Capacity=1, Cutover=false, with the tested AMI. Prove
   bootstrap and workload restore on the replacement via SSM. Keep live jobs
   out of the manifest until they have stopped on the original host. The
   original jobs are not members of the new supervisor's lock domain.
6. Test graceful termination and a forced worker kill using a disposable job
   and compare its final result to uninterrupted execution. Test EFS locking
   across two instances, restart behavior, restore rejection of corrupt state,
   and a lost instance without an interruption warning.
7. After migration validation, review the stack update with Cutover=true;
   LifecycleEnabled is already true in the deployed configuration. A prompt
   then starts a worker; the IP moves once bootstrap is ready. This interrupts
   SSH connections. Run this
   from an independent session, not solely the SSH session being replaced.
8. Verify reconnect using the original command and prove restart from a real
   durable checkpoint before retiring the original instance.

Do not run `terminate-instances` against the original host as a shortcut.
For rollback, disable Cutover first to prevent the controller moving the address
back, then associate the existing allocation with the original running host.
Retained EFS and the original volume need explicit retention/cost management.

## Validation status

42 system tests and 20 native discovery tests passed locally; 35 earlier system
tests also passed with temporary state on the actual
mounted EFS before the final short-job timer and user-process guard cases. Tests include idle threshold boundaries, wake
races, missing/stale activity, failed probes, draining-worker admission,
hook merging and existing checkpoint tests,
including forced worker death, checkpoint resume,
spec-change rejection, duplicate supervisor serialization, and safe IP decisions.
AWS validated the CloudFormation syntax; cfn-lint validated the resource schema.
AWS resolved the requirements to six equivalent-size instance types.
The real worker-report protocol (including the DynamoDB lease) published a busy,
unknown report for the original host, and the packaged client invoked wake and
release against the deployed Lambda and received the expected staged response.
Both node endpoints passed checks and a node-service restart preserved the
database identity. No live idle termination, new-worker cold boot, or workstation
hook event has been exercised.

The native discovery timer is installed on the original host and refreshes
`/srv/experiments/discovery/current-host.json`. It identified the five existing
factor-base workers without checkpoint interfaces. Full archive coverage is
partial and explicitly reported; see `validation/current-host-discovery.json`.
Discovery's native kill/restore fixture proves generation selection and supervisor
integration, not recovery of these non-checkpointed live workers.

The current host successfully mounted `fs-001b57936c90979ed` at `/srv/experiments`
using TLS and IAM. amazon-efs-utils was installed on the host for that check.
The mount is temporary; it has not been added to this host's fstab.
No live experiment directories or private SSH keys have been copied into EFS.
These checks do not establish a successful cross-AZ EFS mount, running-job
migration, application checkpoint correctness, or a complete live failover.

Refresh actual Spot quotes at any time with `python3 prices.py`. At the first
observation, m6a.8xlarge in us-east-2c was $0.2362/hour and the current
m8i.8xlarge pool in us-east-2c was $0.4690/hour. These are observations,
not price guarantees or throughput comparisons.

AWS design references: [mixed-instance placement](https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-mixed-instances-group-attribute-based-instance-type-selection.html),
[Capacity Rebalancing](https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-capacity-rebalancing.html),
[Elastic IP reassociation](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_AssociateAddress.html).
