# N83 current-source cross-family comparison

The current-source K3450 wide3 and K3500 root4 bases have 572,700 and
581,000 usable subgroup points, respectively. Their first 3,450 and 3,500
signed-Frobenius orbits reconstruct the historical ordered bases exactly:
the BLAKE3 base hashes, ordered-dump SHA-256 values, and point-set SHA-256
values all match. The source-updated IDs are
`IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0h622efa367d76`
and `IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0h0de90f01951d`.
They have no current-source timing yet. `seal_k3450.py` and `seal_k3500.py`
reproduce the identities and receipts from the archived current-source
K3596 base dump.

Sixteen new public points of `EC1N83Ckb1a1haacb69ee9f37` are sealed in
`targets.jsonl`, with scalar replay sidecars and 16 separate canonical
single-target workload IDs. `generate_fixture.py` deterministically
regenerates the points from the domain
`n83-crossfamily-current16-20261025-v1`. The exclusion audit checks all
35,172 earlier distinct public points in the prior inventory, nearby-grid
screen, and sealed nearby-grid confirmation; it reports zero overlap and zero
duplicates. Each point belongs to the subgroup of order `8569786107849059`.
This `a=1` curve is the 53-bit-subgroup diagnostic in the broader N83 sweep.
The sweep's primary `a=0` curve has subgroup order
`2417851639230796216685689` and needs its own candidate and measurement
comparison. The validated row and selector carry this curve ID and subgroup
order explicitly.

The measured panel compares the current-source K3300 wide3, K3450 wide3,
K3500 root4, and K3596 root4 candidates on these same 16 points. The fixed panel has two
eight-point Tier 1 strata. Each candidate-point pair uses a fresh IC process
and an independently verified same-point rho process, with three fixed-point
rho calibrations before and after. The 512 scheduled children rotate arm
order and alternate IC/rho order. The IC child cap is 1,200 seconds, the rho
cap is 120 seconds, the per-stratum producer cap is 300 minutes, and the
host requires at least four visible CPUs and enough memory for the frozen
15.25 GiB child RSS cap. The runner and independent verifier preserve every
failed, incomplete, or noisy pair. The primary measure is each candidate's
verified one-target online wall time; fresh-process cold cost is
supplementary. A later fresh-point bridge will compare an admitted high-K
winner with the nearby-grid small-base selection after its confirmation.
The panel has not yet dispatched or selected a winner.

`run.py` freezes the event schedule, per-child caps and exact IC flags.
`verify.py` independently replays base construction, relations, scalar
answers, phase timing, child resources, Tier 1 telemetry and calibration.
`source_bundle.py` packages the exact committed source and retained inputs into
a content-hashed archive; the Pod checks every extracted file against that
archive. `runpod_runner.py` binds each stratum to its archive and Pod allocation;
`runpod_provider.py` requires two distinct machines with overlapping run
intervals. `aggregate_runpod.py` binds the raw archives to local replay and
applies `select.py`. The selector tests all 16 paired point differences,
uses an exact sign-flip test with a 0.05/6 pairwise threshold, and leaves the
online and cold selections unknown if any required row or host gate fails.
The allocated Pods, binary hashes, source head and archive hashes must be frozen in provider
receipts before running; no such allocation has been made for this panel.
`allocate_runpod.py` checks the clean exact-source checkout, sealed inputs,
the full historical exclusion inventory, and four manifests before any
provider mutation. It requires a verified source bundle before `--execute`.
The Pod runner checks sealed file hashes, canonical
single-target workload IDs, subgroup membership, and all 16 scalar replays
without needing the repository's historical Git objects. Both allocations
request NVIDIA GeForce RTX 4090 community Pods, each with
at least 27 GiB of system memory and four vCPUs. The allocator verifies the
returned machine GPU types, distinct machine IDs, and observed rates at most
$0.34/hour per Pod. It requests a provider `terminateAfter` deadline of 360
minutes, arms a separate local deletion watcher before allocation, and starts
a Pod-side `runpodctl` deletion task before computation. RunPod's provider
setting alone is not an enforced expiry; the two watchers independently
request deletion at the deadline and retain retry/absence receipts. If both
watchers and provider access fail, the $4.20 projection is not a guaranteed
account billing ceiling. Each Pod requests exactly 30 GB of container disk
and zero persistent volume. The conservative six-hour projection at both rate
ceilings is $4.14, including $4.08 of compute and $0.06 of container disk
rounded upward using a 28-day month. The earlier A5000 and 3080 Ti requests
returned provider errors without Pod IDs; subsequent provider readbacks found
zero matching N83 Pods. This allocation retains the same host requirements.
It reads a configured
RunPod key from the process environment or `~/.runpod/config.toml`, never
copies it into the receipts, verifies provider readback and distinct machine
IDs, and attempts to terminate every discovered Pod if allocation fails.
The storage rate is pinned from [RunPod's Pod pricing documentation](https://docs.runpod.io/pods/pricing),
which lists $0.10 per GB per month for running container disk and separate
per-second compute and storage billing. The allocation receipt retains the
rate, disk size, clock limit and observed Pod rates.
The allocation output belongs outside the repository. Raw result transfer
and termination receipts are handled by `dispatch_runpod.py`: `--stage`
transfers the sealed source and allocation receipts to two public SSH
endpoints, and repeatable `--collect` calls retrieve checksummed raw archives
and logs, then terminate both Pods after both have finished. A failed stage
attempt records cleanup; a failed transfer keeps both Pods available until
the provider auto-termination deadline. `--stage-and-watch` combines dispatch
with 30-second collection retries through that deadline and writes a final
custody status. `--watch` resumes an interrupted watcher.
`launch_on_pod.sh` installs the pinned Rust toolchain, tests and builds the
two exact producers, runs one stratum with four exposed CPU IDs, verifies the
raw receipts, and archives them even when a stage fails. The dispatch output
and source bundle both belong outside the repository. Stage and collect require
the configured RunPod key and SSH public-key access to the account's Pods.

### Lower-cost RunPod Tier 1 allocation

The [Ada 2000 allocation protocol](RUNPOD_ADA2000_PROTOCOL.md) retains this
sealed workload and the two-machine verification gate with a $0.25/hour
per-Pod rate ceiling and $3.10 six-hour projection cap. It is a separate
hardware profile in `allocate_runpod.py`; the original RTX 4090 profile
remains unchanged. This profile needs separate spend approval and an
authenticated local controller before `--execute`.

### AWS Spot Tier 1 alternative

`plan_aws_spot.py` checks an exact-source, two-instance Tier 1 alternative
before any AWS mutation. It requires two separately identified `g5g.4xlarge`
ARM64 Spot instances, each exposing 16 vCPUs and 32 GiB RAM, in
`us-west-2c`. This same-zone placement follows the practical cloud-instance
Tier 1 contract; it does not establish physical host isolation. The six-hour
maximum bid is $0.27 per instance-hour. At those bids, two 50 GB gp3 root
volumes and a $0.05 S3 allowance, its six-hour projection is $3.37 under a
$4.20 allocation cap. This projection is not a guaranteed account billing
ceiling. On 2026-10-27 the AWS API returned G/VT Spot quota 32 vCPUs, a
$3.25 projection at the $0.2592 hourly quote, and placement score 9 for two
units in `us-west-2c`. Prices and scores must be rechecked before allocation.
The source bucket remains in `us-east-1`; its presigned transfer URLs and
versioned collection use that bucket region while EC2 and the local deletion
guard use `us-west-2`. The west-region revision is recorded in
[`AWS_WEST2_ADDENDUM.md`](AWS_WEST2_ADDENDUM.md). The gp3
rate and meaning of placement scores follow [AWS EBS pricing](https://aws.amazon.com/ebs/pricing/)
and [AWS Spot placement guidance](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/how-sps-works.html).
`allocate_aws.py` refreshes this plan from the provider immediately before
allocation. Its default invocation is read only. The `--execute` option
stages the exact source in a versioned S3 bucket and requests two Spot
instances, with a fixed bid, encrypted deletion-on-termination root disk,
outbound-only security group, host-side shutdown timer, and independently
running local termination guard. The host checks its IMDSv2 instance identity,
builds both pinned ARM64 producers, runs the frozen alternating IC/rho schedule,
and uploads raw results and a status archive. `collect_aws.py` downloads exact
S3 object versions and checks archive bytes; `aggregate_aws.py` replays both
strata, binds the EC2 API snapshot to the host receipts, and applies the sealed
selector. Missing, partial, noisy, or failed pairs remain bounded unknown.
The earlier east-region allocation attempt was refused by its placement gate
before any instance was launched. The west-region allocation requires approval
of this revised region and source head. The bid projection is not a guaranteed
account billing ceiling if both deletion guards fail.

```sh
python3 -m research.n83_crossfamily_current_20261025.plan_aws_spot \
  --source-bundle /absolute/path/outside/repository/n83-source-bundle \
  --output /absolute/path/outside/repository/n83-aws-plan.json
# Use a dedicated Python environment with requirements-aws-control.txt.
python3 -m research.n83_crossfamily_current_20261025.allocate_aws \
  --plan /absolute/path/outside/repository/n83-aws-plan.json \
  --source-bundle /absolute/path/outside/repository/n83-source-bundle \
  --output /absolute/path/outside/repository/n83-aws-allocation
# After separate approval, add --execute to allocate two Spot hosts.
python3 -m research.n83_crossfamily_current_20261025.collect_aws \
  --controller-receipt /absolute/path/outside/repository/n83-aws-allocation/allocation.json \
  --output /absolute/path/outside/repository/n83-aws-collected
python3 -m research.n83_crossfamily_current_20261025.aggregate_aws \
  --snapshot /absolute/path/outside/repository/n83-aws-collected/provider_snapshot.json \
  --stratum0 /absolute/path/outside/repository/n83-aws-collected/stratum0/results \
  --stratum1 /absolute/path/outside/repository/n83-aws-collected/stratum1/results \
  --archive0 /absolute/path/outside/repository/n83-aws-collected/stratum0.raw_results.tar.gz \
  --archive1 /absolute/path/outside/repository/n83-aws-collected/stratum1.raw_results.tar.gz \
  --head-sha "$(git rev-parse HEAD)" \
  --output /absolute/path/outside/repository/n83-aws-replay
```

Replay the sealed preparation in a full checkout with Python `blake3==1.0.8`
installed and the archived K3596 dump available:

```sh
python3 -m research.n83_crossfamily_current_20261025.generate_fixture
python3 research/n83_crossfamily_current_20261025/seal_k3500.py \
  /Volumes/data/cryptanalysis/n83_k3596_current_source_base_20261021_1208/base_dump.jsonl \
  --check
python3 research/n83_crossfamily_current_20261025/seal_k3450.py \
  /Volumes/data/cryptanalysis/n83_k3596_current_source_base_20261021_1208/base_dump.jsonl \
  --check
python3 -m research.n83_crossfamily_current_20261025.run --preflight --stratum 0
python3 -m research.n83_crossfamily_current_20261025.run --preflight --stratum 1
python3 -m research.n83_crossfamily_current_20261025.test_select
python3 -m research.n83_crossfamily_current_20261025.test_runpod_provider
python3 -m research.n83_crossfamily_current_20261025.test_allocate_runpod
python3 -m research.n83_crossfamily_current_20261025.test_dispatch_runpod
python3 -m research.n83_crossfamily_current_20261025.source_bundle \
  --build /absolute/path/outside/repository/n83-source-bundle
python3 -m research.n83_crossfamily_current_20261025.allocate_runpod \
  --source-bundle /absolute/path/outside/repository/n83-source-bundle \
  --output /absolute/path/outside/repository/n83-allocation
# Once the source bundle and SSH account are ready, add --execute to allocate.
python3 -m research.n83_crossfamily_current_20261025.dispatch_runpod \
  --allocation-dir /absolute/path/outside/repository/n83-allocation \
  --source-bundle /absolute/path/outside/repository/n83-source-bundle \
  --output /absolute/path/outside/repository/n83-dispatch --stage-and-watch
# After an interrupted watcher, resume it against the same output directory.
python3 -m research.n83_crossfamily_current_20261025.dispatch_runpod \
  --allocation-dir /absolute/path/outside/repository/n83-allocation \
  --source-bundle /absolute/path/outside/repository/n83-source-bundle \
  --output /absolute/path/outside/repository/n83-dispatch --watch
```
