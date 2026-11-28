# Deployment readiness for partial dataset artifacts

Read-only inspection on 2026-10-28, source
`c87174cddc02a47da0fdac7312e955db599bd1b2`, found no supported launch contract
that supplies the independent durable campaign authority required by the new
quota implementation. This is not evidence that no suitable volume exists in
the cluster: the caller could not inventory that storage.

| Route | Source evidence | Remaining gap |
|---|---|---|
| Ordinary MCP jobs | `taskq/deploy/eks-karpenter/mcp/src/main.rs:38` exposes no mount/PVC argument; `submit-job:83` hardcodes `/work` and `/tmp` as `emptyDir` | No supported way to attach the campaign authority; ordinary `/work/results` is temporary |
| Portable bridge | `taskq/deploy/eks-karpenter/remote/validate-request:9` admits fixed benchmark fields; `remote/run-request:47` invokes the benchmark method | No native dataset command or durable mount extension |
| CRIU/EBS controller | `taskq/deploy/eks-karpenter/resume/controller/src/cloud.rs:230` creates an 8 GiB gp3 ReadWriteOnce PVC, optionally restored from a VolumeSnapshot, and mounts `/work`; `resume/request.schema.json:6` and `controller/src/contract.rs:97` admit ecbench only | Worker storage can roll back with the checkpoint; it cannot be the independent quota authority, and the command contract does not admit the dataset CLI |
| Existing storage declaration | `taskq/deploy/eks-karpenter/workloads.yaml:73` declares encrypted gp3, WaitForFirstConsumer and Retain | A declaration establishes neither an available campaign PVC nor an attachment route |

The existing Kubernetes principal was
`spot-experiments-WorkerRole-G1FZ84yk3gS6`. Read-only requests to list PVCs at
cluster scope, in `isolab`, and in `isolab-recovery-system`, plus PV and
StorageClass inventories, returned Forbidden. No IAM, RBAC, infrastructure or
job was changed. Ordinary job storage and recovery storage are different
contracts; neither finding establishes a global absence of durable storage.

The deployment must retain the **parent of `authority_dir`**, including its
initialization marker (`src/ml_dataset/budget.rs:284`), and support Unix file
locking plus file/directory synchronization (`:295`). Worker allocations bind
exact journal paths (`:593`). Each admission verifies authoritative state and
commits its reservation there before updating the worker mirror (`:735`). A
replacement needs the same non-rolled-back authority and matching journal; a
lost journal forfeits the allocation. Copying an old journal from S3 cannot
re-create previously spent credit.

Before a dataset launch, identify an approved durable filesystem, expose a
constrained mount through the launch contract, and verify the stable authority
and journal paths from original and replacement workers. For a retained EBS
volume, placement and concurrency must respect its availability zone and access
mode. See [AWS EBS volumes](https://docs.aws.amazon.com/ebs/latest/userguide/ebs-volumes.html)
and [Kubernetes access modes](https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes).
This document proposes no new quota engine or implicit infrastructure purchase.

The [scoped worker S3 policy](../ml_dataset_20261028/worker-s3-policy-proposal.md)
separately awaits human approval. Actual-worker conditional PUT, fresh GET and
hash verification have not run for this source. Byte/request limits do not
replace compute-cost admission against the existing **USD200 total ceiling**.
The [durable-launch problem](../../problems/ml-ecdlp-dataset-durable-launch.md)
retains these acceptance conditions. No additional worker, retry, deadline or
spending allowance follows from passing software tests.
