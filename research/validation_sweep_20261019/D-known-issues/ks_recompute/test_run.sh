#!/bin/sh
# Verify subprocess failure propagation and successful output labeling.
set -eu
here=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
scratch=$(mktemp -d "${TMPDIR:-/tmp}/ks-run-test.XXXXXX")
trap 'rm -rf "$scratch"' 0
trap 'exit 1' HUP INT TERM
mkdir -p "$scratch/data/case/arm" "$scratch/tmp" "$scratch/bin with spaces"
printf '%s\n' '{"arms":[{"arm":"sample","tests":{"ks_d":0,"ks_p":1}}]}' > "$scratch/data/case/arm/results.json"
printf '%s\n' 'unused fixture' > "$scratch/data/case/arm/trials.csv"
printf '%s\n' '#!/bin/sh' 'exit 42' > "$scratch/bin with spaces/fail"
printf '%s\n' '#!/bin/sh' 'printf "sample\\t1\\t1\\t0\\t1\\t0\\t1\\t0\\t1\\n"' > "$scratch/bin with spaces/pass"
chmod +x "$scratch/bin with spaces/fail" "$scratch/bin with spaces/pass"
if TMPDIR="$scratch/tmp" KS_BIN="$scratch/bin with spaces/fail" sh "$here/run.sh" "$scratch/data" > "$scratch/failed.tsv"; then
  echo 'failed native recomputation was accepted' >&2
  exit 1
else
  status=$?
  test "$status" -eq 42
fi
test ! -s "$scratch/failed.tsv"
test -z "$(ls -A "$scratch/tmp")"
TMPDIR="$scratch/tmp" KS_BIN="$scratch/bin with spaces/pass" sh "$here/run.sh" "$scratch/data" > "$scratch/success.tsv"
printf 'case/arm\tsample\t1\t1\t0\t1\t0\t1\t0\t1\n' > "$scratch/expected.tsv"
cmp "$scratch/expected.tsv" "$scratch/success.tsv"
test -z "$(ls -A "$scratch/tmp")"
echo 'native failure propagation, output labeling, and temporary-file cleanup passed'
