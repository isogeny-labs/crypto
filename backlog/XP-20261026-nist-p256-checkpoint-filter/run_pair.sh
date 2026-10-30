#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 ]] || { echo 'usage: run_pair.sh NEW_OUTPUT' >&2; exit 2; }
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
: "${NIST_R06_STATE:?Restore the original R06 state}"
: "${NIST_R08_STATE:?Restore the original R08 state}"
[[ ! -e $1 ]] || { echo 'Output exists; retain it and choose a new run id' >&2; exit 2; }
mkdir -p -- "$1"
out=$(cd -- "$1" && pwd)
exec 8>"$out/pair.lock"
flock -n 8
for arm in reference candidate; do
  input=$NIST_R06_STATE
  if [[ $arm == candidate ]]; then input=$NIST_R08_STATE; fi
  set +e
  timeout --verbose --signal=TERM --kill-after=30s 900 bash "$here/run_filter.sh" "$arm" "$input" "$out/$arm" > "$out/$arm-launch.stdout" 2> "$out/$arm-launch.stderr"
  rc=$?
  set -e
  printf '%s\n' "$rc" > "$out/$arm-launch.exit"
  [[ $rc == 0 ]] || { echo "Incomplete $arm execution; candidate interpretation blocked" >&2; exit "$rc"; }
done
bash "$here/verify_pair.sh" "$out"
