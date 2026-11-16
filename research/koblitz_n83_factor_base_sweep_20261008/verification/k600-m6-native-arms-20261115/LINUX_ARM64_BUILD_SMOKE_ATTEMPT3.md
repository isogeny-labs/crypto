# Linux ARM64 build smoke: attempt 3

**Observed status: sparse-checkout input failure after the library compiled.**
The [precommitted protocol](LINUX_ARM64_BUILD_SMOKE_ATTEMPT3_PROTOCOL.md)
froze a warm-target five-example build with a 12 GiB container cap. This
engineering result does not provide a cold build-cost sample or an N83 IC
runtime measurement.

Source was clean committed HEAD `687f7d3f162b3b3f173db4b825b61e57f4052c88`,
mounted read-only with its Git metadata and alternate object store. The
Linux ARM64 Rust 1.99.1 image ID was
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`.
Docker Desktop's memory allocation was temporarily raised from 7,424 to
14,336 MiB. The container had 12 GiB RAM, four CPUs, 512 PIDs, one Cargo
job, and the partial target from attempts 1 and 2. It started at
**2026-11-15 18:37:17 UTC** and exited at **18:47:10 UTC**, about 593 s
later, with Docker exit 101 and `OOMKilled=false`. The Docker VM was
restored to 7,424 MiB afterward; `docker info` reported 7,529,156,608
bytes. Scratch free space was 2.6 GiB after exit, above the 1 GiB stop
threshold.

The library compiled far enough for Cargo to start an example. One observed
`docker stats` sample was 8.131 GiB of 12 GiB; it is **not** a peak-memory
measurement. `koblitz_n83_mixed_orbit_worker` then failed at three
`include_bytes!` declarations because these tracked files were absent from
the sparse working tree:

- `verification/matched-k1182-k2048-900s-20261011/evidence-01/panel-k1182/manifest.json`
- `verification/remaining-budget-k2048-20261010/manifest.json`
- `verification/mixed-orbit-known-witness-source-20261012/fixture.json`

All three paths are listed by `git ls-files`. None of the five requested
executable outputs was present. The [10,249-byte raw log](linux-arm64-build-smoke-attempt3.log.gz)
has uncompressed SHA-256
`b0bed2c4fc0619abad693c35077645e2dee15c0323e1706169f7d338cf25731f`.
The [Docker state](linux-arm64-build-smoke-attempt3.state.json) records the
non-OOM exit, timestamps and resource caps. Docker events were unavailable
after the settings restart; no event-based claim is made.

The [diagram](linux-arm64-build-smoke-attempt3.svg) and
[PDF](linux-arm64-build-smoke-attempt3.pdf) describe the observed path.
The next bounded check must materialize these three tracked files using
the repository sparse-checkout helper, verify the working tree and their
hashes, and freeze a separate retry. The 1.7 GiB partial target remains a
local cache, not a durable build artifact. A fresh-target source-bound
charged build, runtime image, K600 preflight and search arms, rank,
unseen-target solve, matched rho, and fully charged cold runtime remain
open. No canonical IC performance graph changed because no IC workload ran.
