#!/bin/bash
# Validator replay chain (scratch only).  Instrumented full replay, then pristine-source and producer-binary samples.
V=/tmp/claude-0/-home-user/aeefaead-5838-5329-9e0a-13b7d93fe240/scratchpad/v3val
F=$V/frozen/research
cd $V
python3 scripts/replay.py dbgbuild/target/release/mt_fieldops $F/mt_fieldops_v3_20261111/results/holdout/rows.jsonl $F/mt_fieldops_v3_20261111/instances out/dbg_v3_holdout.jsonl --stderr-json --dump-dir dumps/v3 --dump out/dump_v3.json --tag v3_ > out/log_dbg_v3.txt 2>&1
python3 scripts/replay.py dbgbuild/target/release/mt_fieldops $F/mt_rho_start_fix_20261111/results/merged.jsonl $F/mt_fieldops_v2_20261111/instances out/dbg_b_merged.jsonl --stderr-json --dump-dir dumps/b --dump out/dump_b.json --tag b_ > out/log_dbg_b.txt 2>&1
python3 scripts/replay.py dbgbuild/target/release/mt_fieldops $F/mt_fieldops_v3_20261111/results/tune/rows.jsonl $F/mt_fieldops_v3_20261111/instances out/dbg_v3_tune.jsonl --stderr-json > out/log_dbg_tune.txt 2>&1
python3 scripts/replay.py vbuild/target/release/mt_fieldops $F/mt_fieldops_v3_20261111/results/holdout/rows.jsonl $F/mt_fieldops_v3_20261111/instances out/src_v3_sample.jsonl --subset out/src_sample_v3.json > out/log_src_v3.txt 2>&1
python3 scripts/replay.py vbuild/target/release/mt_fieldops $F/mt_rho_start_fix_20261111/results/merged.jsonl $F/mt_fieldops_v2_20261111/instances out/src_b_sample.jsonl --subset out/src_sample_b.json > out/log_src_b.txt 2>&1
python3 scripts/replay.py producer_bin_mt_fieldops $F/mt_fieldops_v3_20261111/results/holdout/rows.jsonl $F/mt_fieldops_v3_20261111/instances out/prod_v3_sample.jsonl --subset out/prod_sample_v3.json > out/log_prod_v3.txt 2>&1
echo ALLDONE > out/done.txt
