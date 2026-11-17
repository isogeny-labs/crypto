#!/usr/bin/env bash
# Frozen command list for the CUDA-port study: gated host run of the per-thread code, then the offline
# CUDA compile for four architectures.  raw/ is write-once.  No GPU is used or needed.
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
RAW=$HERE/raw
CP=${CUDAPATH:?set CUDAPATH to a tree with include/ (cuda runtime, cccl, curand), bin/ptxas, nvvm/libdevice}
mkdir -p "$RAW" && cd "$HERE" || exit 1
make host_emul > "$RAW/build_host.log" 2>&1
./host_emul --psteps 12 --bsteps 6 > "$RAW/host_emul.stdout" 2> "$RAW/host_emul.stderr"
echo "host_emul exit=$?" > "$RAW/exit_codes.txt"
./host_emul --psteps 200 --bsteps 40 > "$RAW/host_emul_long.stdout" 2> "$RAW/host_emul_long.stderr"
echo "host_emul_long exit=$?" >> "$RAW/exit_codes.txt"
mkdir -p build
printf '#include <cuda_runtime.h>\nextern "C" cudaError_t cudaConfigureCall(dim3, dim3, size_t, cudaStream_t);\n' > build/cudashim.h
clang++ -x cuda --cuda-host-only --cuda-path="$CP" -nocudalib -Wno-unknown-cuda-version -O2 -std=c++17 -include build/cudashim.h -c walk_cuda.cu -o build/walk_host.o > "$RAW/compile_host.log" 2>&1
echo "compile_host exit=$?" >> "$RAW/exit_codes.txt"
for a in sm_80 sm_86 sm_89 sm_90; do
  clang++ -x cuda --cuda-device-only --cuda-gpu-arch=$a --cuda-path="$CP" -Wno-unknown-cuda-version -O3 -std=c++17 -include build/cudashim.h -S walk_cuda.cu -o build/walk_$a.ptx > "$RAW/compile_$a.log" 2>&1
  echo "clang_$a exit=$?" >> "$RAW/exit_codes.txt"
  "$CP/bin/ptxas" -arch=$a -O3 -v build/walk_$a.ptx -o build/walk_$a.cubin > "$RAW/ptxas_$a.txt" 2>&1
  echo "ptxas_$a exit=$?" >> "$RAW/exit_codes.txt"
done
sed -e 's/^\.version .*/.version 8.7/' -e 's/^\.target sm_90/.target sm_120/' build/walk_sm_90.ptx > build/walk_sm_120_retarget.ptx
"$CP/bin/ptxas" -arch=sm_120 -O3 -v build/walk_sm_120_retarget.ptx -o build/walk_sm_120.cubin > "$RAW/ptxas_sm_120_retarget.txt" 2>&1
echo "ptxas_sm_120_retarget exit=$? (sm_90 PTX with .version 8.7 and .target sm_120 edited in; static statistics only)" >> "$RAW/exit_codes.txt"
"$CP/bin/ptxas" --version > "$RAW/ptxas_version.txt" 2>&1
clang++ --version > "$RAW/clang_version.txt" 2>&1
sha256sum gpu_walk.cuh host_common.hpp host_emul.cpp walk_cuda.cu > "$RAW/source_sha256.txt"
echo done >> "$RAW/exit_codes.txt"
