#!/usr/bin/env bash
set -euo pipefail
aux_repo=$(git rev-parse --show-toplevel)
aux_prefix=$(realpath "${1:?usage: build-cold.sh PREFIX NEW_BUILD}")
aux_build=$(realpath -m "${2:?fresh output directory}")
mkdir "$aux_build"
for aux_program in cold core auxiliary; do
 if [ "$aux_program" = core ]; then aux_source="$aux_repo/research/norm_core_elimination_20261115/core.cpp"
 else aux_source="$aux_repo/research/projective_aux_norm_20261115/$aux_program.cpp"; fi
 g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror -I"$aux_prefix/usr/include" \
  "$aux_source" -L"$aux_prefix/usr/lib/x86_64-linux-gnu" -Wl,-rpath,"$aux_prefix/usr/lib/x86_64-linux-gnu" \
  -lflint -lgmp -o "$aux_build/$aux_program"
done
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 "$aux_repo/research/shared_norm_algebraic_20261115/replay.cpp" -lcrypto -o "$aux_build/parent-replay"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 "$aux_repo/research/projective_aux_norm_20261115/replay.cpp" -lcrypto -o "$aux_build/replay"
sha256sum "$aux_repo/research/projective_aux_norm_20261115/"*.cpp \
 "$aux_repo/research/shared_norm_algebraic_20261115/"*.cpp \
 "$aux_repo/research/prime_one_shot_20261029/continuation/solver/direct_ic.cpp" \
 "$aux_build/"{cold,core,auxiliary,parent-replay,replay} "$aux_prefix/usr/bin/msolve" > "$aux_build/build-source-binary.sha256"
g++ --version > "$aux_build/compiler.txt"
openssl version > "$aux_build/openssl.txt"
