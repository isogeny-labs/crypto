#!/bin/sh
set -eu
cd "$(dirname "$0")"
root=${1:?Specify completed kernel3 study}
inverse_system=${2:?Specify qualified inverse-system result}
out=${3:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient weil3 quotient_theta2 riemann quartic_descent target_weil good_functions target_kummer_fit lift_system primitive_field; do
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
        target_kummer_fit) source=target_kummer_fit.cpp; dest=target_kummer_fit_arithmetic ;;
        lift_system) source=lift_system.cpp; dest=lift_system_arithmetic ;;
        primitive_field) source=primitive_field.cpp; dest=primitive_field_arithmetic ;;
    esac
    { printf '#pragma once\n'; sed '/^int main(/,$d' "$source"; } > "$tmp/$dest.hpp"
done
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" inverse_backend_export.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/inverse-export" 2> "$out/build.stderr"
sha256sum inverse_backend_export.cpp INVERSE_BACKEND_PROTOCOL.md "$inverse_system" "$root/inputs/p192-d7-prym.json" "$tmp/"*.hpp "$tmp/inverse-export" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/inverse-export"
"$tmp/inverse-export" "$root" "$inverse_system" "$out/inverse.giac" > "$out/export.json" 2> "$out/export.stderr"
sha256sum "$out/inverse.giac" /workspace/giac-local/usr/bin/icas > "$out/pre-backend-manifest.sha256"
set +e
(
    ulimit -v 1048576
    LD_LIBRARY_PATH=/workspace/giac-local/usr/lib/x86_64-linux-gnu timeout --signal=TERM --kill-after=5s 60s /workspace/giac-local/usr/bin/icas "$out/inverse.giac"
) > "$out/backend.stdout" 2> "$out/backend.stderr"
backend_exit=$?
set -e
printf '%s\n' "$backend_exit" > "$out/BACKEND_EXIT_CODE.txt"
sha256sum inverse_backend_export.cpp > "$out/post-call-source-manifest.sha256"
exit "$backend_exit"
