#!/bin/bash
set -euo pipefail
test "$#" -eq 2
root=$1
out=$2
study=$root/research/nist_cover_three_tests_20261116
c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" \
    "$out/source/replay_certificate.cpp" -lgmpxx -lgmp -lcrypto -o "$out/bin/replay-certificate" 2> "$out/certificate-build.stderr"
c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic "$study/review_relation_replay.cpp" \
    -lcrypto -o "$out/bin/portable-reviewer" 2> "$out/reviewer-build.stderr"
chmod 0555 "$out/bin/replay-certificate" "$out/bin/portable-reviewer"
sha256sum "$out/bin/replay-certificate" "$out/bin/portable-reviewer" > "$out/adapter-binary-manifest.sha256"
cd "$study"
./check-relation-e2e.sh "$out/native"
"$out/bin/portable-reviewer" "$root" "$study/evidence/relation-e2e-v1" "$out/native" \
    research/nist_cover_three_tests_20261116/evidence/relation-portable-inputs-v1.sha256 \
    > "$out/comparison.json" 2> "$out/comparison.stderr"
