#!/bin/bash
# Launch the ECC2K-130 KernelAgent run on a RunPod GPU pod.
#
#   ecc2k130/kernelagent/launch_runpod.sh [--model ID] [--gpu-type ID] [--out DIR]
#
# Reads RUNPOD_API_KEY from the environment, else from ~/.runpod/api_key (so
# the key need not appear on a command line). FIREWORKS_API_KEY is required in
# the environment: it is handed to the pod with --env and used by
# run_kernelagent.py. Results land in DIR/results (default: a timestamped dir
# under ${TMPDIR:-/tmp}), with launch.json, kernelagent.json, kernel.py,
# bench.json, job.log and KernelAgent's run directories.
#
# KA_S3_BUCKET names the bucket used to move the tree up and the results back
# (default: ecc2k130-<aws account>). The pod's start command carries
# FIREWORKS_API_KEY, so it is visible in the pod's own configuration.
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
E2K="$(dirname "$HERE")"          # .../ecc2k130
REPO="$(dirname "$E2K")"

MODEL="${KA_MODEL:-}"
GPU_TYPE="${KA_GPU_TYPE:-}"
GPU_TYPES="${KA_GPU_TYPES:-NVIDIA GeForce RTX 4090,NVIDIA RTX PRO 4500 Blackwell,NVIDIA L40S,NVIDIA RTX A6000,NVIDIA GeForce RTX 5090,NVIDIA A100 80GB PCIe,NVIDIA RTX PRO 6000 Blackwell Server Edition}"
OUT=""
WORKERS="${KA_WORKERS:-4}"
ROUNDS="${KA_ROUNDS:-4}"
JOB_MINUTES="${KA_JOB_MINUTES:-180}"
WAIT_MINUTES="${KA_WAIT_MINUTES:-240}"
BUCKET="${KA_S3_BUCKET:-}"
IMAGE="${KA_CUDA_IMAGE:-nvidia/cuda:12.8.1-devel-ubuntu24.04}"
CUDA_VERSIONS="${KA_CUDA_VERSIONS:-12.4,12.5,12.6,12.7,12.8,12.9}"

while [ $# -gt 0 ]; do
  case "$1" in
    --model) MODEL="$2"; shift 2 ;;
    --gpu-type) GPU_TYPE="$2"; shift 2 ;;
    --out) OUT="$2"; shift 2 ;;
    --workers) WORKERS="$2"; shift 2 ;;
    --rounds) ROUNDS="$2"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

if [ -z "${RUNPOD_API_KEY:-}" ] && [ -r "$HOME/.runpod/api_key" ]; then
  RUNPOD_API_KEY="$(tr -d '[:space:]' < "$HOME/.runpod/api_key")"
  export RUNPOD_API_KEY
fi
[ -n "${RUNPOD_API_KEY:-}" ] || { echo "RUNPOD_API_KEY is not set (env or ~/.runpod/api_key)" >&2; exit 2; }
[ -n "${FIREWORKS_API_KEY:-}" ] || { echo "FIREWORKS_API_KEY is not set" >&2; exit 2; }

if [ -z "$OUT" ]; then
  OUT="${TMPDIR:-/tmp}/ka-walk-runpod-$(date -u +%Y%m%dT%H%M%SZ)"
fi
mkdir -p "$(dirname "$OUT")"   # runpod_job.py creates OUT itself and refuses an existing one

ARGS=(--job kernelagent/runpod_job.sh
      --extra kernelagent
      --out "$OUT"
      ${BUCKET:+--s3-bucket "$BUCKET"}
      --image "$IMAGE"
      --cuda-versions "$CUDA_VERSIONS"
      --container-disk-gb 60
      --cloud-type SECURE
      --job-minutes "$JOB_MINUTES"
      --wait-minutes "$WAIT_MINUTES"
      --env "FIREWORKS_API_KEY=$FIREWORKS_API_KEY"
      --env "KA_MODEL=$MODEL"
      --env "KA_WORKERS=$WORKERS"
      --env "KA_ROUNDS=$ROUNDS")
if [ -n "$GPU_TYPE" ]; then
  ARGS+=(--gpu-type "$GPU_TYPE")
else
  ARGS+=(--gpu-types "$GPU_TYPES")
fi

exec python3 "$E2K/runpod_job.py" "${ARGS[@]}"
