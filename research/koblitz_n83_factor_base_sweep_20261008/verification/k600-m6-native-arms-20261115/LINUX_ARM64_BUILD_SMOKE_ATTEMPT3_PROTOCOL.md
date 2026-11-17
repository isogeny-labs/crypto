# Linux ARM64 build smoke: attempt 3 protocol

Frozen before execution. This is a **warm-target engineering retry** of the
two 6 GiB container OOM attempts. It tests whether the same five Linux ARM64
examples can compile with more memory. It is neither a cold build-cost sample
nor an N83 index-calculus measurement.

## Identity and resources

- Use the clean committed HEAD containing this protocol; record its full SHA-1
  before launch. Source, Git metadata, and alternate object store are mounted
  read-only. Do not fetch or change source during the build.
- Pin `rust:1.99-bookworm` Linux ARM64 image ID
  `sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`.
- Docker Desktop's original memory allocation was 7,424 MiB. It was raised
  through the Resources UI to 14,336 MiB solely for this retry; engine memory
  must be verified before launch and restored to 7,424 MiB afterward. No
  containers were running when the setting changed.
- Limit this container to 12 GiB memory, four CPUs and 512 PIDs. Reuse the
  partial target directory from attempts 1 and 2 at
  `/Volumes/SSD990-2/tmp/n83-arm64-build-smoke-20261115-3ff510d8`.
  Record that cache reuse; its wall interval is not a cold cost.
- Cargo uses `--release --locked --no-default-features -j 1` for
  `n83_k600_m6_linux_build`, `n83_k600_m6_native_arms`,
  `n83_k600_m6_native_preflight`, `n83_k600_m6_budget`, and
  `koblitz_n83_mixed_orbit_worker`. Cap elapsed build time at 1,200 seconds.

## Outcomes and stop rules

Success requires Docker exit 0 and five executable Linux ARM64 ELF outputs;
record each SHA-256 and byte count. Stop and retain the log for container OOM,
timeout, another nonzero exit, or scratch space below 1 GiB. Preserve Docker
events and sampled resource state when feasible. Never count a partial target
or a compiler start as a successful build.

If the smoke succeeds, the source-bound charged Linux build receipt, frozen
runtime image, zero-trial preflight, K600 search arms, rank, and fully charged
cold runtime remain open. Do not promote a factor-base or IC performance claim
from this control. Restore the original Docker allocation even on failure.
