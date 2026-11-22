#!/bin/bash
set -eu
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
mkdir "$out/source" "$out/input"
cp evidence/opaque-km-primary-v2/source/*.hpp "$out/source/"
cp relation_pipeline.cpp relation_openssl_check.cpp RELATION_PIPELINE_PROTOCOL.md check-relation-pipeline.sh "$out/source/"
archive=evidence/kernel3-resume-v1.tar.gz
(cd evidence && sha256sum -c kernel3-resume-v1.archive.sha256) > "$out/archive-check.txt"
prefix=kernel3-resume-v1/replay/nist_cover_three_tests_20261116
tar -xzf "$archive" -C "$out/input" --strip-components=3 \
    "$prefix/inputs/p192-d7-prym.json" \
    "$prefix/evidence/p192-d7-kernel3-plane-quartic.json" \
    "$prefix/evidence/destination-relations-v2/result.json"
cp evidence/elliptic-backmap-v1/result.json "$out/input/backmap.json"
cp evidence/public-classes-v2/result.json "$out/input/public-classes.json"
sha256sum "$out/source/"* "$archive" "$out/input/"*.json \
    "$out/input/inputs/"*.json "$out/input/evidence/"*.json > "$out/build-input-manifest.sha256"
set +e
{ time \
    c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" \
    "$out/source/relation_pipeline.cpp" -lgmpxx -lgmp -lcrypto -o "$out/producer" 2> "$out/build.stderr"; } 2> "$out/build-cost.json"
build_exit=$?
printf '%s\n' "$build_exit" > "$out/BUILD_EXIT_CODE.txt"
set -e
test "$build_exit" -eq 0
set +e
{ time \
    c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic -I "$out/source" \
    "$out/source/relation_openssl_check.cpp" -lgmpxx -lgmp -lcrypto -o "$out/checker" 2> "$out/checker-build.stderr"; } 2> "$out/checker-build-cost.json"
build_exit=$?
printf '%s\n' "$build_exit" > "$out/CHECKER_BUILD_EXIT_CODE.txt"
set -e
test "$build_exit" -eq 0
chmod 0555 "$out/producer" "$out/checker"
sha256sum "$out/source/"* "$out/producer" "$out/checker" \
    "$out/input/"*.json "$out/input/inputs/"*.json "$out/input/evidence/"*.json > "$out/pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 600 "$out/producer" "$out/input" \
    "$out/input/backmap.json" "$out/input/public-classes.json" "$out/result.json" \
    > "$out/stdout.txt" 2> "$out/stderr.txt"; } 2> "$out/producer-cost.json")
run_exit=$?
printf '%s\n' "$run_exit" > "$out/EXIT_CODE.txt"
set -e
test "$run_exit" -eq 0
sha256sum "$out/result.json" > "$out/checker-input-manifest.sha256"
set +e
(ulimit -v 1048576; { time timeout 600 "$out/checker" \
    "$out/input/evidence/p192-d7-kernel3-plane-quartic.json" "$out/input/backmap.json" "$out/result.json" \
    > "$out/checker-result.json" 2> "$out/checker.stderr"; } 2> "$out/checker-cost.json")
run_exit=$?
printf '%s\n' "$run_exit" > "$out/CHECKER_EXIT_CODE.txt"
set -e
sha256sum relation_pipeline.cpp relation_openssl_check.cpp RELATION_PIPELINE_PROTOCOL.md \
    check-relation-pipeline.sh > "$out/post-call-source-manifest.sha256"
exit "$run_exit"
