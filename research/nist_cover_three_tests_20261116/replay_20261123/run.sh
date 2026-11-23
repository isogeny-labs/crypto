#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
test "$#" -eq 1
mkdir "$1"
out=$(cd "$1" && pwd)
root=$(git rev-parse --show-toplevel)
study=$root/research/nist_cover_three_tests_20261116
revision=$(git rev-parse HEAD)
mkdir "$out/source" "$out/bin"
cp process_clock.cpp replay_certificate.cpp run.sh body.sh PROTOCOL.md "$out/source/"
cp "$study/json.hpp" "$out/source/"
{ printf '#pragma once\n'; sed '/^int main(/,$d' "$study/relation_openssl_check.cpp"; } > "$out/source/replay_openssl.hpp"
{ printf '#pragma once\n'; sed '/^int main(/,$d' "$study/review_relation_replay.cpp"; } > "$out/source/portable_arithmetic.hpp"
(cd "$root" && sha256sum -c "$study/evidence/relation-portable-inputs-v1.sha256" && \
    sha256sum -c "$study/replay_20261123/LEGACY_INPUTS.sha256") > "$out/input-bindings.txt"
sha256sum "$out/source/"* > "$out/build-input-manifest.sha256"
TIMEFORMAT='{"wall_seconds":%R,"user_seconds":%U,"system_seconds":%S}'
set +e
{ time c++ -std=c++17 -O2 -Wall -Wextra -Werror -pedantic "$out/source/process_clock.cpp" \
    -o "$out/bin/process-clock" 2> "$out/clock-build.stderr"; } 2> "$out/clock-build-cost.json"
status=$?
printf '%s\n' "$status" > "$out/CLOCK_BUILD_EXIT_CODE.txt"
set -e
test "$status" -eq 0
chmod 0555 "$out/bin/process-clock"
sha256sum "$out/source/"* "$out/bin/process-clock" "$study/replay_20261123/LEGACY_INPUTS.sha256" \
    "$study/evidence/relation-portable-inputs-v1.sha256" > "$out/pre-call-manifest.sha256"
set +e
"$out/bin/process-clock" "$out/exit7-clock.json" /bin/sh -c 'exit 7' \
    > "$out/exit7.stdout" 2> "$out/exit7.stderr"
status=$?
set -e
test "$status" -eq 7
set +e
(ulimit -v 1048576; "$out/bin/process-clock" "$out/pipeline-clock.json" \
    timeout --kill-after=5 1800 bash "$out/source/body.sh" "$root" "$out") \
    > "$out/body.stdout" 2> "$out/body.stderr"
status=$?
printf '%s\n' "$status" > "$out/PIPELINE_EXIT_CODE.txt"
set -e
test "$status" -eq 0
sha256sum "$out/bin/replay-certificate" "$out/native/relations/"*result.json \
    "$out/native/transfer/"*result.json "$out/comparison.json" "$out/pipeline-clock.json" > "$out/certificate-pre-call-manifest.sha256"
set +e
(ulimit -v 1048576; "$out/bin/process-clock" "$out/certificate-clock.json" \
    timeout 60 "$out/bin/replay-certificate" "$root" "$out" "$revision") \
    > "$out/certificate-summary.json" 2> "$out/certificate.stderr"
status=$?
printf '%s\n' "$status" > "$out/CERTIFICATE_EXIT_CODE.txt"
set -e
sha256sum process_clock.cpp replay_certificate.cpp run.sh body.sh PROTOCOL.md > "$out/post-call-source-manifest.sha256"
exit "$status"
