#!/usr/bin/env bash
set -euo pipefail

repo=/Volumes/SSD990-2/tmp/n83-native-rank-query-wt
output=/Volumes/SSD990-2/tmp/n83-colima-output/bootstrap-20261116-attempt-1
log=/private/tmp/n83-colima-bootstrap-20261116-attempt-1.log
child_receipt=/private/tmp/n83-colima-bootstrap-20261116-attempt-1.child.receipt
outer_receipt=/private/tmp/n83-colima-bootstrap-20261116-attempt-1.outer.receipt
inspect_json=/private/tmp/n83-colima-bootstrap-20261116-attempt-1.inspect.json
source_file=/private/tmp/n83-colima-bootstrap-20261116-attempt-1.source.commit
container=n83-colima-bootstrap-20261116-attempt-1
image=sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573
wrapper=/Volumes/SSD990-2/tmp/n83-colima-home/run-with-receipt.sh

cd "$repo"
test -z "$(git status --porcelain)" || { echo 'source not clean' >&2; exit 91; }
source_sha=$(git rev-parse HEAD)
test "$(docker --context colima-n83-arm64 image inspect "$image" --format '{{.Id}} {{.Os}}/{{.Architecture}}')" = "$image linux/arm64" || { echo 'toolchain image mismatch' >&2; exit 92; }
test "$(shasum -a 256 "$wrapper" | awk '{print $1}')" = 090a3020c0ec2862735af0b1c0ab10258199fd4bcefc11c1b55daa0113c48bb3 || { echo 'wrapper mismatch' >&2; exit 93; }
test "$(df -Pk / | awk 'NR == 2 {print $4}')" -ge 20971520 || { echo 'boot disk below 20 GiB' >&2; exit 94; }
test "$(df -Pk /Volumes/SSD990-2 | awk 'NR == 2 {print $4}')" -ge 83886080 || { echo 'SSD below 80 GiB' >&2; exit 95; }
test "$(COLIMA_HOME=/Volumes/SSD990-2/tmp/n83-colima-home colima ssh -p n83-arm64 -- df -Pk / | awk 'NR == 2 {print $4}')" -ge 10485760 || { echo 'VM root below 10 GiB' >&2; exit 96; }
for candidate in "$output" "$log" "$child_receipt" "$outer_receipt" "$inspect_json" "$source_file"; do
  test ! -e "$candidate" || { echo "output already exists: $candidate" >&2; exit 97; }
done
test -z "$(docker --context colima-n83-arm64 ps -aq --filter "name=^/${container}$")" || { echo 'container name exists' >&2; exit 98; }
printf '%s\n' "$source_sha" > "$source_file"

started_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
set +e
bash "$wrapper" "$log" "$child_receipt" -- \
  gtimeout 5400 docker --context colima-n83-arm64 run --pull=never \
  --name "$container" --platform linux/arm64 \
  --memory=12g --memory-swap=12g --cpus=4 --pids-limit=512 \
  --network=bridge --cap-drop=ALL --security-opt=no-new-privileges \
  --env GIT_OPTIONAL_LOCKS=0 --env GIT_CONFIG_COUNT=1 \
  --env GIT_CONFIG_KEY_0=safe.directory --env GIT_CONFIG_VALUE_0="$repo" \
  --env EXPECT_SOURCE="$source_sha" \
  --env CARGO_TARGET_DIR="$output/cargo-target" \
  --env CARGO_INCREMENTAL=0 \
  --mount "type=bind,src=$repo,dst=$repo,readonly" \
  --mount type=bind,src=/Volumes/SSD990/crypto/.git,dst=/Volumes/SSD990/crypto/.git,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019,dst=/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-colima-output,dst=/Volumes/SSD990-2/tmp/n83-colima-output \
  "$image" sh -ec '
    repo=/Volumes/SSD990-2/tmp/n83-native-rank-query-wt
    output=/Volumes/SSD990-2/tmp/n83-colima-output/bootstrap-20261116-attempt-1
    test "$(uname -m)" = aarch64
    test "$(cat /sys/fs/cgroup/memory.max)" = 12884901888
    test "$(cat /sys/fs/cgroup/memory.swap.max)" = 0
    test "$(cat /sys/fs/cgroup/cpu.max)" = "400000 100000"
    test "$(cat /sys/fs/cgroup/pids.max)" = 512
    test "$(git -C "$repo" rev-parse HEAD)" = "$EXPECT_SOURCE"
    test -z "$(git -C "$repo" status --porcelain)"
    mkdir "$output"
    cd "$repo"
    printf "source=%s manifest_dir=%s target_dir=%s\n" "$EXPECT_SOURCE" "$repo" "$CARGO_TARGET_DIR"
    cargo build --release --locked --no-default-features -j 1 --example n83_k600_m6_linux_build
    install -m 755 "$CARGO_TARGET_DIR/release/examples/n83_k600_m6_linux_build" "$output/build-producer"
    sha256sum "$output/build-producer"
    test -z "$(git status --porcelain)"
  '
child_exit=$?
docker --context colima-n83-arm64 inspect "$container" > "$inspect_json"
inspect_exit=$?
docker --context colima-n83-arm64 rm -f "$container" >/dev/null
cleanup_exit=$?
set -e
ended_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
printf 'schema=n83.colima-bootstrap-outer-receipt/v1\nstarted_utc=%s\nended_utc=%s\nsource_commit=%s\nchild_exit_code=%s\ninspect_exit_code=%s\ncleanup_exit_code=%s\n' \
  "$started_utc" "$ended_utc" "$source_sha" "$child_exit" "$inspect_exit" "$cleanup_exit" > "$outer_receipt"
test "$child_exit" -eq 0 && test "$inspect_exit" -eq 0 && test "$cleanup_exit" -eq 0
