#!/usr/bin/env bash
set -euo pipefail
if [ "$#" -ne 2 ]; then echo 'usage: build.sh PREFIX NEW_BUILD' >&2; exit 2; fi
big_repo=$(git rev-parse --show-toplevel)
big_study="$big_repo/research/big_prime_halving_20261115"
big_prefix=$(realpath "$1")
big_build=$(realpath -m "$2")
mkdir "$big_build"
g++ --version > "$big_build/compiler.txt"
openssl version > "$big_build/openssl.txt"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 -isystem "$big_prefix/usr/include" "$big_study/setup.cpp" \
 -L"$big_prefix/usr/lib/x86_64-linux-gnu" -Wl,-rpath,"$big_prefix/usr/lib/x86_64-linux-gnu" \
 -lflint -lgmpxx -lgmp -o "$big_build/setup"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 "$big_study/replay.cpp" -lcrypto -o "$big_build/replay"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 "$big_study/mutate.cpp" -lgmpxx -lgmp -o "$big_build/mutate"
sha256sum "$big_study/"*.cpp "$big_study/big.hpp" "$big_study/build.sh" \
 "$big_prefix/usr/include/flint/"{fmpz.h,fmpz_mod_poly.h,fmpz_mod_poly_factor.h} \
 "$big_prefix/usr/lib/x86_64-linux-gnu/"libflint.so* \
 "$big_build/"{setup,replay,mutate} > "$big_build/build-source-binary.sha256"
ldd "$big_build/setup" > "$big_build/candidate-linker.txt"
ldd "$big_build/replay" > "$big_build/reference-linker.txt"
