# Spot experiment host

Source and tests live here. Download the complete private deployment bundle,
compiled runtime and workstation installer using [ARTIFACTS.md](ARTIFACTS.md)
and its SHA-256 manifest. Publication checks are in [VALIDATION.md](VALIDATION.md).

[Run records](RUN_RECORDS.md) describe each managed experiment in versioned,
harness-independent JSON. Codex, Claude, OpenCode or a terminal can submit
`experiment-runner --root ROOT --resume state/JOB/run.json`; the background
service resumes the latest valid checkpoint without the original agent session.

[Checkpoint discovery](CHECKPOINT_DISCOVERY.md) inventories live experiments and
checkpoint candidates. New manifest-contract jobs validate generation checksums
and automatically select the newest valid state before resuming. A read-only
timer writes the durable inventory 15 minutes after its previous run finishes.
[Native factor-base checkpoints](NATIVE_CHECKPOINTS.md) save and restore the
actual sparse-elimination state for new runs; existing old processes are separate.

The AWS compute lifecycle and guarded IP failover are enabled in an isolated
managed workspace. The controller keeps the original address on the source while
it runs. The applied Standard Spot quota is 96 vCPUs. Private memory backups run
every 15 minutes, and automatic recovery has passed two successive replacement
hops on disposable hosts.
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
which arms guarded IP movement. Both are true in the deployed configuration;
`ProtectedSource` prevents handoff while the original source is alive.
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
machine where Claude/Codex/OpenCode run, then run `bash install_client.sh` inside its
`spot-client` directory (`bash install_client.sh NAME` selects an existing AWS CLI profile).
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
ready when the protected running source or a ready replacement holds the Elastic IP.
The installed SSH ProxyCommand wakes capacity and obtains the current public host
key through authenticated AWS before connecting. It retains strict host verification;
no private host key is copied.

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
- The five original factor-base jobs finished normally after private CRIU backups
  were enabled. Their completed summaries remain backed up.
- The running Diem generator PID 400334 has independent private memory backups
  and a tested four-thread compatible-worker copy restore. Both backup timers
  run at 15-minute wall-clock boundaries; automatic recovery is armed. See
  [CRIU_LOCAL.md](CRIU_LOCAL.md) and [validation](VALIDATION.md).

## Architecture and limits

CloudFormation creates encrypted, retained EFS with IAM-authenticated TLS mounts in all three
default-VPC AZs, a Spot-only Auto Scaling group, a scheduled Lambda IP
controller, a restricted worker role, and a controller error alarm. An optional
SNS ARN enables alarm notifications; without it the alarm has no delivery target.
EFS automatic backups are enabled; account backup policy/restore must be tested.
EFS storage, throughput, backup, EC2 roots, IPv4, and snapshot charges are additional
to compute. No snapshot expiry policy has been installed.

The ordinary pool includes m6a, m6i, m7a, m7i, m8a and m8i `.8xlarge`.
Unfinished legacy CRIU work temporarily restricts new placement to m8i; the
controller releases this restriction after every group completes.
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

## Deployment, operations and remaining activation

The applied system and exact validation scope are documented in
[AWS_DEPLOYMENT.md](AWS_DEPLOYMENT.md), [VALIDATION.md](VALIDATION.md) and
[CRIU_LOCAL.md](CRIU_LOCAL.md). `aws-config.json` records current settings;
`artifacts.json` identifies immutable private S3 releases. Keep prior releases
for rollback. Stack updates preserve lifecycle-owned desired capacity and do not
refresh active workers.

The current host and its original jobs are outside the managed ASG. Never
terminate that source or move its IP as a validation shortcut. Replacements may
restore registered legacy work only after EC2 confirms the previous owner and
source have terminated. Each group must be ready on the same worker before
address handoff. If capacity is unavailable, demand persists and the system waits.

The original retained disk is a separate rollback resource. Native managed jobs
use EFS; legacy images and complete outputs use private S3. New arbitrary jobs
need a tested checkpoint contract or explicit, restore-tested CRIU enrollment.
The native scanner's historical-archive coverage is explicitly partial.

The laptop package is prepared and tested in temporary client homes. Installation
on the user's laptop has not been executed because this session has only EC2
access. After running it, review Codex `/hooks` and trust the exact definitions.
Then the original command `ssh -i rootkey.pem ec2-user@3.134.144.201` uses the wake
proxy. Existing SSH sessions still disconnect on host loss; reconnect resumes
access after Spot recovery. Hooks transmit only lifecycle actions, not prompts.

The newest validation includes 72 infrastructure tests, the 4,399-pass release
library gate and two example tests, guarded controller/public-key invocation,
managed cold boot, recurring private backup checks, automatic two-hop legacy
recovery and the Diem copy restore. Earlier AMD cold boot, native cross-worker
restore and live 30-minute managed shutdown also passed. Laptop event execution,
universal memory recovery and partition-safe database failover are not claimed.

Refresh actual Spot quotes at any time with `python3 prices.py`. At the first
observation, m6a.8xlarge in us-east-2c was $0.2362/hour and the current
m8i.8xlarge pool in us-east-2c was $0.4690/hour. These are observations,
not price guarantees or throughput comparisons.

AWS design references: [mixed-instance placement](https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-mixed-instances-group-attribute-based-instance-type-selection.html),
[Capacity Rebalancing](https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-capacity-rebalancing.html),
[Elastic IP reassociation](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_AssociateAddress.html).
