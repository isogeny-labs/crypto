# Native ecbench CRIU/EBS qualification

A native crypto session resumed on a replacement c8a.2xlarge Spot worker and
completed all 36 computations. Native replay independently reproduced all 30
measured records. They remain computational evidence at **L0**, with no admitted
benchmark timing. A distinct subsequent epoch passed **36/36 L2 observations,
30/30 native replays, 10 admitted A/A pairs and 19/19 S3 payload hashes**.

This was an operator-controlled qualification. Automatic submission, periodic
checkpointing, recovery ownership and epoch transition are still implementation
work; this report does not certify those missing components.

## Frozen work and observed failures

[Protocols](protocol-v3.json) retain the original deadline of 2026-10-21 06:25 UTC,
exact source/binary/image identity, CPU class and admission criteria. The first
request used prime_search with 36 bits; the deployed harness rejected it before
starting a child because it supports at most 32 bits. That failure remains in
[dump-attempt.txt](dump-attempt.txt). The supported 32-bit request retains two
public seeded targets, rho.negation, rho.plain and matching A/A, five measured
rounds and one warmup.

The normal measured process could not checkpoint: CRIU 4.2.1 rejected an open
`anon_inode:[perf_event]` descriptor ([dump log](dump-v2.log)). The new native
[computation launcher](../../progress/README.md) explicitly forces L0/no CPU
reservation and denies performance-counter creation. It preserves the original
math workload and binds both original and derived specifications by hash. The
normal L2 benchmark path retains its original measurement checks.

That computational process tree checkpointed successfully. Its stopped filesystem
was captured in encrypted EBS snapshot `snap-0ead1fd75c8add942`; all **52** listed
checkpoint/runtime/output hashes matched before restore. Its private versioned
S3 archive passed complete download/byte comparison. Raw CRIU memory is not in
this repository. See [COMMITTED.json](COMMITTED.json) and [verification.json](verification.json).

The original EC2 worker `i-0ab01e249ab5475f7` was confirmed terminated before the
replacement Pod was created. Restore ran on `i-0ab0f5f4afc72d5d6`, in another AZ,
with the same kernel, container runtime, instance type and immutable image.
Karpenter's first replacement request hit a transient quota/capacity failure;
retry succeeded within the existing 96-vCPU account quota and 64-vCPU pool cap.
The parent and active native child continued from the actual CRIU state, and the
session completed with all 36 records verified; all grades were L0.

The initial restore inherited supplementary group 0 from the root supervisor,
although its UID/GID were 1000 and all capabilities remained zero. The failure is
retained in [restore-result.txt](restore-result.txt). A second nonconcurrent restore
from the same committed snapshot, after the first copy completed and its Pod was
deleted, cleared the supervisor's supplementary groups before invoking CRIU.
The restored parent and child then had empty supplementary groups, UID/GID 1000,
zero capabilities, no_new_privs and the computation filter. That copy also
completed all 36 records and passed all 30 native replays; its completed session
and credentials were recovered read-only from the PVC after the Pod expired.
This correction qualifies the tested trusted-process procedure; it does not
establish isolation for untrusted code.

## Fresh measurement epoch

[The frozen fresh-epoch protocol](fresh-epoch-protocol.json) uses the same math
workload and original L2 settings in a separate ordinary Restricted Job. Its
[launch intent](fresh-epoch-launch-intent.json) bounds the Kubernetes deadline to
the remaining original UTC budget with provisioning/receipt margin. No failed
L2/A/A sample was selectively retried.

- Job: `criu-ecbench-20261021-epoch-1`.
- Pod UID: `848af771-ccde-46ce-aef5-7e73e2de357e`.
- Source: `cb2ec6873f30b683f7885b53c410d69965f02c20`.
- Native SHA-256: `583e62f0836eae43330c4b36bbe3d145dacb01de3132436c71f6924445c6a5a9`.
- Image: `isolab/tasks@sha256:67eea58f5d4a7a8882b55f089c6063bb49fad58d40260ee9b0ceb349fc0e4581`.
- All 36 executions earned L2; all 30 measured executions replayed correctly.
- Ten A/A pairs were admitted; the recorded 95% ratio interval was
  `[0.9997380265281529, 1.0011447799716735]`.
- All 19 S3 payload hashes and actual Pod image/build identity passed verification.
- Results: `s3://isolab-results-104186811093-us-east-2/runs/criu-ecbench-20261021-epoch-1/848af771-ccde-46ce-aef5-7e73e2de357e/`.

This is the deployed pinned native image through the maintained AWS/CodeBuild
harness. It is not PR #2342's pending full main-branch commit-building Action.
No algorithm speed comparison or leaderboard promotion follows from this
infrastructure qualification. Existing performance graphs are unchanged because
no performance claim is made. The diagram and PDF report describe this run.

The small native session trees and payload files remain loose because their
session manifests, SHA256SUMS and replay verification bind individual paths.
Readback checks can therefore be repeated in place. Raw CRIU memory archives
remain private S3 objects and are excluded from Git.
