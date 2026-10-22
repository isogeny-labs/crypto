# Queue-scheduled experiments on Spot nodes cannot yet checkpoint and resume

- **Status:** `open`
- **Kind:** implementation gap (taskq worker, envelope compiler); access/tool
  limit (no cluster in the authoring session); untested design (CRIU in a pod)
- **Affected work:** `taskq/deploy/spot-checkpoint/README.md` (the design);
  the `queue-experiment` skill; every throughput, search or training job that
  a protocol routes to Spot capacity with `checkpoint.policy` other than `none`
- **Next action owner:** unassigned (taskq owner lane)
- **Last updated:** 2026-10-22

## Problem

A protocol-bound job submitted to the taskq Spot pool cannot survive a Spot
interruption with its progress, because the worker has no checkpoint
directory, no resume path and no `resumed` accounting: on SIGTERM it hands
the task back and the next attempt starts from scratch, counting an
attempt. The design in `taskq/deploy/spot-checkpoint/README.md` fixes the
contract (route A: application manifest generations; route B: CRIU process
images) and the measurement semantics after a restore, but the worker,
the envelope compiler and the CRIU pool do not exist, and nothing in this
tree has run a checkpoint-and-restore cycle inside a Kubernetes pod.

## Evidence and uncertainty

- **Observed:** `taskq/taskq/worker.py` (SIGTERM hands the task back;
  `request_shutdown`) and `execute.py` (no `TASKQ_CHECKPOINT_*` variables);
  `taskq/README.md` "Not built yet" (no S3 artifact store, no DAG);
  `spotlab/README.md` "Not built yet" (no CRIU: a large dump rarely fits a
  30 s GCP notice); `infra/spot-system/CRIU_LOCAL.md` (CRIU 4.2.1 built from
  source works on AL2023 kernel 6.18 where the packaged 3.17.1 fails on
  `rseq`; dumps freeze the process 0.46 to 1.29 s; restore tested on a
  disposable host with the pool pinned to one family);
  `research/eks_crypto_validation_20261019` (the EKS Spot pool runs real
  ecbench work and persists to S3; no checkpointing).
- **Derived or proved:** not established. The hook script
  `checkpoint-hook.sh` passes `bash -n` and a local validation check of its
  manifest logic; it has not run on the cluster.
- **Suspected explanation:** none needed; the pieces are known and
  unassembled.
- **Missing capability:** worker support for the `checkpoint` spec block
  (directory, store sync, SIGTERM grace, `resumed` accounting, manifest
  validation on claim, `wall_valid` in the result); a compiler from
  `experiment-job/v1` to the backend specs; a `cpu-spot-criu` pool with the
  capabilities CRIU needs and a dump-rate admission check; verification
  that the EKS AL2023 containerd restores a checkpoint image (route C).
- **What this evidence does not establish:** that a CRIU dump of a real
  solver fits inside the 120 s notice on this pool, or that a restored image
  is bit-identical in its counted work; both need the self-check of route B
  run on the cluster.

## What would unblock it

- [ ] `taskq worker` accepts `--checkpoint-dir`, `--checkpoint-store` and
      `--checkpoint-grace-seconds`, exports `TASKQ_CHECKPOINT_DIR`,
      `TASKQ_RESUMED` and `TASKQ_CHECKPOINT_MANIFEST`, forwards SIGTERM with
      the grace, syncs committed generations, hands back without counting an
      attempt, validates the newest generation on claim, and writes `resumed`,
      `checkpoint.*` and `wall_valid` per run in `taskq.task-result/v1`;
      `tests/test_queue.py` covers a kill-and-resume on a demo job whose final
      hash proves continuity (as `spotlab demo-job` does).
- [ ] `taskq submit --job envelope.json` compiles `experiment-job/v1` into a
      task spec by the README's table and refuses a draft protocol.
- [ ] One recorded cycle on the EKS pool: submit, inject an interruption
      (AWS Fault Injection `aws:ec2:send-spot-instance-interruptions`),
      resume on a replacement node, export with `resumed: 1` and
      `wall_valid: false`, `check-export` clean.
- [ ] Route B: a `cpu-spot-criu` pool manifest, a dump-rate measurement on
      a representative solver, and the same recorded cycle with
      `restore_validation: criu_selfcheck`.
- [ ] Route C: a documented attempt to restore a kubelet checkpoint image on
      this cluster's AMI, pass or fail.
