# Automatic CRIU/EBS recovery and fresh L2 qualification

This is an infrastructure/correctness qualification on public synthetic crypto
inputs, not an algorithm speed comparison. The AWS-only native harness submitted
the work; CodeBuild reconciled EKS resources. It is not the pending main-only
commit-building GitHub Action.

## Requirements and observed outcomes

| Requirement | Status and evidence |
| --- | --- |
| Submit from an external harness | Verified: native AWS client, strict versioned request, CodeBuild private EKS route; frozen requests/protocols in `runs/attempts/` |
| CRIU plus coherent EBS snapshots | Verified: periodic stopped-tree dump, encrypted complete snapshot, versioned S3 full readback before generation commit |
| Continue after worker loss | Verified E: original worker terminated, controller restarted, 1,140 checkpointed records continued to 1,806 with byte-identical original prefix |
| Fresh benchmark epoch | Verified E: 1,806 L2 observations, 1,800 native replays, 600 A/A pairs, 19 S3 payload hashes; separate Job/Pod identities |
| Duplicate submission | Verified E: terminal request kept attempt 1, restore count 1 and the original benchmark UID |
| Original deadline and failure gates | Offline native tests cover expired deadline, ambiguous/nonterminal owner, missing/stopped worker, incomplete generation and failed benchmark refusal |
| Corrupt checkpoint archive | Offline actual archive/readback tests reject wrong version/manifest, byte corruption and truncation; live multi-generation fallback not injected |
| Claude use | Shared skill and native CLI documented; Claude symlink validated; lab host MCP/client upgraded and a fresh connection exposed nine tools and read F as succeeded |
| Main GitHub Action | Implemented, pending exact-head CI/review and merges; AWS qualification is not proof of main OIDC dispatch |
| Broader recovery | Not qualified: arbitrary commands, network sessions, GPU/ARM, cross-family restore; fresh non-resumable jobs retain broad x86 Spot placement |

The final runtime retest F passed and is recorded below. It specifically
qualifies dropping root privileges for computation replay as well as the original
payload. E's replay verifier ran as the trusted supervisor; that was corrected
before freezing F. E's restored payload itself already passed UID/GID/group/cap
checks. This distinction is retained rather than rewriting E's evidence.

## Frozen workload and runtime

C used 5 measured rounds and one warmup; E/F used 300 measured rounds and one
warmup so the computation remained active long enough for periodic checkpointing.
Each change was frozen before launch. All use `prime_search` bits 32, curve seed
59297, two public targets with seed 20261002, `rho.negation` reference,
`rho.plain` baseline and a matching `rho.negation` A/A control. Execution order is
alternate, seed 1, per-operation timeout 120 seconds. The logical deadline is an
absolute UTC timestamp fixed at submission and never extended by restoration.
Computation uses the same math spec with L0/perf disabled; every computation
timing sample is invalid. The fresh epoch uses the original L2 spec unchanged.

Crypto source: `cb2ec6873f30b683f7885b53c410d69965f02c20`.
Native binary SHA-256: `583e62f0836eae43330c4b36bbe3d145dacb01de3132436c71f6924445c6a5a9`.
Base OCI digest: `sha256:67eea58f5d4a7a8882b55f089c6063bb49fad58d40260ee9b0ceb349fc0e4581`.
The approved ECR repository is
`104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks`.
Controller/runtime package identities and exact frozen source archives are in
the protocols and `runs/source/`. F uses native source revision
`93bdeb723f4ede560662ad730f7ba8caf1def7a2`; subsequent edits add tests/docs only.

The qualified runtime class is c8a.2xlarge, AMD family 26/model 2/stepping 1,
AL2023 kernel `6.18.54-121.166.amzn2023.x86_64`, containerd
`2.3.5+unknown`, CRIU 4.2.1. CPU feature identity and exact tool hashes are in
the committed generation manifests. Both the original and replacement runtime
must agree. The trusted Pod supervisor is privileged, with no service-account
token or AWS identity. Original/restored computation runs UID/GID 1000, empty
supplementary groups, all capabilities zero, no_new_privs and seccomp. Every
restored process/thread is checked while stopped before continuation.

## Attempt accounting

- **A:** bootstrap failed before any Pod/computation. Atomic replacement of a
  relative filename attempted to open an empty directory path. Both automatic
  controller retries failed similarly. Fixed with a native regression test.
- **B:** cancelled for a controller-package provenance mismatch while deployment
  was finishing. A pending Pod raced controller stop; durable state remained
  provisioning, with no start operation. After STOPPED, state was CAS-fenced
  failed and the owned Pod/PVC removed. No computation began.
- **C:** automatic no-restore path passed. Computation finished before the first
  checkpoint; it proves no recovery. Fresh epoch: 36 L2, 30 native replays, 10
  A/A pairs, 19 S3 hashes. A/A 95% interval:
  `[0.9981475730852468, 1.004390497090755]`.
- **D:** CodeBuild rejected StartBuild at its single-controller limit while C
  was active. No build or workload was created. Frozen input retained.
- **E:** periodic checkpoint, terminal old controller, terminated old worker,
  new controller and automatic restored computation all passed. Fresh epoch and
  duplicate terminal submission passed. E's replay ran under the supervisor;
  F corrects and retests that detail.
- **F:** final runtime passed. It preserved 1,147 records / 5,128,482 bytes and
  completed 1,806 computations after worker loss. The replay verifier was
  observed live as UID/GID 1000, empty groups, all capabilities zero and
  no_new_privs; its audit file is owned by 1000:1000. The distinct fresh epoch
  passed 1,806 L2 / 1,800 native replay / 600 A/A / 19 S3 hashes.

These are separate infrastructure attempts, not selectively retried failed L2
samples. No failed benchmark admission was rerun. The account retained its
96-vCPU Standard Spot quota and the lab its 64-vCPU cap. Transient replacement
capacity/quota lag is included in the original deadline; no other user's worker
was terminated or resized. Fault injection verified all Pods on the disposable
worker were owned by this run, completed, or kube-system DaemonSets before its
one exact EC2 termination. The deployed controller has no EC2 terminate permission.

## Test E identities and readback

| Item | Recorded value |
| --- | --- |
| Logical run | `ecbench-auto-20261021-e` |
| Request SHA-256 | `56d28a98320322ac88ed82b23945a984564ee3501f6d3bce4780c057dcaf10d1` |
| Original controller | `isolab-recovery-controller:d52ac882-c4ac-4ee9-a2af-eafd9008b0f7` (STOPPED) |
| Recovery controller | `isolab-recovery-controller:c688de5e-21d1-4230-a16c-d0f2fca1d392` (SUCCEEDED) |
| Original → replacement worker | `i-0ad9c1eaaaf9033a7` → `i-0ae32d58bdccafdf4` |
| Committed encrypted snapshot | `snap-0af1b03e30d462fce` |
| Checkpoint archive | 45,844,480 bytes; SHA-256 `ab617118bf80e9dd12b37d3abb8ff896d448958eb5ef18306ce87d27a385a442` |
| Preserved prefix | 1,140 records / 5,096,143 bytes; final count 1,806 |
| Restored tree | root PID 531, active child PID 5133; both stopped with the required credentials |
| Fresh Job UID | `f97dce1d-45f5-4b51-bad3-0fa7c123d550` |
| Fresh Pod UID | `dffc2fc4-fb8f-48f7-92a6-e2d79d0e8178` |
| A/A 95% interval | `[0.999546254387097, 1.0006296636908705]`, 600 admitted pairs |
| Duplicate controller | `isolab-recovery-controller:6a325be3-ef12-4ad6-aebe-ccd6bcb7b74e` (SUCCEEDED), unchanged benchmark UID |

Private E result prefix:
`s3://isolab-results-104186811093-us-east-2/runs/ecbench-auto-20261021-e-epoch-1/dffc2fc4-fb8f-48f7-92a6-e2d79d0e8178/`.
Logical receipts: `s3://isolab-results-104186811093-us-east-2/recovery/runs/ecbench-auto-20261021-e/`.
`runs/e/result.json`, `continuation-verification.json`, `duplicate-verification.json`,
the native audits and fresh payload manifest contain machine-readable evidence.

## Reproduction and scope

### Final test F

| Item | Recorded value |
| --- | --- |
| Logical run | `ecbench-auto-20261021-f` |
| Request SHA-256 | `6ade2bcb116b28f74562fa5b10ff5376ecc3094de0b9ceb09a762729d7196eae` |
| Original → recovery controller | `dda8cab6-962b-49bf-bb72-b6c785715fc3` (STOPPED) → `c2efd933-04d8-4c29-bc34-af401f53d854` (SUCCEEDED), project `isolab-recovery-controller` |
| Original → replacement Spot worker | `i-0d2a387e3b5d66249` (us-east-2a) → `i-0bb65e14ae8e48182` (us-east-2c) |
| Committed encrypted snapshot | `snap-04d2dd0bbe427b232` |
| Checkpoint archive | 45,404,160 bytes; SHA-256 `9462b7912c459036178a41abfcd2185264fddd3956e54a2fbbb8c3ebf2956768` |
| Preserved prefix | 1,147 records / 5,128,482 bytes; final 1,806 |
| Restored tree | root PID 531 and active child PID 5161, required stopped credentials verified |
| Final recovery OCI digest | `sha256:5a12f51fa0e287f8024e66d2880549f71af56dfa397c9f0fc2f7f3d7affa221d` |
| Runtime archive SHA-256 | `156139a4e580db9af731136bea7d84e579f42d4580fc8cf90557770fadf7d323` |
| Fresh Job UID | `b8f53045-38a3-4f86-b6a1-ea65107d0053` |
| Fresh Pod UID | `8a5f3da5-38b1-42d0-8ed5-1a51ef71720d` |
| Fresh A/A 95% interval | `[0.9998302217833182, 1.0003545982056818]`, 600 admitted pairs |
| Topology | 8 logical CPUs / 8 physical cores, one sibling per CPU; no SMT |

The original deadline was Unix `1792604440` (2026-10-21 17:40:40 UTC).
The fresh Job completed at 16:59:39 UTC and its controller at 16:59:59 UTC.
Karpenter retained the original request through transient Spot quota and AZ
capacity errors; replacement succeeded in another AZ without operator changes
to the request, workload, scheduling policy, deadline or capacity caps.

Private F results:
`s3://isolab-results-104186811093-us-east-2/runs/ecbench-auto-20261021-f-epoch-1/8a5f3da5-38b1-42d0-8ed5-1a51ef71720d/`.
Logical receipts: `s3://isolab-results-104186811093-us-east-2/recovery/runs/ecbench-auto-20261021-f/`.
See `runs/f/result.json`, `continuation-verification.json`,
`f-live-verifier-status2.txt`, `f-verifier-output-ownership.txt`, native audits,
`topology-check.json` and `s3-independent-hashes.txt`. The full checkpoint archive
and computation archive were downloaded by version and rehashed independently.

### Public evidence bundle

Verify `runs.tar.xz` against `SHA256SUMS`, then extract it beside this report:

```bash
sha256sum -c SHA256SUMS
tar -xJf runs.tar.xz
```

The deterministic archive retains public inputs, native output records, complete
fresh S3 payloads, process credential receipts, frozen injector commands, source
archives and failure evidence. Raw CRIU `.img` files and private checkpoint
archives are excluded. The full private archive versions and hashes are recorded
for authorized readback. Do not upload process memory to Git/GitHub artifacts.

The maintained native client commands and schema are in the
[shared skill](../../../../../../.agents/skills/isolab-eks/SKILL.md) and
[controller README](../../controller/README.md). Run with a new explicitly
frozen request to perform another experiment; do not edit these old deadlines
and reuse the historical run names. The operational injector is a qualification
tool only and checks ownership before termination.

Sixteen native controller tests passed, plus publisher integrity/no-execution
checks. The parent refresh passed 4,474 library tests with 119 ignored, 49 gate
tests, repository validation and harness contracts. The first gate run lacked
tracked sparse-checkout metadata; after materializing it, the checks passed.
Offline failure tests are labelled separately from live EKS evidence.

No canonical performance graph, leaderboard or `benchmarks/` promotion changes:
this run qualifies recovery and admission, not a speedup. The explanatory diagram
and PDF describe those observed transitions. Remaining gaps are tracked in
[the integration problem](../../../../../../problems/eks-resumable-l2-experiments.md).

After evidence collection, the owned C/E/F disposable PVCs were removed. Their
committed encrypted snapshots and private versioned S3 archives were retained.
The temporary qualification cluster-admin grant was removed; the AWS-only client
still read F as succeeded without it. Cleanup receipts are in `runs/checks/`.

The installed client remains the exact qualified runtime binary from source
revision `93bdeb723f`. A later local rebuild after adding only offline tests has
different symbol metadata; stripping both binaries' symbol tables yields
byte-identical executables (SHA-256
`2e47845abe7c9b2c8637930c8159b694dad0a0c4deab58ca060b21a20928004c`).
Both full hashes and the comparison are retained. The rebuilt full binary was
not substituted for the tested S3 runtime package.

The lab host's MCP executable and qualified native client are installed. A fresh
initialized MCP connection exposed all nine tools and read F's successful state.
Existing long-lived connections keep their old process and must reconnect to
discover the two recovery tools; no other client's running process was killed.

The installed MCP `isolab_k8s_resumable_submit` was then exercised with the
identical completed F request after PVC cleanup. CodeBuild
`isolab-recovery-controller:3d7a541e-0acc-4e94-84c0-e0b1f89b6370` succeeded;
request hash, attempt 1, restore count 1 and benchmark UID were unchanged.
`runs/f/mcp-duplicate-verification.json` records that live adapter/idempotence
check. The initial diagnostic omitted MCP initialization and was correctly
rejected before any tool execution; the initialized transcript is retained.

The public evidence bundle and report are also retained under private S3 prefix
`s3://isolab-results-104186811093-us-east-2/recovery/validation/20261021-controller/`.
