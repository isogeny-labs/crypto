# Current EKS submission route audit

2026-10-29. Read-only capability audit for the continuing prime-field solver work.
No job, workflow dispatch, identity, permission change, repository-variable change,
cluster mutation, or infrastructure deployment was performed.

## Conclusion

The old blanket statement “no EKS route” is too broad. There is a merged,
commit-building GitHub route for **native ecbench specifications**, and a separate
ARC Actions infrastructure for approved native research workflows. Neither is a
currently callable, qualified generic command endpoint in this executor.

`isolab.eks.command/v1` is **not implemented** in the inspected current portable
schema, launcher or GitHub workflow. Direct `isolab_k8s_run` does support an argv
and digest-pinned image, but the current tool inventory has no EKS MCP and the
executor has no configured AWS identity or host/in-cluster setup. Loading a CLI or
copying a kubeconfig example would not confer the missing authorization.

A legitimate route forward is to integrate the candidate as a genuine ecbench
method and use the existing source/spec workflow, or use a reviewed, protocol-bound
native research workflow on the already deployed ARC EKS scale set after its live
readiness is established. Do not masquerade a standalone solver as ecbench or alter
OIDC trust/routing variables to make it run.

## Pinned source

Current main inspected through GitHub was:

    21f663511f140bb9c4649e9b98a6678297582063

Files were retained read-only under `/workspace/audit/continuation-route/`:

- `taskq/deploy/eks-karpenter/HARNESS.md`
- `taskq/deploy/eks-karpenter/ACCESS.md`
- `.agents/skills/isolab-eks/references/submission.md`
- `.github/workflows/isolab-experiment.yml`
- `.github/workflows/isolab.yml`
- `contracts/request.schema.json`, `contracts/mcp-tools.json`
- `remote/run-request`, `remote/launch-remote`, `build-bundle`
- `arc/README.md`

Runtime tool discovery found no isolab/EKS MCP. `aws`, `kubectl`,
`isolab-eks-mcp`, and `isolab-eks-recovery` were not on PATH. The documented
harness kubeconfig and in-cluster projected-token paths were absent. These are
observations of this executor, not statements that no authorized machine exists.
The current parent environment status reports no configured secrets, runtime
variables or outbound identities. GitHub repository metadata reports the connected
user has admin/maintain/push permissions, but Actions-variable and registered-runner
inspection return HTTP403 “Resource not accessible by integration.” Those denials
mean live readiness could not be checked through those endpoints, not that the
runners or variables are absent.

## 1. Portable request and GitHub workflow remain ecbench-specific

The schema `isolab.eks.request/v1` has `additionalProperties:false` and accepts:

- `job_name`, `instance_type`, `memory`, `seconds` (maximum3600), `spec`;
- optional `build` containing image digest URI, source revision and ecbench hash.

There is no command, argv or arbitrary executable field. The remote launcher
invokes the benchmark path, and the build helper builds a static musl **ecbench**
executable using Rust1.99.1. A caller-provided provenance SHA is not permission to
replace the code in the deployed image. The optional build object pins a new real
ecbench build, not an unrelated program.

The workflow's dispatch inputs are source_revision, spec_path, size_sweep,
instance_type, seconds, resumable, checkpoint_seconds and max_restores. Its trusted
publisher/experiment job runs only for `refs/heads/main`; feature-branch dispatches
can build but skip the AWS experiment. It retrieves the spec from the exact tested
commit, obtains AWS credentials through the existing main-branch OIDC role, publishes
a digest-pinned image and uses the maintained launcher/recovery client. This is the
prescribed path once the candidate is a genuine supported ecbench method.

ACCESS.md still says PR2342 was unmerged as of October24. That sentence is stale:
PR2342 merged October26 at22:35:56Z, merge commit
`d1f6f50a81734f389c59cad79740dbb436e236e9`. The workflow is now on main. Do not retain
the old premerge status as the present blocker.

## 2. Recent successful workflows were build-only

Read-only run inspections:

| Run | Observed outcome | Scope |
|---|---|---|
|40938439022, Oct29 19:15Z|contracts/build passed; experiment skipped|feature branch `codex/pencil-timing-export-20261029`, attempt1|
|40919051188, Oct29 17:12Z|contracts/build passed; experiment skipped|not live EKS qualification|
|40954647328, Oct29 21:04Z|build failed; experiment skipped|missing pencil path crate before sparse-build fix|
|40962387006, Oct29 22:00Z, current attempt2|contracts/build successful; experiment failed at artifact download|no AWS credential or EKS submission step reached|

PR2881 merged the sparse-build/artifact-pull fix at21:59:49Z, merge commit
`f7e8ea8a645b40c07022a11e30be21f04c180e44`. This addresses the missing path-crate build
failure. Its merge alone does not prove publisher or EKS admission.

The latest artifact failure is precise and narrower than a dead route:

- run40962387006 is now `run_attempt:2`;
- its failed experiment step requested `isolab-build-40962387006-2`;
- the run's retained artifact API returns only `isolab-build-40962387006-1`,
  id12936518230, 76,882,935bytes, not expired;
- the log says “Artifact not found for name: isolab-build-40962387006-2.”

This is consistent with rerunning only the failed experiment job while keeping the
attempt-one build artifact. A fresh source-bound dispatch would upload and consume
its same-attempt artifact and avoid this exact mismatch; it has not been tried in
this audit. Do not automatically rerun the experiment or silently point attempt2 at
attempt1 while claiming fresh source/build identity. Reconcile the original attempt
before any new dispatch under the no-duplicate rule. The failed log is retained at
`continuation-route/dispatch-40962387006-failed.log`.

## 3. Direct generic command MCP exists but is not connected here

The exact current `isolab_k8s_run` input schema supports:

- required job_name and command argv (1..256items);
- optional image (must be digest-pinned by runtime admission), cpu (even2..28),
  memory, seconds and retries.

It submits without waiting, inserts no shell, and uses job_name as retry identity.
Custom images must support UID1000 and upload outputs to the job's configured
results URI. This route can run a standalone native solver **from an existing
properly authenticated harness host**, provided the image/source and scientific
protocol are frozen. It is not callable through the active `tools` list here.

The documented AWS CodeBuild bridge removes the private-Kubernetes routing need,
but it still requires an authorized caller AWS identity. The existing GitHub OIDC
role trusts main-branch Actions, not this arbitrary executor. Creating a new identity,
changing endpoint allowlists or repurposing that role is not routine setup authorized
by merely reading the skill.

PR2853 merged the owner's approval for a future command request kind but explicitly
says an administrator implements it. Exact-string issue/PR search found that approval
PR; no command-v1 implementation was found in the inspected schema/workflow/launcher.
The current artifact and command tools are separate contracts.

## 4. ARC is another EKS infrastructure path, with different admission

`taskq/deploy/eks-karpenter/arc/README.md` documents ephemeral Actions pods on
Karpenter-provisioned Spot nodes, separate from Iso Lab measurement worker pools.
The source declares:

| Scale set | Request | Limit / concurrency |
|---|---|---|
|crypto-eks-linux|2.8CPU,24GiB plus Docker sidecar|28GiB runner, maximum7|
|crypto-eks-highmem|14CPU,112GiB plus Docker sidecar|120GiB runner, maximum1|
|crypto-eks-xmem|14CPU,440GiB plus sidecar|480GiB runner, maximum1|
|crypto-eks-xmem1t|30CPU,900GiB plus sidecar|960GiB runner, maximum1|

These are configured upper bounds, not a live allocation or available budget.
Scientific deterministic counts/degree studies can be implemented as reviewed native
research workflows on these runners. ARC placement itself does not provide L2
hardware isolation or admit a physical runtime comparison.

Owner routing rule: Linux workflows fall back to hosted Ubuntu unless their
validated filenames occur in CRYPTO_EKS_WORKFLOWS or HIGHMEM equivalent; some
specialized registered workflows name scale sets directly. Do not mutate those
variables, runner credentials, IAM or cluster policies implicitly. The same source
says ARC registration predates the repository transfer and gives administrative
rotation/redeployment steps. This audit did not execute any of them.

The recent smoke-run list contains only October20 successes39460365188,
39459423179,39459409233. The current registered-runner endpoint returns403; no
post-transfer live readiness is established by that historical smoke evidence.
A new frozen candidate workflow would need actual observed runner placement,
protocol/control gates, source/image identity, retained failures, checkpoint policy
as requested, and independently verified stored outputs. Falling back to hosted
Ubuntu would not satisfy a claimed EKS run.

An actively maintained example is open PR2890, `twist-relations.yml` native TR04
suite. Its description requires frozen source/protocol/schema gates, existing
artifact credentials, a real checkpoint gate and three crypto-eks-linux shards.
It reports one actual dispatch40973940936 still at initial preflight/checkpoint,
not a completed primary cloud experiment. This is a useful implementation pattern,
not evidence that the current candidate can already run there.

## 5. Unmerged generic command automation is not a deployed endpoint

Open PR2668 (`codex/nist-r08-report-and-filter-experiment-20261026`, inspected head
`6f101fcf212a825afab8b4fcfa1d1e4578c54b55`) adds experiment-queue.yml with eligible
native command jobs, literal argv, a CodeBuild/MCP backend, frozen inputs/protocols
and return-of-results PRs. Its own description states:

- fake GitHub/AWS/native-MCP service tests passed;
- live EKS qualification remains open;
- workflow must land on main before its schedule operates;
- deployment prerequisites and credential readiness remain separate.

It cannot be used as if already deployed. The present task does not authorize
merging unrelated PRs or broadening infrastructure access to activate it.

## Actionable next choices

1. Continue local **development correctness** work, which the skill explicitly
   permits. This can produce a real unknown-coefficient native solver and exact
   finite model-set equality; do not market those checks as admitted scaling data.
2. If fitting the existing native ecbench interface is scientifically appropriate,
   implement a genuine method/spec adapter and use the supported main-branch
   commit/spec dispatch. Preserve both algorithms' complete accounting and the
   required native L2/A-A/replay admission. Do not substitute the integration fixture.
3. For a standalone deterministic degree/count study, freeze a reviewed native
   workflow following the existing ARC research pattern and establish actual runner,
   checkpoint/store and protocol readiness before calling it an EKS experiment.
   The active request authorizes research implementation; administrative changes
   remain a separate boundary.
4. Otherwise the precise missing capability is an already authorized direct
   command-MCP harness connection or the implemented/qualified command request
   bridge. Supplying a command field to request-v1, installing an empty CLI,
   copying credentials, loosening trust, or launching an unrelated existing spec
   does not fill that gap.

No mathematical impossibility or solver-scaling conclusion follows from this route
audit. No experiment is running as a result of this audit.
