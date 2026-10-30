# Proposed offline dataset startup deployment inputs

Status on 2026-10-29: **templates only; no deployment, grant, image build, permission change, or generated cell.** The startup module at SHA-256 `625c08c913d63cbe33598086baff8bb142afcc0dc8dc44cc683b58be118233aa` was written and independently source-reviewed. Its CLI registration and centralized runtime validation remain separate prerequisites. None of the historical `9c3d14b` validation proves this new gate or these templates works in Kubernetes.

This directory prepares the existing [startup contract](../../../docs/ml-dataset/STARTUP-GATE-PROPOSAL.md), [delivery proposal](../../../docs/ml-dataset/STARTUP-GRANT-DELIVERY.md), and [launch adapter contract](../../../docs/ml-dataset/LAUNCH-ADAPTER.md). The [durable launch obstruction](../../../problems/ml-ecdlp-dataset-durable-launch.md) remains open. The full requested corpus and the frozen 720-cell pilot remain intact; this proposed first invocation is at most 36 cells. Unsupported and censored outcomes are not solved instances.

All `__PLACEHOLDERS__` are unresolved, including numeric values represented as strings in JSON. The templates deliberately cannot be passed directly to the native gate or deployed as an admitted Job. Rendering must replace numeric placeholders with JSON integers, preserve field types, reject every remaining placeholder, and retain the exact rendered bytes. No renderer, grant issuer, Kubernetes client, or launch automation is added here.

## Files and fixed wiring

| File | Purpose |
| --- | --- |
| `startup-gate.json.template` | Native typed gate input; fixed pilot ceilings, unresolved policy/reservation/deadline and six-coordinate cell allocation. |
| `Dockerfile.proposed` | Separate recipe for the newly tested binary and unchanged frozen plan/protocol at the gate's required paths. No image built here. |
| `job.template.yaml` | One bounded Job, explicit image digest and instance/resource placeholders, actual downward identities, two different read-only mounts, and private per-Pod writable work volumes. |
| `startup-config-object.template.json` | Immutable configuration object created before the Pod; its one exact file is mounted using `subPath`. |
| `mailbox-provisioning.template.yaml` | Unapplied administrator-only empty mailbox and exact-object issuer `get/update` permissions. It grants no worker Kubernetes API access. |
| `mailbox-delivery-update.template.json` | One optimistic update of the existing mailbox, with actual UID/resourceVersion, complete bundle bytes, and `immutable: true`. |
| `observed-owner.template.json` | Shape of real Job/Pod ownership evidence required after creation; not a fabricated owner or grant. |
| `source-inputs.json` | Exact local source/input hashes and frozen limits used while drafting; no claim of deployed image identity. |

The old [Dockerfile](../Dockerfile), [pilot plan](../pilot-plan.json), [protocol](../protocol.json), and historical image/evidence directories remain unchanged. The old image path `/opt/ml-dataset` does not meet the new gate's `/opt/crypto/dataset` contract. The proposed build context contains the actual tested `crypto`, `build-info.json`, the original `pilot-plan.json` copied byte-for-byte as `plan.json`, and the original protocol. Build arguments must be actual source/binary/file digests, not the template module's source hash. The final image must be digest-pinned and its executable independently checked against the reservation's binary SHA.

The image creates an invalid sentinel at `/opt/crypto/dataset/startup-gate.json`. An **already immutable, pre-provisioned** configuration object replaces that exact target with a read-only single-file `subPath` bind mount. It must never be used for mutable or late configuration. Plan/protocol remain ordinary root-owned image files. The launcher must pin the configuration object's identity and exact file SHA in its retained observation and manifest. An annotation records a claimed hash; it does not itself verify the bytes.

The late bundle mounts the complete optional ConfigMap directory at `/run/dataset-admission`, with no `subPath`. Initially only the `bundle.json` key may be absent. The exact object must already be provisioned and reconciled; `optional: true` permits waiting for the key, not an assumption that provisioning is authorized. The gate accepts confined `..data` symlink rotation using `openat2`. It rejects a normal projected symlink leaf for the *frozen* config/plan/protocol reader, which is why those inputs use a different mount strategy. Kubernetes documents that directory-mounted ConfigMaps update eventually and `subPath` mounts receive no updates; `immutable: true` prevents subsequent content changes. The 300-second gate wait is a configured ceiling, not a kubelet latency guarantee. [ConfigMaps](https://kubernetes.io/docs/concepts/configuration/configmap/), [optional volume references](https://kubernetes.io/docs/tasks/configure-pod-container/configure-pod-configmap/).

Both ConfigMap files use decimal mode 292 (`0444`). The gate additionally checks actual read-only mount flags, regular-file type, expected owner, one hard link, and protected ancestors. The new image provides ordinary root-owned `/opt/crypto/dataset` and `/run/dataset-admission` parents, and the Pod runs UID/GID 1000 with a read-only root filesystem. Actual kubelet/runtime ownership and mount flags, Linux `openat2`, seccomp allowance, and single-file mount behavior still require deployment validation; YAML alone does not establish them. [Pod security context](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/).

`/work` is writable `emptyDir` with the approved group ownership. The gate creates `/work/admission` and `/work/budget` itself as UID-1000-owned mode-0700 directories. Do not precreate them with generic group-writable modes or mount another volume over them. Budget state stays at `/work/budget/worker.sqlite`; dataset output stays at `/work/dataset`. The work and tmp size limits are unresolved approved values. Kubernetes volume/resource limits and budget `local_bytes` allocations are not a claim that every local write is synchronously capped or that all intermediate artifacts survive OOM or Pod deletion.

## Binding and delivery order

The following describes future authorized operator work; no commands were executed for these steps.

1. Finish gate/CLI validation and build the actual binary/image. Record clean source, executable SHA, image digest/architecture and the exact frozen input bytes. The image is independent of per-launch configuration; this avoids a cycle where a config embeds a reservation hash while its image digest is embedded in the reservation's resource evidence.
2. Reconcile prior campaign obligations, permitted us-east-2 hardware, whole-resource finite lifetime, pricing and remaining capacity. Reserve the full startup/execution/termination envelope with native `budget reserve-launch` before creating billable work. Preserve the authoritative allocator and the exact ownerless reservation. Use `/work/budget/worker.sqlite` in the reservation. Do not invent a fresh policy, zero prior obligations, copy a spendable journal, or assume earlier estimated headroom is current billing.
3. Render the gate configuration using the actual native policy and reservation digests. Freeze its exact bytes. The configuration's startup deadline must be no later than both grant and policy expiry; include provisioning and delivery delay in the admitted lifetime. Create/reconcile its immutable ConfigMap and the separate empty late-bundle mailbox under approved administrator authority. The issuer's proposed exact-mailbox role permits `get/update` only; it cannot create either object and cannot modify the immutable configuration object.
4. Render the exact Job using the reservation's name/namespace, image digest, approved instance type, and resource limits. Frozen pilot placement is two CPUs and 4 GiB, with one native solver thread; filling resource placeholders must preserve that reviewed protocol or use an explicitly amended future protocol. Keep at most 36 cases, 30 seconds per cell, 2048 MiB address-space limit, 64 MiB trace cap, and the 3600-second Job deadline. These limits do not price the EC2/node billing tail. Submit exactly once and reconcile an ambiguous response under the same Job name.
5. Retain the actual Job response UID; reconcile the actual Pod UID, controller owner reference, controller-UID label, namespace, admitted image/container arguments, node/provider identity, capacity type and finite lifetime. `POD_UID`, `JOB_CONTROLLER_UID`, `POD_NAMESPACE`, and `POD_NAME` come from native downward-API fields. Never substitute a Job name for its UID or prefill the controller-UID label. These are consistency inputs, not independent attestation. [Downward API fields](https://kubernetes.io/docs/concepts/workloads/pods/downward-api/).
6. Bind the existing reservation once using the actual owner and retained observation evidence through native `budget bind-launch`. Compose one `crypto.ml-dataset.startup-bundle/v1` object with exact parsed `policy`, `reservation`, accepted `binding`, and issued `grant` objects. The bundle must fit 256 KiB. Do not copy CLI wrapper fields into the typed objects, change allocations, manufacture a grant, or replace an observed owner.
7. Serialize that complete bundle exactly once into the mailbox update's JSON string. An authorized issuer updates the pre-provisioned object with its fresh UID/resourceVersion precondition and sets `immutable: true` in the same operation. Preserve unrelated existing metadata. On conflict or ambiguous update, read back the same object; equal content is recoverable evidence, differing content stops. Never delete/recreate the mailbox to repair ambiguity. Record exact bundle SHA, object UID/resourceVersion, grant/binding hashes, and retained response.
8. The worker gate validates the delivered bytes before creating its budget journal, calling STS/S3, or starting a solver. It then uses the existing metered native activation/generation handlers. Retain actual worker conditional PUT/GET/hash, completed-cell verification, accounting and independent restore/replay evidence before counting a verified cloud cell.

The owner file and delivered policy/grant/config/observation records are restricted operational provenance. They are not public ML features, do not belong in the structural training export, and contain no AWS credentials or web-identity token bytes.

For the RoleBinding template, a `ServiceAccount` subject uses `apiGroup: ""` and its actual namespace; a `User` or `Group` subject uses `apiGroup: rbac.authorization.k8s.io` and omits the namespace field. Choose only the approved actual issuer. Immutability does not establish object identity or protect against delete/recreate by another namespace principal: retain UID/content readback and the reviewed namespace/issuer trust boundary for both objects. This preparation creates no such trust or authorization.

## Budget arithmetic and credentials

For every coordinate of `Resources`, require checked arithmetic:

```text
36 * cell_envelope <= grant.resources - grant.finalization
```

Coordinates are money in micro-USD, unique output bytes, PUT attempts, read attempts (GET/HEAD, plus admitted STS), download bytes and local bytes. These are envelopes; they are not observed billing or generated artifact measurements. Retain the real native dry-admission output and allocator snapshot before resource creation. Every retry and repeated finalization remains charged; no automatic refunds exist.

The current gate's total protected finalization precheck reserves the generator's remaining floor plus initial activation headroom. IRSA reserves two bootstrap responses because activation and generation use separate contexts:

| Minimum total coordinate | Explicit static credentials | Role-bound IRSA used by this template |
| --- | ---: | ---: |
| Output bytes | 69,632 | 69,632 |
| PUT attempts | 3 | 3 |
| Read attempts | 5 | 7 |
| Download bytes | 266,240 | 397,312 |

These are **admission floors**, not recommended final allocations or guarantees against retries, credential refresh, checkpoint/catalog size, accumulated finalization use or incomplete trace retention. The actual approved allocation must additionally cover its work and publication behavior. Money/local-byte constraints and positive compute pricing still come from the full policy/grant validation. No arbitrary resource grant is supplied here.

The Job names an unresolved approved IRSA service account and expects its admitted Pod to contain the actual `AWS_ROLE_ARN` and `AWS_WEB_IDENTITY_TOKEN_FILE` pair from the EKS admission path. Verify the pair, role trust, audience, token readability and granted destination scope against the issued grant before delivering it. `automountServiceAccountToken: false` prevents the ordinary worker API-token mount; it does not prove whether the separate IRSA projection was installed. If admission omits that pair, the current transport fails after local admission; the gate does not synthesize credentials. The Kubernetes webhook/token transport and IAM permissions remain deployment prerequisites. [AWS IRSA](https://docs.aws.amazon.com/eks/latest/userguide/iam-roles-for-service-accounts.html).

No static AWS keys, `envFrom`, endpoint override, proxy override, alternate bucket, or shell entrypoint is included. The native transport rejects nonempty HTTP(S)/ALL proxy and AWS endpoint overrides. The retained base image's inherited environment and admission mutations therefore need inspection; they are not assumed clean from this template.

`lab_receipt_prefix` is null and `ISOLAB_RESULTS_URI` is absent. Do not reuse the generic launcher's old-bucket receipt environment. If a separately reviewed configuration enables a receipt prefix, the manifest value must be exactly the approved slash-terminated prefix + reserved Job name + `/$(POD_UID)/`, with `POD_UID` declared earlier. Its requests need budget capacity and policy scope before activation.

## Recovery and unresolved deployment boundaries

`restartPolicy: Never` and `backoffLimit: 0` disable configured automatic retries. They do not prove at-most-one Pod or eliminate controller races; reconcile actual owners and account for every provisioned resource. A duplicate/replacement Pod cannot use a bundle bound to another Pod UID. Do not add automatic `--resume` arguments or a retry shell.

Retained same-owner, same-host authority can use explicit native `start --resume`: an active budget advances without another activation call; pending activation uses admitted recovery; `generation_started` without a terminal record stops as unknown. The proposed `emptyDir` state does not survive Pod deletion or node loss. A new Pod/host, missing original journal, or copied audit checkpoint cannot resume spending. This is not a shared-volume, CRIU or cross-host authority-recovery design.

Outstanding concrete prerequisites are CLI integration/runtime validation; an actual matching image; trusted config/subPath and late-mailbox mount validation; approved exact-object issuer permissions and real owner observation; approved IRSA/worker S3 behavior; current cumulative capacity and independently finite resource lifetime; and a first worker publication plus independent replay. The generic `submit-job` currently lacks this config/mailbox wiring and supplies incompatible legacy receipt environment. These templates do not silently modify that launcher or establish an approved replacement.

## Preparation record

Conductor `check --scope path:research/ml_dataset_20261028/startup-deployment-proposed` returned clear for this offline task under parent T770. Sources checked: the startup module and proposal, native budget/S3 transport, frozen pilot plan/protocol, old Dockerfile, generic `submit-job`, `workloads.yaml`, and the official Kubernetes/AWS documentation linked above. No new mathematical claim or experiment is introduced. Parent owns final review, session-log integration, commits and artifact checkpoints.
