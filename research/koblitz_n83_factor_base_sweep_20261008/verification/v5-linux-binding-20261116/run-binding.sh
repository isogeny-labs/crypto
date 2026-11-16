#!/usr/bin/env bash
set -euo pipefail

repo=$(git rev-parse --show-toplevel)
source_commit=cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff
source_dir=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-v5-exporter-source-20261116
binary=/private/tmp/n83-v5-colima-output-20261116/attempt-5/target/release/examples/koblitz_n83_factor_base_export
binary_sha=87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236
manifest_source=$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/k600-v3-s3-adoption-20261115/attempt-1/manifest.json
manifest_sha=d2095084767804af1525665307c9ad1c9eb8dbee2fbdb5adcac9f849e765d119
object_sha=3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf
bucket=setec-astronomy-artifacts
key=store/sha256/3e/$object_sha
profile=ecc-challenges-new
out=/Volumes/SSD990-2/tmp/n83-colima-output/v5-linux-binding-20261116/attempt-1
colima_home=/Volumes/SSD990-2/tmp/n83-colima-home
image=sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573
container=n83-v5-linux-binding-20261116-attempt-1
export DOCKER_HOST="unix://$colima_home/n83-arm64/docker.sock"

cd "$repo"
test -z "$(git status --porcelain)" || { echo 'checkout not clean' >&2; exit 90; }
git diff --quiet "$source_commit" HEAD -- src examples crates gpu tools Cargo.toml Cargo.lock \
  research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs \
  research/koblitz_n83_factor_base_sweep_20261008/{size-frontier-v2,paired-frontier-v3,paired-frontier-v4,paired-frontier-v5}.json \
  || { echo 'pinned source differs' >&2; exit 91; }
test "$(shasum -a 256 "$binary" | awk '{print $1}')" = "$binary_sha" || { echo 'binary hash differs' >&2; exit 92; }
test "$(shasum -a 256 "$manifest_source" | awk '{print $1}')" = "$manifest_sha" || { echo 'manifest hash differs' >&2; exit 93; }
test "$(jq -r '.schema + " " + .source_commit + " " + (.status_clean|tostring)' "$source_dir/attestation.json")" = "n83.chain-s3-source-attestation/v1 $source_commit true" || { echo 'source attestation differs' >&2; exit 94; }
test "$(docker image inspect "$image" --format '{{.Id}} {{.Os}}/{{.Architecture}}')" = "$image linux/arm64" || { echo 'image differs' >&2; exit 95; }
test -z "$(docker ps -q)" && test -z "$(docker ps -aq --filter "name=^/$container$")" || { echo 'container overlap' >&2; exit 96; }
test ! -e "$out" || { echo 'attempt output exists' >&2; exit 97; }
mkdir -p "$out/input/good/objects" "$out/input/bad-manifest" "$out/run"
date -u +%Y-%m-%dT%H:%M:%SZ > "$out/start.utc"
finish() {
  status=$?
  date -u +%Y-%m-%dT%H:%M:%SZ > "$out/end.utc"
  printf 'schema=n83.v5-linux-binding-outer/v1\nsource_commit=%s\nexit_code=%s\n' "$source_commit" "$status" > "$out/outer.receipt"
}
trap finish EXIT

aws --profile "$profile" sts get-caller-identity --output json > "$out/aws-identity.json"
test "$(jq -r .Account "$out/aws-identity.json")" = 104186811093 || { echo 'AWS account differs' >&2; exit 98; }
aws --profile "$profile" s3api head-object --bucket "$bucket" --key "$key" --output json > "$out/s3-head.json"
test "$(jq -r .ContentLength "$out/s3-head.json")" = 3025798 || { echo 'S3 length differs' >&2; exit 99; }
gtimeout 90 aws --profile "$profile" s3api get-object --bucket "$bucket" --key "$key" \
  "$out/input/good/objects/$object_sha.jsonl.gz" > "$out/s3-get.json"
test "$(wc -c < "$out/input/good/objects/$object_sha.jsonl.gz" | tr -d ' ')" = 3025798 || { echo 'GET length differs' >&2; exit 100; }
test "$(shasum -a 256 "$out/input/good/objects/$object_sha.jsonl.gz" | awk '{print $1}')" = "$object_sha" || { echo 'GET hash differs' >&2; exit 101; }
cp "$manifest_source" "$out/input/good/manifest.json"
jq '.base.compressed_sha256 = ("0" * 64)' "$manifest_source" > "$out/input/bad-manifest/manifest.json"

set +e
# The quoted program is evaluated by the guest shell after Docker sets its env.
# shellcheck disable=SC2016
gtimeout 120 docker run --pull=never --name "$container" --platform linux/arm64 \
  --memory=2g --memory-swap=2g --cpus=1 --pids-limit=128 \
  --network none --read-only --cap-drop=ALL --security-opt=no-new-privileges \
  --tmpfs /tmp:rw,size=64m \
  --env RAYON_NUM_THREADS=1 --env ICV1_FROZEN_SOURCE_DIR="$source_dir" \
  --env BINARY_PATH="$binary" --env INPUT_ROOT="$out/input" --env OUTPUT_ROOT="$out/run" \
  --mount "type=bind,src=$source_dir,dst=$source_dir,readonly" \
  --mount "type=bind,src=$binary,dst=$binary,readonly" \
  --mount "type=bind,src=$out/input,dst=$out/input,readonly" \
  --mount "type=bind,src=$out/run,dst=$out/run" \
  "$image" sh -ec '
    test "$(uname -m)" = aarch64
    test "$(cat /sys/fs/cgroup/memory.max)" = 2147483648
    test "$(cat /sys/fs/cgroup/memory.swap.max)" = 0
    test "$(cat /sys/fs/cgroup/cpu.max)" = "100000 100000"
    test "$(cat /sys/fs/cgroup/pids.max)" = 128
    test "$(sha256sum "$INPUT_ROOT/good/objects/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf.jsonl.gz" | cut -d " " -f 1)" = 3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf
    "$BINARY_PATH" primary-v5-retained-binding-check 26960859236 "$INPUT_ROOT/good" > "$OUTPUT_ROOT/exact.json" 2> "$OUTPUT_ROOT/exact.stderr"
    set +e
    "$BINARY_PATH" primary-v5-retained-binding-check 26960859237 "$INPUT_ROOT/good" > "$OUTPUT_ROOT/adjacent.stdout" 2> "$OUTPUT_ROOT/adjacent.stderr"
    adjacent=$?
    "$BINARY_PATH" primary-v5-retained-binding-check 26960859236 "$INPUT_ROOT/bad-manifest" > "$OUTPUT_ROOT/bad-manifest.stdout" 2> "$OUTPUT_ROOT/bad-manifest.stderr"
    bad_manifest=$?
    set -e
    printf "%s\n" "$adjacent" > "$OUTPUT_ROOT/adjacent.exit"
    printf "%s\n" "$bad_manifest" > "$OUTPUT_ROOT/bad-manifest.exit"
    test "$adjacent" -ne 0 && test "$bad_manifest" -ne 0
    grep -Fq "no retained v5 adapter for this ordinal" "$OUTPUT_ROOT/adjacent.stderr"
    grep -Fq "retained K600 store manifest differs" "$OUTPUT_ROOT/bad-manifest.stderr"
  ' > "$out/docker.stdout" 2> "$out/docker.stderr"
docker_exit=$?
docker inspect "$container" > "$out/container.inspect.json"
inspect_exit=$?
docker rm -f "$container" > "$out/docker-cleanup.stdout" 2> "$out/docker-cleanup.stderr"
cleanup_exit=$?
set -e
printf 'docker_exit=%s\ninspect_exit=%s\ncleanup_exit=%s\n' "$docker_exit" "$inspect_exit" "$cleanup_exit" > "$out/docker.receipt"
test "$docker_exit" -eq 0 && test "$inspect_exit" -eq 0 && test "$cleanup_exit" -eq 0
test "$(jq -r .status "$out/run/exact.json")" = PASS_EXACT_ADAPTER_BINDING
test "$(jq -r .base_sha256 "$out/run/exact.json")" = "$object_sha"
test "$(jq -r .ordinal "$out/run/exact.json")" = 26960859236
test "$(jq -r '.[0].State.OOMKilled' "$out/container.inspect.json")" = false
test -z "$(docker ps -aq --filter "name=^/$container$")"
