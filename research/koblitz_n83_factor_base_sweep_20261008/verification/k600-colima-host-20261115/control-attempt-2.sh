#!/usr/bin/env bash
set -euo pipefail
cd /Volumes/SSD990-2/tmp/n83-native-rank-query-wt
test -z "$(git status --porcelain)" || { echo 'source not clean' >&2; exit 91; }
test "$(docker --context colima-n83-arm64 info --format '{{.Architecture}}')" = aarch64 || { echo 'Docker not aarch64' >&2; exit 92; }
test ! -e /private/tmp/n83-colima-control-attempt-2.log && test ! -e /private/tmp/n83-colima-control-attempt-2.receipt || { echo 'control output exists' >&2; exit 93; }
wrapper=/Volumes/SSD990-2/tmp/n83-colima-home/run-with-receipt.sh
test "$(shasum -a 256 "$wrapper" | awk '{print $1}')" = 090a3020c0ec2862735af0b1c0ab10258199fd4bcefc11c1b55daa0113c48bb3 || { echo 'wrapper mismatch' >&2; exit 94; }

bash "$wrapper" /private/tmp/n83-colima-control-attempt-2.log /private/tmp/n83-colima-control-attempt-2.receipt -- \
  gtimeout 600 docker --context colima-n83-arm64 run --rm \
  --name n83-colima-host-control --platform linux/arm64 \
  --memory=4g --memory-swap=4g --cpus=1 --pids-limit=64 \
  --network=none --read-only --cap-drop=ALL --security-opt=no-new-privileges \
  --tmpfs /tmp:rw,nosuid,nodev,noexec,size=64m \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-native-rank-query-wt,dst=/Volumes/SSD990-2/tmp/n83-native-rank-query-wt,readonly \
  --mount type=bind,src=/Volumes/SSD990/crypto/.git,dst=/Volumes/SSD990/crypto/.git,readonly \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-colima-inputs,dst=/Volumes/SSD990-2/tmp/n83-colima-inputs,readonly \
  rust@sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c \
  sh -ec '
    test "$(uname -m)" = aarch64
    test "$(cat /sys/fs/cgroup/memory.max)" = 4294967296
    test "$(cat /sys/fs/cgroup/memory.swap.max)" = 0
    test "$(cat /sys/fs/cgroup/cpu.max)" = "100000 100000"
    test "$(cat /sys/fs/cgroup/pids.max)" = 64
    test -f /Volumes/SSD990-2/tmp/n83-native-rank-query-wt/.git
    test -f /Volumes/SSD990/crypto/.git/HEAD
    test -f /Volumes/SSD990-2/tmp/n83-colima-inputs/cryptominisat.tar.gz
    printf "arch=%s memory.max=%s memory.swap.max=%s cpu.max=%s pids.max=%s\n" \
      "$(uname -m)" "$(cat /sys/fs/cgroup/memory.max)" "$(cat /sys/fs/cgroup/memory.swap.max)" \
      "$(cat /sys/fs/cgroup/cpu.max)" "$(cat /sys/fs/cgroup/pids.max)"
  '
