# Baseline and execution setup

Baseline source is d188c5a6775dff07129084ab2f3f7ae39547c9fc.
Development build: Rust 1.99.1, release, line-tables-only debug info, default
portable target, no target-cpu override. Saved binary SHA-256:
`e522c1c17cddb3eb17d744525b49eb5b44757dfa04a7b99fcff09bfd501a84b7`.
Local path `/tmp/hashed-ecdl-speed-baseline/hashed-ecdl`; NOT_UPLOADED to S3.
This is a preserved development build, not a measured baseline result.

Observed host: Linux x86-64 KVM, AMD EPYC 9V74, five exposed logical CPUs,
one thread per exposed core, 18,882,674,688 bytes reported memory. No measured
isolation, profiler samples, A/A floor, A/B speed ratio or calibration exists.

## Actual unsuccessful setup commands

- `tools/profile.sh probe --out .../profile-capabilities`: exit 2, `needs gawk`.
  perf, valgrind and gawk are absent. No profile was recorded or estimated.
- `apt-get download gawk libsigsegv2`: exit 100, package indexes unavailable;
  apt config permission warning. sudo is unavailable. No privileged workaround.
- `scripts/checkpoint.sh push` and `gate`: exit 1. No AWS profile,
  CHECKPOINT_UPLOAD_TOKEN or configured upload-token transport; nothing uploaded.
- GitHub Actions variable query: HTTP 403, resource inaccessible to integration.
  No EKS worker, S3 result, image digest or lab readiness is claimed.
- The existing GitHub workflow builds ecbench rather than an arbitrary tensor
  runner. It must not be dispatched with the unrelated deployed validation
  fixture as a substitute for the frozen native workload.

User was asked for the name/host of an existing authenticated checkpoint/lab
runner, without requesting credentials. Benchmark and profiling obligations
remain pending. Local implementation, builds and regression controls continue
as development work, without a runtime speedup claim.

## Candidate development build

Same release and line-tables-only settings as the preserved baseline. Saved
candidate binary SHA-256:
`84276e2c508d5889abd101798dd4fd966140332148204ab4d98c556e0b86ac48`.
Path `/tmp/hashed-ecdl-speed-candidate/hashed-ecdl`; NOT_UPLOADED to S3.
The current source identity is pinned in native-sources.sha256. The candidate
build transcript is development-candidate-build.txt. No timed A/A or A/B was run.
Conductor scope check exits 127 because the CLI is unavailable; the repository's
documented warn-and-allow development fallback applies, with no reservation.
