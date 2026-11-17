#!/usr/bin/env bash
set -euo pipefail
umask 077
proposal=/home/ec2-user/ml-dataset-resume-20261029/original-owned-search-phase-accounting-proposal-20261107
transport=/home/ec2-user/ml-dataset-resume-20261029/original-owned-search-transport-proposal-20261107
source_root=/home/ec2-user/ml-dataset-resume-20261029/sat-cli-native-recovery-20261107/source
review_root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
out=$review_root/assembly
mkdir "$out"
mkdir "$out/snapshots" "$out/staged" "$out/logs" "$out/phase-delta"
: > "$out/INPUTS.jsonl"
# Bound reads and same-copied-byte hash checks. Trusted fixed metadata only.
snapshot() {
  local kind=$1 input=$2 want_hash=$3 want_bytes=$4 copy_hash copy_bytes
  test -f "$input" && test ! -L "$input"
  test "$want_bytes" -le 2097152
  local destination=$out/snapshots/$want_hash
  if test ! -e "$destination"; then
    dd if="$input" of="$destination" bs=65536 count=33 iflag=nofollow,nonblock status=none
  fi
  copy_bytes=$(stat -c %s "$destination")
  copy_hash=$(sha256sum "$destination"); copy_hash=${copy_hash%% *}
  test "$copy_hash" = "$want_hash" && test "$copy_bytes" = "$want_bytes"
  jq -nc --arg kind "$kind" --arg path "$input" --arg sha256 "$copy_hash" --argjson bytes "$copy_bytes" '{kind:$kind,path:$path,sha256:$sha256,bytes:$bytes}' >> "$out/INPUTS.jsonl"
}
snapshot phase-manifest "$proposal/MANIFEST.json" 4f4d1fe5968c0f9da3ff800ace32e1c656b34d8f7936a314e7dfa26841a39150 2040
snapshot transport-manifest "$transport/MANIFEST.json" edde080415bea86cdec339188b744e4c8553a02c0b7a172e6a93a9b6ed43f387 1608
for manifest_name in phase transport; do
  if test "$manifest_name" = phase; then packet=$proposal; hash=4f4d1fe5968c0f9da3ff800ace32e1c656b34d8f7936a314e7dfa26841a39150; else packet=$transport; hash=edde080415bea86cdec339188b744e4c8553a02c0b7a172e6a93a9b6ed43f387; fi
  while IFS=$'\t' read -r name digest bytes; do
    [[ "$name" =~ ^[A-Za-z0-9_.-]+$ ]]
    snapshot "$manifest_name-member" "$packet/$name" "$digest" "$bytes"
  done < <(jq -r '.files|to_entries[]|[.key,.value.sha256,.value.bytes]|@tsv' "$out/snapshots/$hash")
done
phase_bindings=$out/snapshots/15f37ece72c1386b1d7ef17919ecb5e04f54f11dd54e6e751096b2bcb8eade6f
while IFS=$'\t' read -r path digest bytes; do snapshot phase-binding "$path" "$digest" "$bytes"; done < <(jq -r '[.base_adapter,.base_source_bindings,.corrected_registration,.budget]+.inherited_source_bindings|.[]|[.path,.sha256,.bytes]|@tsv' "$phase_bindings")
transport_bindings=$out/snapshots/4adf02341a1fff60cb8fb5d8cc1ab59228f26b2784124c6f983a4c78948b1293
while IFS=$'\t' read -r path digest bytes; do snapshot transport-binding "$path" "$digest" "$bytes"; done < <(jq -r '.[]|[.path,.sha256,.bytes]|@tsv' "$transport_bindings")
# Current declaration dependencies absent from proposal bindings are separately
# labelled observed supplemental source, not silently called preregistered input.
for rel in src/ml_dataset/capture.rs src/ml_dataset/types.rs src/cryptanalysis/sparse_equations.rs src/cryptanalysis/binary_semaev_chain_sat.rs; do
  path=$source_root/$rel
  digest=$(sha256sum "$path"); digest=${digest%% *}; bytes=$(stat -c %s "$path")
  snapshot supplemental-observed "$path" "$digest" "$bytes"
done
# Stage bound source files at their actual crate-relative paths, no surrogate types.
while IFS=$'\t' read -r path digest; do
  case "$path" in "$source_root"/*)
    rel=${path#"$source_root"/}; mkdir -p "$out/staged/$(dirname "$rel")"
    cp "$out/snapshots/$digest" "$out/staged/$rel";;
  esac
done < <(jq -sr 'unique_by(.path)|.[]|[.path,.sha256]|@tsv' "$out/INPUTS.jsonl")
for name in original_boolean_search.rs dynamic_boolean_search.rs dynamic_boolean_search_scheduler.rs dynamic_boolean_search_bqi.rs dynamic_boolean_search_check.rs; do
  digest=$(jq -r --arg n "$name" '.files[$n].sha256' "$out/snapshots/4f4d1fe5968c0f9da3ff800ace32e1c656b34d8f7936a314e7dfa26841a39150")
  cp "$out/snapshots/$digest" "$out/staged/src/ml_dataset/$name"
done
cp "$out/snapshots/83672aa852df178fc78a08a8d58df1873d9b56b1ebb48b93faacc06ba4235876" "$out/staged/src/cryptanalysis/checked_binary_search_lift.rs"
# Git is used only as a plain working-file patch applier outside any repository.
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null
unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_COMMON_DIR
for patch in registration.patch worker-transport.patch; do
  digest=$(jq -r --arg n "$patch" '.files[$n].sha256' "$out/snapshots/4f4d1fe5968c0f9da3ff800ace32e1c656b34d8f7936a314e7dfa26841a39150")
  (cd "$out/staged" && git apply --check --whitespace=nowarn "$out/snapshots/$digest" && git apply --whitespace=nowarn "$out/snapshots/$digest") > "$out/logs/$patch.log" 2>&1
done
cmp "$out/staged/src/ml_dataset/solvers.rs" "$out/snapshots/269d30baa998439f1a29055c61b6f73be7a8c1d9d9a188797450a93a8c28e9ee"
# Independently apply the exact phase delta to the bound older adapter.
cp "$out/snapshots/722a63e79d07152e0cc8f32f9f0b31ca6c50c790b5df709962ca5795f0f5b8ed" "$out/phase-delta/original_boolean_search.rs"
(cd "$out/phase-delta" && git apply --check --whitespace=nowarn "$out/snapshots/bde4abebf7d6a78e14c69d7e0b1281d36fe26ae39de7a2ef48b2bb21fdb79e89" && git apply --whitespace=nowarn "$out/snapshots/bde4abebf7d6a78e14c69d7e0b1281d36fe26ae39de7a2ef48b2bb21fdb79e89") > "$out/logs/phase-accounting.patch.log" 2>&1
cmp "$out/phase-delta/original_boolean_search.rs" "$out/staged/src/ml_dataset/original_boolean_search.rs"
(cd "$out/staged" && find src -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum) > "$out/STAGED_SHA256SUMS"
(cd "$out/staged" && sha256sum --quiet -c "$out/STAGED_SHA256SUMS")
jq -sn --slurpfile inputs "$out/INPUTS.jsonl" '{schema:"local.original-owned-search-source-assembly/v1",passed:true,input_references:($inputs|length),unique_input_paths:($inputs|unique_by(.path)|length),input_kinds:($inputs|group_by(.kind)|map({kind:.[0].kind,count:length})),registration_patch_applied:true,worker_patch_applied:true,worker_matches_supplied_bytes:true,phase_delta_applied:true,phase_adapter_matches_supplied_bytes:true,compiled:false,type_checked:false,scientific_trials:0,cloud_requests:0,production_source_modified:false,scope:"Bound source staging and manual type/module review; not a complete crate or compiler test"}' > "$out/RECEIPT.json"
cat "$out/RECEIPT.json"
