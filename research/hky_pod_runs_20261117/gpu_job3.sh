#!/usr/bin/env bash
# GPU pod, third job: as gpu_job2.sh with gawk installed (profile.sh needs it) and the triad scored as fma_f64 at 24 bytes per operation.
# (expected attainment near 1); the triad as the memory-bound control.
set -uo pipefail
W=/workspace; O=$W/out/gpu3; mkdir -p $O
exec > >(tee -a $O/gpu_job3.log) 2>&1
date -u +%FT%TZ
export DEBIAN_FRONTEND=noninteractive; (apt-get update -qq && apt-get install -y -qq jq gawk) > $O/apt.log 2>&1; echo "apt rc=$?"
rm -rf $W/src2 && mkdir -p $W/src2 && tar xzf $W/in/optlevel2.tar.gz -C $W/src2 && cd $W/src2
OPT="python3 tools/optlevel/optlevel.py"
echo "=== profile probe with ncu control"; bash tools/profile.sh probe --gpu-control --out $O/probe | tail -40
nvcc -O3 -std=c++17 -o $W/gpu_one tools/optlevel/kernels/gpu_calib.cu || exit 8
$OPT gpu calibrate --out $O/gpu_cal.json --repeat 3; echo "calibrate rc=$?"
for cls in muladd_u32 fma_f64 triad; do
  echo "=== $cls"; $W/gpu_one $cls
  kern=stream; oc=$cls; extra=""; [ $cls = triad ] && { kern=triad; oc=fma_f64; extra="--bytes-per-op 24"; }
  bash tools/profile.sh ncu --out $O/ncu_$cls --kernel $kern -- $W/gpu_one $cls > $O/ncu_$cls.log 2>&1; echo "ncu rc=$?"; tail -15 $O/ncu_$cls.log; ls $O/ncu_$cls
  $OPT gpu measure --out $O/$cls.measure.json --label gpu-$cls --repeat 5 --op-class $oc $extra -- $W/gpu_one $cls; echo "measure rc=$?"
  K=$(find $O/ncu_$cls -name kernels.json | head -1)
  if [ -n "$K" ]; then $OPT score --calibration $O/gpu_cal.json --measurement $O/$cls.measure.json --ncu $K --out $O/$cls.record.json; else $OPT score --calibration $O/gpu_cal.json --measurement $O/$cls.measure.json --out $O/$cls.record.json; fi
  echo "score rc=$?"; $OPT validate $O/$cls.record.json; echo "validate rc=$?"
  python3 -c "import json,sys;d=json.load(open(sys.argv[1]));print(json.dumps({k:d.get(k) for k in ('tier','grade','score','limiting_factor','flags','attainment')},default=str)[:1500])" $O/$cls.record.json
done
$OPT plot --calibration $O/gpu_cal.json --out $O/gpu-roofline.svg $O/*.record.json; echo "plot rc=$?"
tar czf $W/out/gpu3.tar.gz -C $W/out gpu2; echo "=== ALL DONE"; date -u +%FT%TZ
