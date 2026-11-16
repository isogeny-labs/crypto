# Linux ARM64 build smoke: attempt 4

**Observed status: five executable Linux ARM64 examples built.** This is the
warm-target engineering check frozen in the
[attempt-4 protocol](LINUX_ARM64_BUILD_SMOKE_ATTEMPT4_PROTOCOL.md). It is not
a source-bound charged cold build receipt or an N83 IC runtime measurement.

The clean source HEAD was `45c5a8c49e8aecb3e36ed51bdd9518e0ff89031c`,
mounted read-only with Git metadata. All three tracked JSON inputs named in
the protocol were materialized and matched their frozen SHA-256 values.
The Linux ARM64 Rust 1.99.1 image ID was
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`.
The container reused the partial target of attempts 1–3, had 12 GiB RAM,
four CPUs, 512 PIDs and one Cargo job, and ran
**2026-11-15 18:54:13–18:54:39 UTC**. Cargo reported 26.21 s;
Docker exited **0**, `OOMKilled=false`. Free scratch space after exit was
about 1.8 GiB, above the 1 GiB stop threshold. The original Docker Desktop
allocation of 7,424 MiB was restored and `docker info` reported
7,529,156,608 bytes afterward.

Each of the five [output records](linux-arm64-build-smoke-attempt4.outputs.tsv)
passed `test -x` and `file` identification as an ELF 64-bit ARM aarch64
Linux executable. Their byte counts and SHA-256 are recorded separately
from the source tree. They remain in the local partial target directory;
the smoke did not publish a Docker runtime image or a durable binary bundle.
The [8,539-byte raw log](linux-arm64-build-smoke-attempt4.log.gz) has
uncompressed SHA-256
`7fad77a577738b87bbc10b32b41c45f3ceb219973927c8427dba4aeff1c16617`.
The [Docker state](linux-arm64-build-smoke-attempt4.state.json) records the
timestamps, exit and resource cap. See the [diagram](linux-arm64-build-smoke-attempt4.svg)
and [PDF](linux-arm64-build-smoke-attempt4.pdf).

This shows that the five examples can compile on this host with a warm
partial target, 12 GiB container cap and materialized inputs. It does not
measure a cold source-bound build or establish the minimum required RAM.
The separate `charge build` producer still needs a host-specific committed
plan, fresh target, solver binary and runtime image receipt. Subsequent
zero-trial preflight, K600 pinned/positive/natural arms, rank/logarithms,
unseen-target solve, matched rho and fully charged cold comparison remain
open. No canonical IC performance graph changed because this control did
not execute an IC workload.
