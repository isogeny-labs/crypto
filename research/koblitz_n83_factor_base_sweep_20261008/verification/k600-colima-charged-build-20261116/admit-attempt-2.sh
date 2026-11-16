#!/usr/bin/env bash
set -euo pipefail
cd /Volumes/SSD990-2/tmp/n83-native-rank-query-wt
test -z "$(git status --porcelain)" || { echo 'source not clean' >&2; exit 91; }
source_sha=$(git rev-parse HEAD)
printf '%s\n' "$source_sha" > /private/tmp/n83-colima-build-admit-attempt-2-source.commit
test ! -e /private/tmp/n83-colima-build-admit-attempt-2.log && test ! -e /private/tmp/n83-colima-build-admit-attempt-2.receipt || { echo 'admission output exists' >&2; exit 92; }
image=sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573
test "$(docker --context colima-n83-arm64 image inspect "$image" --format '{{.Id}} {{.Os}}/{{.Architecture}}')" = "$image linux/arm64" || { echo 'toolchain image mismatch' >&2; exit 93; }
wrapper=/Volumes/SSD990-2/tmp/n83-colima-home/run-with-receipt.sh
test "$(shasum -a 256 "$wrapper" | awk '{print $1}')" = 090a3020c0ec2862735af0b1c0ab10258199fd4bcefc11c1b55daa0113c48bb3 || { echo 'wrapper mismatch' >&2; exit 94; }

bash "$wrapper" /private/tmp/n83-colima-build-admit-attempt-2.log /private/tmp/n83-colima-build-admit-attempt-2.receipt -- \
  gtimeout 120 docker --context colima-n83-arm64 run --rm --pull=never \
  --name n83-colima-build-admission --platform linux/arm64 \
  --memory=12g --memory-swap=12g --cpus=4 --pids-limit=512 \
  --network=none --read-only --cap-drop=ALL --security-opt=no-new-privileges \
  --tmpfs /tmp:rw,nosuid,nodev,noexec,size=64m \
  --env GIT_OPTIONAL_LOCKS=0 --env GIT_CONFIG_COUNT=1 \
  --env GIT_CONFIG_KEY_0=safe.directory \
  --env GIT_CONFIG_VALUE_0=/Volumes/SSD990-2/tmp/n83-native-rank-query-wt \
  --env EXPECT_SOURCE="$source_sha" \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-native-rank-query-wt,dst=/Volumes/SSD990-2/tmp/n83-native-rank-query-wt,readonly \
  --mount type=bind,src=/Volumes/SSD990/crypto/.git,dst=/Volumes/SSD990/crypto/.git,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019,dst=/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-colima-inputs,dst=/inputs,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-colima-output,dst=/build-output \
  "$image" sh -ec '
    repo=/Volumes/SSD990-2/tmp/n83-native-rank-query-wt
    test "$(uname -m)" = aarch64
    test "$(cat /sys/fs/cgroup/memory.max)" = 12884901888
    test "$(cat /sys/fs/cgroup/memory.swap.max)" = 0
    test "$(cat /sys/fs/cgroup/cpu.max)" = "400000 100000"
    test "$(cat /sys/fs/cgroup/pids.max)" = 512
    test -d /build-output
    test -d /Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019
    : > /tmp/git.err
    git -C "$repo" cat-file -e "$EXPECT_SOURCE^{commit}" 2>> /tmp/git.err
    test "$(git -C "$repo" rev-parse HEAD 2>> /tmp/git.err)" = "$EXPECT_SOURCE"
    test -z "$(git -C "$repo" status --porcelain 2>> /tmp/git.err)"
    test "$(git -C "$repo" show HEAD:research/koblitz_n83_factor_base_sweep_20261008/verification/k600-colima-charged-build-20261116/CHARGED_BUILD_PLAN_COLIMA.json 2>> /tmp/git.err | sha256sum | cut -d " " -f1)" = bb4cc0209fca70133cd28240a3f6ac7017ac15327ffe06a8df474843a6b3841b
    test ! -s /tmp/git.err
    test "$(sha256sum "$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/k600-colima-charged-build-20261116/CHARGED_BUILD_PLAN_COLIMA.json" | cut -d " " -f1)" = bb4cc0209fca70133cd28240a3f6ac7017ac15327ffe06a8df474843a6b3841b
    test "$(sha256sum "$repo/research/koblitz_n83_factor_base_sweep_20261008/verification/k600-colima-charged-build-20261116/CHARGED_BUILD_STEPS.sh" | cut -d " " -f1)" = d13ce79155685437c7483f8d2e1d30461a14fc08e51e9c3cd4e3cdc0d3ad0c64
    test "$(sha256sum /bin/bash | cut -d " " -f1)" = f5918390c5b15392ad8835e8368a79c1ee9937fbb19f2349cccf5d4524183299
    test "$(sha256sum /inputs/cryptominisat.tar.gz | cut -d " " -f1)" = 2ca8e47c3ab989c3b9842c3f230aca1c6f74a767797a24584c578dee264080a1
    test "$(sha256sum /inputs/cadical.tar.gz | cut -d " " -f1)" = beaeb17a751db88d5384b35d65be82501d94cf60f758b768b59bfb2b437cb34b
    test "$(sha256sum /inputs/cadiback.tar.gz | cut -d " " -f1)" = e9b29931f4d8e12a18eb12b862054af462332cf747be174eb9f7bdfdf4658918
    printf "PASS arch=%s memory.max=%s memory.swap.max=%s cpu.max=%s pids.max=%s source=%s\n" \
      "$(uname -m)" "$(cat /sys/fs/cgroup/memory.max)" "$(cat /sys/fs/cgroup/memory.swap.max)" \
      "$(cat /sys/fs/cgroup/cpu.max)" "$(cat /sys/fs/cgroup/pids.max)" "$(git -C "$repo" rev-parse HEAD)"
  '
