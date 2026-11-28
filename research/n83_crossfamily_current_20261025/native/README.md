# Native N83 current-source AWS panel

The native controller packages the exact committed N83 source, checks the
16 sealed public points and four candidate manifests, refreshes AWS Spot
placement and price, and starts two separately identified Tier 1 instances.
Each host executes the frozen alternating IC and same-point rho schedule,
records all child outcomes and resource telemetry, replays arithmetic and
calibration receipts, and uploads its raw archive. The local selector uses
all 16 paired points, six exact sign-flip tests, and a 0.05/6 threshold.

This branch proposes the single two-instance `g5g.4xlarge` Spot allocation in
`us-east-1d` with the replacement security group. It requires separate owner
approval before `aws-allocate` may run. Each instance retains the $0.27/hour
maximum bid and a 50 GB encrypted gp3
root disk, and a 21,600-second lifetime. The bid-based six-hour projection
is $3.37 under the owner's $4.20 allocation cap. The host shutdown timer
and an independently detached local termination guard retain their
outcomes. The projection depends on both guards or provider access
functioning through the deadline.

This proposed native route follows the east-region AWS protocol at commit
`c1b40731907eb7db238186e1b11f58d754572fd0` and the
[2026-11-14 proposed route amendment](AWS_EAST1D_SG_PROPOSAL_20261114.md). The parent
README's west-region alternative is a separate Python dispatch route. This
native run binds its own region, zone, instance type, source hash, and budget
in the controller receipt.

Use Rust 1.99.1 for this isolated crate and a full, clean checkout of the
exact source commit. All output directories belong outside the repository.

```sh
native_manifest=research/n83_crossfamily_current_20261025/native/Cargo.toml
rustup run 1.99.1 cargo run --release --locked --manifest-path "$native_manifest" -- \
  bundle-build . /absolute/path/outside/repository/source-bundle
  aws-plan . /absolute/path/outside/repository/source-bundle \
  /absolute/path/outside/repository/plan.json
  aws-allocate . /absolute/path/outside/repository/source-bundle \
  /absolute/path/outside/repository/plan.json \
  /absolute/path/outside/repository/allocation
```

`aws-plan` is read-only. `aws-allocate` mutates S3 and EC2 and requires
separate owner approval for this exact proposed route. It repeats the
provider price and placement gate immediately before mutation. The
controller receipt and source manifests retain exact SHA-256 values. Do
not put presigned URLs, AWS credentials, or EC2 user data in a PR or log.
The raw result archives and host status objects remain outside the tree
until independently collected and checked.

After both uploads appear, collect exact S3 object versions and request
early termination. Then replay each extracted result independently from
the committed source and select from the local analyses:

  aws-collect /absolute/path/outside/repository/allocation/allocation.json \
  /absolute/path/outside/repository/collected
  replay . 0 /absolute/path/outside/repository/collected/stratum0/results \
  /absolute/path/outside/repository/collected/stratum0/local-analysis.json
  replay . 1 /absolute/path/outside/repository/collected/stratum1/results \
  /absolute/path/outside/repository/collected/stratum1/local-analysis.json
  select /absolute/path/outside/repository/collected/stratum0/local-analysis.json \
  /absolute/path/outside/repository/collected/stratum1/local-analysis.json \
  /absolute/path/outside/repository/selection.json

The selector records `BOUNDED_UNKNOWN` when any required replay, cloud
instance, overlap, calibration, base identity, or pairwise significance
gate fails. The earlier Python protocol remains retained as a versioned
historical design; this native executable is the dispatch path for the
approved allocation.


The owner approved the [west-2b route amendment](AWS_WEST2B_PROPOSAL_20261114.md)
on 2026-11-14 for the same single allocation, superseding the earlier
[`us-east-1a` amendment](AWS_EAST1A_AMENDMENT_20261114.md). Its controller
binds two `g5g.4xlarge` Spot instances in `us-west-2b`, each with a
$0.31/hour maximum bid, 50 GB encrypted gp3 root disk, and a 21,600-second
lifetime. The six-hour bid projection,
including a bounded cross-region S3 transfer allowance, is $4.15 under the
owner's $4.20 allocation cap. The host shutdown timer and an
independently detached local termination guard retain their outcomes.

This native route extends the east-region AWS protocol at commit
`c1b40731907eb7db238186e1b11f58d754572fd0`. The parent README's
west-region alternative is a separate Python dispatch route. The native
controller binds its region, zone, instance type, source hash, and budget in
its receipt. The existing S3 bucket remains in `us-east-1`.



`aws-plan` is read-only. `aws-allocate` mutates S3 and EC2 under the single
approved west-2b allocation. The 2026-11-14 attempt reached EC2 and returned
`Client.Blocked`; do not retry until AWS confirms account launch access is
restored. A retry requires a fresh clean source bundle and plan, plus all
account, source, provider, budget, security-group, guard, and no-overlap gates.
The controller repeats the provider price and placement gate immediately
before mutation. Its receipt and source manifests retain exact SHA-256 values. Do
until independently collected and verified. Each host limits raw upload to
3 GiB and final status upload to 256 MiB; a cap failure is retained in the
versioned status archive.

After both final status uploads appear, collect exact S3 object versions and
request early termination. A missing raw archive remains an explicit failure
in the collection receipt. Then replay each extracted result independently from




This branch proposes the single allocation of two `g5g.4xlarge` Spot instances
in `us-east-1d` using `sg-030ffadf6a62d2ce2` and a $0.28/hour maximum bid
per instance. The zone and bid change requires separate owner approval. Each
instance has a 50 GB encrypted gp3 root disk and a 21,600-second lifetime.
The bid-based six-hour projection is $3.49 under the owner's $4.20 allocation
cap. The host shutdown timer

This native allocation follows the east-region AWS protocol at commit
[2026-11-14 proposed route amendment](AWS_EAST1D_028_PROPOSAL_20261114.md). The parent



owner approval for this exact proposed zone, group, and bid. It repeats the
until independently collected and verified.



proposed single allocation after approval.
