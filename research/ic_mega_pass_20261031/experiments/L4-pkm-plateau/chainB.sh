cd "$(dirname "$0")"
P1=2013265921
for t in 1 2 3 4 5 6 7 8 9 10; do python3 run_msolve.py kummer $P1 $t 1 4 1200; done > raw/msolve_kummer_P1.jsonl
for t in 2 3 4 5 6 7 8 9; do python3 run_msolve.py null $P1 $t 1 4 900; done > raw/msolve_null_P1.jsonl
python3 run_msolve.py kummer $P1 11 1 2 2400 > raw/msolve_kummer_P1_t11.jsonl
echo done > raw/chainB.done
