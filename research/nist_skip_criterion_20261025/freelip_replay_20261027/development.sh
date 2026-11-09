#!/bin/sh
# Local build/unit checks only; deliberately contains no historical input path.
set -eu
: "${LIP_SOURCE:?original extracted source directory}"
: "${LIP_OUT:?fresh development output}"
: "${LIP_ADAPTER:?strict driver source}"
: "${LIP_ZIG:?verified Zig0.15.2 compiler}"
: "${LIP_QEMU:?verified QEMU i3867.2.0 user-mode executable}"
[ ! -e "$LIP_OUT" ]
mkdir -p "$LIP_OUT/source" "$LIP_OUT/bin" "$LIP_OUT/units"
cp "$LIP_SOURCE/lip.c" "$LIP_SOURCE/lip.h" "$LIP_OUT/source/"
: > "$LIP_OUT/source/lippar.h"
cp "$LIP_ADAPTER" "$LIP_OUT/source/replay.c"
cp "$(dirname "$LIP_ADAPTER")/bridge.c" "$(dirname "$LIP_ADAPTER")/bridge.h" "$LIP_OUT/source/"
export ZIG_LOCAL_CACHE_DIR="$LIP_OUT/.zig-cache"
: "${ZIG_GLOBAL_CACHE_DIR:=/workspace/nist-freelip-toolchain/global-cache}"
export ZIG_GLOBAL_CACHE_DIR
lip_cc() { "$LIP_ZIG" cc -target x86-linux-musl "$@"; }
lip_cc -std=gnu89 -O0 -fcommon -fwrapv -DSINGLE_MUL -DDOUBLES_LOW_HIGH -include stdlib.h -include string.h -I"$LIP_OUT/source" -c "$LIP_OUT/source/lip.c" -o "$LIP_OUT/bin/lip.o" > "$LIP_OUT/build-legacy.stdout" 2> "$LIP_OUT/build-legacy.stderr"
lip_cc -std=gnu89 -O0 -fcommon -fwrapv -DSINGLE_MUL -DDOUBLES_LOW_HIGH -I"$LIP_OUT/source" -c "$LIP_OUT/source/bridge.c" -o "$LIP_OUT/bin/bridge.o" > "$LIP_OUT/build-bridge.stdout" 2> "$LIP_OUT/build-bridge.stderr"
lip_cc -std=c99 -O2 -Wall -Wextra -Werror -fsanitize=undefined -I"$LIP_OUT/source" "$LIP_OUT/source/replay.c" "$LIP_OUT/bin/lip.o" "$LIP_OUT/bin/bridge.o" -lm -o "$LIP_OUT/bin/replay" > "$LIP_OUT/build-driver.stdout" 2> "$LIP_OUT/build-driver.stderr"
sha256sum "$LIP_OUT/source/"* "$LIP_OUT/bin/"* > "$LIP_OUT/build.sha256"
for lip_method in probprime ordinary; do
  for lip_value in 2 3 65537 2147483647 170141183460469231731687303715884105727 0 1 4 15 341 561 2047 3215031751; do
    case "$lip_value" in 2|3|65537|2147483647|170141183460469231731687303715884105727) lip_expected=1 ;; *) lip_expected=0 ;; esac
    lip_case="$LIP_OUT/units/$lip_method-$lip_value"
    printf '%s\n' "$lip_value" > "$lip_case.input"
    (ulimit -c 0 && ulimit -v 786432 && UBSAN_OPTIONS=halt_on_error=1 timeout --kill-after=5s 30s "$LIP_QEMU" -R 268435456 "$LIP_OUT/bin/replay" "$lip_method" < "$lip_case.input") > "$lip_case.stdout" 2> "$lip_case.stderr"
    test "$(sed -n '2p' "$lip_case.stdout")" = "INPUT $lip_value"
    test "$(sed -n '3p' "$lip_case.stdout")" = "RESULT $lip_method $lip_expected"
  done
done
printf 'Both methods passed all13 generic development fixtures; no historical order executed.\n' > "$LIP_OUT/unit-summary.txt"
