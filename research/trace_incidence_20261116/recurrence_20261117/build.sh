#!/usr/bin/env bash
set -euo pipefail
study=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
parent=$(dirname "$study")
prefix=${FLINT_PREFIX:-/workspace/native-deps/flint/usr}
out=${1:?output directory required}
mkdir -p "$out"
flags=(-O2 -std=c++17 -Wall -Wextra -Werror -I"$prefix/include" -I/workspace/native-deps -I"$parent/native")
libs=(-L"$prefix/lib/x86_64-linux-gnu" -Wl,-rpath,"$prefix/lib/x86_64-linux-gnu" -lflint -lmpfr -lgmp)
for mode in 0 1 2 3; do
  source_file="$study/translation.cpp"
  if ((mode==3)); then source_file="$study/preinverse.cpp"; fi
  if ((mode==0)); then source_file="$parent/native/translation.cpp"; fi
  g++ "${flags[@]}" -DROOT_RECURRENCE_MODE="$mode" "$source_file" "$parent/native/translation_tests.cpp" "${libs[@]}" -o "$out/test-$mode"
  "$out/test-$mode" > "$out/test-$mode.json"
  g++ "${flags[@]}" -DROOT_RECURRENCE_MODE="$mode" "$source_file" "$parent/native/paper.cpp" "$parent/native/pullback.cpp" "$parent/native/isogeny.cpp" "$parent/native/incidence.cpp" "$parent/runpod/incidence_lazy.cpp" "$parent/runpod/benchmark.cpp" "${libs[@]}" -o "$out/benchmark-$mode"
  chmod 0555 "$out/test-$mode" "$out/benchmark-$mode"
done
sha256sum "$out"/benchmark-* "$out"/test-* > "$out/qualified.sha256"
