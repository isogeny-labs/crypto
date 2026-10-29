#!/bin/bash
# G1 driver (run.sh = run_g1.sh with stage B before stage A, plus the two post-freeze addenda at the end).  Full run: about 66 minutes wall on a contended machine (summed cell time about 2.1 CPU hours); for a reduced replication use repro_quick.sh.  Executes PREREGISTRATION.md section 8.  Aborts if any frozen file differs from FROZEN_SHA256.txt.
set -u
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/G1-sound-negation-map-rho-baseline
cd $D
sha256sum -c FROZEN_SHA256.txt > out/FROZEN_CHECK.txt 2>&1 || { echo "FROZEN FILE MISMATCH, abort"; cat out/FROZEN_CHECK.txt; exit 2; }
: > out/RC.txt
cellN1() { # arm b inst lo hi total
  ARM=$1 B=$2 INST=$3 TLO=$4 THI=$5 TOTAL=$6 timeout 840 gp -q -f g1_run_cell.gp < /dev/null > raw/cell_$1_b$2_i$3_c$4.txt 2> raw/cell_$1_b$2_i$3_c$4.err
  echo "$1 b$2 i$3 c$4-$5 rc=$?" >> out/RC.txt
}
cellK() { # arm b inst trials  (E5 run_cell.gp, byte-identical copy)
  ARM=$1 B=$2 INST=$3 TRIALS=$4 timeout 840 gp -q -f run_cell.gp < /dev/null > raw/cell_$1_b$2_i$3.txt 2> raw/cell_$1_b$2_i$3.err
  echo "$1 b$2 i$3 K rc=$?" >> out/RC.txt
}
export -f cellN1 cellK
# stage A: negative controls (sabotage) and the sensitivity arm r = 64
{ for b in 24 28 32; do echo "n1sab $b 1 1 30 30"; echo "n1c2 $b 1 1 30 30"; done
  echo "n1sab 36 1 1 20 20"; echo "n1c2 36 1 1 20 20"
  for i in 1 2; do echo "n1r64 36 $i 1 20 20"; echo "n1r64 40 $i 1 12 12"; done; } > cells_A.list
# stage B: N1 on the E5 instances (1,2) and the extension instances (3,4); positive control
{ for i in 1 2; do
    echo "n1 24 $i 1 80 80"; echo "n1 28 $i 1 80 80"; echo "n1 32 $i 1 60 60"
    echo "n1 36 $i 1 20 40"; echo "n1 36 $i 21 40 40"; echo "n1 40 $i 1 20 40"; echo "n1 40 $i 21 40 40"
  done
  for i in 3 4; do echo "n1 24 $i 1 40 40"; echo "n1 28 $i 1 40 40"; echo "n1 32 $i 1 30 30"; echo "n1 36 $i 1 20 20"; echo "n1 40 $i 1 12 12"; done; } > cells_B.list
# stage C: K-pure, K-mix, A0 on the extension instances (E5 instances are reused from e5_inputs/)
{ for arm in kmix kpure a0; do for i in 3 4; do echo "$arm 24 $i 40"; echo "$arm 28 $i 40"; echo "$arm 32 $i 30"; echo "$arm 36 $i 20"; echo "$arm 40 $i 12"; done; done; } > cells_C.list
# ORDER DEVIATION (see DEVIATIONS.md #1): positive control (stage B) runs before the negative controls (stage A)
cat cells_B.list | xargs -P 2 -L 1 bash -c 'cellN1 $0 $1 $2 $3 $4 $5'
cat cells_A.list | xargs -P 2 -L 1 bash -c 'cellN1 $0 $1 $2 $3 $4 $5'
cat cells_C.list | xargs -P 2 -L 1 bash -c 'cellK $0 $1 $2 $3'
: > out/solve.csv; : > out/ratios.csv
gp -q -f analyze_g1.gp < /dev/null > out/summary.txt 2> out/analyze.err
gp -q -f opcounts_g1.gp < /dev/null > out/opcounts.txt 2> out/opcounts.err   # post-freeze addendum (DEVIATIONS.md #2): operation-count tables
gp -q -f make_results_g1.gp < /dev/null > out/make_results.txt 2>&1              # post-freeze addendum: results.json
ls raw out > out/FILES.txt
echo done
