#!/usr/bin/env bash
# Runs inside the RunPod pod (contract of ecc2k130/runpod_job.py): source at
# /work, results to $RESULTS.  Steps: install CuPy, GPU self-test against
# brute force (must pass), n=7 count timing, n=7 end-to-end toy ECDL.
set -uo pipefail
cd /work
: "${RESULTS:=/results}"
: "${TOY_SEEDS:=1}"
: "${MEM_GB:=16}"
: "${STEP_MINUTES:=60}"
: "${TOY_MINUTES:=240}"
mkdir -p "$RESULTS"
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a "$RESULTS/job.log"; }

apt-get install -y -qq python3-pip python3-venv >/dev/null
python3 -m venv /venv && . /venv/bin/activate
pip install -q numpy
pip install -q cupy-cuda13x || pip install -q "cupy-cuda12x[ctk]"
nvidia-smi > "$RESULTS/nvidia-smi.txt" 2>&1
python3 - > "$RESULTS/env.json" <<'PY'
import json, platform, numpy, cupy
d = cupy.cuda.runtime.getDeviceProperties(0)
print(json.dumps(dict(python=platform.python_version(), numpy=numpy.__version__, cupy=cupy.__version__,
                      device=d["name"].decode(), mem_gb=round(d["totalGlobalMem"] / 2**30, 1),
                      cuda_runtime=cupy.cuda.runtime.runtimeGetVersion()), indent=1))
PY
cat "$RESULTS/env.json"
cat SOURCE_REV > "$RESULTS/SOURCE_REV" 2>/dev/null || true

log "self-test (n=5, CuPy vs btn.contract vs brute force)"
timeout $((STEP_MINUTES * 60)) python3 gpu_contract.py --backend cupy --selftest --brute 2>&1 | tee "$RESULTS/selftest.log"
[ "${PIPESTATUS[0]}" -eq 0 ] || { log "self-test FAILED; stopping"; exit 1; }

log "n=7 count, 7-bit cut, R=(40,59), vs brute force"
timeout $((STEP_MINUTES * 60)) python3 gpu_contract.py --backend cupy --n 7 --cut 7 --R 40 59 --brute --mem-gb "$MEM_GB" 2>&1 | tee "$RESULTS/n7_count.log"
[ "${PIPESTATUS[0]}" -eq 0 ] || { log "n=7 count FAILED; stopping"; exit 1; }

rc=0
for s in $TOY_SEEDS; do
  log "end-to-end toy ECDL n=7 r=29 seed $s"
  timeout $((TOY_MINUTES * 60)) python3 run_toy.py "$s" --n 7 --backend gpu --cut 7 --mem-gb "$MEM_GB" \
      --json "$RESULTS/toy_n7_seed$s.json" 2>&1 | tee "$RESULTS/toy_n7_seed$s.log"
  st=${PIPESTATUS[0]}; log "seed $s exit $st"; [ "$st" -eq 0 ] || rc=1
done
exit $rc
