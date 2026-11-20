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
sha256sum "$out/source/"* "$out/reviewer" "$root/$manifest" \
    "$root/$study/evidence/relation-pipeline-v3/result.json" \
    "$root/$study/evidence/relation-pipeline-v3/checker-result.json" \
    "$root/$study/evidence/relation-transfer-v1/result.json" \
    "$root/$study/evidence/relation-transfer-v1/checker-result.json" > "$out/pre-call-manifest.sha256"
views=$(mktemp -d)
relocation=$(mktemp -d)
trap 'rm -rf "$views" "$relocation"' EXIT
ln -s "$root/$study/evidence/relation-pipeline-v3" "$views/relations"
ln -s "$root/$study/evidence/relation-transfer-v1" "$views/transfer"
set +e
(ulimit -v 1048576; timeout 60 "$out/reviewer" "$root" \
    "$root/$study/evidence/relation-e2e-v1" "$views" "$manifest") \
    > "$out/result.json" 2> "$out/stderr.txt"
status=$?
printf '%s\n' "$status" > "$out/EXIT_CODE.txt"
set -e
test "$status" -eq 0
mkdir "$relocation/checkout" "$relocation/observed" "$relocation/observed/relations" "$relocation/observed/transfer"
while IFS= read -r line; do
    name=${line:66}
    (cd "$root" && cp --parents "$name" "$relocation/checkout")
done < "$root/$manifest"
(cd "$root" && cp --parents "$manifest" "$relocation/checkout")
cp "$views/relations/result.json" "$views/relations/checker-result.json" "$relocation/observed/relations/"
cp "$views/transfer/result.json" "$views/transfer/checker-result.json" "$relocation/observed/transfer/"
sha256sum "$relocation/checkout/$manifest" "$relocation/observed/relations/"*.json \
    "$relocation/observed/transfer/"*.json > "$out/relocated-pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; timeout 60 "$out/reviewer" "$relocation/checkout" \
    "$relocation/checkout/$study/evidence/relation-e2e-v1" "$relocation/observed" "$manifest") \
    > "$out/relocated-result.json" 2> "$out/relocated.stderr"
status=$?
printf '%s\n' "$status" > "$out/RELOCATED_EXIT_CODE.txt"
set -e
sha256sum review_relation_replay.cpp PORTABLE_REPLAY_PROTOCOL.md check-portable-replay.sh > "$out/post-call-source-manifest.sha256"
exit "$status"
