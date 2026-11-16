#!/usr/bin/env bash
# Thin build orchestration only; all research algorithms run in native binaries.
set -euo pipefail
shared_norm_root=$(git rev-parse --show-toplevel)
shared_norm_prefix=$(realpath "${1:?usage: build.sh PREFIX OUTDIR}")
shared_norm_build=$(realpath -m "${2:?usage: build.sh PREFIX OUTDIR}")
mkdir -p "$shared_norm_build"
cd "$shared_norm_root"
shared_norm_study=research/shared_norm_algebraic_20261115
for shared_norm_binary in quotient_extract norm_algebraic; do
    g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
        -I"$shared_norm_prefix/usr/include" \
        "$shared_norm_study/$shared_norm_binary.cpp" \
        -L"$shared_norm_prefix/usr/lib/x86_64-linux-gnu" \
        -Wl,-rpath,"$shared_norm_prefix/usr/lib/x86_64-linux-gnu" \
        -lflint -lgmp -o "$shared_norm_build/$shared_norm_binary"
done
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
    "$shared_norm_study/replay.cpp" -lcrypto -o "$shared_norm_build/replay"
sha256sum "$shared_norm_study"/*.cpp "$shared_norm_build"/quotient_extract \
    "$shared_norm_build"/norm_algebraic "$shared_norm_build"/replay \
    "$shared_norm_prefix/usr/bin/msolve" > "$shared_norm_build/source-binary.sha256"
g++ --version > "$shared_norm_build/compiler.txt"
openssl version > "$shared_norm_build/openssl.txt"
printf 'Built native programs in %s\n' "$shared_norm_build"
