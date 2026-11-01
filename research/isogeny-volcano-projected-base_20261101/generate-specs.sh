#!/usr/bin/env bash
set -euo pipefail

root=$(git rev-parse --show-toplevel)
study=research/isogeny-volcano-projected-base_20261101
source_study=research/isogeny-volcano-cost-v2_20261017
dest=$root/$study/specs
mode=${1:-write}
if [[ $mode != write && $mode != --check ]]; then
  echo "usage: $0 [--check]" >&2
  exit 2
fi

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/specs" "$tmp/meta"

shard_size=40
# The source population is immutable even after this prospective package is
# committed. Do not make the generated manifest depend on the caller's HEAD.
source_revision=ff7ad61cfed11908fb79de9dd485b696d0ada779
for source in "$root"/$source_study/classes/l2-s*/spec.json; do
  class=$(basename "$(dirname "$source")")
  source_sha=$(sha256sum "$source" | awk '{print $1}')
  curves=$(jq -er '.workloads.curves | length' "$source")
  shard=0
  for ((start=0; start<curves; start+=shard_size)); do
    shard=$((shard+1))
    stop=$((start+shard_size))
    if ((stop>curves)); then stop=$curves; fi
    count=$((stop-start))
    cell=$(printf '%s-shard%02d' "$class" "$shard")
    out=$tmp/specs/$cell.json
    jq -S \
      --arg cell "$cell" --arg class "$class" \
      --argjson start "$start" --argjson stop "$stop" '
      .label = ("projected volcano discriminator: " + $cell) |
      .description = (.description + "; prospectively sliced without changing curves or targets for EX-VOLCANO-PROJECTED-20261101, class " + $class + ", cell " + $cell) |
      .workloads.curves = .workloads.curves[$start:$stop] |
      .arms = [
        {"name":"rho-neg","role":"reference","method":{"id":"rho.negation"}},
        {"name":"ic-raw","role":"baseline","method":{"id":"ic.pipeline","params":{"factor_base":"prime-abscissa:size=32","oracle":"mitm:negation_folded=1"}}},
        {"name":"ic-projected","role":"candidate","method":{"id":"ic.pipeline","params":{"factor_base":"prime-projected-abscissa:size=32","oracle":"mitm:negation_folded=1"}}},
        {"name":"rho-neg-aa","role":"control","method":{"id":"rho.negation"}}
      ] |
      .measurement.rounds = 5 |
      .measurement.warmup = 1 |
      .measurement.order = "shuffle" |
      .measurement.isolation_required = "L2" |
      .measurement.timeout_seconds = 600
      ' "$source" > "$out"
    out_sha=$(sha256sum "$out" | awk '{print $1}')
    jq -n -S \
      --arg cell "$cell" --arg class "$class" \
      --arg source_spec "$source_study/classes/$class/spec.json" \
      --arg source_spec_sha256 "$source_sha" --arg spec "specs/$cell.json" \
      --arg spec_sha256 "$out_sha" --argjson shard "$shard" \
      --argjson start "$start" --argjson stop "$stop" --argjson curves "$count" \
      --argjson executions "$((count*8*4*6))" \
      '{cell:$cell,class:$class,shard:$shard,source_spec:$source_spec,
        source_spec_sha256:$source_spec_sha256,spec:$spec,spec_sha256:$spec_sha256,
        curve_models:$curves,slice:{start:$start,stop:$stop},targets_per_curve:8,
        arms:4,warmup_rounds:1,measured_rounds:5,planned_executions:$executions}' \
      > "$tmp/meta/$cell.json"
  done
done

jq -c -s -S \
  --arg revision "$source_revision" --arg source "$source_study" \
  '{schema:"projected-volcano-cell-manifest/v1",generated_from_revision:$revision,
    source_study:$source,shard_size:40,cells:.}' \
  "$tmp"/meta/*.json > "$tmp/specs/manifest.json"
(cd "$tmp/specs" && sha256sum ./*.json > SHA256SUMS)

if [[ $mode == --check ]]; then
  diff -ruN "$dest" "$tmp/specs"
  exit
fi
mkdir -p "$dest"
cp "$tmp"/specs/*.json "$dest"/
cp "$tmp/specs/SHA256SUMS" "$dest/SHA256SUMS"
