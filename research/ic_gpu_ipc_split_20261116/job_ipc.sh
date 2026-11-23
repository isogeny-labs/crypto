#!/bin/bash
# Runs on a Modal GPU container.  /work/ab holds ic-before, ic-main (release binaries), gpu/fes (sources) and
# exp/ (this directory).  Builds the CUDA worker and the null worker, smoke-tests both protocols, runs the arms.
set -uo pipefail
cd /work/ab || exit 1
export DEBIAN_FRONTEND=noninteractive
R=${RESULTS:-/results}; mkdir -p "$R"
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a "$R/progress.log"; }
nvidia-smi -q > "$R/nvidia-smi-q.txt" 2>&1; nvcc --version > "$R/nvcc.txt" 2>&1; lscpu > "$R/lscpu.txt" 2>&1; nproc > "$R/nproc.txt"
nvidia-smi --query-gpu=name,driver_version,clocks.max.sm,power.limit,memory.total --format=csv,noheader | tee "$R/gpu.txt"
uname -a > "$R/uname.txt"; dmesg 2>/dev/null | head -3 > "$R/dmesg-head.txt"; cat /proc/version > "$R/proc-version.txt"
sha256sum ic-before ic-main > "$R/binaries.sha256"
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
log "building gpu/fes for $ARCH"
( cd gpu/fes && make cuda host-worker ARCH=$ARCH > "$R/fes-build.log" 2>&1 ); rc=$?
if [ $rc -ne 0 ] || [ ! -x gpu/fes/fes_cuda ]; then log "BUILD FAILED rc=$rc"; tail -40 "$R/fes-build.log"; exit 3; fi
g++ -O2 -std=c++17 -I gpu/fes -o gpu/fes/null_bin exp/null_bin.cpp > "$R/null-build.log" 2>&1 || { log "null_bin BUILD FAILED"; tail -20 "$R/null-build.log"; exit 3; }
sha256sum gpu/fes/fes_cuda gpu/fes/null_bin >> "$R/binaries.sha256"
( cd gpu/fes && ./fes_cuda > "$R/fes-selftest.log" 2>&1 ) && log "fes_cuda selftest done" || { log "fes_cuda selftest FAILED"; tail -5 "$R/fes-selftest.log"; }
# serve smoke: the text protocol and the binary protocol, CUDA against the host worker
printf '3 2\n0\n1 2 0\n0 0 0 0 0 3\n2 1\n1\n1 1\n0 0 0\n' > "$R/serve-smoke.in"
gpu/fes/fes_cuda --serve < "$R/serve-smoke.in" > "$R/serve-smoke-cuda.txt" 2>&1
gpu/fes/fes_solve --serve < "$R/serve-smoke.in" > "$R/serve-smoke-host.txt" 2>&1
if cmp -s "$R/serve-smoke-cuda.txt" "$R/serve-smoke-host.txt"; then log "text serve smoke: cuda == host"; else log "text serve smoke: DIFFER (stopping)"; cat "$R/serve-smoke-cuda.txt" "$R/serve-smoke-host.txt"; exit 4; fi
python3 - "$R" <<'PY'
import struct, sys, subprocess
R = sys.argv[1]
MAGIC = None
import re
src = open('gpu/fes/fes_io.hpp').read()
m = re.search(r'FES_BIN_MAGIC\s*=\s*(0x[0-9A-Fa-f]+|\d+)', src)
MAGIC = int(m.group(1), 0)
def req(n, mm, cst, lin, quad):
    t = n * (n + 1) // 2
    return struct.pack('<4I', MAGIC, n, mm, 0) + struct.pack('<Q', cst) + struct.pack('<%dQ' % n, *lin) + struct.pack('<%dQ' % t, *quad)
data = req(3, 2, 0, [1, 2, 0], [0, 0, 0, 0, 0, 3]) + req(2, 1, 1, [1, 1], [0, 0, 0])
outs = []
for w in ('gpu/fes/fes_cuda', 'gpu/fes/fes_solve'):
    outs.append(subprocess.run([w, '--serve-bin'], input=data, capture_output=True).stdout)
open(R + '/serve-bin-smoke-cuda.bin', 'wb').write(outs[0]); open(R + '/serve-bin-smoke-host.bin', 'wb').write(outs[1])
print('bin smoke equal:', outs[0] == outs[1], len(outs[0]), len(outs[1]))
sys.exit(0 if outs[0] == outs[1] else 5)
PY
[ $? -eq 0 ] && log "binary serve smoke: cuda == host" || { log "binary serve smoke: DIFFER (stopping)"; exit 5; }
export INSTANCES="${INSTANCES:-koblitz-n23 char2-n25}"
log "arms on $INSTANCES"
bash exp/run_arms.sh "$PWD" "$PWD/exp" "$R/ab" 2>&1 | tee "$R/ab.out"
log "done"
