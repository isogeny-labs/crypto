#!/bin/bash
# usage: run-prof.sh BIN LABEL [names...] -> dhat + massif per input
B=$1; L=$2; shift 2; export RAYON_NUM_THREADS=1
for n in "$@"; do
  valgrind --tool=dhat --dhat-out-file=dh.$L.$n.json $B exec < $n.in > dh.$L.$n.out 2> dh.$L.$n.err
  valgrind --tool=massif --massif-out-file=ms.$L.$n.out $B exec < $n.in > /dev/null 2> ms.$L.$n.err
  echo "$L $n done"
done
