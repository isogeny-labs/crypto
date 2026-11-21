#!/bin/sh
set -eu
cd "$(dirname "$0")"
root=${1:?Specify completed kernel3 study}
out=${2:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient weil3 quotient_theta2 riemann quartic_descent; do
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
    esac
    sed '/^int main(/,$d' "$source" > "$tmp/$dest.hpp"
done
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" elliptic_backmap.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/elliptic-backmap" 2> "$out/build.stderr"
sha256sum elliptic_backmap.cpp ELLIPTIC_BACKMAP_PROTOCOL.md "$root/evidence/p192-d7-kernel3-plane-quartic.json" "$root/evidence/differential-twist-v2/result.json" "$root/evidence/destination-relations-v2/result.json" "$tmp/"*.hpp "$tmp/elliptic-backmap" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/elliptic-backmap"
set +e
"$tmp/elliptic-backmap" "$root" "$out/candidate.json" > "$out/result.json" 2> "$out/stderr.txt"
backmap_exit=$?
set -e
printf '%s\n' "$backmap_exit" > "$out/EXIT_CODE.txt"
sha256sum elliptic_backmap.cpp > "$out/post-call-source-manifest.sha256"
exit "$backmap_exit"
