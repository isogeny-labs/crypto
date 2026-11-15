#!/bin/sh
# Native jq makes malformed fixtures; the C++ converter must refuse each one.
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
build=${1:?usage: test-native-preparation.sh BUILD FRESH_TEST_DIR}
out=${2:?usage: test-native-preparation.sh BUILD FRESH_TEST_DIR}
test ! -e "$out"
mkdir -p "$out"
"$build/copy-score-fields" "$here/sources/heldout.json" "$out/copied.tsv"
cmp "$here/inputs/external-score-rows.tsv" "$out/copied.tsv"
sha256sum "$out/copied.tsv" > "$out/copied.sha256"
refuse() {
 name=$1
 filter=$2
 jq "$filter" "$here/sources/heldout.json" > "$out/$name.json"
 if "$build/copy-score-fields" "$out/$name.json" "$out/$name.tsv" > "$out/$name.stdout" 2> "$out/$name.stderr"; then
  echo "Unexpected acceptance: $name" >&2
  exit 1
 else
  status=$?
  test "$status" -eq 1
  test ! -e "$out/$name.tsv"
 fi
 printf '%s\trefused1\tno_output\n' "$name"
}
refuse missing-score 'del(.heldout[0].split)'
refuse unknown-label '.heldout[0].label = "unknown"'
refuse duplicate-identity '.heldout[1] = .heldout[0]'
refuse noninteger-score '.heldout[0].split = 55.5'
refuse wrong-cutoff '.cutoff = 692'
refuse wrong-count '.heldout |= .[1:]'
if "$build/copy-score-fields" "$here/sources/heldout.json" "$out/copied.tsv" > "$out/overwrite.stdout" 2> "$out/overwrite.stderr"; then
 echo 'Unexpected overwrite' >&2
 exit 1
else
 test "$?" -eq 1
fi
cmp "$here/inputs/external-score-rows.tsv" "$out/copied.tsv"
printf 'overwrite\trefused1\tbytes_unchanged\n'
printf 'native_preparation\tpass\t68_rows_byte_identical\n'
