cd "$(dirname "$0")"
P1=2013265921; P2=998244353
for t in 1 2 3 4 5 6 7 8 9; do python3 run_closure.py kummer $P1 $t 1 4 7 900; done > raw/closure_kummer_P1.jsonl
for t in 2 3 4 5 6 7 8; do python3 run_closure.py null $P1 $t 1 4 7 600; done > raw/closure_null_P1.jsonl
for t in 2 3 4 5 6 7 8; do python3 run_closure.py kummer $P2 $t 1 3 7 600; done > raw/closure_kummer_P2.jsonl
echo done > raw/chainA.done
