# Native command recovery admission

This component prepares the requested 60,000-accepted-curve baseline. It does
not launch a sample, publish an image or turn a development control into a
production receipt. Keep the October 29 design and old proof subjects intact.

## Maintained route

The current controller implements native commands under
`isolab.eks.resumable/v1`. Read the actual examples and command-mode contract:

- `taskq/deploy/eks-karpenter/resume/request.command.example.json`
- `taskq/deploy/eks-karpenter/resume/runtime.command.json`
- `taskq/deploy/eks-karpenter/resume/CONTROLLER.md` (Command mode)
- `.agents/skills/isolab-eks/references/recovery.md`

These were inspected at main commit
`85ee54f57262767f20caf817f7a31ff1254dfa8e`. The command runtime archive is pinned
to SHA-256 `be4a5aeaa5e005699069127d9f302667bc838b8e3bc72292d5f14f44868734ea`;
its own qualification field says NONE, with the live command CRIU qualification
pending. An ecbench restore receipt is not a native-payload qualification.

The command payload's argv starts `/opt/nist/bin/run-field`, followed by mode,
one frozen field, the frozen nonce and `ACK_FROZEN_CHECKPOINT`. The legacy
`build.ecbench_sha256` field binds **argv[0]** in this documented command mode;
it does not mean that ecbench was used. Bind the sampler and journal reader in
`command.binaries` as well, together with the exact source and OCI digest.
`spec` is absent, output is a normalized directory under `/work/compute`, and
the original finite absolute deadline and restore allowance must be frozen.

The supervisor supplies no AWS identity to the payload. CRIU/EBS restores the
existing process tree and volume; this launcher adds no automatic restart or
replacement nonce. The payload runs the producer once and then the separate
full-journal reader, including all mapped rejected curves. Operational errors
stop and preserve partial output. The supervisor's success is not sufficient:
inspect `command_exit_code`, `command-result.json`, request/source/image hashes,
the sealed output manifest and S3 versions/full readback. A resumed count can
remain valid while its interrupted clock is invalid.

`resume/publish-runtime` currently selects `runtime.json`, not
`runtime.command.json`. Do not silently publish the old runtime for this payload.
A reviewed explicit command-runtime selection/composition and actual source-image
receipt remain necessary. The existing main OIDC experiment workflow builds
ecbench; there is no native NIST baseline dispatch workflow in the inspected
main. Never substitute its integration fixture or expose Actions variables.

## Required before spending or drawing a study nonce

| Requirement | Current state / next step |
| --- | --- |
| Full-run total dollar cap | Pending one numeric question; no selected default is an answer |
| Shared budget and capacity | Read authoritative charged, committed, reserved daily/monthly headroom, actual pools/quota and current jobs |
| Existing authentication | Local normal-chain STS says Unable to locate credentials; evaluate reviewed main OIDC route without new keys or permission changes |
| Uploaded study checkpoint | Local upload token absent; use existing authorized authentication, inspect an allowlisted payload and require canonical gate success |
| Exact image and runtime | Publish reviewed source first, build matching payload and command runtime, retain OCI/binary/source identities |
| Worker-loss restore | Separate funded qualification on the actual native payload; preserve original deadline, journals and all failure receipts |
| Six-field cost pilot | Ten accepted per field, separate nonce, at most two requested CPUs and 3600 seconds per field, no automatic retries; retain failures and all attempts |
| Primary freeze | After pilot, publish native analysis/export adapters and a canonical production execution amendment with finite resources, dollar cap and original absolute deadline, before the primary nonce |
| Final results | Independent full-journal replay, all six exact target counts, ten tied percentiles/uncertainty/Holm outputs and every descriptive cutoff 100–5000 |

The existing blocked backlog card stays blocked and unfunded. Acknowledgement
strings and environment markers are checks on the interface, not evidence that
these external gates passed. A local image config digest is not an ECR manifest
digest or a CRIU compatibility receipt. No runtime or production estimate is
invented from development control duration.

The root `.resume/RESUME.md` belongs to another study and stays untouched. A
future checkpoint must use an isolated, explicitly inspected custody payload;
never include the denied older 39-member auxiliary paper archive, repository
secrets or unrelated process-memory images. CRIU images stay in private encrypted
recovery storage. This document neither uploads nor approves excluded material.
