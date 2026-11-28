#!/bin/bash
# Re-run every measurement in this directory. Curves = the ones in ../ic_e2e/runs_lp_slack1 (same seeds).
cd "$(dirname "$0")"
run(){ python3 ic_pm.py $1 $2 1,2,4 --curve=$3 --slack=1 --reps-ic=200 --reps-rho=400 --W=1,16,64 --m=0,1,4 > runs/pm_p$1_s$2.json 2> runs/pm_p$1_s$2.err; echo "p=$1 s=$2 rc=$?"; }
run 10007 1 4748,353,10037 &
run 10007 2 5072,4970,9833 &
run 10007 3 702,4936,10067 &
run 100003 1 8272,33433,100379 &
run 100003 2 88406,73386,99859 &
run 100003 3 38223,80016,99871 &
run 1000003 1 800876,66173,1000033 &
run 1000003 2 475414,665926,998653 &
run 1000003 3 915163,124694,1000763 &
wait
