#!/bin/sh
# Local development only. This script never reads inputs/cohort.tsv.
set -eu
: "${GB_NATIVE_PREFIX:?set the PARI/GMP/OpenSSL prefix}"
: "${GB_BUILD:?set a fresh external build directory}"
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
test ! -e "$GB_BUILD"
mkdir -p "$GB_BUILD"
cp "$here/analyze.cpp" "$here/verify.cpp" "$here/controls/make_fixture.cpp" "$GB_BUILD/"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -I"$GB_NATIVE_PREFIX/include" \
 "$here/analyze.cpp" -L"$GB_NATIVE_PREFIX/lib" -Wl,-rpath,"$GB_NATIVE_PREFIX/lib" \
 -lpari -lcrypto -o "$GB_BUILD/analyze" > "$GB_BUILD/analyze-build.stdout" 2> "$GB_BUILD/analyze-build.stderr"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -I"$GB_NATIVE_PREFIX/include" \
 "$here/verify.cpp" -L"$GB_NATIVE_PREFIX/lib" -Wl,-rpath,"$GB_NATIVE_PREFIX/lib" \
 -lgmp -lcrypto -o "$GB_BUILD/verify" > "$GB_BUILD/verify-build.stdout" 2> "$GB_BUILD/verify-build.stderr"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -I"$GB_NATIVE_PREFIX/include" \
 "$here/controls/make_fixture.cpp" -L"$GB_NATIVE_PREFIX/lib" -Wl,-rpath,"$GB_NATIVE_PREFIX/lib" \
 -lpari -lcrypto -o "$GB_BUILD/make_fixture" > "$GB_BUILD/fixture-build.stdout" 2> "$GB_BUILD/fixture-build.stderr"
"$GB_BUILD/analyze" --self-test > "$GB_BUILD/analyze-self-test.stdout" 2> "$GB_BUILD/analyze-self-test.stderr"
"$GB_BUILD/verify" --self-test > "$GB_BUILD/verify-self-test.stdout" 2> "$GB_BUILD/verify-self-test.stderr"
"$GB_BUILD/make_fixture" "$GB_BUILD/synthetic-input" > "$GB_BUILD/fixture.stdout" 2> "$GB_BUILD/fixture.stderr"
timeout 60s "$GB_BUILD/analyze" "$GB_BUILD/synthetic-input" "$GB_BUILD/synthetic-output" > "$GB_BUILD/analysis.stdout" 2> "$GB_BUILD/analysis.stderr"
timeout 60s "$GB_BUILD/verify" "$GB_BUILD/synthetic-input" "$GB_BUILD/synthetic-output" "$GB_BUILD/synthetic-verification.json" > "$GB_BUILD/verification.stdout" 2> "$GB_BUILD/verification.stderr"
GB_VERIFY="$GB_BUILD/verify" GB_SYNTH_INPUT="$GB_BUILD/synthetic-input" \
 GB_SYNTH_OUTPUT="$GB_BUILD/synthetic-output" GB_NEGATIVE_OUT="$GB_BUILD/negative-controls" \
 node "$here/controls/negative_controls.mjs" > "$GB_BUILD/negative.stdout" 2> "$GB_BUILD/negative.stderr"
sha256sum "$GB_BUILD/analyze" "$GB_BUILD/verify" "$GB_BUILD/make_fixture" > "$GB_BUILD/binaries.sha256"
