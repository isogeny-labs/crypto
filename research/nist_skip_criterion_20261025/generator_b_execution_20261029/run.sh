#!/bin/sh
set -eu
: "${GB_ANALYZER:?}"
: "${GB_VERIFIER:?}"
: "${GB_RUN:?fresh external run directory}"
: "${NIST_CHECKPOINT_WRAPPER:?authorized checkpoint client wrapper}"
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
b="$here/../generator_b_20261029"
repo=$(git -C "$here" rev-parse --show-toplevel)
test ! -e "$GB_RUN"
mkdir -p "$GB_RUN"
node - "$here/evidence/preparation-delivery.json" <<'JS'
const fs=require('node:fs');const r=JSON.parse(fs.readFileSync(process.argv[2]));
if(r.all_full_get_bytes_verified!==true||r.all_preparation_members_verified!==true)throw Error('Preparation not admitted');
JS
(cd "$b" && sha256sum -c source.sha256 && sha256sum -c input.sha256) > "$GB_RUN/input-source-check.before" 2>&1
test "$(sha256sum "$GB_ANALYZER" | cut -d' ' -f1)" = "$(sed -n '1s/ .*//p' "$b/binaries.sha256")"
test "$(sha256sum "$GB_VERIFIER" | cut -d' ' -f1)" = "$(sed -n '2s/ .*//p' "$b/binaries.sha256")"
"$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" gate > "$GB_RUN/checkpoint.stdout" 2> "$GB_RUN/checkpoint.stderr"
git -C "$repo" rev-parse HEAD > "$GB_RUN/source-commit.txt"
sha256sum "$GB_ANALYZER" "$GB_VERIFIER" > "$GB_RUN/executed-binaries.sha256"
phase() {
 name=$1; limit=$2; shift 2
 set +e
 (ulimit -v 786432; timeout "$limit" env -i PATH=/usr/bin:/bin LANG=C LC_ALL=C "$@") > "$GB_RUN/$name.stdout" 2> "$GB_RUN/$name.stderr"
 status=$?
 set -e
 printf '%s\n' "$status" > "$GB_RUN/$name.status"
 if test "$status" -ne 0; then exit "$status"; fi
}
phase positive 30s "$GB_ANALYZER" --self-test
phase production 300s "$GB_ANALYZER" "$b/inputs" "$GB_RUN/results"
phase verification 300s "$GB_VERIFIER" "$b/inputs" "$GB_RUN/results" "$GB_RUN/verification.json"
(cd "$b" && sha256sum -c source.sha256 && sha256sum -c input.sha256) > "$GB_RUN/input-source-check.after" 2>&1
