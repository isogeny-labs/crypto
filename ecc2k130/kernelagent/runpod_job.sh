#!/bin/bash
# Run KernelAgent on the ECC2K-130 walk step inside an ecc2k130/runpod_job.py pod.
#
# The pod starts from nvidia/cuda:13.3.1-devel-ubuntu24.04 with the tree at
# /work and RESULTS pointing at the result directory that runpod_job.py
# uploads back. Everything this script writes under $RESULTS is returned:
# launch.json, kernelagent.json, kernel.py, test_walk_step.py, bench.json,
# gpu.txt, job.log and KernelAgent's own .fuse/.optimize run directories.
#
# Env (all optional except FIREWORKS_API_KEY):
#   FIREWORKS_API_KEY     required, read by run_kernelagent.py
#   KA_MODEL              Fireworks model id; default: auto-pick GLM 5.3 Flash
#   KA_WORKERS            default 4
#   KA_ROUNDS             default 10
#   KERNELAGENT_COMMIT    pinned KernelAgent commit; default below
set -uo pipefail

: "${RESULTS:?RESULTS must be set by runpod_job.py}"
: "${FIREWORKS_API_KEY:?FIREWORKS_API_KEY must be passed with --env}"
KA_MODEL="${KA_MODEL:-}"
KA_WORKERS="${KA_WORKERS:-4}"
KA_ROUNDS="${KA_ROUNDS:-10}"
KERNELAGENT_COMMIT="${KERNELAGENT_COMMIT:-23a1fb4ebd611e2faeb7effc87eb5b23732039cf}"
VENV=/opt/ka-venv
KA=/opt/kernelagent

exec > >(tee -a "$RESULTS/job.log") 2>&1
echo "== kernelagent job $(date -u +%FT%TZ) =="
nvidia-smi -L

export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq git python3 python3-venv python3-pip curl

# torch cu128 covers sm_120 (RTX PRO 6000 Blackwell) and runs on a CUDA 13 driver.
python3 -m venv "$VENV"
"$VENV/bin/pip" install -q --upgrade pip
"$VENV/bin/pip" install -q "torch==2.8.0" --index-url https://download.pytorch.org/whl/cu128
"$VENV/bin/python" - <<'EOF'
import torch
print("torch", torch.__version__, "cuda", torch.version.cuda,
      "available", torch.cuda.is_available(),
      "device", torch.cuda.get_device_name() if torch.cuda.is_available() else None)
assert torch.cuda.is_available(), "no CUDA device visible"
EOF

git clone --quiet https://github.com/aburan28/KernelAgent "$KA"
git -C "$KA" checkout --quiet "$KERNELAGENT_COMMIT"
"$VENV/bin/pip" install -q -e "$KA"
export PYTHONPATH="$KA"

rc=0
args=(--out "$RESULTS" --workers "$KA_WORKERS" --rounds "$KA_ROUNDS")
[ -n "$KA_MODEL" ] && args+=(--model "$KA_MODEL")
"$VENV/bin/python" /work/kernelagent/run_kernelagent.py "${args[@]}" || rc=$?

bench_rc=""
if [ -f "$RESULTS/kernel.py" ]; then
  "$VENV/bin/python" /work/kernelagent/bench_candidate.py \
      --kernel "$RESULTS/kernel.py" --out "$RESULTS" || bench_rc=$?
fi

nvidia-smi --query-gpu=name,driver_version,memory.total --format=csv,noheader > "$RESULTS/gpu.txt" 2>&1
printf '{"exitCode":%d,"benchExitCode":%s,"kernelagentCommit":"%s","model":"%s","workers":%d,"rounds":%d,"gpu":"%s"}\n' \
  "$rc" "${bench_rc:-null}" "$KERNELAGENT_COMMIT" "${KA_MODEL:-auto}" \
  "$KA_WORKERS" "$KA_ROUNDS" "$(head -1 "$RESULTS/gpu.txt" 2>/dev/null | tr '"' "'")" \
  > "$RESULTS/meta.json"
echo "== done rc=$rc bench_rc=${bench_rc:-none} =="
exit "$rc"
