#!/usr/bin/env bash
# Restore a recorded native checkpoint, then append one bounded NFS attempt.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
id=$1 out=$2 seconds=${3:-1800} option=${4:---resume}
[[ $id == published_224-twist2 || $id == skipped_256_876-twist2 ]]
[[ $option == --resume || $option == --prepare-only ]]
mkdir -p "$out"
out=$(cd -- "$out" && pwd)
if [[ ! -f $out/input.txt ]]; then
  [[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]] || exit 1
  if [[ -n ${NIST_FACTOR_CHECKPOINT:-} ]]; then
    archive=$NIST_FACTOR_CHECKPOINT
    sums="$archive.sha256"
  elif [[ -n ${NIST_FACTOR_RAW_ARCHIVE:-} ]]; then
    archive=$NIST_FACTOR_RAW_ARCHIVE
    sums="$here/runs.tar.xz.sha256"
  elif [[ -f $here/checkpoint-$id.tar.xz ]]; then
    archive="$here/checkpoint-$id.tar.xz"
    sums="$archive.sha256"
  else
    archive="$here/runs.tar.xz"
    sums="$archive.sha256"
  fi
  expected=$(awk '{print $1}' "$sums")
  actual=$(sha256sum "$archive"); actual=${actual%% *}
  [[ $actual == "$expected" ]]
  stage=$(mktemp -d)
  trap 'rm -rf -- "$stage"' EXIT
  tar -xJf "$archive" -C "$stage" "runs/nfs/$id"
  cp -a "$stage/runs/nfs/$id/." "$out/"
fi
jq --arg id "$id" '.[]|select(.id==$id)' "$here/baseline-results.json" > "$out.parent.json"
expected=$(jq -r '.residuals|sort_by(.[0]|[length,.])|last|.[0]' "$out.parent.json")
[[ $(cat "$out/input.txt") == "$expected" ]]
cp "$out.parent.json" "$out/parent-partition.json"
if [[ $option == --prepare-only ]]; then
  jq -n --arg id "$id" --arg n "$expected" '{prepared:true,id:$id,input:$n,search_executed:false}'
  exit 0
fi
if jq -e '.status=="complete"' "$out/result.json" >/dev/null; then
  echo 'The recorded target is already complete; no further search started.'
  exit 0
fi
if [[ ! -s $out/state/nfs.job && ! -s $out/state/nfs.dat.p ]]; then
  echo 'No native-recognized master polynomial/job checkpoint. Per-thread .p files require validated recovery; refusing to restart from scratch.' >&2
  exit 1
fi
bash "$here/run_nfs_target.sh" "$id" "$out.parent.json" "$out" "$seconds" resume
jq -n --arg id "$id" --arg n "$expected" \
  '{schema:"nist-nfs-checkpoint/v1",id:$id,input:$n,state_directory:"state",
    parent_partition:"parent-partition.json",result:"result.json"}' > "$out/checkpoint.json"
