#!/usr/bin/env bash
set -euo pipefail
# Run from an unmodified msolve checkout at the commit in provenance.json.
# DEPS points to an extracted Debian prefix; BUILD is a task-local directory.
: "${DEPS:?set DEPS to extracted root}"
: "${BUILD:?set BUILD to output directory}"
mkdir -p "$BUILD"
printf '/* Scalar configuration: optional SIMD macros intentionally undefined. */\n' > "$BUILD/config.h"
gcc -O2 -fopenmp -DHAVE_CONFIG_H=1 \
  "-I$BUILD" "-I$DEPS/usr/include" \
  "-L$DEPS/usr/lib/x86_64-linux-gnu" \
  "-Wl,-rpath,$DEPS/usr/lib/x86_64-linux-gnu" \
  src/msolve/main.c src/neogb/gb.c src/fglm/fglm_core.c src/usolve/usolve.c \
  -o "$BUILD/msolve" -lflint -lmpfr -lgmp -lm
