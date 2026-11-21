#!/usr/bin/env bash
set -euo pipefail
pc_repo=$(git rev-parse --show-toplevel)
pc_prefix=$(realpath "${1:?usage: build-cold.sh PREFIX EXISTING_BUILD}")
pc_build=$(realpath "${2:?existing producer/verifier build directory required}")
test ! -e "$pc_build/cold"
test ! -e "$pc_build/replay"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 -I"$pc_prefix/usr/include" "$pc_repo/research/norm_pc_closure_20261115/cold.cpp" \
 -L"$pc_prefix/usr/lib/x86_64-linux-gnu" -Wl,-rpath,"$pc_prefix/usr/lib/x86_64-linux-gnu" \
 -lflint -lgmp -o "$pc_build/cold"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 "$pc_repo/research/shared_norm_algebraic_20261115/replay.cpp" -lcrypto -o "$pc_build/replay"
sha256sum "$pc_repo/research/norm_pc_closure_20261115/"*.cpp \
 "$pc_repo/research/norm_pc_closure_20261115/"*.hpp \
 "$pc_repo/research/shared_norm_algebraic_20261115/"*.cpp \
 "$pc_repo/research/norm_core_elimination_20261115/core.cpp" \
 "$pc_build/"{produce,verify,cold,replay} > "$pc_build/cold-source-binary.sha256"
openssl version > "$pc_build/cold-openssl.txt"
