#!/bin/sh
set -eu
cd "$(dirname "$0")"
root=${1:?Specify completed kernel3 study}
global=${2:?Specify global correspondence result}
formal=${3:?Specify fixed formal result}
kernel=${4:?Specify actual kernel result}
bridge=${5:?Specify generic bridge result}
out=${6:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient weil3 quotient_theta2 riemann quartic_descent elliptic_backmap backmap_relations closed_fibers global_correspondence_v2; do
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
        backmap_relations) source=backmap_relations.cpp; dest=backmap_relations_arithmetic ;;
        closed_fibers) source=closed_fibers.cpp; dest=closed_fibers_arithmetic ;;
        global_correspondence_v2) source=global_correspondence_v2.cpp; dest=global_correspondence_v2_arithmetic ;;
    esac
    { printf '#pragma once\n'; sed '/^int main(/,$d' "$source"; } > "$tmp/$dest.hpp"
done
cp degree_polarization.cpp "$out/source.cpp"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" degree_polarization.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/degree-polarization" 2> "$out/build.stderr"
sha256sum degree_polarization.cpp DEGREE_PROTOCOL.md "$global" "$formal" "$kernel" "$bridge" "$root/evidence/p192-d7-kernel3-plane-quartic.json" "$tmp/"*.hpp "$tmp/degree-polarization" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/degree-polarization"
set +e
(ulimit -v 1048576; timeout 600 "$tmp/degree-polarization" "$root" "$global" "$formal" "$kernel" "$bridge") > "$out/result.json" 2> "$out/stderr.txt"
global_exit=$?
set -e
printf '%s\n' "$global_exit" > "$out/EXIT_CODE.txt"
sha256sum degree_polarization.cpp DEGREE_PROTOCOL.md > "$out/post-call-source-manifest.sha256"
exit "$global_exit"
