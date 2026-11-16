#!/usr/bin/env bash
set -euo pipefail

repo=$(git rev-parse --show-toplevel)
study=research/koblitz_n83_factor_base_sweep_20261008
source_commit=cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff
source_dir=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-v5-exporter-source-20261116
binary=/private/tmp/n83-v5-colima-output-20261116/attempt-5/target/release/examples/koblitz_n83_factor_base_export
binary_sha=87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236
object_sha=3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf
bucket=setec-astronomy-artifacts
key=store/sha256/3e/$object_sha
profile=ecc-challenges-new
base_dir=$source_dir/$study/verification/k600-v3-s3-adoption-20261115/attempt-1
fixture_dir=$source_dir/$study/pilot-01
index_source=$repo/$study/reports/run-20261116-k600-preflight/attempt-6-zero-trial-pass/store-index.json
out=/Volumes/SSD990-2/tmp/n83-colima-output/v5-ordered-preflight-20261116/attempt-2
image=sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573
container=n83-v5-ordered-preflight-20261116-attempt-2
export DOCKER_HOST=unix:///Volumes/SSD990-2/tmp/n83-colima-home/n83-arm64/docker.sock
watch_pid=

sha256() { shasum -a 256 "$1" | awk '{print $1}'; }
available_kib() { df -k "$1" | awk 'NR==2 {print $4}'; }
check_disk() {
  local volume free
  for volume in /private/tmp /Volumes/SSD990-2; do
    free=$(available_kib "$volume")
    printf '%s %s %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$volume" "$free" >> "$out/disk-samples.txt"
    if (( free < 2097152 )); then
      printf '%s %s %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$volume" "$free" > "$out/disk-stop.txt"
      return 1
    fi
  done
}
finish() {
  local status=$?
  trap - EXIT
  if [[ -n "$watch_pid" ]]; then kill "$watch_pid" 2>/dev/null || :; wait "$watch_pid" 2>/dev/null || :; fi
  if [[ ! -e "$out/container.inspect.json" ]]; then
    if docker inspect "$container" > "$out/container.inspect.json" 2> "$out/inspect.stderr"; then
      docker rm -f "$container" > "$out/docker-cleanup.stdout" 2> "$out/docker-cleanup.stderr" || status=1
    fi
  fi
  date -u +%Y-%m-%dT%H:%M:%SZ > "$out/end.utc"
  printf 'schema=n83.v5-ordered-preflight-outer/v1\nsource_commit=%s\nexit_code=%s\n' "$source_commit" "$status" > "$out/outer.receipt"
  exit "$status"
}

cd "$repo"
test -z "$(git status --porcelain)" || { echo 'checkout not clean' >&2; exit 90; }
git diff --quiet "$source_commit" HEAD -- src examples crates gpu tools Cargo.toml Cargo.lock \
  "$study/primary_adapter.rs" "$study/size-frontier-v2.json" \
  "$study/paired-frontier-v3.json" "$study/paired-frontier-v4.json" \
  "$study/paired-frontier-v5.json" || { echo 'pinned source differs' >&2; exit 91; }
test "$(sha256 "$binary")" = "$binary_sha" || { echo 'binary hash differs' >&2; exit 92; }
test "$(jq -r '.schema + " " + .source_commit + " " + (.status_clean|tostring)' "$source_dir/attestation.json")" = \
  "n83.chain-s3-source-attestation/v1 $source_commit true" || { echo 'source attestation differs' >&2; exit 93; }
for entry in \
  "manifest.json:d2095084767804af1525665307c9ad1c9eb8dbee2fbdb5adcac9f849e765d119" \
  "replay.json:60725bf905ed7ccf3ebef24fbee98cda8b32422075a8708e268e60f2e7916843" \
  "storage.json:8b8452192a6344e8afd98737765e5d944a02efca57f37eae17251f092c2fa98b"; do
  name=${entry%%:*}; expected=${entry#*:}
  test "$(sha256 "$base_dir/$name")" = "$expected" || { echo "$name hash differs" >&2; exit 94; }
done
test "$(sha256 "$fixture_dir/probe-corpus.json")" = 760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd || exit 95
test "$(sha256 "$fixture_dir/probe-validation.json")" = 1c6d3a45c8b4654ff208751258fa2e34f0cc60e1c30ba48e9214db62be60472f || exit 96
test "$(sha256 "$index_source")" = 298b5c1396b58a1b4025813be092934be6f65e34544856d92f1edadabeabd28b || exit 97
test "$(docker image inspect "$image" --format '{{.Id}} {{.Os}}/{{.Architecture}}')" = "$image linux/arm64" || exit 98
test -z "$(docker ps -q)" && test -z "$(docker ps -aq --filter "name=^/$container$")" || exit 99
test ! -e "$out" || { echo 'attempt output exists' >&2; exit 100; }
mkdir -p "$out/input/cache/sha256/3e" "$out/output"
trap finish EXIT
date -u +%Y-%m-%dT%H:%M:%SZ > "$out/start.utc"
printf 'source=%s\nbinary_sha256=%s\nimage=%s\n' "$source_commit" "$binary_sha" "$image" > "$out/inputs.receipt"
check_disk

aws --profile "$profile" sts get-caller-identity --output json > "$out/aws-identity.json"
test "$(jq -r .Account "$out/aws-identity.json")" = 104186811093 || exit 101
aws --profile "$profile" s3api head-object --bucket "$bucket" --key "$key" --output json > "$out/s3-head.json"
test "$(jq -r .ContentLength "$out/s3-head.json")" = 3025798 || exit 102
gtimeout 90 aws --profile "$profile" s3api get-object --bucket "$bucket" --key "$key" \
  "$out/input/cache/sha256/3e/$object_sha" > "$out/s3-get.json"
test "$(wc -c < "$out/input/cache/sha256/3e/$object_sha" | tr -d ' ')" = 3025798 || exit 103
test "$(sha256 "$out/input/cache/sha256/3e/$object_sha")" = "$object_sha" || exit 104
cp "$index_source" "$out/input/store-index.json"
check_disk

(
  while :; do
    sleep 5
    if ! check_disk; then
      docker stop -t 1 "$container" >> "$out/disk-stop-docker.txt" 2>&1 || :
      break
    fi
  done
) &
watch_pid=$!

set +e
# The quoted program is expanded by the guest shell after Docker sets its env.
# shellcheck disable=SC2016
gtimeout 300 docker run --pull=never --name "$container" --platform linux/arm64 \
  --memory=4096m --memory-swap=4096m --cpus=1 --pids-limit=128 \
  --network none --read-only --cap-drop=ALL --security-opt=no-new-privileges \
  --tmpfs /tmp:rw,noexec,nosuid,size=64m --user 501:20 \
  --env RAYON_NUM_THREADS=1 --env ICV1_FROZEN_SOURCE_DIR="$source_dir" \
  --env CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store \
  --env CRYPTO_STORE_INDEX="$out/input/store-index.json" \
  --env CRYPTO_STORE_CACHE="$out/input/cache" \
  --mount "type=bind,src=$source_dir,dst=$source_dir,readonly" \
  --mount "type=bind,src=$binary,dst=$binary,readonly" \
  --mount "type=bind,src=$out/input,dst=$out/input,readonly" \
  --mount "type=bind,src=$out/output,dst=$out/output" \
  --workdir "$source_dir" "$image" sh -ec '
    test "$(uname -m)" = aarch64
    test "$(cat /sys/fs/cgroup/memory.max)" = 4294967296
    test "$(cat /sys/fs/cgroup/memory.swap.max)" = 0
    test "$(cat /sys/fs/cgroup/cpu.max)" = "100000 100000"
    test "$(cat /sys/fs/cgroup/pids.max)" = 128
    "$1" primary-v5-retained-chain-cold 26960859236 "$2" "$3" \
      0 125000 350000 1 10000 240 4096 "$4"
  ' sh "$binary" "$base_dir" "$fixture_dir" "$out/output/run" \
  > "$out/docker.stdout" 2> "$out/docker.stderr"
docker_exit=$?
docker inspect "$container" > "$out/container.inspect.json"
inspect_exit=$?
docker rm -f "$container" > "$out/docker-cleanup.stdout" 2> "$out/docker-cleanup.stderr"
cleanup_exit=$?
set -e
printf 'docker_exit=%s\ninspect_exit=%s\ncleanup_exit=%s\n' "$docker_exit" "$inspect_exit" "$cleanup_exit" > "$out/docker.receipt"
test "$docker_exit" -eq 0 && test "$inspect_exit" -eq 0 && test "$cleanup_exit" -eq 0
test "$(jq 'length' "$out/container.inspect.json")" = 1
test ! -e "$out/disk-stop.txt"
test "$(jq -r .status "$out/output/run/summary.json")" = PREFLIGHT_ONLY
test "$(jq -r .strategy "$out/output/run/summary.json")" = ordered-orbit-chain-s3
test "$(jq -r '.report.trials, .report.relations, .report.sat_calls, .max_trials' "$out/output/run/summary.json" | sort -u)" = 0
test "$(jq -r .source_commit "$out/output/run/summary.json")" = "$source_commit"
test "$(jq -r '.[0].State.OOMKilled' "$out/container.inspect.json")" = false
test -z "$(docker ps -aq --filter "name=^/$container$")"
check_disk
