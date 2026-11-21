#!/usr/bin/env bash
set -euo pipefail
if [ "$#" -ne 2 ]; then echo "usage: build.sh PREFIX NEW_BUILD" >&2; exit 2; fi
halving_repo=$(git rev-parse --show-toplevel)
halving_study="$halving_repo/research/auxiliary_halving_setup_20261115"
halving_prefix=$(realpath "$1")
halving_build=$(realpath -m "$2")
mkdir "$halving_build"
g++ --version > "$halving_build/compiler.txt"
openssl version > "$halving_build/openssl.txt"
for halving_program in halving cold; do
 g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror -I"$halving_prefix/usr/include" \
  "$halving_study/$halving_program.cpp" -L"$halving_prefix/usr/lib/x86_64-linux-gnu" \
  -Wl,-rpath,"$halving_prefix/usr/lib/x86_64-linux-gnu" -lflint -lgmp -o "$halving_build/$halving_program"
done
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 "$halving_study/replay.cpp" -lcrypto -o "$halving_build/replay"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror -I"$halving_prefix/usr/include" \
 "$halving_repo/research/projective_aux_norm_20261115/cold.cpp" \
 -L"$halving_prefix/usr/lib/x86_64-linux-gnu" -Wl,-rpath,"$halving_prefix/usr/lib/x86_64-linux-gnu" \
 -lflint -lgmp -o "$halving_build/legacy-cold"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 "$halving_repo/research/projective_aux_norm_20261115/replay.cpp" -lcrypto -o "$halving_build/legacy-replay"
PATH=/home/agent/.cargo/bin:$PATH CARGO_TARGET_DIR="$halving_build/cargo" \
 cargo build --release --manifest-path "$halving_study/Cargo.toml" > "$halving_build/identity-build.log" 2>&1
cp "$halving_build/cargo/release/halving-identity" "$halving_build/identity"
sha256sum "$halving_study/"*.cpp "$halving_study/halving.hpp" "$halving_study/identity.rs" \
 "$halving_study/Cargo.toml" "$halving_study/Cargo.lock" \
 "$halving_repo/src/cryptanalysis/identity_certificate.rs" "$halving_repo/src/hash/sha256.rs" \
 "$halving_repo/research/projective_aux_norm_20261115/"*.cpp \
 "$halving_repo/research/shared_norm_algebraic_20261115/"*.cpp \
 "$halving_repo/research/prime_one_shot_20261029/continuation/"{canonical,solver}/*.cpp \
 "$halving_build/"{halving,cold,replay,legacy-cold,legacy-replay,identity} \
 "$halving_prefix/usr/bin/msolve" > "$halving_build/build-source-binary.sha256"
