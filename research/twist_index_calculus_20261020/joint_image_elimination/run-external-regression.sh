#!/usr/bin/env bash
# Thin orchestration of fixed official msolve inputs. No admitted timings.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_out=${1:?new output directory}
test ! -e "$joint_out"
mkdir -p "$joint_out/metadata" "$joint_out/systems"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
git rev-parse HEAD > "$joint_out/metadata/source-commit.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
joint_docker=(sudo -n env -u DOCKER_HOST -u DOCKER_CONTEXT -u DOCKER_TLS -u DOCKER_TLS_VERIFY -u DOCKER_CERT_PATH docker --host=unix:///var/run/docker.sock)
joint_image=twist-joint-msolve:0.11.0
"${joint_docker[@]}" image inspect "$joint_image" > "$joint_out/metadata/image.json"
"${joint_docker[@]}" run --rm --network=none "$joint_image" sh -c 'cat /opt/msolve-source-commit.txt; sha256sum /usr/local/bin/msolve /usr/local/lib/libmsolve.so.4.0.0; gcc --version | head -1' > "$joint_out/metadata/versions.txt"
joint_case() {
  local joint_id=$1 joint_source=$2 joint_name joint_rc joint_input
  joint_name="twist-joint-ext-v1-$joint_id"
  mkdir "$joint_out/systems/$joint_id"
  if ! test -f "$joint_source.ms" || ! test -f "$joint_source.json"; then
    printf 'not-run-input-unavailable\n' > "$joint_out/systems/$joint_id/status.txt"
    return
  fi
  sha256sum "$joint_source.json" "$joint_source.ms" > "$joint_out/systems/$joint_id/inputs.sha256"
  joint_input=$(cd "$(dirname "$joint_source")" && pwd)
  if "${joint_docker[@]}" container inspect "$joint_name" > "$joint_out/systems/$joint_id/preexisting-container.json" 2>/dev/null; then
    printf 'not-run-container-already-exists-reconcile-before-retry\n' > "$joint_out/systems/$joint_id/status.txt"
    return
  fi
  local joint_command=("${joint_docker[@]}" run --rm --name "$joint_name" --network=none --cpus=1 --memory=8g --pids-limit=64 --ulimit core=0 --user "$(id -u):$(id -g)" --env OMP_NUM_THREADS=1 \
    --mount "type=bind,src=$joint_input,dst=/inputs,readonly" \
    --mount "type=bind,src=$joint_out/systems/$joint_id,dst=/output" \
    "$joint_image" prlimit --as=8589934592 -- timeout --kill-after=10s 1180s \
    /usr/local/bin/msolve --random-seed 0 -t 1 -v 2 -g 2 -r 1 \
    -f "/inputs/$(basename "$joint_source").ms" -o /output/basis.txt)
  printf '%q ' "${joint_command[@]}" > "$joint_out/systems/$joint_id/command.txt"
  printf '\n' >> "$joint_out/systems/$joint_id/command.txt"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/systems/$joint_id/started-utc.txt"
  set +e
  python3 "$joint_repo/tools/isolated_bench.py" busy -- timeout --kill-after=10s 1200s "${joint_command[@]}" > "$joint_out/systems/$joint_id/stdout" 2> "$joint_out/systems/$joint_id/stderr"
  joint_rc=$?
  set -e
  printf '%s\n' "$joint_rc" > "$joint_out/systems/$joint_id/exit"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/systems/$joint_id/finished-utc.txt"
  if "${joint_docker[@]}" container inspect "$joint_name" > "$joint_out/systems/$joint_id/post-container.json" 2>/dev/null; then
    "${joint_docker[@]}" kill "$joint_name" > "$joint_out/systems/$joint_id/deadline-cleanup.txt" 2>&1 || true
  fi
  printf 'process-terminal-basis-and-roots-unverified\n' > "$joint_out/systems/$joint_id/status.txt"
}
for joint_arm in l0-peel phi-l0-active double-l0-active; do
  for joint_encoding in finite-domain geometric; do
    joint_case "$joint_arm-global-$joint_encoding" "$joint_study/runs/algebra-v1/exports/p37-s0-$joint_arm-q2/$joint_encoding"
  done
  while IFS= read -r joint_block; do
    joint_case "$joint_arm-$joint_block" "$joint_study/runs/regression-v1/orbit-inputs/$joint_arm/$joint_block/system"
  done < <(jq -r '.blocks[].id' "$joint_study/runs/regression-v1/orbit-inputs/$joint_arm/manifest.json")
  for joint_encoding in system geometric; do
    joint_case "$joint_arm-variant-$joint_encoding" "$joint_study/runs/verification-v7/variants/$joint_arm/$joint_encoding"
  done
done
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Fixed external GB processes terminal or explicitly not run. Basis verification, complete roots and same signed physical sets are still required. No speedup claim.\n' > "$joint_out/complete.txt"
