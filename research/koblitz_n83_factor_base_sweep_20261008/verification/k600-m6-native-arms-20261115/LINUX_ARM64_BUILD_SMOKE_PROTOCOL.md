# Linux ARM64 build feasibility smoke, frozen before execution

Status: **engineering feasibility only; no K600 experiment and no runtime
comparison**. The source revision is the clean `HEAD` containing this
protocol; record its full SHA-1 and all command output before invocation.
Inputs are the exact tracked source tree and `Cargo.lock`; use
`--locked`. The host is the local Docker Desktop Linux aarch64 engine
reporting 7,529,156,608 bytes of RAM. The Rust base image is
`rust:1.99-bookworm`, arm64 digest
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`,
with Rust 1.99.1. The smoke output directory and log must be new and
outside the worktree.

Question: can the current clean source compile the five Linux ARM64 Rust
programs needed for the K600 build, using at most 6 GiB container RAM,
four assigned CPUs, two Cargo build jobs and 1,200 wall seconds? The five
examples are `n83_k600_m6_linux_build`, `n83_k600_m6_native_arms`,
`n83_k600_m6_native_preflight`, `n83_k600_m6_budget` and
`koblitz_n83_mixed_orbit_worker`. The source worktree, its shared Git
metadata and alternate object store are mounted read-only at their
original absolute paths; the fresh target directory is mounted at
`/build-target`. Use network access only for locked Cargo dependencies.

The container command is:

```sh
timeout 1200 cargo build --release --locked --no-default-features -j 2 \
  --example n83_k600_m6_linux_build \
  --example n83_k600_m6_native_arms \
  --example n83_k600_m6_native_preflight \
  --example n83_k600_m6_budget \
  --example koblitz_n83_mixed_orbit_worker
```

Before starting, record host disk free and source identity. Run with
`docker run --rm --platform linux/arm64 --memory 6g --cpus 4
--pids-limit 512` and `CARGO_TARGET_DIR=/build-target`.
Success requires exit 0, five output binaries, and Linux ARM64 ELF
headers. Record each file size and SHA-256. Stop on exit, 1,200-second
timeout, container OOM, or host free space below 1 GiB; retain the
failure class and partial log. A success is only a build feasibility
receipt. It does not satisfy the source-bound `charge build` receipt:
the image still needs the solver, executable placement, source label,
frozen plan and measured outer interval. Do not infer relation yield,
rank, or total IC runtime from this smoke.

The local Debian bookworm arm64 package probe before this freeze found
`cryptominisat` version `5.11.4+dfsg1-2+b1`; installing it in an
ephemeral container and running `cryptominisat5 --version` exited 0.
The solver package is a candidate for the later runtime image, not an
input to this Rust build smoke.
