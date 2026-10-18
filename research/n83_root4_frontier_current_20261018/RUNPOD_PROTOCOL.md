# RunPod Tier 1 execution of the sealed N83 root4 reserve

The existing [root4 protocol](PROTOCOL.md), 16 public points, three candidate
manifests, source digest, event order, per-child limits, independent replay,
and exact paired selector apply without change. This adapter records RunPod
allocation identity while the GitHub Actions reserve remains a separate run.

Allocate two new, distinct, simultaneously running Secure Cloud Pods, one
per eight-point stratum. Require at least four visible CPUs, 16 GiB usable
memory plus margin, and 3 GiB free disk in each Pod. Check the actual
`costPerHr` and cap combined Pod spend at $5 and elapsed allocation at 330
minutes. Do not reuse Pods already running in the account. Keep the API key
only in the local controller process; never copy it to a Pod or artifact.
Terminate both Pods after raw results are retrieved or when either cap is
reached. Preserve allocation failures and partial raw results as their own
status rather than filling missing rows.

Before execution, capture one authenticated provider snapshot containing
both live Pod IDs, distinct machine IDs, status, image, actual hourly cost,
and source commit. Give each Pod its corresponding allocation receipt with
the snapshot SHA-256. `runpod_runner.py` binds the receipt to the host and
source records and executes the unchanged sealed runner. It checks the
actual checkout commit, saves the adapter hash, and copies its allocation
receipt into the raw result directory. `runpod_provider.py` independently
checks allocation hashes, source and adapter identity, distinct machines,
and overlapping benchmark intervals. The existing `verify.py` replays each
base, relation, rank, scalar, phase total, resource cap, calibration, and
same-point rho record. Only a complete two-stratum independent replay may
enter the existing exact selector; a partial or noisy result remains
unselected.

The Pod image must supply Python 3, a pinned Rust toolchain, and Git.
Build the same two Rust examples using `cargo test --release --locked` and
`cargo build --release --locked`, retaining toolchain and build logs. Preserve
the raw per-stratum directories, provider snapshot, allocation receipts,
binary digests, run logs, verification logs, and archive SHA-256 values before
termination. The final result table identifies this as Tier 1 RunPod with
the observed topology, cgroup, throttling, steal, and calibration envelope.
