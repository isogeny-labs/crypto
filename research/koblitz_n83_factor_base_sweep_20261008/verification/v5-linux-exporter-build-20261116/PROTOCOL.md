# Retained K600 V5 Linux exporter build

**Frozen before any build attempt.** This packet builds the exact V5 exporter
source already committed at `cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`.
It is a build and source-admission check, not a relation search or an IC
runtime comparison. A successful binary does not select a factor base.

## Fixed source and host

Run the committed `run-build.sh` from a clean branch whose current head has
no changes from the pinned source commit in `src`, `examples`, `crates`,
`Cargo.toml`, `Cargo.lock`, `gpu`, `tools`, the N83 primary adapter, or the
V2–V5 design JSON files. The launcher copies the materialized source to a
new Colima read-only input path, compares critical Rust and design files to
the checkout, and creates a source attestation naming the pinned commit.
The frozen K600 store manifest and object are not fetched for this build.

Use the already running custom Colima profile at
`/Volumes/SSD990-2/tmp/n83-colima-home`, Linux arm64, and the pinned
Rust 1.99.1 toolchain image
`sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573`.
The source mount is read-only. The container has no network, four CPUs,
12 GiB hard memory, zero swap, 512 PIDs and a 1,800-second outer wall cap.
The host must start with at least 2 GiB available on the boot volume,
4 GiB on SSD990-2, and 10 GiB on the Colima VM root. A host watchdog
samples both host volumes every ten seconds and stops the container if
either falls below 2 GiB. These are build-only floors for a fresh sparse
source copy and output target; they do not amend the separate full-library
validation protocol's larger floors.

The exact child command is
`cargo build --release --locked --offline -j 1 --example koblitz_n83_factor_base_export`.
Record the clean checkout head, copied-source attestation, UTC bounds,
host disk samples, complete build log, child and outer exits, Docker inspect
including OOM status, container cleanup, and binary byte count and SHA-256.
Only terminal exit zero with a Linux arm64 ELF, matching source, no OOM and
clean cleanup passes. Any capacity stop, timeout, OOM, compiler failure,
source drift or incomplete receipt is retained under its actual class.
No retries are included in this packet; changes require a versioned
amendment before another attempt.

The resulting binary remains a local build artifact. Before any solver
run, freeze a separate protocol with the exact S3 input readback, public
fixture, constructed positive control, matched singleton control, cgroup
and wall limits, relation/rank/individual-log accounting, and matched rho.
This build alone establishes none of those measurements.
