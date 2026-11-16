#!/bin/sh
set -eu
cd "$(dirname "$0")"
root=${1:?Specify completed kernel3 study}
formal=${2:?Specify fixed formal-result}
out=${3:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient weil3 quotient_theta2 riemann quartic_descent elliptic_backmap; do
    case "$item" in
        point) source=point_replay.cpp; dest=point_arithmetic ;;
        cover) source=cover_replay.cpp; dest=cover_arithmetic ;;
        kernel_point) source=kernel_point_replay.cpp; dest=kernel_arithmetic ;;
        module) source=module_audit.cpp; dest=module_arithmetic ;;
        quotient) source=quotient_replay.cpp; dest=quotient_arithmetic ;;
        weil3) source=weil3.cpp; dest=weil3_arithmetic ;;
        quotient_theta2) source=quotient_theta2.cpp; dest=quotient_theta2_arithmetic ;;
        riemann) source=riemann_quartic.cpp; dest=riemann_arithmetic ;;
        quartic_descent) source=quartic_descent.cpp; dest=quartic_descent_arithmetic ;;
        elliptic_backmap) source=elliptic_backmap.cpp; dest=elliptic_backmap_arithmetic ;;
    esac
    { printf '#pragma once\n'; sed '/^int main(/,$d' "$source"; } > "$tmp/$dest.hpp"
done
cp global_correspondence.cpp "$out/source.cpp"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" global_correspondence.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/global-correspondence" 2> "$out/build.stderr"
sha256sum global_correspondence.cpp GLOBAL_CORRESPONDENCE_PROTOCOL.md "$formal" "$root/evidence/p192-d7-kernel3-plane-quartic.json" "$tmp/"*.hpp "$tmp/global-correspondence" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/global-correspondence"
set +e
(ulimit -v 1048576; timeout 600 "$tmp/global-correspondence" "$root" "$formal") > "$out/result.json" 2> "$out/stderr.txt"
global_exit=$?
set -e
printf '%s\n' "$global_exit" > "$out/EXIT_CODE.txt"
sha256sum global_correspondence.cpp GLOBAL_CORRESPONDENCE_PROTOCOL.md > "$out/post-call-source-manifest.sha256"
exit "$global_exit"
