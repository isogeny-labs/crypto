#!/bin/bash
V=/tmp/claude-0/-home-user/aeefaead-5838-5329-9e0a-13b7d93fe240/scratchpad/v3val
while [ ! -f $V/out/ks_done.txt ]; do sleep 20; done
cd $V && python3 scripts/run_ks_cfg.py ksbuild/target/release/mt_fieldops frozen/research/mt_rho_start_fix_20261111/results/merged.jsonl frozen/research/mt_fieldops_v2_20261111/instances out/ks_b_rows.jsonl 4 > out/log_ks_b.txt 2>&1
echo KSBDONE > out/ks_b_done.txt
