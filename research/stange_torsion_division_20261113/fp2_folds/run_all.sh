#!/bin/bash
# Track B sweep. base: ext_field curves (seed s), tors: 2-torsion family. 5 curves/size, 3 targets/curve, 3 rho/target.
cd "$(dirname "$0")"
{
for k in 5 4 3 2 1 0; do
  bp=(251 1019 4091 16363 65519 262139); tp=(503 2039 8191 32719 131071 524287)
  for s in 1 2 3 4 5; do
    for v in naive memo table; do echo "tors:$v ${tp[$k]} $s"; done
    for v in naive table; do echo "base:$v ${bp[$k]} $s"; done
  done
done
} | while read arm p s; do
  f="runs/${arm/:/_}_p${p}_s${s}"
  echo "python3 ic_fp2_folds.py $arm $p $s 3 3 $f.json > $f.log 2>&1"
done | xargs -P 4 -I{} bash -c "{}"
