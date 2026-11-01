#!/bin/sh
# Build the native implementation and correctness verifiers; no benchmark runs.
set -eu
SOURCE_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
BUILD_DIR=${BUILD_DIR:-"$SOURCE_DIR/build"}
NATIVE_PREFIX=${NATIVE_PREFIX:-/workspace/env/sage}
CC=${CC:-cc}
CXX=${CXX:-c++}
mkdir -p "$BUILD_DIR"
"$CC" -O3 -Wall -Wextra -std=c11 -I"$NATIVE_PREFIX/include" -Dprime_log_norm_jet=baseline -Dprime_log_norm_free=baseline_free -c "$SOURCE_DIR/baseline.c" -o "$BUILD_DIR/baseline.o"
"$CC" -O3 -Wall -Wextra -std=c11 -I"$NATIVE_PREFIX/include" -Dprime_log_norm_jet=optimized -Dprime_log_norm_free=optimized_free -c "$SOURCE_DIR/optimized.c" -o "$BUILD_DIR/optimized.o"
for PROGRAM in bench check; do
    OBJECTS="$BUILD_DIR/optimized.o"
    if [ "$PROGRAM" = bench ]; then
        "$CC" -O3 -Wall -Wextra -std=c11 -I"$NATIVE_PREFIX/include" "$SOURCE_DIR/bench.c" "$BUILD_DIR/baseline.o" "$BUILD_DIR/optimized.o" -L"$NATIVE_PREFIX/lib" -Wl,-rpath,"$NATIVE_PREFIX/lib" -lflint -lgmp -o "$BUILD_DIR/bench"
    else
        "$CC" -O3 -Wall -Wextra -std=c11 -I"$NATIVE_PREFIX/include" "$SOURCE_DIR/check.c" "$OBJECTS" -L"$NATIVE_PREFIX/lib" -Wl,-rpath,"$NATIVE_PREFIX/lib" -lflint -lgmp -o "$BUILD_DIR/check"
    fi
done
"$CC" -O3 -Wall -Wextra -std=c11 -I"$NATIVE_PREFIX/include" "$SOURCE_DIR/search80.c" -L"$NATIVE_PREFIX/lib" -Wl,-rpath,"$NATIVE_PREFIX/lib" -lflint -lgmp -o "$BUILD_DIR/search80"
"$CC" -O2 -Wall -Wextra -std=c11 -I"$NATIVE_PREFIX/include" "$SOURCE_DIR/pari_verify.c" -L"$NATIVE_PREFIX/lib" -Wl,-rpath,"$NATIVE_PREFIX/lib" -lpari -o "$BUILD_DIR/pari_verify"
"$CXX" -O2 -Wall -Wextra -std=c++17 -I"$NATIVE_PREFIX/include" "$SOURCE_DIR/measurement_preflight.cpp" -L"$NATIVE_PREFIX/lib" -Wl,-rpath,"$NATIVE_PREFIX/lib" -lgmpxx -lgmp -o "$BUILD_DIR/measurement_preflight"
"$CC" -O2 -Wall -Wextra -std=c11 -I"$NATIVE_PREFIX/include" "$SOURCE_DIR/root_chunk.c" -L"$NATIVE_PREFIX/lib" -Wl,-rpath,"$NATIVE_PREFIX/lib" -lflint -lgmp -o "$BUILD_DIR/root_chunk"
"$CC" -O2 -Wall -Wextra -std=c11 -I"$NATIVE_PREFIX/include" "$SOURCE_DIR/check_root_chunk.c" -L"$NATIVE_PREFIX/lib" -Wl,-rpath,"$NATIVE_PREFIX/lib" -lpari -o "$BUILD_DIR/check_root_chunk"
