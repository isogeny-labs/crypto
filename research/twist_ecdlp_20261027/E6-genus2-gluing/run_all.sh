#!/bin/bash
# E6 frozen command card.  Run from this directory.  Sequential (shared 4-core machine).  Each gp call < 15 min.
set -u
cd /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E6-genus2-gluing
SEED=20261021
run_main() { # p N BRUTE
  E6_P=$1 E6_N=$2 E6_BRUTE=$3 E6_SEED=$SEED timeout 900 gp -q e6_main.gp </dev/null > out/main_p$1.txt 2> out/main_p$1.err
  echo "main p=$1 rc=$?" >> out/rc.txt
}
run_ctrl() { # p NPOS NNEG NNULL
  E6_P=$1 E6_NPOS=$2 E6_NNEG=$3 E6_NNULL=$4 E6_SEED=$SEED timeout 900 gp -q e6_controls.gp </dev/null > out/ctrl_p$1.txt 2> out/ctrl_p$1.err
  echo "ctrl p=$1 rc=$?" >> out/rc.txt
}
: > out/rc.txt
run_main 101   20 4
run_main 211   20 3
run_main 409   15 0
run_main 1009  10 0
run_main 4001   5 0
run_main 10007  4 0
run_ctrl 101  300 300 3000
run_ctrl 211  300 300 3000
run_ctrl 409  300 300 2000
run_ctrl 1009 300 300 1000
run_ctrl 4001 100 100 200
run_main 20011  1 0     # last; may be cut by timeout, which is then reported as a missing run
E6_FILES=$(ls out/main_p*.txt | paste -sd,) E6_CFILES=$(ls out/ctrl_p*.txt | paste -sd,) \
  timeout 600 gp -q e6_summary.gp </dev/null > out/summary.txt 2> out/summary.err
echo "summary rc=$?" >> out/rc.txt
sha256sum e6lib.gp e6_main.gp e6_controls.gp e6_summary.gp run_all.sh out/main_p*.txt out/ctrl_p*.txt out/summary.txt > out/SHA256SUMS.txt
