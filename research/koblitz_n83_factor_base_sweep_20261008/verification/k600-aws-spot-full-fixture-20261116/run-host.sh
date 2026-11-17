#!/usr/bin/env bash
set -euo pipefail

if [[ $# != 2 ]]; then
  echo 'usage: run-host.sh FULL_SOURCE_CHECKOUT NEW_OUTPUT_DIR' >&2
  exit 2
fi

repo=$(realpath "$1")
output=$2
source_sha=a91b8b66d0f1ffc175fdce984d1059144ce232f6
fixture=research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl
fixture_sha=8970667e2f18587eeb08d033123dab029b94042ac5c07c870b08988f1ee5b738
fixture_bytes=2293097
image=n83-lib-validation:20261116
container=n83-merged-full-fixture-20261116-attempt-1

[[ $(uname -m) == aarch64 ]] || { echo 'host is not ARM64' >&2; exit 90; }
[[ $(git -C "$repo" rev-parse HEAD) == "$source_sha" ]] || { echo 'source SHA differs' >&2; exit 91; }
[[ $(git -C "$repo" config --get core.sparseCheckout) == false ]] || { echo 'source is sparse' >&2; exit 92; }
[[ -z $(git -C "$repo" status --porcelain) ]] || { echo 'source is not clean' >&2; exit 93; }
[[ -f "$repo/gpu/fes/fes_solve.cpp" ]] || { echo 'GPU fixture absent' >&2; exit 94; }
[[ -f "$repo/research/ic_planner_validation_20261020/sessions/r24/records.jsonl" ]] || { echo 'planner fixture absent' >&2; exit 95; }
[[ $(sha256sum "$repo/$fixture" | awk '{print $1}') == "$fixture_sha" ]] || { echo 'archived fixture SHA differs' >&2; exit 96; }
[[ $(wc -c < "$repo/$fixture" | tr -d ' ') == "$fixture_bytes" ]] || { echo 'archived fixture length differs' >&2; exit 97; }
[[ $(docker image inspect "$image" --format '{{.Os}}/{{.Architecture}}') == linux/arm64 ]] || { echo 'Docker image is not Linux ARM64' >&2; exit 98; }
[[ -z $(docker ps -aq --filter "name=^/${container}$") ]] || { echo 'container name already exists' >&2; exit 99; }
[[ ! -e $output ]] || { echo 'output already exists' >&2; exit 100; }
root_free_kib=$(df -Pk / | awk 'NR == 2 {print $4}')
[[ $root_free_kib -ge 20971520 ]] || { echo 'root disk below 20 GiB' >&2; exit 101; }

mkdir -p "$output"
output=$(realpath "$output")
printf '%s\n' "$source_sha" > "$output/source.commit"
printf '%s\n' "$root_free_kib" > "$output/root-free-kib-before.txt"
docker image inspect "$image" > "$output/image.inspect.json"
docker version > "$output/docker-version.txt"
started_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
set +e
# shellcheck disable=SC2016 # The inner shell expands these variables in the container.
timeout --signal=TERM --kill-after=30s 7200 docker run --pull=never \
  --name "$container" --platform linux/arm64 \
  --memory=12g --memory-swap=12g --cpus=4 --pids-limit=512 \
  --network=bridge --cap-drop=ALL --security-opt=no-new-privileges \
  --env GIT_OPTIONAL_LOCKS=0 --env CARGO_INCREMENTAL=0 \
  --env CARGO_HOME=/output/cargo-home --env CARGO_TARGET_DIR=/output/cargo-target \
  --env EXPECT_SOURCE="$source_sha" --env EXPECT_FIXTURE_SHA="$fixture_sha" \
  --env EXPECT_FIXTURE_BYTES="$fixture_bytes" \
  --mount "type=bind,src=$repo,dst=/source,readonly" \
  --mount "type=bind,src=$output,dst=/output" \
  --workdir /source "$image" sh -ec '
    test "$(uname -m)" = aarch64
    test "$(rustc --version)" = "rustc 1.99.1 (58cf8f9c3 2026-10-13)"
    test "$(cat /sys/fs/cgroup/memory.max)" = 12884901888
    test "$(cat /sys/fs/cgroup/memory.swap.max)" = 0
    test "$(cat /sys/fs/cgroup/cpu.max)" = "400000 100000"
    test "$(cat /sys/fs/cgroup/pids.max)" = 512
    test "$(git -c safe.directory=/source rev-parse HEAD)" = "$EXPECT_SOURCE"
    test -z "$(git -c safe.directory=/source status --porcelain)"
    path=/source/research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl
    test "$(sha256sum "$path" | cut -d " " -f 1)" = "$EXPECT_FIXTURE_SHA"
    test "$(wc -c < "$path" | tr -d " ")" = "$EXPECT_FIXTURE_BYTES"
    test -f /source/gpu/fes/fes_solve.cpp
    test -f /source/research/ic_planner_validation_20261020/sessions/r24/records.jsonl
    printf "source=%s fixture=%s memory.max=%s swap.max=%s cpu.max=%s pids.max=%s\n" \
      "$EXPECT_SOURCE" "$EXPECT_FIXTURE_SHA" "$(cat /sys/fs/cgroup/memory.max)" \
      "$(cat /sys/fs/cgroup/memory.swap.max)" "$(cat /sys/fs/cgroup/cpu.max)" \
      "$(cat /sys/fs/cgroup/pids.max)"
    cargo test --release --lib --locked -j 1
    test -z "$(git -c safe.directory=/source status --porcelain)"
  ' > "$output/lib-test.log" 2>&1
child_exit=$?
docker inspect "$container" > "$output/container.inspect.json"
inspect_exit=$?
docker rm -f "$container" > "$output/container-removal.txt" 2>&1
cleanup_exit=$?
set -e
ended_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
source_clean=true
[[ -z $(git -C "$repo" status --porcelain) ]] || source_clean=false
root_free_kib_after=$(df -Pk / | awk 'NR == 2 {print $4}')
printf '%s\n' "$root_free_kib_after" > "$output/root-free-kib-after.txt"
printf 'schema=n83.aws-spot-full-fixture-outer/v1\nstarted_utc=%s\nended_utc=%s\nsource_commit=%s\nfixture_sha256=%s\nchild_exit_code=%s\ninspect_exit_code=%s\ncleanup_exit_code=%s\nsource_clean=%s\n' \
  "$started_utc" "$ended_utc" "$source_sha" "$fixture_sha" "$child_exit" \
  "$inspect_exit" "$cleanup_exit" "$source_clean" > "$output/outer.receipt"
[[ $child_exit == 0 && $inspect_exit == 0 && $cleanup_exit == 0 && $source_clean == true ]]
