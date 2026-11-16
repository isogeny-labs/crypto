#!/bin/sh
# No fallback, implicit retry, or production execution before the checkpoint gate.
set -eu
: "${GB_ANALYZER:?set the admitted producer binary}"
: "${GB_VERIFIER:?set the admitted independent verifier binary}"
: "${GB_OUT:?set a fresh external result directory}"
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
test ! -e "$GB_OUT"
mkdir -p "$GB_OUT"
checkpoint() {
 if test -n "${NIST_CHECKPOINT_WRAPPER:-}"; then
  "$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" "$@"
 else
  "$repo/scripts/checkpoint.sh" "$@"
 fi
}
# The preparation receipt is absent until the actual archive has a full remote
# byte/member check. A local-only bundle is not that receipt.
test -s "$here/evidence/preparation-delivery.json"
node - "$here/evidence/preparation-delivery.json" <<'JS'
const fs=require('node:fs');const r=JSON.parse(fs.readFileSync(process.argv[2]));
if(r.all_full_get_bytes_verified!==true||r.all_preparation_members_verified!==true)throw Error('Preparation delivery is not admitted');
JS
(cd "$here" && sha256sum -c input.sha256) > "$GB_OUT/input-check.before" 2>&1
(cd "$here" && sha256sum -c source.sha256) > "$GB_OUT/source-check.before" 2>&1
checkpoint gate > "$GB_OUT/checkpoint-gate.stdout" 2> "$GB_OUT/checkpoint-gate.stderr"
git -C "$repo" rev-parse HEAD > "$GB_OUT/source-commit.txt"
sha256sum "$GB_ANALYZER" "$GB_VERIFIER" > "$GB_OUT/executed-binaries.sha256"
test "$(sha256sum "$GB_ANALYZER" | cut -d' ' -f1)" = "$(sed -n '1s/ .*//p' "$here/binaries.sha256")"
test "$(sha256sum "$GB_VERIFIER" | cut -d' ' -f1)" = "$(sed -n '2s/ .*//p' "$here/binaries.sha256")"
set +e
(ulimit -v 786432; timeout 300s env -i PATH=/usr/bin:/bin LANG=C LC_ALL=C "$GB_ANALYZER" "$here/inputs" "$GB_OUT/results") > "$GB_OUT/analysis.stdout" 2> "$GB_OUT/analysis.stderr"
status=$?
set -e
printf '%s\n' "$status" > "$GB_OUT/analysis.status"
if test "$status" -ne 0; then checkpoint push > "$GB_OUT/failure-checkpoint.log" 2>&1 || true; exit "$status"; fi
set +e
(ulimit -v 786432; timeout 300s env -i PATH=/usr/bin:/bin LANG=C LC_ALL=C "$GB_VERIFIER" "$here/inputs" "$GB_OUT/results" "$GB_OUT/verification.json") > "$GB_OUT/verification.stdout" 2> "$GB_OUT/verification.stderr"
status=$?
set -e
printf '%s\n' "$status" > "$GB_OUT/verification.status"
(cd "$here" && sha256sum -c input.sha256) > "$GB_OUT/input-check.after" 2>&1
checkpoint push > "$GB_OUT/final-checkpoint.log" 2>&1
exit "$status"
