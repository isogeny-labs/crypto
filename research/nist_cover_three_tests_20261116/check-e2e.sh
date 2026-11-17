#!/bin/sh
# Complete development correctness suite. It makes no P-192 speed claim.
set -eu
cd "$(dirname "$0")"
out=${1:?Specify a NEW output directory}
mkdir "$out"
out=$(cd "$out" && pwd)
./check.sh "$out/p192-cover"
./check-quotient.sh "$out/p192-quotient"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
for item in point cover kernel_point module quotient; do
    case "$item" in
        point) source=point_replay.cpp; dest=point_arithmetic ;;
        cover) source=cover_replay.cpp; dest=cover_arithmetic ;;
        kernel_point) source=kernel_point_replay.cpp; dest=kernel_arithmetic ;;
        module) source=module_audit.cpp; dest=module_arithmetic ;;
        quotient) source=quotient_replay.cpp; dest=quotient_arithmetic ;;
    esac
    sed '/^int main(/,$d' "$source" > "$tmp/$dest.hpp"
done
flags='-std=c++17 -O2 -Wall -Wextra -Werror -pedantic'
${CXX:-c++} $flags -I . -I "$tmp" roundtrip_receipt.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/receipt"
${CXX:-c++} $flags openssl_roundtrip_check.cpp -lcrypto -o "$tmp/checker"
mkdir "$out/p192-openssl"
sha256sum roundtrip_receipt.cpp openssl_roundtrip_check.cpp json.hpp "$tmp/"*.hpp "$tmp/receipt" "$tmp/checker" > "$out/p192-openssl/pre-call-manifest.sha256"
chmod 0555 "$tmp/receipt" "$tmp/checker"
"$tmp/receipt" "$PWD" > "$out/p192-openssl/points.jsonl" 2> "$out/p192-openssl/producer.stderr"
"$tmp/checker" "$out/p192-openssl/points.jsonl" > "$out/p192-openssl/checker.txt" 2> "$out/p192-openssl/checker.stderr"
if "$tmp/checker" "$out/p192-openssl/points.jsonl" --canary > "$out/p192-openssl/canary.stdout" 2> "$out/p192-openssl/canary.stderr"; then
    echo 'ERROR: OpenSSL accepted a false P-192 point' >&2; exit 1
fi
cat "$out/p192-openssl/checker.txt" "$out/p192-openssl/canary.stderr"
./check-toy.sh "$out/toy"
printf '%s\n' 'PASS complete native development suite: P-192 maps and two public small-curve IC recoveries; no P-192 DLP solve or speed result.' > "$out/COMPLETE.txt"
cat "$out/COMPLETE.txt"
