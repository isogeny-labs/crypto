#!/bin/sh
set -eu
cd "$(dirname "$0")"
root=${1:?Specify completed kernel3 study}
global=${2:?Specify global correspondence result}
bridge=${3:?Specify generic bridge result}
out=${4:?Specify NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
mkdir "$out/source"
cp complete_maps.cpp COMPLETE_MAPS_PROTOCOL.md json.hpp weil_chart.hpp geometry_field.hpp even_kummer.hpp first_order.hpp target_contact_algebra.hpp "$out/source/"
for item in point cover kernel_point module quotient weil3 quotient_theta2 riemann quartic_descent elliptic_backmap backmap_relations closed_fibers destination_relations quartic_km_v3 public_classes generic_bridge; do
    case "$item" in
        point) source=point_replay.cpp; dest=point_arithmetic ;;
        cover) source=cover_replay.cpp; dest=cover_arithmetic ;;
        kernel_point) source=kernel_point_replay.cpp; dest=kernel_arithmetic ;;
        module) source=module_audit.cpp; dest=module_arithmetic ;;
        quotient) source=quotient_replay.cpp; dest=quotient_arithmetic ;;
        riemann) source=riemann_quartic.cpp; dest=riemann_arithmetic ;;
        *) source=$item.cpp; dest=${item}_arithmetic ;;
    esac
    if test "$item" = destination_relations; then
        test "$(rg -c '^template<class Fn> void rejects\(' "$source")" -eq 1
        { printf '#pragma once\n'; sed '/^int main(/,$d' "$source" | sed '/^template<class Fn> void rejects(/,/^}/d'; } > "$out/source/$dest.hpp"
    else
        { printf '#pragma once\n'; sed '/^int main(/,$d' "$source"; } > "$out/source/$dest.hpp"
    fi
done
sha256sum "$out/source/"* > "$out/build-input-manifest.sha256"
set +e
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" "$out/source/complete_maps.cpp" -lgmpxx -lgmp -lcrypto -o "$out/qualifier" 2> "$out/build.stderr"
build_exit=$?
set -e
printf '%s\n' "$build_exit" > "$out/BUILD_EXIT_CODE.txt"
test "$build_exit" -eq 0
chmod 0555 "$out/qualifier"
sha256sum "$out/source/"* "$out/qualifier" "$global" "$bridge" "$root/inputs/p192-d7-prym.json" "$root/evidence/p192-d7-kernel3-plane-quartic.json" "$root/evidence/destination-relations-v2/result.json" > "$out/pre-call-manifest.sha256"
printf '%s\n' "$root" "$global" "$bridge" > "$out/arguments.txt"
set +e
(ulimit -v 1048576; timeout 600 "$out/qualifier" "$root" "$global" "$bridge") > "$out/result.json" 2> "$out/stderr.txt"
run_exit=$?
set -e
printf '%s\n' "$run_exit" > "$out/EXIT_CODE.txt"
sha256sum complete_maps.cpp COMPLETE_MAPS_PROTOCOL.md > "$out/post-call-source-manifest.sha256"
exit "$run_exit"
