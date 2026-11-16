#!/bin/bash
# generate dev (smoke) data at toy sizes with non-preregistered seeds, only to test code/analysis
D=research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm
O=$D/dev/data; rm -rf $O; mkdir -p $O
for k in 6 7 8; do for h in 1 2; do SRC=$D K=$k HALF=$h OUT=$O/main_k${k}_h$h.txt MSEED=777 gp -q $D/main.gp </dev/null >/dev/null 2>&1 & done; done; wait
for k in 8; do for c in PCA PCC NCA NCB NCC; do SRC=$D CTRL=$c K=$k OUT=$O/ctrl_${c}_k$k.txt MSEED=777 timeout 300 gp -q $D/controls.gp </dev/null >/dev/null 2>&1 & done; done; wait
for k in 6 7; do for c in PCA; do SRC=$D CTRL=$c K=$k OUT=$O/ctrl_${c}_k$k.txt MSEED=777 timeout 300 gp -q $D/controls.gp </dev/null >/dev/null 2>&1 & done; done; wait
wc -l $O/*
