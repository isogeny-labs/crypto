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

## RTX PRO 6000 device validation on RunPod

The owner selected RunPod after the AWS quota block. This run supersedes the
pending device status above; the earlier failed launches and CPU-only records
remain historical evidence. PR #2889 merged as
`21f663511f140bb9c4649e9b98a6678297582063`. The run used its pinned source
`05d292935cc9d74f57d6322f601ee0e1c88e3084`; executable sources in the standalone
service, driver, field header, reference module and tensor dependency are
unchanged in that merge commit.

RunPod pod `09ollojw9j8vbo`, named `explicit-x-pr2889-05d2929-20261029`, was
created at `2026-10-29T23:57:14.413Z` in Secure Cloud `EUR-IS-2`. The account
was reconciled before submission; unrelated pods were not used or changed.
The controller allowed one GPU, a 40-minute worker timeout, and a 45-minute
wall deadline with deletion on completion or failure. The observed compute
rate was $2.49/hour; this is a provider rate, not a billing receipt.

| Identity | Observed value |
| --- | --- |
| GPU | NVIDIA RTX PRO 6000 Blackwell Server Edition |
| Device UUID | `GPU-cb3aa7f7-3a3f-39c2-c3cf-d221e0698fa4` |
| Device memory / compute capability | 97,887 MiB / 12.0 |
| Driver / CUDA compiler | 595.91.07 / 13.2.51 |
| C++ / Rust | Ubuntu GCC 13.3.0 / Rust 1.88.0 |
| CUDA image | `nvidia/cuda@sha256:d266e59b88c295bc5fa0e4cef9064eaff84939381b09e2c3d76a5532a303e42d` |

The image digest was resolved from NVIDIA's public CUDA 13.2.0 development
image. RunPod's live GPU catalog and [official API v2 documentation](https://docs.runpod.io/api-reference-v2/overview)
were checked for GPU identity, CUDA placement constraints, current rate,
pod reconciliation, log collection and deletion. No credential was sent to
the pod or included in the evidence archive.

The worker ran `bash gpu/explicit-relations/validate-rtx.sh` with
`EPHEMERAL_OUT=/results`. The bootstrap, validator and controller all exited
zero. The command compiled and executed the CUDA service on the device.

| Check | Observed result |
| --- | --- |
| `make test`: standalone Rust unit tests | 10 passed, zero failed |
| Host service arithmetic, explicit lifts, signed hash bases and relation tests | Passed |
| Host driver oracle differential, deterministic replay and complete bounded reduction certificates | Passed |
| `make ARCH=sm_120 test-cuda`: device service tests | Passed |
| CUDA driver hash inventories, all-target CPU oracle differential and reduction/certificate replay | Passed |
| Standalone release Clippy with `-D warnings` | Passed |
| `compute-sanitizer --tool memcheck --error-exitcode 86 build/explicit-relations-cuda --self-test` | Passed, `ERROR SUMMARY: 0 errors` |
| Result collection and pod deletion | Archive recovered; provider list confirmed the pod absent |

Raw evidence, launch scripts, pinned request, sanitized pod receipt and
deletion receipt are retained at
`s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/runpod-09ollojw9j8vbo/`.
The local copy is `/home/ec2-user/ecbench-runpod-20261029/`.

| Artifact | SHA-256 |
| --- | --- |
| `results.tgz` | `26ebf7132441afa2be4c0727c8d536d6230d631b4fa53d0ecea1edcffe0ad48a` |
| Raw provider log stream, `logs-final.sse` | `2125c099f49e24475c0215c707b491ad57bf6007229643622a4e2160d26ce872` |
| `validation.log` inside the archive | `050c01dded6a8cb1c44ad9483d5fd5c61586d5ee072c2b47c977d47f8e2764f6` |
| Host service binary | `0510bc8f1bccd81edd730e9deb46b4ebd1eff67423053c7449476f66997f5dd8` |
| CUDA service binary | `76ceaa94a78e5d20f29154cd7767f5b3fa313278ed6006eda1123d5289b4790a` |
| Rust driver binary | `76b782b4d8791ae9878f988ceb7bfb59010f176889a3b38bb5b28d3c35881b7c` |

This is correctness validation on the included public fixtures, not a timed
comparison or an asymptotic result. It does not cover the root crypto library
and ecbench integration suites, sanitizer racecheck/synccheck, wider-field
paper reductions, or a GPU port of the separate tensor backend. The bounded
driver still requires `p <= 65521`, cofactor one and at most 512 columns.
No EKS image was built. The RunPod pod has been deleted; no continuing GPU
service is implied by this successful execution.

## Full recovery frontier, 2026-10-30

The historical validation above used the original tiny driver. The later
source `4014622e18caef9a6b6c89d79653e7633f5aaaff` uses an exact prime-order
witness and Tonelli–Shanks for `p < 2^60`, with explicit column, row and work
limits. It completed the frozen v7 panel on RTX PRO 6000 pod
`ex8tm7votw4h4g` (97,887 MiB, CUDA 13.2.51, driver 595.91.07). All 12 cells
ran: ten independently replayed recoveries and two 60-bit timeouts at
1,200 seconds. Both 58-bit targets recovered in 489.753544246 and
530.119796136 seconds cold. Additional fresh replay and corruption checks
bring their full audit times to 490.031164606 and 530.390733243 seconds.

The 60-bit tables passed conservative memory admission at 57,709,730,784
and 49,236,951,816 bytes before timing out. These are allocation bounds,
not measured peak VRAM; the failures do not establish a hardware maximum.
All 13 Rust tests, host and full/cached/compact device checks, Clippy,
maximum-cap line-protocol headers and full/compact query memcheck,
racecheck and synccheck passed. Worker/controller exited zero, the archive
hash matched and pod deletion was verified.

- [Frozen protocol](../../research/explicit_relations_e2e_20261030/protocol-v7.json)
- [Complete raw export](../../research/explicit_relations_e2e_20261030/results-v7/export.json)
- [Exact cell-coverage audit](../../research/explicit_relations_e2e_20261030/frontier-audit-v7.txt)
- [Archive-member and certificate custody](../../research/explicit_relations_e2e_20261030/results-v7/archive-manifest.json)

Archive SHA-256:
`01cd78dbd5cb64410d0c02ba12473964683299f42702a7c5cb46a8f4b7c37891`.
Earlier v1/v2/v6 setup failures and all v3/v4/v5 outcomes remain in the
same study directory. Schema/flow/file verification passes; four canonical
rectangular-coverage findings remain disclosed for v7's explicitly
nonrectangular plan. Native analysis and the strict audit pass the actual
contract. This is not an independent scientific validation-gate approval.

All wall clocks remain shared-host L0 observations. The GPU relation oracle
is generic meet-in-the-middle decomposition; the separate CPU tensor backend
is not ported here. Root crypto/ecbench was not compiled locally. Later
merged CPU arithmetic and the optional wider prefix index were tested
under separately frozen v8 on source `bbf0f2f51e73afd7e41174094feb2c56a72d3d9d`;
v7 does not validate that later revision.


## Complete wider-index panel

V8 completed all 12 executions, including both 60-bit targets, on pod
`royvvmmehidtwt`. Cold solve times at 60 bits are 770.270416129 and
845.179937170 seconds; full audit totals are 770.303987251 and
845.214672718 seconds. All successful outputs passed fresh replay and
scalar-plus-one rejection. Matched arms produced byte-identical certificates.

All 17 Rust tests, full/cached/compact CUDA checks, Clippy, maximum-cap
headers and full/compact/wide query memcheck, racecheck and synccheck passed.
The wide-index boundary check validates 2^24 exact offsets. Worker/controller
exit zero, archive collection and pod deletion were verified. Archive SHA-256:
`f8924a3418fd268f0f8492f260b8e7fda66616017e1b60fc84488342a98e7d10`.

[Full report](../../research/explicit_relations_e2e_20261030/RESULTS.md),
[v8 export](../../research/explicit_relations_e2e_20261030/results-v8/export.json),
[strict coverage audit](../../research/explicit_relations_e2e_20261030/frontier-audit-v8.txt).
Schema/flow/hashes pass; four canonical rectangular-coverage findings remain.
This validates the recorded source and fixed public fixtures. Later merged
revisions, fields above the driver's p < 2^60 guard, a GPU tensor backend,
and matched rho/fastest external comparisons are not covered by this run.
