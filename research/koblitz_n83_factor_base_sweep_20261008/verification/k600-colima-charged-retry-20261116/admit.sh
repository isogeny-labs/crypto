#!/usr/bin/env bash
set -euo pipefail
repo=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-full-validation-20261116
plan=$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/k600-colima-charged-retry-20261116/CHARGED_BUILD_PLAN_RETRY.json
producer=/Volumes/SSD990-2/tmp/n83-colima-output/bootstrap-20261116-attempt-1/build-producer
wrapper=/Volumes/SSD990-2/tmp/n83-colima-home/run-with-receipt.sh
log=/private/tmp/n83-colima-charged-admit-20261116-attempt-2.log
receipt=/private/tmp/n83-colima-charged-admit-20261116-attempt-2.receipt
source_file=/private/tmp/n83-colima-charged-admit-20261116-attempt-2.source.commit
image=sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573

cd "$repo"
test -z "$(git status --porcelain)" || { echo 'source not clean' >&2; exit 91; }
source_sha=$(git rev-parse HEAD)
git diff --quiet 0f558a0f7998416a9db0924ba9425d69e9c0a7aa HEAD -- examples/n83_k600_m6_linux_build.rs Cargo.toml Cargo.lock || { echo 'producer source changed' >&2; exit 92; }
test "$(shasum -a 256 "$producer" | awk '{print $1}')" = 5d7dea0ae6afea06528b5680652f83e1a7f3309a9ce3241c607976a20213edb7 || { echo 'producer hash mismatch' >&2; exit 93; }
test "$(shasum -a 256 "$plan" | awk '{print $1}')" = 80c57ce2d32b629ee0a479bb924d41df4dcac809bfe3228faa8ac503a6d4fb1f || { echo 'plan hash mismatch' >&2; exit 104; }
test "$(shasum -a 256 "$wrapper" | awk '{print $1}')" = 090a3020c0ec2862735af0b1c0ab10258199fd4bcefc11c1b55daa0113c48bb3 || { echo 'wrapper hash mismatch' >&2; exit 94; }
test "$(docker --context colima-n83-arm64 image inspect "$image" --format '{{.Id}} {{.Os}}/{{.Architecture}}')" = "$image linux/arm64" || { echo 'toolchain image mismatch' >&2; exit 95; }
test "$(df -Pk / | awk 'NR == 2 {print $4}')" -ge 20971520 || { echo 'boot disk below 20 GiB' >&2; exit 96; }
test "$(df -Pk /Volumes/SSD990-2 | awk 'NR == 2 {print $4}')" -ge 41943040 || { echo 'SSD below 40 GiB' >&2; exit 97; }
test "$(COLIMA_HOME=/Volumes/SSD990-2/tmp/n83-colima-home colima ssh -p n83-arm64 -- df -Pk / | awk 'NR == 2 {print $4}')" -ge 10485760 || { echo 'VM root below 10 GiB' >&2; exit 98; }
for candidate in "$log" "$receipt" "$source_file" /Volumes/SSD990-2/tmp/n83-colima-output/charged-20261116-attempt-2; do
  test ! -e "$candidate" || { echo "output exists: $candidate" >&2; exit 99; }
done
test -z "$(docker --context colima-n83-arm64 ps -aq --filter name=^/n83-colima-charged-admit-20261116-attempt-2$)" || { echo 'admission container exists' >&2; exit 100; }
printf '%s\n' "$source_sha" > "$source_file"

bash "$wrapper" "$log" "$receipt" -- \
  gtimeout 120 docker --context colima-n83-arm64 run --rm --pull=never \
  --name n83-colima-charged-admit-20261116-attempt-2 --platform linux/arm64 \
  --memory=12g --memory-swap=12g --cpus=4 --pids-limit=512 \
  --network=none --read-only --cap-drop=ALL --security-opt=no-new-privileges \
  --tmpfs /tmp:rw,nosuid,nodev,size=64m \
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
    test -S /var/run/docker.sock
    test -w /build-output
    test ! -e /build-output/charged-20261116-attempt-2
    : > /tmp/git.err
    git -C "$repo" cat-file -e "$EXPECT_SOURCE^{commit}" 2>>/tmp/git.err
    test "$(git -C "$repo" rev-parse HEAD 2>>/tmp/git.err)" = "$EXPECT_SOURCE"
    test -z "$(git -C "$repo" status --porcelain 2>>/tmp/git.err)"
    test "$(git -C "$repo" show HEAD:research/koblitz_n83_factor_base_sweep_20261008/verification/k600-colima-charged-retry-20261116/CHARGED_BUILD_PLAN_RETRY.json 2>>/tmp/git.err | sha256sum | cut -d " " -f1)" = 80c57ce2d32b629ee0a479bb924d41df4dcac809bfe3228faa8ac503a6d4fb1f
    test ! -s /tmp/git.err
    test "$(sha256sum "$plan" | cut -d " " -f1)" = 80c57ce2d32b629ee0a479bb924d41df4dcac809bfe3228faa8ac503a6d4fb1f
    test "$(sha256sum "$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/k600-colima-charged-retry-20261116/CHARGED_BUILD_STEPS.sh" | cut -d " " -f1)" = d13ce79155685437c7483f8d2e1d30461a14fc08e51e9c3cd4e3cdc0d3ad0c64
    test "$(sha256sum /bin/bash | cut -d " " -f1)" = f5918390c5b15392ad8835e8368a79c1ee9937fbb19f2349cccf5d4524183299
    test "$(sha256sum "$producer" | cut -d " " -f1)" = 5d7dea0ae6afea06528b5680652f83e1a7f3309a9ce3241c607976a20213edb7
    test "$(sha256sum /inputs/cryptominisat.tar.gz | cut -d " " -f1)" = 2ca8e47c3ab989c3b9842c3f230aca1c6f74a767797a24584c578dee264080a1
    test "$(sha256sum /inputs/cadical.tar.gz | cut -d " " -f1)" = beaeb17a751db88d5384b35d65be82501d94cf60f758b768b59bfb2b437cb34b
    test "$(sha256sum /inputs/cadiback.tar.gz | cut -d " " -f1)" = e9b29931f4d8e12a18eb12b862054af462332cf747be174eb9f7bdfdf4658918
    docker info --format "{{.Architecture}}" | grep -qx aarch64
    if image_output=$(docker image inspect "n83-k600-m6:$EXPECT_SOURCE" 2>&1); then
      echo "runtime image tag already exists" >&2
      exit 101
    else
      printf "%s\n" "$image_output" | grep -Eq "No such (image|object)" || exit 102
    fi
    printf "PASS source=%s arch=%s memory.max=%s swap.max=%s cpu.max=%s pids.max=%s producer=%s\n" \
      "$EXPECT_SOURCE" "$(uname -m)" "$(cat /sys/fs/cgroup/memory.max)" \
      "$(cat /sys/fs/cgroup/memory.swap.max)" "$(cat /sys/fs/cgroup/cpu.max)" \
      "$(cat /sys/fs/cgroup/pids.max)" "$(sha256sum "$producer" | cut -d " " -f1)"
  '
