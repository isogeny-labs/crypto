#!/bin/sh
set -eu
cd "$(dirname "$0")"
root=${1:?Specify completed kernel3 study}
good=${2:?Specify qualified good-function result JSON}
out=${3:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient weil3 quotient_theta2 riemann quartic_descent target_weil good_functions; do
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
        target_weil) source=target_weil.cpp; dest=target_weil_arithmetic ;;
        good_functions) source=good_functions.cpp; dest=good_functions_arithmetic ;;
    esac
    sed '/^int main(/,$d' "$source" > "$tmp/$dest.hpp"
done
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" target_kummer_fit.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/target-kummer" 2> "$out/build.stderr"
sha256sum target_kummer_fit.cpp even_kummer.hpp target_contact_algebra.hpp EVEN_KUMMER_PROTOCOL.md EVEN_KUMMER_CHART_AMENDMENT.md "$good" "$root/evidence/quotient-theta2-v1/result.json" "$root/evidence/descent-v1/result.json" "$root/evidence/target-weil-v3/result.json" "$tmp/"*.hpp "$tmp/target-kummer" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/target-kummer"
"$tmp/target-kummer" "$root" "$good" > "$out/result.json" 2> "$out/stderr.txt"
sha256sum target_kummer_fit.cpp even_kummer.hpp target_contact_algebra.hpp > "$out/post-call-source-manifest.sha256"
