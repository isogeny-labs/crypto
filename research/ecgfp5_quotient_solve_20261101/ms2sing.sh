#!/usr/bin/env bash
# Write a Singular script that computes a signature-based (sba) or Buchberger (std) Groebner
# basis of an msolve input file, in grevlex (dp) with the file's variable order, and prints
# vdim, the basis size and the CPU time. Usage: ms2sing.sh FILE.ms sba|std [sbaOrder arri]
set -euo pipefail
f=$1; alg=$2; o=${3:-0}; a=${4:-0}
vars=$(sed -n 1p "$f"); p=$(sed -n 2p "$f")
echo "int t0 = timer; system(\"--ticks-per-sec\", 1000); t0 = timer;"
echo "ring r = $p, ($vars), dp;"
echo "option(redSB); option(prot);"
printf 'ideal i = '
tail -n +3 "$f" | tr -d '\n'
echo ';'
if [ "$alg" = sba ]; then echo "ideal g = sba(i, $o, $a);"; else echo "ideal g = std(i);"; fi
echo 'print("RESULT vdim=" + string(vdim(g)) + " size=" + string(size(g)) + " ms=" + string(timer - t0));'
echo 'quit;'
