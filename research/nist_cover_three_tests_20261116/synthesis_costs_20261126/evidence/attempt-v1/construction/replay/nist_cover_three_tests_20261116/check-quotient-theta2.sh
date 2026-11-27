#!/bin/sh
set -eu
cd "$(dirname "$0")"
out=${1:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient weil3; do
    case "$item" in
        point) source=point_replay.cpp; dest=point_arithmetic ;;
        cover) source=cover_replay.cpp; dest=cover_arithmetic ;;
        kernel_point) source=kernel_point_replay.cpp; dest=kernel_arithmetic ;;
        module) source=module_audit.cpp; dest=module_arithmetic ;;
        quotient) source=quotient_replay.cpp; dest=quotient_arithmetic ;;
        weil3) source=weil3.cpp; dest=weil3_arithmetic ;;
    esac
    sed '/^int main(/,$d' "$source" > "$tmp/$dest.hpp"
done
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" quotient_theta2.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/quotient-theta2"
sha256sum quotient_theta2.cpp weil_chart.hpp geometry_field.hpp json.hpp "$tmp/"*.hpp "$tmp/quotient-theta2" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/quotient-theta2"
"$tmp/quotient-theta2" "$PWD" > "$out/result.json" 2> "$out/stderr.txt"
sha256sum quotient_theta2.cpp weil_chart.hpp geometry_field.hpp json.hpp > "$out/post-call-source-manifest.sha256"
