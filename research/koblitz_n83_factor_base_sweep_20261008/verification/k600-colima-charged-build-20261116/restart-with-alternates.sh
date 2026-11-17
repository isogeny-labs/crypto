#!/usr/bin/env bash
set -euo pipefail
cd /Volumes/SSD990-2/tmp/n83-native-rank-query-wt
test -z "$(git status --porcelain)" || { echo 'source not clean' >&2; exit 91; }
alt=/Volumes/SSD990-2/.codex/git-objects/crypto-n83-20261019
test "$(cat /Volumes/SSD990/crypto/.git/objects/info/alternates)" = "$alt" && test -d "$alt" || { echo 'alternate object path mismatch' >&2; exit 92; }
test -z "$(docker --context colima-n83-arm64 ps -q)" || { echo 'Colima container active' >&2; exit 93; }
image=sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573
test "$(docker --context colima-n83-arm64 image inspect "$image" --format '{{.Id}} {{.Os}}/{{.Architecture}}')" = "$image linux/arm64" || { echo 'image mismatch before restart' >&2; exit 94; }
for item in /private/tmp/n83-colima-alt-stop-20261116.log /private/tmp/n83-colima-alt-stop-20261116.receipt /private/tmp/n83-colima-alt-start-20261116.log /private/tmp/n83-colima-alt-start-20261116.receipt; do
  test ! -e "$item" || { echo "restart output exists: $item" >&2; exit 95; }
done
wrapper=/Volumes/SSD990-2/tmp/n83-colima-home/run-with-receipt.sh
test "$(shasum -a 256 "$wrapper" | awk '{print $1}')" = 090a3020c0ec2862735af0b1c0ab10258199fd4bcefc11c1b55daa0113c48bb3 || { echo 'wrapper mismatch' >&2; exit 96; }
printf '%s\n' "$(git rev-parse HEAD)" > /private/tmp/n83-colima-alt-restart-20261116.commit

bash "$wrapper" /private/tmp/n83-colima-alt-stop-20261116.log /private/tmp/n83-colima-alt-stop-20261116.receipt -- \
  /usr/bin/env COLIMA_HOME=/Volumes/SSD990-2/tmp/n83-colima-home gtimeout 120 colima stop n83-arm64

bash "$wrapper" /private/tmp/n83-colima-alt-start-20261116.log /private/tmp/n83-colima-alt-start-20261116.receipt -- \
  /usr/bin/env COLIMA_HOME=/Volumes/SSD990-2/tmp/n83-colima-home gtimeout 900 colima start n83-arm64 \
  --cpus 4 --memory 16 --disk 64 --root-disk 32 --arch aarch64 \
  --runtime docker --vm-type vz --mount-type virtiofs --activate=false \
  --mount /Volumes/SSD990-2/tmp/n83-native-rank-query-wt \
  --mount /Volumes/SSD990/crypto/.git \
  --mount /Volumes/SSD990-2/tmp/n83-colima-inputs \
  --mount /Volumes/SSD990-2/tmp/n83-colima-output:w \
  --mount "$alt"

test "$(docker --context colima-n83-arm64 image inspect "$image" --format '{{.Id}} {{.Os}}/{{.Architecture}}')" = "$image linux/arm64" || { echo 'image mismatch after restart' >&2; exit 97; }
COLIMA_HOME=/Volumes/SSD990-2/tmp/n83-colima-home colima ssh -p n83-arm64 -- test -d "$alt" || { echo 'alternate object path not mounted' >&2; exit 98; }
test "$(docker --context colima-n83-arm64 info --format '{{.Architecture}} {{.CgroupVersion}}')" = 'aarch64 2' || { echo 'Docker host changed' >&2; exit 99; }
test -z "$(git status --porcelain)" || { echo 'source changed during restart' >&2; exit 100; }
