#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 ]] || exit 2
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
: "${NIST_FILTER_CHECKER:?Build the frozen native counter checker}"
out=$(cd -- "$1" && pwd)
temp=$(mktemp -d)
trap 'rm -rf -- "$temp"' EXIT
for arm in reference candidate; do
  lines=4592399
  if [[ $arm == candidate ]]; then lines=10441117; fi
  [[ $(cat "$out/$arm-launch.exit") == 0 ]]
  cmp -- "$here/$arm-inputs.sha256" "$out/$arm/input-sha256.txt"
  "$NIST_FILTER_CHECKER" "$out/$arm/native-filter.log" "$out/$arm/exit-status.txt" "$arm" "$lines" > "$temp/$arm.json"
  cmp -- "$temp/$arm.json" "$out/$arm/counts.json"
  head -n "$lines" "$out/$arm/state/nfs.dat" | sha256sum > "$temp/$arm-prefix.sha256"
  [[ $(cut -d' ' -f1 "$temp/$arm-prefix.sha256") == $(awk '$2=="nfs.dat" {print $1}' "$here/$arm-inputs.sha256") ]]
  awk '$2!="nfs.dat"' "$here/$arm-inputs.sha256" > "$temp/$arm-static.sha256"
  (cd "$out/$arm/state" && sha256sum --check "$temp/$arm-static.sha256")
done
printf '%s\n' 'Counter accounting and input prefixes verified; matrix rank and factorization not certified.'
