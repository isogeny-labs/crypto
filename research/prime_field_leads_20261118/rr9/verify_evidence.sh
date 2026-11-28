#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
sha256sum -c evidence.sha256
tmp="$(mktemp -d)"
trap 'rm -rf -- "$tmp"' EXIT
g++ -Wall -Wextra -Wno-return-type -std=c++20 -O3 verify.cpp -o "$tmp/verify"
"$tmp/verify" evidence/neg1.reduced.txt 625037715 2100627251 reduced
"$tmp/verify" evidence/neg2.reduced.txt 1923055775 307731458 reduced
"$tmp/verify" evidence/pos.reduced.txt 1911579686 548820096 reduced
if diagnostic="$("$tmp/verify" evidence/neg1.reduced.txt 625037715 2100627251 reduced mutate 2>&1)"; then
    printf '%s\n' 'mutated reduced certificate accepted' >&2
    exit 1
fi
test "$diagnostic" = "identity does not equal one"
printf 'negative control rejected: %s\n' "$diagnostic"
