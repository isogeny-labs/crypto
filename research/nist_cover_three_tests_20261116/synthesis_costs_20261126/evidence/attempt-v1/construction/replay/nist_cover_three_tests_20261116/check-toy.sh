#!/bin/sh
# Public functional correctness fixtures, not a performance experiment.
set -eu
cd "$(dirname "$0")"
out=${1:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
# Exact, audited adapters: general source coefficient a and working-field Frobenius.
sed '/^int main(/,$d' point_replay.cpp | sed 's/(F(3)\*x\*x-F(3))/(F(3)*x*x+toy_source_a())/' > "$tmp/point_arithmetic.hpp"
test "$(rg -o 'toy_source_a' "$tmp/point_arithmetic.hpp" | wc -l)" -eq 1
sed '/^int main(/,$d' cover_replay.cpp > "$tmp/cover_arithmetic.hpp"
sed '/^int main(/,$d' kernel_point_replay.cpp > "$tmp/kernel_arithmetic.hpp"
sed '/^int main(/,$d' module_audit.cpp > "$tmp/module_arithmetic.hpp"
sed '/^int main(/,$d' quotient_replay.cpp | sed 's/gfrob(k)/toy_kernel_frobenius(k)/' > "$tmp/quotient_arithmetic.hpp"
test "$(rg -o 'toy_kernel_frobenius' "$tmp/quotient_arithmetic.hpp" | wc -l)" -eq 1
sed '/^int main(/,$d' openssl_roundtrip_check.cpp > "$tmp/openssl_core.hpp"
flags='-std=c++17 -O2 -Wall -Wextra -Werror -pedantic'
${CXX:-c++} $flags -I . -I "$tmp" toy_ic_v4.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/toy-ic"
${CXX:-c++} $flags -I . -I "$tmp" openssl_toy_check.cpp -lcrypto -o "$tmp/openssl-toy-check"
sha256sum toy_ic_v4.cpp openssl_toy_check.cpp json.hpp "$tmp/"*.hpp "$tmp/toy-ic" "$tmp/openssl-toy-check" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/toy-ic" "$tmp/openssl-toy-check"
for p in 109 1009; do
    mkdir "$out/$p"
    "$tmp/toy-ic" "$p" "$out/$p/public-witness.json" > "$out/$p/result.json" 2> "$out/$p/stderr.txt"
    "$tmp/openssl-toy-check" "$out/$p/public-witness.json" > "$out/$p/openssl-check.txt" 2> "$out/$p/openssl-check.stderr"
    if "$tmp/openssl-toy-check" "$out/$p/public-witness.json" --canary > "$out/$p/canary.stdout" 2> "$out/$p/canary.stderr"; then
        echo 'ERROR: OpenSSL accepted corrupted relation' >&2; exit 1
    fi
    cat "$out/$p/result.json" "$out/$p/openssl-check.txt" "$out/$p/canary.stderr"
done
sha256sum toy_ic_v4.cpp openssl_toy_check.cpp json.hpp > "$out/post-call-source-manifest.sha256"
