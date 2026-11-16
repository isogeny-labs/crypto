#!/bin/sh
# Build the generator (W0's qsolve.rs plus the uring arm). No crates, no cargo.
set -eu
cd "$(dirname "$0")"
mkdir -p bin
rustc -O --edition 2021 -C target-cpu=x86-64-v2 -o bin/qsolve src/qsolve.rs
rustc --version
sha256sum src/qsolve.rs bin/qsolve
# Amendment A3: the u-ring finish (needs PML as in ../ecgfp5_quotient_bw_finish_20261102/build.sh,
# whose bin/libpml.a it links).
if [ -n "${PML:-}" ]; then
  g++ -O3 -march=native -std=c++17 src/hnfsolve_u.cpp -o bin/hnfsolve_u
  g++ -O3 -march=native -std=c++17 -I"$PML/include" -I/usr/include/x86_64-linux-gnu/openblas-pthread \
    src/bwfinish_u.cpp ../ecgfp5_quotient_bw_finish_20261102/bin/libpml.a -lopenblas -lntl -lgmp -lpthread -o bin/bwfinish_u
  sha256sum src/hnfsolve_u.cpp src/bwfinish_u.cpp bin/hnfsolve_u bin/bwfinish_u
fi
