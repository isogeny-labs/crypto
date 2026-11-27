#!/bin/sh
set -eu
cd "$(dirname "$0")"
out=${1:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient weil3 quotient_theta2; do
    case "$item" in
        point) source=point_replay.cpp; dest=point_arithmetic ;;
        cover) source=cover_replay.cpp; dest=cover_arithmetic ;;
        kernel_point) source=kernel_point_replay.cpp; dest=kernel_arithmetic ;;
        module) source=module_audit.cpp; dest=module_arithmetic ;;
        quotient) source=quotient_replay.cpp; dest=quotient_arithmetic ;;
        weil3) source=weil3.cpp; dest=weil3_arithmetic ;;
        quotient_theta2) source=quotient_theta2.cpp; dest=quotient_theta2_arithmetic ;;
    esac
    sed '/^int main(/,$d' "$source" > "$tmp/$dest.hpp"
done
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" riemann_quartic.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/riemann"
sha256sum riemann_quartic.cpp geometry_field.hpp weil_chart.hpp json.hpp "$tmp/"*.hpp "$tmp/riemann" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/riemann"
"$tmp/riemann" "$PWD" > "$out/result.json" 2> "$out/stderr.txt"
sha256sum riemann_quartic.cpp geometry_field.hpp weil_chart.hpp json.hpp > "$out/post-call-source-manifest.sha256"
