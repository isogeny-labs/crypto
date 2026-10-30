# Validation record — 2026-10-29

## Initial source handoff

At the initial source-only handoff, no executable, CUDA kernel, Cargo test,
benchmark, image build or EKS job had been run for this change. The table
below preserves that initial evidence; later attempts are recorded separately.

| Check | Observed outcome |
| --- | --- |
| `rustfmt --check --edition 2021 --config skip_children=true` on changed Rust sources and the retained binary-basis fix | Passed |
| `make -C gpu/explicit-relations syntax` | Passed: `g++ -std=c++17 -Wall -Wextra -Werror -Wno-unknown-pragmas -fsyntax-only service.cpp` |
| `clang-format --dry-run -Werror` on the four native source files | Passed, clang-format 23.1.3 |
| `git diff --cached --check` | Passed |
| Existing curve-name checker's `lint_diff` applied to staged additions against `origin/main` | Passed, zero problems |
| Full `python3 scripts/check_curve_names.py --diff origin/main` | Blocked: sparse checkout lacks tracked `.agents/skills/audit-runs/SKILL.md` |
| Rust type checking, Cargo tests and Clippy | Not run; user requested source-only delivery without local plug-in compilation |
| Host executable self-tests, Rust service integration and certificate replay | Written, not run |
| CUDA compilation, GPU differential tests and Compute Sanitizer | Written/documented, not run; no local nvcc or NVIDIA tool available |
| Performance comparison | Not run; no speedup or asymptotic claim |

The C++ command checks the host compilation path only. It does not compile the
`ER_CUDA` branch, validate CUDA barriers, link an executable or run arithmetic.
Rustfmt parses syntax but does not type-check. These checks do not establish
that the change passes CI or is safe to merge.

An initial C++ syntax pass found three misleading-indentation warnings; these
were corrected and the final pass succeeded. An initial whole-file scoped
curve-name scan included unchanged legacy labels in existing modules; the
table records the repository's actual added-line check instead. The full-tree
gate remains pending, without exemptions or source changes to the checker.

Run the build and device checks listed in [README.md](README.md) and the full
repository checks in [the handoff](../../docs/ecbench/explicit-bases.md) on a
builder with adequate disk. Any EKS image must be built from the resulting
`crypto/main` revision and retain its commit and image digest. GPU work remains
a standalone service; its row diagnostics are not CPU GAE measurements.

## PR and RTX launch attempt

[PR #2889](https://github.com/isogeny-labs/crypto/pull/2889) publishes the
change. Source `a14de6b2324f81acd4dc114100bec0534f19d846` includes a standalone
locked Cargo manifest to resolve the tensor re-export introduced on main.
The GPU service still uses the bounded meet-in-the-middle reduction; it does
not accelerate the separate arbitrary-precision tensor crate.

The managed RTX launch `20261029-232943-explicit-x-rtx-validatio-90b272`
requested one `g7e.2xlarge` Spot instance with a 45-minute hard lifetime.
It failed before instance creation with `MaxSpotInstanceCountExceeded`.
The G/VT Spot and On-Demand quotas in us-east-1 were both zero. No GPU
executable ran, and no GPU speed or correctness conclusion follows.

At the owner's request, increases to 8 vCPUs each were submitted in us-east-1:

| Quota | Request | AWS case |
| --- | --- | --- |
| G/VT Spot, `L-3819A6DF` | `d1db611ed1eb4a3abfe04fcee704ff1dH9P2X9B0` | `179331672900750` |
| G/VT On-Demand, `L-DB2E81BA` | `c244a3e3d4bf4b649d32996bdb17b4f8iw9Qfc1A` | `179331672900480` |

Both requests reached `CASE_OPENED`; that is not approval or applied quota.
Recheck their status and applied quota before a new GPU submission. The
future device command is `bash gpu/explicit-relations/validate-rtx.sh`, through
the managed runner on a pinned pushed revision. It checks device identity,
host and CUDA oracles, full bounded reduction/replay, and memcheck.

## First native cloud build

Job `20261029-233215-explicit-x-cloud-build-38a7eb` used a `c7i.2xlarge`
CPU Spot instance, NVIDIA toolkit AMI `ami-0f964da32f9793d2b`, source
`a14de6b2324f81acd4dc114100bec0534f19d846`, Rust 1.99.1 and CUDA 13.2.51.
It passed the host build, all 10 included Rust unit tests, the service
arithmetic/base/relation self-tests, and the driver reduction/replay tests.
`nvcc -O3 -std=c++17 -arch=sm_120 -DER_CUDA -x cu service.cpp` compiled
successfully. This is cross-compilation, not device execution.

The job then failed because the minimal Rust toolchain did not include
`cargo-clippy`. Its final exit was 1. Instance `i-08ecfa8783cc00fc3` was
confirmed terminated. A warning about the included reference's unused tensor
re-export was fixed by forwarding the export at the standalone crate root.
The validation script now installs Clippy, and the shared self-test message
says "service" rather than "GPU" because the same test also runs on CPU.

Raw artifacts are retained at
`s3://ephemeral-runs-104186811093-us-east-1/jobs/20261029-233215-explicit-x-cloud-build-38a7eb/`.
The fetched `out/build.log` SHA-256 is
`66527fd2adffb63cf05d45329fd3220bb02fb351e4bdcadc8d5c94f844e35b22`;
`job.json` SHA-256 is
`7157f7855eed52b6fccd224dead24c2a899826f85c828db6f1e12a7da0dc93dd`.
The initial log's "PASS GPU hash bases" line came from a test running the
host service; it is not GPU evidence.

## Native cloud recheck

Job `20261029-233705-explicit-x-build-recheck-e3f7bc` repeated the build on
source `398d3b291f0c8293423fddb7e2d619cf012d0f6b`, with Clippy installed.
The 10 Rust tests, service and reduction/replay self-tests, and CUDA `sm_120`
compilation passed. Clippy failed on the standalone parser's constant-sized
`chunks_exact(2)`. The fix uses `as_chunks::<2>()` after the existing exact
length check and declares Rust 1.88 as the standalone driver's minimum.
Instance `i-07fca9ca650ae16e3` was confirmed terminated.

Raw artifacts are retained at
`s3://ephemeral-runs-104186811093-us-east-1/jobs/20261029-233705-explicit-x-build-recheck-e3f7bc/`.
The fetched `out/build.log` SHA-256 is
`43a72233723e43dc23dac1bad40568c0eff5a937ec44d411f3755840fa70a88c`;
`job.json` SHA-256 is
`47ee29a4b4c284c0f89a21f6e974b2f5fb675636b1257aa233b5dc40b43bc3e9`.

## Final standalone driver check

Job `20261029-234059-explicit-x-driver-check-17589d` ran source
`42ce1daff18c8f00b3210256e6ff0d5d500b5b60` on a `c7i.2xlarge` CPU Spot
instance with Rust 1.99.1. The managed runner reported `done`, exit 0.

| Check | Result |
| --- | --- |
| Standalone `cargo clippy --locked --release --all-targets -- -D warnings` | Passed |
| Standalone `cargo test --locked --release` | 10 passed, 0 failed |
| Native host arithmetic, lifting, hashes and relation self-tests | Passed |
| Driver all-target oracle differential, deterministic witnesses, full bounded reduction and independent certificate replay | Passed |

The CUDA source files are unchanged from the two successful `sm_120`
compilations above, so this final Rust-only fix did not repeat that compile.
A subsequent Makefile dependency adds the tensor crate manifest to rebuild
tracking; `make -n host` verified the recipe. Subsequent documentation changes
do not change the tested algorithm sources.

Raw artifacts are retained at
`s3://ephemeral-runs-104186811093-us-east-1/jobs/20261029-234059-explicit-x-driver-check-17589d/`.
The fetched `out/build.log` SHA-256 is
`d258a17aba844a8f1a45a8d3ca6df8f4fab26c65b3c410d99b6000439648e106`;
`job.json` SHA-256 is
`fa6e631c67a00f509255f749a8fe3eec6954e30756303853aba8169aff56938e`.
The emitted host binary SHA-256 is
`4f5b815cd22bb37a70e2b849c0724e91d49b8878bedeed4613ceb9aba7362657`;
the driver SHA-256 is
`558d722a2f4924e128d9eba4fc020ad6cfa0a5cf5c910a4bca37ffde5a609096`.

These results cover the standalone service and driver. Root crypto library
tests and ecbench integration tests are separate checks and have not been
run by this cloud build. GPU device tests, Compute Sanitizer and performance
comparisons remain pending. No EKS image or GPU deployment was produced.
