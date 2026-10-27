#!/usr/bin/env bash
# Continue terminal build receipts without repeating successful compilation.
# Invoke through the common busy wrapper; no live gate may own these inputs.
set -euo pipefail
zero_old=${1:?reconciled compiled gate}
zero_out=${2:?fresh continuation output}
zero_target=${3:?compiled Cargo target directory}
test ! -e "$zero_out"
test "$(cat "$zero_old/metadata/compile-tests.exit")" = 0
test "$(cat "$zero_old/metadata/build-runner.exit")" = 0
mkdir -p "$zero_out/metadata" "$zero_out/bin"
cp "$0" "$zero_out/metadata/executed-script.sh"
printf '%s\n' "$$" > "$zero_out/metadata/owner-pid.txt"
printf '%s\n' "$zero_old" > "$zero_out/metadata/reused-build-directory.txt"
git rev-parse HEAD > "$zero_out/metadata/source-commit.txt"
date -u +%FT%TZ > "$zero_out/metadata/started-utc.txt"
cp "$zero_old/metadata/sources.sha256" "$zero_out/metadata/sources.sha256"
sha256sum -c "$zero_out/metadata/sources.sha256" > "$zero_out/metadata/sources-unchanged-before.txt"
cp "$zero_target/release/twist-endomorphism-image-study" "$zero_out/bin/study"
cp "$zero_target/release/deps/twist_endomorphism_image_study-b7010094e3901a6f" "$zero_out/bin/native-tests"
chmod 500 "$zero_out/bin/study" "$zero_out/bin/native-tests"
sha256sum "$zero_out/bin/study" > "$zero_out/metadata/runner.sha256"
sha256sum "$zero_out/bin/native-tests" > "$zero_out/metadata/test-runner.sha256"
export RAYON_NUM_THREADS=1
printf '%q ' prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s "$zero_out/bin/native-tests" --test-threads=1 > "$zero_out/metadata/native-tests.command.txt"
if prlimit --as=8589934592 -- timeout --signal=TERM --kill-after=10s 1200s "$zero_out/bin/native-tests" --test-threads=1 > "$zero_out/metadata/native-tests.stdout" 2> "$zero_out/metadata/native-tests.stderr"; then
  zero_rc=0
else
  zero_rc=$?
fi
printf '%s\n' "$zero_rc" > "$zero_out/metadata/native-tests.exit"
sha256sum -c "$zero_out/metadata/sources.sha256" > "$zero_out/metadata/sources-unchanged.txt"
date -u +%FT%TZ > "$zero_out/metadata/finished-utc.txt"
printf '%s\n' "$zero_rc" > "$zero_out/metadata/exit"
tail -n 4 "$zero_out/metadata/native-tests.stdout"
exit "$zero_rc"
