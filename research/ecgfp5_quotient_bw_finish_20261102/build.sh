#!/bin/sh
# Build hnfsolve and bwfinish into bin/.
#   PML=<path to vneiger/pml ntl-extras at a26906b8, objects built into $PML/lib> sh build.sh
# Needs NTL 11.5.1, GMP, OpenBLAS 0.3.26 (libopenblas-dev).
set -eu
cd "$(dirname "$0")"
: "${PML:?set PML to pml/ntl-extras}"
mkdir -p bin
ar rcs bin/libpml.a "$PML"/lib/*.o
g++ -O3 -march=native -std=c++17 -I"$PML/include" src/hnfsolve.cpp bin/libpml.a -lntl -lgmp -lpthread -o bin/hnfsolve
g++ -O3 -march=native -std=c++17 -I"$PML/include" -I/usr/include/x86_64-linux-gnu/openblas-pthread \
  src/bwfinish.cpp bin/libpml.a -lopenblas -lntl -lgmp -lpthread -o bin/bwfinish
echo built bin/hnfsolve bin/bwfinish
