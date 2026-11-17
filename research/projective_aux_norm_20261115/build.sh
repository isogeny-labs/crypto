#!/usr/bin/env bash
set -euo pipefail
aux_repo=$(git rev-parse --show-toplevel)
aux_prefix=$(realpath "${1:?usage: build.sh PREFIX NEW_BUILD}")
aux_build=$(realpath -m "${2:?new output directory}")
mkdir "$aux_build"
for aux_program in auxiliary core; do
 if [ "$aux_program" = auxiliary ]; then aux_source="$aux_repo/research/projective_aux_norm_20261115/auxiliary.cpp"
 else aux_source="$aux_repo/research/norm_core_elimination_20261115/core.cpp"; fi
 g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror -I"$aux_prefix/usr/include" \
  "$aux_source" -L"$aux_prefix/usr/lib/x86_64-linux-gnu" -Wl,-rpath,"$aux_prefix/usr/lib/x86_64-linux-gnu" \
  -lflint -lgmp -o "$aux_build/$aux_program"
done
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
 "$aux_repo/research/projective_aux_norm_20261115/replay.cpp" -lcrypto -o "$aux_build/replay"
sha256sum "$aux_repo/research/projective_aux_norm_20261115/auxiliary.cpp" \
 "$aux_repo/research/shared_norm_algebraic_20261115/norm_algebraic.cpp" \
 "$aux_repo/research/norm_core_elimination_20261115/core.cpp" \
 "$aux_repo/research/projective_aux_norm_20261115/replay.cpp" \
 "$aux_build/"{auxiliary,core,replay} "$aux_prefix/usr/bin/msolve" > "$aux_build/build-source-binary.sha256"
g++ --version > "$aux_build/compiler.txt"
