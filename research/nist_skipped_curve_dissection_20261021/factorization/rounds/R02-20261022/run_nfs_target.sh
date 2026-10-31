#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
id=$1 parent=$2 out=$3 seconds=$4 mode=${5:-fresh}
[[ $id =~ ^[a-zA-Z0-9_-]+$ && ($mode == fresh || $mode == resume) ]]
jq -e --arg id "$id" '.id==$id and .status=="partial"' "$parent" >/dev/null
number=$(jq -r '.residuals|sort_by(.[0]|[length,.])|last|.[0]' "$parent")
mkdir -p "$(dirname -- "$out")"
exec 8> "$out.target.lock"
flock -x 8
if [[ $mode == fresh ]]; then
  [[ ! -e $out && ! -e $out.parent.json ]] || { echo 'Choose a fresh target directory' >&2; exit 1; }
  cp "$parent" "$out.parent.json"
fi
bash "$here/run_nfs.sh" "$number" "$out" "$seconds" "$mode"
attempt=1
while [[ -e $out/refinement-$attempt ]]; do attempt=$((attempt+1)); done
cat "$out"/attempt-*/candidate-factors.txt | sort -u > "$out/all-candidate-factors.txt"
bash "$here/refine_partition.sh" "$out.parent.json" "$out/all-candidate-factors.txt" \
  "$out/refinement-$attempt" "$number"
cp "$out/refinement-$attempt/result.json" "$out/result.json"
echo "NFS target result: $id ($(jq -r .status "$out/result.json"))"
