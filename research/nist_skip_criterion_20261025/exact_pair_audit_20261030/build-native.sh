#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
build=${1:?usage: build-native.sh FRESH_BUILD}
# Keep the original three-program build, then add native preparation.
sh "$here/build.sh" "$build"
g++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic "$here/summarize.cpp" -o "$build/summarize"
# Word splitting is intentional for pkg-config's compiler/linker arguments.
g++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic $(pkg-config --cflags json-c) \
 "$here/copy-score-fields.cpp" $(pkg-config --libs json-c) -o "$build/copy-score-fields"
sha256sum "$here/copy-score-fields.cpp" "$here/build-native.sh" "$here/summarize.cpp" \
 "$build/copy-score-fields" "$build/summarize" > "$build/native-preparation-sha256.txt"
