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
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" weil_chart.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/weil-chart"
${CXX:-c++} -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I . -I "$tmp" chart_receipt_check.cpp -lgmpxx -lgmp -o "$tmp/checker"
sha256sum weil_chart.cpp weil_chart.hpp json.hpp "$tmp/"*.hpp "$tmp/weil-chart" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/weil-chart"
"$tmp/weil-chart" "$PWD" > "$out/result.json" 2> "$out/stderr.txt"
"$tmp/checker" inputs/p192-d7-prym.json evidence/weil3-v2/result.json "$out/result.json" > "$out/checker.txt"
if "$tmp/checker" inputs/p192-d7-prym.json evidence/weil3-v2/result.json "$out/result.json" --canary > "$out/checker-canary.stdout" 2> "$out/checker-canary.stderr"; then
    echo 'ERROR: accepted altered special-chart value' >&2
    exit 1
fi
sha256sum weil_chart.cpp weil_chart.hpp json.hpp > "$out/post-call-source-manifest.sha256"
