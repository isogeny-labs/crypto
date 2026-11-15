# GPU pool for Iso Lab on EKS: DRAFT for review

Status: **proposal only. Nothing in this directory has been applied, and no AWS or
cluster call was made to write it.** Applying it widens the lab (a new instance family,
new quota consumption, a new device plugin), so it needs the owner's explicit approval
as a separate administrative action (`.claude/skills/isolab-eks/SKILL.md`, "Select the
submission path"). Reason it exists: the GPU baseline in
`research/gpu_ic_baseline_20261027/PROTOCOL.md` cannot run on the CPU-only pools, and
AGENTS.md §9a forbids silently moving an unsupported workload elsewhere.

## What the draft adds

`nodepool-gpu.yaml`: an `EC2NodeClass` (`gpu`, NVIDIA AL2023 AMI, 200 Gi gp3) and a
`gpu-spot` `NodePool` (Spot only, `g7e.2xlarge`/`g7e.4xlarge`, double taint, limit one
GPU / 16 vCPU / 128 Gi). It leaves `cpu-spot` and `cpu-l2-spot` and the 64-vCPU CPU cap alone.

## What it does not contain, and must be done before it works

1. **AMI id.** The placeholder is the EKS 1.36 AL2023 NVIDIA image; resolve it with the SSM
   command in the file and pin it, as the CPU class does.
2. **Device plugin.** Install the NVIDIA device plugin (DaemonSet) with a toleration for both
   taints, so `nvidia.com/gpu` becomes allocatable. Not drafted here.
3. **Spot quota.** G-family Spot vCPUs have their own service quota in `us-east-2`, separate
   from the 96-vCPU Standard Spot quota the CPU pools share. Its value is unverified. A
   `g7e.4xlarge` is 16 vCPUs; check with `aws service-quotas` and do not request an increase
   implicitly.
4. **Namespace quota.** The `jobs` ResourceQuota in `../workloads.yaml` caps `isolab` pod CPU
   requests and limits at 16 (the lab docs say 64; reconcile that first), memory at 128 Gi,
   ephemeral storage at 80 Gi and pods at 8, and has no `nvidia.com/gpu` entry. A GPU job needs
   `requests.nvidia.com/gpu: "1"` added, and a CUDA build wants more than 80 Gi of ephemeral storage
   headroom to be checked.
5. **Image.** `Dockerfile.ecbench` is CPU-only (aws-cli base, no CUDA). A CUDA 13.3.1 build
   image, digest-pinned from the tested commit, is needed (the Modal route uses
   `nvidia/cuda:13.3.1-devel-ubuntu24.04`).
6. **Submission contract.** `HARNESS.md` and the `isolab.eks.request/v1` schema have no GPU
   field and no GPU admission. A new contract (for example `isolab.eks.gpu/v1`) must define:
   one exclusive device, SM clock/power/ECC trace recorded, no co-resident process, a
   round below 90% of max SM clock invalid, and the receipt/S3 hash checks the CPU path has.
   The CPU L2 and A/A thresholds are not reused or lowered.
7. **Recovery.** CRIU does not cover GPU state. GPU jobs are fresh and non-resumable; a Spot
   interruption invalidates the run.
8. **Cost guard.** One GPU node idles at Spot price until `consolidateAfter: 2m` removes it;
   confirm the `costguard` budget covers it.

## Verification once applied

Schedule a pod that runs `nvidia-smi` and the `selftest` stage of
`gpu/modal/ic_baseline_job.sh`; require the GPU to appear as `NVIDIA RTX PRO 6000 Blackwell`,
CPU pods to stay off the node, and the node to disappear after the job.
