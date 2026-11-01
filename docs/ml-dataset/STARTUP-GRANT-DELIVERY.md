# Dataset startup grant delivery options

Design-only inventory, 2026-10-28 UTC. Local source inspected at `009b3342ecbfadcee1e29e88267901cec0823736`; the parent owns the candidate `launch.rs`. No launcher/source changes, cloud queries, new resources, IAM/RBAC changes, grants, or experiment launches accompanied this artifact. The pending IAM question remains open. Source manifests describe intended permissions; this inspection does not verify their live deployment.

**Conclusion:** no supported existing generic route was found that delivers an actual-Pod-UID grant after Pod creation under the current launcher permissions, without bootstrap S3 requests. A minimal reviewed handoff capability is still required. This is independent of the unresolved finite instance-lifetime evidence.

## What exists and what it can do

| Candidate | Local evidence | Assessment |
|---|---|---|
| Existing launcher permissions | `taskq/deploy/eks-karpenter/workloads.yaml:28–38`: Jobs create/get/list/watch/delete; Pods, logs, and events get/list/watch. | Actual Job/Pod identities can be read. No ConfigMap writes, Pod patch/update, or Pod exec/attach delivery. |
| Downward API | `submit-job:74–76` supplies actual Pod UID, node name, and the Job **name**. | Suitable for local identity checks. It cannot obtain arbitrary controller output without a writable metadata/configuration delivery path. |
| Projected Pod annotations | Ordinary manifest has only the do-not-disrupt annotation and no downward-API volume. | No existing post-creation writer permission. Updating Job template metadata would not update the already-created Pod and is not permitted here anyway. Environment variables are unsuitable for later delivery. |
| Shared durable storage | `submit-job:83–84` mounts only `/work` and `/tmp` emptyDir volumes. `workloads.yaml:73–83` declares a gp3 StorageClass. | A StorageClass is not an existing shared volume. No ordinary-launcher PVC mount, externally writable shared path, or verified writer access was found. |
| Native recovery controller | `resume/rbac.yaml:4–21` grants ConfigMap/PVC/Pod writes and exec to `isolab:recovery` in `isolab-recovery-system`. | Separate namespace, authority, and ecbench recovery contract. Not an available generic dataset grant route; do not repurpose it. |
| Worker polls Kubernetes | Generic Pod uses `runner` and disables API-token automount. The existing `harness` account is bound to the broader submitter role. | No implemented reader path. Giving the worker the harness token would expose Job creation/deletion authority and change its trust boundary. Do not use that as a permission-free shortcut or encode grants in auxiliary Jobs. |
| Worker polls S3 | Budget requires exact owner grant before admitted transport activation. | An unmetered bootstrap GET/STS is outside the contract. Neither a public URL nor a copied controller credential solves accounting or identity binding. |

The native status tool correctly selects Pods by the actual Job controller UID (`mcp/src/main.rs:476–494`). A Job created suspended has no Pod UID to bind; suspension does not by itself solve delivery. Deleting and recreating a Job/Pod changes its identity and invalidates the original grant.

## Smallest proposed handoff: a named, preprovisioned ConfigMap

This proposal is **unapplied** and requires explicit review/provisioning. It adds no worker Kubernetes API access, no service, and no bootstrap S3 traffic.

1. For a frozen launch name, an authorized administrator provisions one empty ConfigMap in `isolab`, for example `dataset-grant-<launch-id>`. The existing launcher cannot create it today. Bind a narrow Role to the actual trusted issuer identity with `apiGroups: [""]`, `resources: [configmaps]`, `resourceNames: [<exact-name>]`, and `verbs: [get, update]`. Do not grant namespace-wide create/list/delete or assume resource-name restrictions constrain create. Provisioning a fixed finite set of mailbox objects/roles is possible, but their existence and authorization must be recorded before launch.
2. Extend only the reviewed dataset manifest to project that object's `bundle.json` at fixed read-only path `/run/dataset-admission/bundle.json`, with a missing key allowed while pending. Mount the directory rather than a `subPath` or environment variable so later projection can become visible. Keep worker API-token automount disabled, UID/GID 1000, Restricted security settings, and no exec side channel. Confirm projection/update behavior and namespace admission with an offline manifest test and a separately authorized deployment test before claiming it works live.
3. Reserve bounded provisioning and waiting compute before creating the Pod. Create exactly one Job, retain its response UID, then reconcile the Pod's owner reference, controller UID, actual UID, and admitted node/resource identity. Only then issue the grant from the authoritative campaign ledger. Update the exact ConfigMap with an optimistic resourceVersion precondition, the issued policy/grant bundle, and `immutable: true`. A conflicting existing bundle must match exactly or stop; an ambiguous update is read back, not replaced. Retain the object UID/resourceVersion, bytes/hash, Job/Pod identities, and issuance receipt.
4. The worker gate waits locally for that fixed file and performs the checks below. Kubernetes projection may be delayed: the bounded wait must tolerate absence but terminate on deadline. Projection is transport, not proof of validity; the gate validates the whole bundle before activation.

This keeps mutable-object authority narrower than Pod patching. A ConfigMap contains budget metadata, not AWS credentials. The namespace and trusted issuer remain part of the trust boundary; the proposal does not establish isolation from other namespace principals with workload creation privileges. Administrative review must assess those existing principals. If grant confidentiality beyond that boundary is required, it needs a separate reviewed mechanism.

## Required worker gate behavior

- Add downward-API inputs for actual `metadata.uid` and `metadata.labels['batch.kubernetes.io/controller-uid']`; retain namespace/name for receipts. The controller independently verifies the owner reference before issuing. Fail if either UID is absent or mismatches the grant. Do not trust a user-authored owner JSON file as the origin of these identities. Existing `budget_inputs` currently reads supplied JSON; this check belongs in the launch gate before any activation.
- Pin the expected campaign policy hash and source/binary/plan/protocol/shard bindings in the frozen launch inputs. Require the delivered grant to match those values, the actual downward-API identities, original expiry, approved remote scope, and fixed journal path `/work/budget/worker.sqlite`. Validate the running executable through the existing budget actor check.
- Use a frozen finite maximum local wait and the original absolute campaign/launch deadline, whichever expires first. Measure the local wait monotonically; do not reset it on file changes, malformed JSON, or restart. Use bounded sleeps and bounded file length; absence can wait, but invalid complete input must fail closed. Do not initialize an SDK credential provider, call STS/S3, run a solver, or fetch extra inputs while waiting.
- Read the projected bundle once it is complete and validate it before use. Kubernetes projected volumes use controlled symlink indirection: a blanket no-follow read of the projected leaf may reject a valid mount. Restrict reads to the exact trusted mount, bound bytes, and copy accepted policy/grant bytes into new local files under a fixed protected directory; never accept a path from the bundle. Use `/work/admission/grant.json` as the fixed CLI grant input and derive `/work/admission/owner.json` from actual downward-API values. Create/validate the local directory safely, refuse unexpected existing files, fsync accepted inputs, and retain their hashes.
- Once accepted, changed input is an error. Activate only through the metered native transport, then invoke the exact frozen dataset command with fixed budget input paths. Missing journal/authority state never means a fresh allowance. A replacement Pod needs a separately reconciled grant; loss of the local volume consumes the unresolved allocation and stops work.

## Alternatives and remaining evidence

A Pod-annotation handoff would require a read-only downward-API file plus new `pods` patch/update permission for the issuer. Existing RBAC does not allow it. RBAC cannot be treated as a grant of authority over only one annotation; the precise object scope and permitted patch must be reviewed, and actual Pod names are known only after creation. This is a possible reviewed alternative, not currently available functionality.

An already-mounted shared volume could avoid a new Kubernetes metadata writer **only if** evidence establishes the exact existing volume, same authorized path visible to issuer and Pod, safe file publication, correct ownership, retained authority state, and no new provisioning. None of those prerequisites was established here. No hostPath, new PVC, EFS mount, or recovery-volume reuse is assumed.

The handoff does not prove finite EC2 life, live node type, S3 permissions, or campaign headroom. The earlier [launch adapter contract](LAUNCH-ADAPTER.md) still applies. Required approval would concern the concrete mailbox/manifest and scoped Kubernetes Role proposal; this artifact applies none of them and requests no expansion of AWS IAM.

## Primary local references

- [Ordinary namespace, accounts, RBAC, and StorageClass](../../taskq/deploy/eks-karpenter/workloads.yaml:1)
- [Generic manifest and current mounts](../../taskq/deploy/eks-karpenter/submit-job:58)
- [Actual Job/Pod identity reconciliation](../../taskq/deploy/eks-karpenter/mcp/src/main.rs:471)
- [Existing launcher access group](../../taskq/deploy/eks-karpenter/remote/bridge.yaml:112)
- [Separate recovery privileges](../../taskq/deploy/eks-karpenter/resume/rbac.yaml:4) and [recovery contract](../../taskq/deploy/eks-karpenter/resume/CONTROLLER.md:15)
- [Budget CLI input loading](../../src/ml_dataset/mod.rs:456) and [current actor build verification](../../src/ml_dataset/mod.rs:479)
