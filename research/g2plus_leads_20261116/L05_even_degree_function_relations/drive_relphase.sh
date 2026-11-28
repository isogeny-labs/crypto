#!/bin/bash
cd /home/user/crypto/research/g2plus_leads_20261116/L05_even_degree_function_relations
source drive_common.sh
for g in 2 3; do for p in 1031 4099 16411 65537; do
  for c in 1 2 3; do for s in 1 2; do
    for arm in O E_0 E_low E_1a E_g O_T E_0_T E_1a_T; do
      logrun raw/rel_g${g}_p${p}_root_c${c}_s${s}_${arm}.jsonl python3 run.py relphase --g $g --p $p --class root --curve-index $c --seed $s \
        --arm $arm --out raw/rel_g${g}_p${p}_root_c${c}_s${s}_${arm}.jsonl
    done; done; done
  for c in 1 2 3; do for arm in E_0 E_1a E_g E_1a_T; do
      logrun raw/rel_g${g}_p${p}_rootless_c${c}_s1_${arm}.jsonl python3 run.py relphase --g $g --p $p --class rootless --curve-index $c --seed 1 \
        --arm $arm --out raw/rel_g${g}_p${p}_rootless_c${c}_s1_${arm}.jsonl
  done; done
  for c in 1 2 3; do for arm in O O_T; do
      logrun raw/rel_g${g}_p${p}_oddrand_c${c}_s1_${arm}.jsonl python3 run.py relphase --g $g --p $p --class oddrand --curve-index $c --seed 1 \
        --arm $arm --out raw/rel_g${g}_p${p}_oddrand_c${c}_s1_${arm}.jsonl
  done; done
done; done
for s in 1 2; do for arm in O E_0 E_low E_1a E_g O_T E_0_T E_1a_T; do
  logrun raw/aa_g2_p4099_root_c1_s${s}_${arm}.jsonl python3 run.py relphase --g 2 --p 4099 --class root --curve-index 1 --seed $s \
    --arm $arm --out raw/aa_g2_p4099_root_c1_s${s}_${arm}.jsonl
done; done
echo DONE > raw/DRIVE_DONE
