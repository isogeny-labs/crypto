#!/bin/sh
set -eu
cd "$(dirname "$0")"
out=${1:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
sed '/^int main(/,$d' issue_identity.cpp > "$tmp/identity_arithmetic.hpp"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" degree_identity.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/degree-identity" 2> "$out/build.stderr"
sha256sum degree_identity.cpp issue_identity.cpp "$tmp/identity_arithmetic.hpp" "$tmp/degree-identity" > "$out/pre-call-manifest.sha256"
"$tmp/degree-identity" "$out" > "$out/ids.txt" 2> "$out/stderr.txt"
sha256sum degree_identity.cpp issue_identity.cpp > "$out/post-call-source-manifest.sha256"
