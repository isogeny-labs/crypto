#!/bin/bash
# usage: run_all.sh  (LQR_OUT must be set)
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
for p in 40961 12289; do for t in 2 3; do for i in 0 1 2; do python3 run_cells.py m3 $p $t $i | tail -1; done; done; done
for p in 40961 12289; do for t in 2 3 4 5 6 7; do for i in 0 1 2; do python3 run_cells.py m2 $p $t $i | tail -1; done; done; done
for p in 40961 12289; do for t in 2 3; do CMAX=7 python3 run_cells.py plant $p $t | tail -1; done; done
