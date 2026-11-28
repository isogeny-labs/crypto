#!/usr/bin/env bash
# Independent replay entry point; do not use to evade this session's closed caps.
set -euo pipefail
[[ $# == 1 && ! -e "$1" ]] || { echo 'Usage: run_complete_replay.sh FRESH_DIR' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$1"
replay_dir=$(cd "$1" && pwd)
for spec in 'complete_law law 180 complete-law-depth8' 'retained_depth8 retained 180 retained-depth8' 'bound_assembly assembly 60 bound-assembly' 'exact_m2 exact 5 exact-m2'; do
  read -r tool folder seconds evidence <<< "$spec"
  bash "$study_dir/build_$tool.sh" "$replay_dir/$folder"
  if [[ "$tool" == complete_law ]]; then
    (ulimit -v 2097152; timeout "$seconds" "$replay_dir/$folder/check" "$replay_dir/$folder/output") > "$replay_dir/$folder/replay.log" 2>&1
  else
    (ulimit -v 2097152; timeout "$seconds" "$replay_dir/$folder/check" "$replay_dir/$folder/cache.txt" "$replay_dir/$folder/output") > "$replay_dir/$folder/replay.log" 2>&1
  fi
  for file in "$replay_dir/$folder/output/"*.json; do
    cmp "$file" "$study_dir/evidence/$evidence/${file##*/}"
  done
done
echo 'New complete package replay matches frozen JSONs. No old failed cell rerun.'
