#!/bin/bash
set -eu
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
root=$(git rev-parse --show-toplevel)
study=research/nist_cover_three_tests_20261116
mkdir "$out/source"
cp review_relation_replay.cpp PORTABLE_REPLAY_PROTOCOL.md check-portable-replay.sh "$out/source/"
sha256sum "$out/source/"* > "$out/build-input-manifest.sha256"
set +e
c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic "$out/source/review_relation_replay.cpp" \
    -lcrypto -o "$out/reviewer" 2> "$out/build.stderr"
status=$?
printf '%s\n' "$status" > "$out/BUILD_EXIT_CODE.txt"
set -e
test "$status" -eq 0
chmod 0555 "$out/reviewer"
manifest=$study/evidence/relation-portable-inputs-v1.sha256
sha256sum "$out/source/"* "$out/reviewer" "$root/$manifest" > "$out/pre-call-manifest.sha256"
views=$(mktemp -d)
trap 'rm -rf "$views"' EXIT
ln -s "$root/$study/evidence/relation-pipeline-v3" "$views/relations"
ln -s "$root/$study/evidence/relation-transfer-v1" "$views/transfer"
set +e
(ulimit -v 1048576; timeout 60 "$out/reviewer" "$root" \
    "$root/$study/evidence/relation-e2e-v1" "$views" "$manifest") \
    > "$out/result.json" 2> "$out/stderr.txt"
status=$?
printf '%s\n' "$status" > "$out/EXIT_CODE.txt"
set -e
sha256sum review_relation_replay.cpp PORTABLE_REPLAY_PROTOCOL.md check-portable-replay.sh > "$out/post-call-source-manifest.sha256"
exit "$status"
