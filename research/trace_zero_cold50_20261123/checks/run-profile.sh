#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
export PATH=/workspace/scratch/profile-tools/root/usr/bin:$PATH
export VALGRIND_LIB=/workspace/scratch/profile-tools/root/usr/libexec/valgrind
export TRACE_NO_PHASE_DUMPS=1 TRACE_DISABLE_TALLY=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
trace_root=research/trace_zero_cold50_20261123
for trace_arm in baseline-cold baseline-aa-cold;do
 taskset -c 0 bash -c 'ulimit -v 2097152; exec timeout 300 bash tools/profile.sh cpu --mode callgrind --out "$1" --label "Exact-half-trace incumbent, fixed uniform ECC2K query" -- "$2" one "$3" elimination' trace-profile "$trace_root/profiles/$trace_arm" research/trace_zero_halftrace_elimination_20261123/bin/elimination research/trace_zero_projected_20261115/inputs/scale-n131-d12-uniform.json
done
