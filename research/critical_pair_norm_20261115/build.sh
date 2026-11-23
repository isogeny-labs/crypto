#!/usr/bin/env bash
set -euo pipefail
if [ "$#" -ne 2 ]; then echo 'usage: build.sh PREFIX NEW_BUILD' >&2; exit 2; fi
critical_repo=$(git rev-parse --show-toplevel)
critical_prefix=$(realpath "$1")
critical_build=$(realpath -m "$2")
mkdir "$critical_build"
g++ --version > "$critical_build/compiler.txt"
openssl version > "$critical_build/openssl.txt"
for critical_entry in critical_pair_norm_20261115/produce norm_pc_closure_20261115/verify macaulay_provenance_20261115/compare projective_aux_norm_20261115/auxiliary; do
 critical_name=$(basename "$critical_entry")
 g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror -I"$critical_prefix/usr/include" \
  "$critical_repo/research/$critical_entry.cpp" -L"$critical_prefix/usr/lib/x86_64-linux-gnu" \
  -Wl,-rpath,"$critical_prefix/usr/lib/x86_64-linux-gnu" -lflint -lgmp -o "$critical_build/$critical_name"
done
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 "$critical_repo/research/projective_aux_norm_20261115/replay.cpp" -lcrypto -o "$critical_build/replay"
sha256sum "$critical_repo/research/critical_pair_norm_20261115/"*.cpp \
 "$critical_repo/research/norm_pc_closure_20261115/"{common.hpp,verify.cpp} \
 "$critical_repo/research/macaulay_provenance_20261115/"{common.hpp,compare.cpp} \
 "$critical_repo/research/projective_aux_norm_20261115/"{auxiliary.cpp,replay.cpp} \
 "$critical_repo/research/shared_norm_algebraic_20261115/"*.cpp \
 "$critical_repo/research/prime_one_shot_20261029/continuation/"{canonical,solver}/*.cpp \
 "$critical_build/"{produce,verify,compare,auxiliary,replay} > "$critical_build/build-source-binary.sha256"
