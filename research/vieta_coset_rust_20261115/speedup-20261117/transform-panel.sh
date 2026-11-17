#!/usr/bin/env bash
set -euo pipefail
root=research/vieta_coset_rust_20261115/speedup-20261117
binary=${1:?immutable worker}
panel=${2:?panel}
for config in "$root/configs/$panel"/*.json; do
 name=$(basename "$config" .json)
 timeout --signal=TERM --kill-after=3s 300 "$binary" panel "$config" "$root/runs/$panel/$name"
done
