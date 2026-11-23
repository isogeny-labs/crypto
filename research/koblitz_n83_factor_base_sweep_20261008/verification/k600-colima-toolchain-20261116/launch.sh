#!/usr/bin/env bash
set -euo pipefail
cd /Volumes/SSD990-2/tmp/n83-native-rank-query-wt
test -z "$(git status --porcelain)" || { echo 'source not clean' >&2; exit 91; }
source_sha=$(git rev-parse HEAD)
printf '%s\n' "$source_sha" > /private/tmp/n83-colima-toolchain-20261116.commit

context=/Volumes/SSD990-2/tmp/n83-colima-toolchain-context
tag=n83-k600-m6-toolchain:colima-20261116
test ! -e "$context" || { echo 'build context exists' >&2; exit 92; }
test ! -e /private/tmp/n83-colima-toolchain-20261116.log && test ! -e /private/tmp/n83-colima-toolchain-20261116.receipt || { echo 'build receipt exists' >&2; exit 93; }
test "$(git rev-parse "$source_sha":research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-arms-20261115/CHARGED_BUILD_TOOLCHAIN.Dockerfile)" = 8af2c7ac304bfb09f20caaec5be9eadfbdc33c75 || { echo 'Dockerfile blob mismatch' >&2; exit 94; }
test "$(docker --context colima-n83-arm64 info --format '{{.Architecture}} {{.CgroupVersion}}')" = 'aarch64 2' || { echo 'Docker host mismatch' >&2; exit 95; }
if docker --context colima-n83-arm64 image inspect "$tag" >/dev/null 2>&1; then echo 'image tag exists' >&2; exit 96; fi
test "$(df -Pk / | awk 'NR == 2 {print $4}')" -ge 20971520 || { echo 'boot disk below 20 GiB' >&2; exit 97; }
test "$(df -Pk /Volumes/SSD990-2 | awk 'NR == 2 {print $4}')" -ge 83886080 || { echo 'SSD below 80 GiB' >&2; exit 98; }
test "$(COLIMA_HOME=/Volumes/SSD990-2/tmp/n83-colima-home colima ssh -p n83-arm64 -- df -Pk / | awk 'NR == 2 {print $4}')" -ge 10485760 || { echo 'VM root below 10 GiB' >&2; exit 99; }

mkdir -p "$context"
git show "$source_sha":research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-arms-20261115/CHARGED_BUILD_TOOLCHAIN.Dockerfile > "$context/Dockerfile"
test "$(shasum -a 256 "$context/Dockerfile" | awk '{print $1}')" = 93e2eada27062e311824f4f5f88a49b1763e0c30a90ae8457517552de2437e88 || { echo 'Dockerfile SHA mismatch' >&2; exit 100; }
wrapper=/Volumes/SSD990-2/tmp/n83-colima-home/run-with-receipt.sh
test "$(shasum -a 256 "$wrapper" | awk '{print $1}')" = 090a3020c0ec2862735af0b1c0ab10258199fd4bcefc11c1b55daa0113c48bb3 || { echo 'wrapper mismatch' >&2; exit 101; }

bash "$wrapper" /private/tmp/n83-colima-toolchain-20261116.log /private/tmp/n83-colima-toolchain-20261116.receipt -- \
  gtimeout 900 docker --context colima-n83-arm64 build \
  --platform linux/arm64 --pull=false --no-cache \
  -t "$tag" "$context"
