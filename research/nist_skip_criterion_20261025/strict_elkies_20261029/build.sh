#!/bin/sh
set -eu
: "${ED_BUILD:?fresh external build directory}"
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
test ! -e "$ED_BUILD"
mkdir -p "$ED_BUILD"
g++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic "$here/producer.cpp" -lgmpxx -lgmp -o "$ED_BUILD/producer"
g++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I/workspace/tools/sage_skeleton_env/include "$here/verify.cpp" -L/workspace/tools/sage_skeleton_env/lib -Wl,-rpath,/workspace/tools/sage_skeleton_env/lib -lpari -o "$ED_BUILD/verify"
