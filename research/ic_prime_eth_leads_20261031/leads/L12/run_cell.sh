#!/bin/bash
# usage: run_cell.sh CELL CAP  (runs the three modes in parallel, 256 targets)
cd "$(dirname "$0")"
for mode in inc fresh units; do
  timeout 3000 ./driver $mode raw/$1/inst.cnf raw/$1/targets.tsv 256 $2 > raw/$1.$mode.tsv 2> raw/$1.$mode.err &
done
wait
