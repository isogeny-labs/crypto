# Native N83 current-source AWS panel

The native controller packages the exact committed N83 source, checks the
16 sealed public points and four candidate manifests, refreshes AWS Spot
placement and price, and starts two separately identified Tier 1 instances.
Each host executes the frozen alternating IC and same-point rho schedule,
records all child outcomes and resource telemetry, replays arithmetic and
calibration receipts, and uploads its raw archive. The local selector uses
all 16 paired points, six exact sign-flip tests, and a 0.05/6 threshold.

The approved allocation is two `g5g.4xlarge` Spot instances in
`us-east-1d`, each with a $0.27/hour maximum bid, 50 GB encrypted gp3
root disk, and a 21,600-second lifetime. The bid-based six-hour projection
is $3.37 under the owner's $4.20 allocation cap. The host shutdown timer
and an independently detached local termination guard retain their
outcomes. The projection depends on both guards or provider access
functioning through the deadline.

This native allocation follows the east-region AWS protocol at commit
`c1b40731907eb7db238186e1b11f58d754572fd0`, which the owner linked in
the spend approval. The parent README's later west-region alternative is a
separate Python dispatch route; this native run binds its own region, zone,
instance type, source hash, and budget in the controller receipt.

Use Rust 1.99.1 for this isolated crate and a full, clean checkout of the
exact source commit. All output directories belong outside the repository.

```sh
native_manifest=research/n83_crossfamily_current_20261025/native/Cargo.toml
rustup run 1.99.1 cargo run --release --locked --manifest-path "$native_manifest" -- \
  bundle-build . /absolute/path/outside/repository/source-bundle
rustup run 1.99.1 cargo run --release --locked --manifest-path "$native_manifest" -- \
  aws-plan . /absolute/path/outside/repository/source-bundle \
  /absolute/path/outside/repository/plan.json
rustup run 1.99.1 cargo run --release --locked --manifest-path "$native_manifest" -- \
  aws-allocate . /absolute/path/outside/repository/source-bundle \
  /absolute/path/outside/repository/plan.json \
  /absolute/path/outside/repository/allocation
```

`aws-plan` is read-only. `aws-allocate` mutates S3 and EC2 and requires
the separate owner approval recorded for this allocation. It repeats the
provider price and placement gate immediately before mutation. The
controller receipt and source manifests retain exact SHA-256 values. Do
not put presigned URLs, AWS credentials, or EC2 user data in a PR or log.
The raw result archives and host status objects remain outside the tree
until independently collected and verified.

After both uploads appear, collect exact S3 object versions and request
early termination. Then replay each extracted result independently from
the committed source and select from the local analyses:

```sh
rustup run 1.99.1 cargo run --release --locked --manifest-path "$native_manifest" -- \
  aws-collect /absolute/path/outside/repository/allocation/allocation.json \
  /absolute/path/outside/repository/collected
rustup run 1.99.1 cargo run --release --locked --manifest-path "$native_manifest" -- \
  replay . 0 /absolute/path/outside/repository/collected/stratum0/results \
  /absolute/path/outside/repository/collected/stratum0/local-analysis.json
rustup run 1.99.1 cargo run --release --locked --manifest-path "$native_manifest" -- \
  replay . 1 /absolute/path/outside/repository/collected/stratum1/results \
  /absolute/path/outside/repository/collected/stratum1/local-analysis.json
rustup run 1.99.1 cargo run --release --locked --manifest-path "$native_manifest" -- \
  select /absolute/path/outside/repository/collected/stratum0/local-analysis.json \
  /absolute/path/outside/repository/collected/stratum1/local-analysis.json \
  /absolute/path/outside/repository/selection.json
```

The selector records `BOUNDED_UNKNOWN` when any required replay, cloud
instance, overlap, calibration, base identity, or pairwise significance
gate fails. The earlier Python protocol remains retained as a versioned
historical design; this native executable is the dispatch path for the
approved allocation.
