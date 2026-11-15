#!/bin/bash
# Exact pod-side command used on RunPod pod 3s0i2d2itxl75h (cpu5m, 16 vCPU), 2026-11-12, via
#   RP_STATE=/tmp/claude-0/rp_closure-falls-crosscheck research/runpod_kit_20261112/rp.sh exec bdr-closure-falls-crosscheck '<below>'
# Binary: gcc -O3 -march=x86-64-v3 -fopenmp -static -o iclose iclose.c  (sha256 a6074818...dfcfe); column seed 7, FIFO, batch 4096.
lscpu | grep -E "Model name|^CPU\(s\)"; nproc; free -g | head -2
for spec in "T35k17s1 0" "T35k17s2 0" "R36k18s1 0" "R36k18s2 0" "T33k17s2 1" "R35k18s6 1"; do
  set -- $spec; echo "== $1 full=$2"; OMP_NUM_THREADS=16 ./iclose 4 sys/$1.sys 4096 $2 7 1 2> log_$1.txt
done; echo DONE
