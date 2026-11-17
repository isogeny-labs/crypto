#!/bin/bash
V=/tmp/claude-0/-home-user/aeefaead-5838-5329-9e0a-13b7d93fe240/scratchpad/v3val
while [ ! -f $V/out/done.txt ]; do sleep 20; done
cd $V && python3 scripts/run_ks.py ksbuild/target/release/mt_fieldops frozen/research/mt_fieldops_v3_20261111 out/ks_rows.jsonl 4 > out/log_ks.txt 2>&1
echo KSDONE > out/ks_done.txt
