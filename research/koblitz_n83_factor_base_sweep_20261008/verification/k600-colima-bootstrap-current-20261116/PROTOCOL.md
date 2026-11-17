# Current-checkout K600 producer bootstrap

**Frozen before execution.** The [charged-build retry](../k600-colima-charged-retry-20261116/BUILD_ATTEMPT_1.md) stopped before stages because its earlier bootstrap producer embedded a checkout path absent from the new full-checkout container. This protocol builds a new producer from the exact current full checkout; it is a charged setup run, not a factor-base comparison.

## Hypothesis and inputs

The unchanged `n83_k600_m6_linux_build` Rust example can compile as a native Linux ARM64 ELF with `CARGO_MANIFEST_DIR=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-full-validation-20261116`. The pinned image is `sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573`. The source path, Git common directory and alternate object directory are read-only mounts; the new `bootstrap-20261116-attempt-2` output is outside source. The command is exactly `cargo build --release --locked --no-default-features -j 1 --example n83_k600_m6_linux_build` with a fresh output-local Cargo target. No prior Cargo cache is copied into it.

The [committed launcher](launch.sh) requires a clean checkout and records its source commit. It checks image ID/architecture, wrapper hash, and host boot/SSD/VM-root free-space floors of **15/40/10 GiB**, then starts a new named container with hard 12 GiB memory, zero swap, four CPUs, 512 PIDs, no AWS credentials and no Docker socket. Its `gtimeout -k 20s 5400s` outer cap covers source checks, cold compilation, ELF installation and cleanup. The archive inputs are irrelevant to this producer-only build and are not mounted.

## Success, stop and accounting

Success requires child and Docker exit 0, `OOMKilled=false`, clean unchanged source, executable ARM64 ELF, a recorded SHA-256, a byte search finding the current checkout path in the binary, and successful removal of the named container. A cap, nonzero exit, missing binary, wrong compiled path, memory kill, source drift or cleanup failure is a STOP. Preserve full raw output, terminal receipts, Docker inspect, exact source and a source-linked diagram/PDF for either outcome. The output ELF/hash must be pinned by a separate versioned charged-build admission before use.

This is an additional setup interval. Retain the earlier successful bootstrap, failed admissions and stopped build intervals in cold one-target accounting. It supplies no relation yield, rank, descent, matched rho or total IC runtime. The existing 3,600-second controller allowance remains a separate worker gate after a complete image exists.
