#!/usr/bin/env bash
# GPU pod: optlevel GPU path (probe, calibration kernel build, calibrate, ncu if present) and the CPU controls.
set -uo pipefail
W=/workspace; O=$W/out; mkdir -p $O/optlevel_gpu
exec > >(tee -a $O/gpu_job.log) 2>&1
date -u +%FT%TZ; nproc; free -g | head -2; nvidia-smi; which nvcc ncu nsys gcc perf; nvcc --version | tail -2
mkdir -p $W/src && tar xzf $W/in/optlevel.tar.gz -C $W/src && cd $W/src
OPT="python3 tools/optlevel/optlevel.py"
echo "=== probe"; $OPT probe | tee $O/optlevel_gpu/probe.json
echo "=== gpu probe"; $OPT gpu probe | tee $O/optlevel_gpu/gpu_probe.json; echo "rc=$?"
$OPT gpu calibrate --help | head -20; $OPT gpu measure --help | head -30
echo "=== gpu calibrate"; $OPT gpu calibrate --out $O/optlevel_gpu/gpu_cal.json; echo "gpu calibrate rc=$?"
cat $O/optlevel_gpu/gpu_cal.json 2>/dev/null | head -c 3000
echo "=== cpu controls on this host"; $OPT controls --out $O/optlevel_gpu/controls --repeat 5; echo "controls rc=$?"
tar czf $O/optlevel_gpu.tar.gz -C $O optlevel_gpu; echo "=== ALL DONE"; date -u +%FT%TZ
