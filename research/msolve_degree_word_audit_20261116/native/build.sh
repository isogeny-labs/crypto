#!/usr/bin/env bash
set -euo pipefail
study=$(cd "$(dirname "$0")/.." && pwd)
task_source=/workspace/scratch/msolve-degree-native-source
task_build=/workspace/scratch/msolve-degree-native-build
task_deps=/workspace/msolve-deps/root
[ ! -e "$task_source" ]
[ ! -e "$task_build" ]
mkdir -p "$task_source" "$task_build"
git -C /workspace/msolve archive 5e72b9d99eaea55fe87eb8ac945ec9e914a69327 src | tar -x -C "$task_source"
python3 "$study/native/prepare_source.py" "$task_source" > "$task_build/source-provenance.json"
cp "$task_source/degree-audit.patch" "$study/native/degree-audit.patch"
printf '/* Scalar source audit; optional SIMD disabled. */\n' > "$task_build/config.h"
gcc --version > "$task_build/gcc-version.txt"
rustc -Vv > "$task_build/rustc-version.txt"
cd "$task_source"
gcc -O1 -g -fno-omit-frame-pointer -fsanitize=address -fno-sanitize-recover=all \
  -fopenmp -DHAVE_CONFIG_H=1 "-I$task_build" "-I$task_deps/usr/include" \
  "-L$task_deps/usr/lib/x86_64-linux-gnu" "-Wl,-rpath,$task_deps/usr/lib/x86_64-linux-gnu" \
  src/msolve/main.c src/neogb/gb.c src/fglm/fglm_core.c src/usolve/usolve.c \
  -o "$task_build/msolve" -lflint -lmpfr -lgmp -lm
rustc --edition=2021 -O "$study/native/analyze.rs" -o "$task_build/analyze"
sha256sum "$task_build/msolve" "$task_build/analyze" > "$task_build/binaries.sha256"
