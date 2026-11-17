#!/usr/bin/env bash
# Restore under both existing locks; require a binary-bound native timeout control.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
id=$1 out=$2 seconds=${3:-1800} option=${4:---resume}
[[ $id == skipped_256_876-twist2 && $seconds =~ ^[1-9][0-9]*$ ]]
[[ $option == --resume || $option == --prepare-only ]]
patched=${YAFU_PATCHED_BIN:?Set the patched native executable}
controls=${YAFU_TIMEOUT_CONTROL_DIR:?Set the completed native control directory}
[[ -x $patched ]]
new_sha=$(sha256sum "$patched"); new_sha=${new_sha%% *}
jq -e --arg sha "$new_sha" '.passed and .original_exit==139 and .patched_exit==0 and
  .patched_integer_timeout_logged and .patched_binary_sha256==$sha' "$controls/summary.json" >/dev/null
mkdir -p "$out"
out=$(cd -- "$out" && pwd)
exec 8> "$out.target.lock"
flock -n -x 8 || { echo 'An active target worker holds this output; refusing tool replacement.' >&2; exit 1; }
exec 9> "$out.lock"
flock -n -x 9 || { echo 'An active native worker holds this output; refusing tool replacement.' >&2; exit 1; }
bash "$here/../rounds/R02-20261022/resume_job.sh" "$id" "$out" "$seconds" --prepare-only
old_sha=$(sha256sum "$out/state/yafu"); old_sha=${old_sha%% *}
if [[ $old_sha != "$new_sha" ]]; then
  event=1
  while [[ -e $out/tool-replacement-$event ]]; do event=$((event+1)); done
  evidence="$out/tool-replacement-$event"
  mkdir "$evidence"
  cp "$out/state/yafu" "$evidence/original-yafu"
  cp "$controls/summary.json" "$evidence/native-control-summary.json"
  cp "$here/timeout-printf.patch" "$evidence/timeout-printf.patch"
  cp "$patched" "$out/state/yafu.new"
  copied_sha=$(sha256sum "$out/state/yafu.new"); copied_sha=${copied_sha%% *}
  [[ $copied_sha == "$new_sha" ]]
  chmod u+x "$out/state/yafu.new"
  mv "$out/state/yafu.new" "$out/state/yafu"
  date -u +%FT%TZ > "$evidence/installed-at.txt"
  jq -n --arg old "$old_sha" --arg new "$new_sha" '{old_binary_sha256:$old,new_binary_sha256:$new,
    previous_binary_retained:true,binary_bound_native_timeout_control_passed:true,
    relation_or_polynomial_files_modified:false}' > "$evidence/receipt.json"
fi
flock -u 9; exec 9>&-
flock -u 8; exec 8>&-
if [[ $option == --prepare-only ]]; then
  jq -n --arg sha "$new_sha" '{prepared:true,patched_binary_sha256:$sha,search_executed:false}'
  exit 0
fi
bash "$here/../rounds/R04-20261022/resume_with_checkpoint.sh" "$id" "$out" "$seconds" --resume
