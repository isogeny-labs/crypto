#!/usr/bin/env bash
set -euo pipefail
repo=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-full-validation-20261116
plan=$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/k600-colima-charged-retry-20261116/CHARGED_BUILD_PLAN_RETRY.json
producer=/Volumes/SSD990-2/tmp/n83-colima-output/bootstrap-20261116-attempt-1/build-producer
output=/Volumes/SSD990-2/tmp/n83-colima-output/charged-20261116-attempt-2
wrapper=/Volumes/SSD990-2/tmp/n83-colima-home/run-with-receipt.sh
log=/private/tmp/n83-colima-charged-build-20261116-attempt-2.log
child_receipt=/private/tmp/n83-colima-charged-build-20261116-attempt-2.child.receipt
outer_receipt=/private/tmp/n83-colima-charged-build-20261116-attempt-2.outer.receipt
inspect_json=/private/tmp/n83-colima-charged-build-20261116-attempt-2.container.inspect.json
image_json=/private/tmp/n83-colima-charged-build-20261116-attempt-2.image.inspect.json
source_file=/private/tmp/n83-colima-charged-build-20261116-attempt-2.source.commit
container=n83-colima-charged-build-20261116-attempt-2
image=sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573
admit_receipt=/private/tmp/n83-colima-charged-admit-20261116-attempt-4.receipt
admit_log=/private/tmp/n83-colima-charged-admit-20261116-attempt-4.log
admit_source=/private/tmp/n83-colima-charged-admit-20261116-attempt-4.source.commit

cd "$repo"
test -z "$(git status --porcelain)" || { echo 'source not clean' >&2; exit 91; }
source_sha=$(git rev-parse HEAD)
test "$(cat "$admit_source")" = "$source_sha" || { echo 'admission used another source' >&2; exit 92; }
test "$(sed -n 's/^child_exit_code=//p' "$admit_receipt")" = 0 || { echo 'admission did not pass' >&2; exit 93; }
rg -q '^PASS source=' "$admit_log" || { echo 'admission pass line absent' >&2; exit 94; }
test "$(shasum -a 256 "$producer" | awk '{print $1}')" = 5d7dea0ae6afea06528b5680652f83e1a7f3309a9ce3241c607976a20213edb7 || { echo 'producer hash mismatch' >&2; exit 95; }
test "$(shasum -a 256 "$plan" | awk '{print $1}')" = 80c57ce2d32b629ee0a479bb924d41df4dcac809bfe3228faa8ac503a6d4fb1f || { echo 'plan hash mismatch' >&2; exit 96; }
test "$(shasum -a 256 "$wrapper" | awk '{print $1}')" = 090a3020c0ec2862735af0b1c0ab10258199fd4bcefc11c1b55daa0113c48bb3 || { echo 'wrapper hash mismatch' >&2; exit 97; }
test "$(docker --context colima-n83-arm64 image inspect "$image" --format '{{.Id}} {{.Os}}/{{.Architecture}}')" = "$image linux/arm64" || { echo 'toolchain image mismatch' >&2; exit 98; }
test "$(df -Pk / | awk 'NR == 2 {print $4}')" -ge 15728640 || { echo 'boot disk below 15 GiB' >&2; exit 99; }
test "$(df -Pk /Volumes/SSD990-2 | awk 'NR == 2 {print $4}')" -ge 41943040 || { echo 'SSD below 40 GiB' >&2; exit 100; }
test "$(COLIMA_HOME=/Volumes/SSD990-2/tmp/n83-colima-home colima ssh -p n83-arm64 -- df -Pk / | awk 'NR == 2 {print $4}')" -ge 10485760 || { echo 'VM root below 10 GiB' >&2; exit 101; }
for candidate in "$output" "$log" "$child_receipt" "$outer_receipt" "$inspect_json" "$image_json" "$source_file"; do
  test ! -e "$candidate" || { echo "output exists: $candidate" >&2; exit 102; }
done
test -z "$(docker --context colima-n83-arm64 ps -aq --filter "name=^/${container}$")" || { echo 'build container exists' >&2; exit 103; }
if docker --context colima-n83-arm64 image inspect "n83-k600-m6:$source_sha" >/dev/null 2>&1; then echo 'runtime image tag exists' >&2; exit 104; fi
printf '%s\n' "$source_sha" > "$source_file"

started_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
set +e
bash "$wrapper" "$log" "$child_receipt" -- \
  gtimeout -k 20s 4800s docker --context colima-n83-arm64 run --pull=never \
  --name "$container" --platform linux/arm64 \
  --memory=12g --memory-swap=12g --cpus=4 --pids-limit=512 \
  --network=bridge --cap-drop=ALL --security-opt=no-new-privileges \
  --env GIT_OPTIONAL_LOCKS=0 --env GIT_CONFIG_COUNT=1 \
  --env GIT_CONFIG_KEY_0=safe.directory --env GIT_CONFIG_VALUE_0="$repo" \
  --env DOCKER_HOST=unix:///var/run/docker.sock --env EXPECT_SOURCE="$source_sha" \
  --mount "type=bind,src=$repo,dst=$repo,readonly" \
  --mount type=bind,src=/Volumes/SSD990/crypto/.git,dst=/Volumes/SSD990/crypto/.git,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019,dst=/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-colima-inputs,dst=/inputs,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-colima-output,dst=/build-output \
  --mount type=bind,src=/var/run/docker.sock,dst=/var/run/docker.sock \
  "$image" sh -ec '
    repo=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-full-validation-20261116
    plan=$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/k600-colima-charged-retry-20261116/CHARGED_BUILD_PLAN_RETRY.json
    producer=/build-output/bootstrap-20261116-attempt-1/build-producer
    test "$(uname -m)" = aarch64
    test "$(cat /sys/fs/cgroup/memory.max)" = 12884901888
    test "$(cat /sys/fs/cgroup/memory.swap.max)" = 0
    test "$(cat /sys/fs/cgroup/cpu.max)" = "400000 100000"
    test "$(cat /sys/fs/cgroup/pids.max)" = 512
    test "$(git -C "$repo" rev-parse HEAD)" = "$EXPECT_SOURCE"
    test -z "$(git -C "$repo" status --porcelain)"
    exec "$producer" run "$plan" /build-output/charged-20261116-attempt-2
  '
child_exit=$?
docker --context colima-n83-arm64 inspect "$container" > "$inspect_json"
inspect_exit=$?
docker --context colima-n83-arm64 image inspect "n83-k600-m6:$source_sha" > "$image_json" 2>&1
image_inspect_exit=$?
docker --context colima-n83-arm64 rm -f "$container" >/dev/null
cleanup_exit=$?
set -e
ended_utc=$(date -u '+%Y-%m-%dT%H:%M:%SZ')
source_clean=true
test -z "$(git status --porcelain)" || source_clean=false
printf 'schema=n83.colima-charged-build-outer-receipt/v1\nstarted_utc=%s\nended_utc=%s\nsource_commit=%s\nchild_exit_code=%s\ncontainer_inspect_exit_code=%s\nimage_inspect_exit_code=%s\ncleanup_exit_code=%s\nsource_clean=%s\n' \
  "$started_utc" "$ended_utc" "$source_sha" "$child_exit" "$inspect_exit" "$image_inspect_exit" "$cleanup_exit" "$source_clean" > "$outer_receipt"
test "$child_exit" -eq 0 && test "$inspect_exit" -eq 0 && test "$image_inspect_exit" -eq 0 && test "$cleanup_exit" -eq 0 && test "$source_clean" = true
