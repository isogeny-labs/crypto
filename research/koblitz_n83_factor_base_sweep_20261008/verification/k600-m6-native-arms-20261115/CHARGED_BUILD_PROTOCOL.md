# K600 m6 Linux ARM64 source-bound build gate

Status: **build-only protocol frozen before execution**. This prepares the
`n83.k600-m6-linux-build-plan/v1` child of the existing native budget
controller. It does not authorize or report a K600 solver arm, relation,
factor-base ranking, or complete IC runtime.

## Question and fixed inputs

Can clean committed N83 source produce the five required Rust programs, a
CryptoMiniSat 5.14.7 solver, and a new source-labelled Linux ARM64 runtime
image under the limits below? The source revision is the clean `HEAD` that
contains [this protocol](CHARGED_BUILD_PROTOCOL.md), the
[plan](CHARGED_BUILD_PLAN.json), and the
[three-step script](CHARGED_BUILD_STEPS.sh); record its full SHA before
running. The build program is `examples/n83_k600_m6_linux_build.rs` from that
same revision. Its three steps and hashes are fixed by the plan.

The prepared Linux ARM64 toolchain image is
`sha256:228b9bea583e6b80fd25b1f74c73419b0808cf751f8e217cd27b3f98080f839b`,
built from the [tracked Dockerfile](CHARGED_BUILD_TOOLCHAIN.Dockerfile) and
the Rust 1.99.1 base
`sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`.
The image supplies Bash, CMake 3.25.1, Docker CLI 20.10.24, GMP headers and
the Rust toolchain. The Bash program digest inside the image is
`f5918390c5b15392ad8835e8368a79c1ee9937fbb19f2349cccf5d4524183299`.
The toolchain image was provisioned before this protocol; its first local
Docker build completed in about 18 seconds. That infrastructure interval is
outside the build producer's child receipt and must be retained separately
in any later fully charged cold comparison.

The three public source archives, downloaded by immutable commit from the
upstream GitHub repositories and pinned by SHA-256 in the plan, are:

| Component | Commit | Archive SHA-256 | Bytes |
|---|---|---|---:|
| [CryptoMiniSat 5.14.7](https://github.com/msoos/cryptominisat/tree/3c8e228e8a48e41276e8ab039f763daa08d61161) | `3c8e228e8a48e41276e8ab039f763daa08d61161` | `2ca8e47c3ab989c3b9842c3f230aca1c6f74a767797a24584c578dee264080a1` | 1,024,575 |
| [CaDiCaL](https://github.com/meelgroup/cadical/tree/818c9562f114b315a9246ced943b66b60b38e8fb) | `818c9562f114b315a9246ced943b66b60b38e8fb` | `beaeb17a751db88d5384b35d65be82501d94cf60f758b768b59bfb2b437cb34b` | 890,753 |
| [CaDiBaCk](https://github.com/meelgroup/cadiback/tree/802e1f6d34b6e613828e591a8359298cd99e6429) | `802e1f6d34b6e613828e591a8359298cd99e6429` | `e9b29931f4d8e12a18eb12b862054af462332cf747be174eb9f7bdfdf4658918` | 25,962 |

The plan uses CMake `FETCHCONTENT_SOURCE_DIR_*` overrides so the solver
cannot silently fetch moving CaDiCaL or CaDiBaCk branches. BreakID and MPI
are disabled. The shell step uses one Cargo or CMake job. The runtime image
starts from the same pinned Rust base, copies only the six built executables,
and labels `org.opencontainers.image.revision` with the source commit.

## Admission, limits, and accounting

Mount the source checkout and Git metadata read-only at their actual absolute
paths, the three archives read-only at `/inputs`, a fresh output parent at
`/build-output`, and the Docker socket for the final image step. Use the
pinned toolchain image, `--platform linux/arm64`, 12 GiB hard memory, no
swap, four CPUs, 512 PIDs, one Cargo/CMake job, and a new Docker image tag.
The source checkout must be clean. The build producer requires a byte-for-byte
committed plan, exact external input hashes, and a new output directory
outside the source tree. Its stage limits are 1,200 seconds for Rust, 900 for
the solver and 300 for the image. Preserve the whole invocation wall as an
outer receipt, including setup, failure and cleanup; the producer's
`wall_to_receipt_start_ms` alone is a lower bound. Stop if scratch falls below
1 GiB, a child times out or exits nonzero, the cgroup OOMs, any hash differs,
or the image label/architecture/ID fails verification. Retain the terminal
receipt and all raw stdout/stderr for every attempted stage.

Success requires Docker exit 0, `OOMKilled=false`, six output-local executable
ARM64 ELF binaries with hashes, a static solver that starts with
`--version`, and a new source-labelled image. The zero-trial hard-cgroup
preflight must then test the actual image and exact K600 S3 input. The
distinct proposed 3,600-second K600 positive/natural execution ceiling and
its controls remain separate; this build protocol does not spend that
search allowance or turn build wall into a measured relation cost.

## Pre-execution checks

Before build launch, verify `bash -n` on the script, JSON parse and schema,
the script/input hashes, toolchain image ID and architecture, the clean
source commit, current host free disk, and that the selected image tag and
output directory do not exist. A failure stops the build before any child.
Afterwards independently inspect the ELF headers, the solver's actual
version and linked libraries, the source label, all hashes and the outer
receipt. Preserve any mismatch as producer failure rather than as a K600
relation-search result.
