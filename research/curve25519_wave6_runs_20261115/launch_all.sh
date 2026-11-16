#!/usr/bin/env bash
# Launch the four leads side by side (one core each). Record of the intended launch; the actual launches were the
# same commands issued inline (see each lead's run.log and the cells' ext.cmd), with L1 on p=16777141,A=7842 after
# its pre-data amendment (the k = 20 member named below is invalid under the full family rule).
set -u
R=/home/user/crypto/research/curve25519_wave6_runs_20261115
BIN=/home/user/ctarget-f1/release/curve25519_family
EB=/home/user/ctarget-f1/release/ecbench
MEM=6000000; CAP=1800; N=6
cd /home/user/crypto
# L1: faithful member, m = 2 then m = 3 chains (even d allowed)
setsid nohup python3 $R/run_cells.py $R/L1_faithful $BIN $MEM $CAP $N \
  "p=1048261,A=383:sub:sym:2:3,4,5,6,10,12,15,20,30,60" \
  "p=1048261,A=383:sub:sym:3:3,4,5,6,10,12,15,20,30,60" >> $R/L1_faithful/run.log 2>&1 < /dev/null &
# L3: binary control (chain reruns) first, then flat m = 3 and m = 2 chains
setsid nohup python3 $R/run_cells.py $R/L3_flat $BIN $MEM $CAP $N \
  "20:sub:sym:3:9,21" "20:sub:sym:2:57" \
  "20:subflat:sym:3:9,12,14,18,21,27,28,36" \
  "20:subflat:sym:2:9,21,27,57,63,133,171,189,219,399" >> $R/L3_flat/run.log 2>&1 < /dev/null &
# L4: step log at d = 19 (m = 2), then sym-efirst at d = 19 and 57
setsid nohup python3 $R/run_cells.py $R/L4_trace $BIN $MEM $CAP $N \
  "20:sub:sym:2:19:env=F4SUB_STEP_LOG=1" \
  "20:sub:sym-efirst:2:19,57" >> $R/L4_trace/run.log 2>&1 < /dev/null &
# L2: ecbench k = 20 then k = 24, one CPU
( for k in 20 24; do
    $EB run --wait --spec $R/L2_ladder_check/spec-k$k.json --out $R/L2_ladder_check/sessions/k$k --cpus 3 --allow-busy
    $EB verify --dir $R/L2_ladder_check/sessions/k$k --replay 8 --out $R/L2_ladder_check/verify-k$k.json
    $EB band --dir $R/L2_ladder_check/sessions/k$k --json $R/L2_ladder_check/band-k$k.json --ranks > $R/L2_ladder_check/band-k$k.txt
  done; echo L2-DONE ) >> $R/L2_ladder_check/run.log 2>&1 < /dev/null &
disown -a
echo launched
