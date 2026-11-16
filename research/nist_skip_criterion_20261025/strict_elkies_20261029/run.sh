#!/bin/sh
set -eu
: "${ED_BUILD:?admitted binaries}"
: "${ED_RUN:?fresh external run directory}"
: "${NIST_CHECKPOINT_WRAPPER:?authorized checkpoint wrapper}"
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
test ! -e "$ED_RUN"
mkdir -p "$ED_RUN"
(cd "$here" && sha256sum -c source.sha256 && sha256sum -c inputs.sha256 && sha256sum -c binaries.sha256) > "$ED_RUN/bindings.before" 2>&1
node - "$here/evidence/preparation-delivery.json" <<'JS'
const fs=require('node:fs');const r=JSON.parse(fs.readFileSync(process.argv[2]));
if(r.full_get_sha256_verified!==true||r.all_member_bytes_match_original!==true)throw Error('preparation not admitted');
JS
"$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" gate > "$ED_RUN/checkpoint.stdout" 2> "$ED_RUN/checkpoint.stderr"
git -C "$repo" rev-parse HEAD > "$ED_RUN/source-commit.txt"
phase() {
  name=$1; limit=$2; shift 2
  set +e
  (ulimit -v 1572864; timeout "$limit" env -i PATH=/usr/bin:/bin LANG=C LC_ALL=C "$@") > "$ED_RUN/$name.stdout" 2> "$ED_RUN/$name.stderr"
  status=$?
  set -e
  printf '%s\n' "$status" > "$ED_RUN/$name.status"
  test "$status" -eq 0 || exit "$status"
}
phase positive 30s "$ED_BUILD/producer" --self-test
phase production 120s "$ED_BUILD/producer" "$here/inputs.tsv" "$ED_RUN/results"
phase verification 120s "$ED_BUILD/verify" "$here/inputs.tsv" "$ED_RUN/results" "$ED_RUN/verification.json"
for kind in character calibration summary; do
  target="$ED_RUN/negative-$kind"
  cp -a "$ED_RUN/results" "$target"
  node "$here/tamper.mjs" "$target" "$kind"
  set +e
  (ulimit -v 1572864; timeout 30s env -i PATH=/usr/bin:/bin LANG=C LC_ALL=C "$ED_BUILD/verify" "$here/inputs.tsv" "$target" "$ED_RUN/negative-$kind-receipt.json") > "$ED_RUN/negative_$kind.stdout" 2> "$ED_RUN/negative_$kind.stderr"
  status=$?
  set -e
  printf '%s\n' "$status" > "$ED_RUN/negative_$kind.status"
  test "$status" -eq 1 || exit 1
done
(cd "$here" && sha256sum -c source.sha256 && sha256sum -c inputs.sha256 && sha256sum -c binaries.sha256) > "$ED_RUN/bindings.after" 2>&1
printf 'Complete: all three main phases and all three expected refusals\n'
