#!/bin/sh
set -eu
cd "$(dirname "$0")"
root=${1:?Specify completed kernel3 study}
source_image=${2:?Specify qualified source images}
inverse_system=${3:?Specify qualified inverse-system result}
out=${4:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient weil3 quotient_theta2 primitive_field; do
    case "$item" in
        point) source=point_replay.cpp; dest=point_arithmetic ;;
        cover) source=cover_replay.cpp; dest=cover_arithmetic ;;
        kernel_point) source=kernel_point_replay.cpp; dest=kernel_arithmetic ;;
        module) source=module_audit.cpp; dest=module_arithmetic ;;
        quotient) source=quotient_replay.cpp; dest=quotient_arithmetic ;;
        weil3) source=weil3.cpp; dest=weil3_arithmetic ;;
        quotient_theta2) source=quotient_theta2.cpp; dest=quotient_theta2_arithmetic ;;
        primitive_field) source=primitive_field.cpp; dest=primitive_field_arithmetic ;;
    esac
    sed '/^int main(/,$d' "$source" > "$tmp/$dest.hpp"
done
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" primitive_field.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/primitive-field" 2> "$out/build-field.stderr"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" primitive_giac_check.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/primitive-check" 2> "$out/build-check.stderr"
sha256sum primitive_field.cpp primitive_giac_check.cpp PRIMITIVE_SOLVER_PROTOCOL.md "$source_image" "$inverse_system" "$root/inputs/p192-d7-prym.json" "$tmp/"*.hpp "$tmp/primitive-field" "$tmp/primitive-check" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/primitive-field" "$tmp/primitive-check"
"$tmp/primitive-field" "$root" "$source_image" "$inverse_system" "$out/control.giac" > "$out/field.json" 2> "$out/field.stderr"
sha256sum "$out/control.giac" /workspace/giac-local/usr/bin/icas > "$out/pre-backend-manifest.sha256"
set +e
(
    ulimit -v 1048576
    LD_LIBRARY_PATH=/workspace/giac-local/usr/lib/x86_64-linux-gnu timeout --signal=TERM --kill-after=5s 60s /workspace/giac-local/usr/bin/icas "$out/control.giac"
) > "$out/backend.stdout" 2> "$out/backend.stderr"
backend_exit=$?
set -e
printf '%s\n' "$backend_exit" > "$out/BACKEND_EXIT_CODE.txt"
test "$backend_exit" -eq 0
"$tmp/primitive-check" "$root" "$out/backend.stderr" > "$out/check.json" 2> "$out/check.stderr"
sha256sum primitive_field.cpp primitive_giac_check.cpp > "$out/post-call-source-manifest.sha256"
