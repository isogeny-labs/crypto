#!/bin/sh
# Full reproduction (about 1 minute on 4 cores, < 300 MB RSS).  Run from this directory.
set -e
python3 semaev.py                      > verify_s4_symbolic.log
python3 verify_semantics.py            > verify_semantics.txt
python3 test_descent.py                > test_descent.txt
python3 rho_generic.py                 > rho_generic.txt
rm -f raw_*.jsonl
python3 run_table.py raw_small.jsonl  "7:3:30,8:3:30,9:3:30,10:3:30,11:3:30,12:3:30,13:3:20,9:4:20,10:4:20,11:4:20,12:4:20,13:4:20,14:4:20,16:4:20,20:4:10" 4 > run_small.log
python3 run_table.py raw_small2.jsonl "14:3:30,16:3:30,20:3:30,10:5:20,12:5:20" 4 > run_small2.log
python3 run_table.py raw_mid.jsonl    "13:5:20,14:5:20,15:5:20,16:5:20,17:5:10,16:6:6,17:6:6,18:6:6,19:6:3,20:6:3" 4 > run_mid.log
python3 run_table.py raw_big.jsonl    "19:7:2,20:7:2,21:7:2" 2 > run_big.log
python3 run_table.py raw_n3stats.jsonl "7:3:300,8:3:300,9:3:300,10:3:300,11:3:300,12:3:300" 4 > run_n3stats.log
python3 structured_V.py > structured_V.txt
python3 peak_mem.py > peak_mem.txt
python3 aggregate.py > aggregate_out.txt
