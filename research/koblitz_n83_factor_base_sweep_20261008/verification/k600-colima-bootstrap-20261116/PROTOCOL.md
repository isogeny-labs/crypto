# Source-bound Colima bootstrap compilation

**Frozen before execution.** This is a setup measurement for the N83 K600 m6 Linux build producer, not a factor-base comparison or the charged five-binary/solver build. It implements step 2 of the [Colima build amendment](../k600-colima-charged-build-20261116/AMENDMENT.md). The owner requested a minimum *fully charged cold one-target* IC runtime; bootstrap time and all failures remain in that ledger until a comparison protocol explicitly prices reusable setup.

## Hypothesis and admission

The committed `n83_k600_m6_linux_build` example can compile as a native Linux ARM64 ELF from a clean, source-bound checkout under the qualified Colima host. The pinned image is `sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573`. The launcher records its own source commit, the 5,400-second outer cap, child log/receipt, container inspect, cgroup limits, binary hash, and cleanup. `CARGO_MANIFEST_DIR` must resolve to `/Volumes/SSD990-2/tmp/n83-native-rank-query-wt`. The checkout, Git common directory, and alternate object directory are mounted read-only at their actual absolute paths. The only persistent build output is a new directory under `/Volumes/SSD990-2/tmp/n83-colima-output`.

Run exactly `cargo build --release --locked --no-default-features -j 1 --example n83_k600_m6_linux_build` with a new output-local `CARGO_TARGET_DIR`. The hard cgroup must report 12 GiB memory, zero swap, four CPU quota, and 512 PIDs. The container may use the network for Cargo registry fetches; no AWS credentials or Docker socket are mounted. Before launch require clean Git status, matching image ID/architecture, host boot/SSD/VM free-space floors of 20/80/10 GiB, and new log/output/container names. The launcher script is [launch.sh](launch.sh); its committed bytes, not a chat command, are the run specification.

## Outcome rule and accounting

Success requires child exit 0, `OOMKilled=false`, clean unchanged checkout, an executable Linux ARM64 ELF whose SHA-256 is recorded, no residual named container, and retained raw log and whole-interval receipt. Timeout, failed compile, memory kill, source drift, missing data, or cleanup failure is a recorded failure and stops the charged build. Preserve the raw result as a new result note and a source-linked run report, diagram, and PDF. The setup interval is recorded separately; this run has no relation yield, rank, descent, matched rho, or cold IC `S` to compare.
