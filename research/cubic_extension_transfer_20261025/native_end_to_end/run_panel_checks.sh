#!/usr/bin/env bash
# Thin host-side development orchestration. No timings are collected.
set -euo pipefail
here=$(cd -- "$(dirname -- "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
study=research/cubic_extension_transfer_20261025/native_end_to_end
container=${NATIVE_DEVELOPMENT_CONTAINER:-cubic-native-map-dev-20261029}
tag=${1:?usage: bash run_panel_checks.sh TAG [--resume]}
resume=${2:-}
[[ -z "$resume" || "$resume" = --resume ]] || { echo 'invalid resume option' >&2; exit 2; }
[[ "$tag" =~ ^[a-zA-Z0-9_-]+$ ]] || { echo 'invalid output tag' >&2; exit 2; }
cd -- "$root"
output="$study/results/panel-$tag"
if [[ "$resume" = --resume ]]; then
  # Correctness-only resume: immutable inputs/binaries and successful completed
  # phases are checked before skipping them. Failed/partial solves are refused.
  sha256sum --check --status "$output/inputs-and-binaries.sha256"
else
  test ! -e "$output" || { echo 'output directory exists; preserve earlier attempts' >&2; exit 2; }
  mkdir -p "$output"
  sha256sum "$study"/fixtures/panel/*.public "$study"/build/transfer_test \
    "$study"/build/libcubic_jacobian.so "$study"/build/source_public > "$output/inputs-and-binaries.sha256"
fi
docker_exec=(env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY \
  -u DOCKER_CERT_PATH docker --host=unix:///var/run/docker.sock exec "$container")
for fixture in "$study"/fixtures/panel/*.public; do
  name=$(basename -- "$fixture" .public)
  prime=${name%%-*}; prime=${prime#p}
  if [[ "$resume" = --resume && -f "$output/$name-ic.json" ]]; then
    test "$(cat "$output/$name-ic.exit")" = 0
  else
    test ! -e "$output/$name-ic.log" || { echo 'partial/failed solve retained; use a fresh tag' >&2; exit 2; }
    bash scripts/checkpoint.sh gate
    if "${docker_exec[@]}" /bin/bash -c \
    'export PATH=/home/sage/sage/local/bin:$PATH; exec timeout 90 build/transfer_test "$1" ../historical/extension-field-experiments/research_lead/results "$2" ic' \
    -- "fixtures/panel/$name.public" "results/panel-$tag/$name-ic.json" \
    > "$output/$name-ic.log" 2>&1; then task_exit=0; else task_exit=$?; fi
    printf '%s\n' "$task_exit" > "$output/$name-ic.exit"
    (( task_exit == 0 )) || exit "$task_exit"
  fi
  answer=$(jq -er 'select(.status=="passed" and .verified_on_original_source==true and .source_log_read==false and .synthetic_target_used==false) | .recovered_scalar' "$output/$name-ic.json")
  expected=$(jq -er --arg name "$name.public" '.workloads[] | select(.file==$name) | .scalar' "$study/results/p$prime-known-answer-panel.json")
  test "$answer" = "$expected"
  if [[ "$resume" = --resume && -s "$output/$name-independent-verification.json" ]]; then
    jq -e --argjson answer "$answer" '.status=="passed" and .independent_cubic_arithmetic==true and .scalar==$answer' "$output/$name-independent-verification.json" >/dev/null
  else
    test ! -e "$output/$name-independent-verification.json" || { echo 'partial verification retained; use a fresh tag' >&2; exit 2; }
    bash scripts/checkpoint.sh gate
    timeout 90 "$study/build/source_public" verify "$fixture" "$answer" \
      > "$output/$name-independent-verification.json" 2> "$output/$name-independent-verification.log"
  fi
  printf 'verified %s\n' "$name"
done
