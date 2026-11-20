#!/bin/sh
set -eu
cd "$(dirname "$0")"
root=${1:?Specify completed kernel3 study}
global=${2:?Specify qualified global correspondence}
formal=${3:?Specify fixed formal receipt}
public=${4:?Specify qualified public class receipt}
out=${5:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient weil3 quotient_theta2 riemann quartic_descent elliptic_backmap backmap_relations closed_fibers destination_relations quartic_km_v3 public_classes; do
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
        destination_relations) source=destination_relations.cpp; dest=destination_relations_arithmetic ;;
        quartic_km_v3) source=quartic_km_v3.cpp; dest=quartic_km_v3_arithmetic ;;
        public_classes) source=public_classes.cpp; dest=public_classes_arithmetic ;;
    esac
    if test "$item" = destination_relations; then
        test "$(rg -c '^template<class Fn> void rejects\(' "$source")" -eq 1
        { printf '#pragma once\n'; sed '/^int main(/,$d' "$source" | sed '/^template<class Fn> void rejects(/,/^}/d'; } > "$tmp/$dest.hpp"
    else
        { printf '#pragma once\n'; sed '/^int main(/,$d' "$source"; } > "$tmp/$dest.hpp"
    fi
done
cp generic_bridge.cpp "$out/source.cpp"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" generic_bridge.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/generic-bridge" 2> "$out/build.stderr"
sha256sum generic_bridge.cpp GENERIC_BRIDGE_PROTOCOL.md "$global" "$formal" "$public" "$root/evidence/p192-d7-kernel3-plane-quartic.json" "$root/evidence/destination-relations-v2/result.json" "$tmp/"*.hpp "$tmp/generic-bridge" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/generic-bridge"
set +e
(ulimit -v 1048576; timeout 600 "$tmp/generic-bridge" "$root" "$global" "$formal" "$public") > "$out/result.json" 2> "$out/stderr.txt"
class_exit=$?
set -e
printf '%s\n' "$class_exit" > "$out/EXIT_CODE.txt"
sha256sum generic_bridge.cpp GENERIC_BRIDGE_PROTOCOL.md > "$out/post-call-source-manifest.sha256"
exit "$class_exit"
