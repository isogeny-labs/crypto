#!/usr/bin/env bash
set -euo pipefail

cd /Volumes/SSD990-2/tmp/n83-native-rank-query-wt
test -z "$(git status --porcelain)" || { echo 'source not clean' >&2; exit 91; }
source_sha=$(git rev-parse HEAD)
printf '%s\n' "$source_sha" > /private/tmp/n83-colima-host-20261115.commit

home=/Volumes/SSD990-2/tmp/n83-colima-home
inputs=/Volumes/SSD990-2/tmp/n83-colima-inputs
output=/Volumes/SSD990-2/tmp/n83-colima-output
test ! -e "$home" && test ! -e "$inputs" && test ! -e "$output" || { echo 'host path exists' >&2; exit 92; }
test "$(sysctl -n hw.memsize)" -ge 51539607552 || { echo 'host RAM too small' >&2; exit 93; }
test "$(df -Pk / | awk 'NR == 2 {print $4}')" -ge 10485760 || { echo 'boot disk below 10 GiB' >&2; exit 94; }
test "$(df -Pk /Volumes/SSD990-2 | awk 'NR == 2 {print $4}')" -ge 83886080 || { echo 'SSD below 80 GiB' >&2; exit 95; }

for item in \
  'n83-cms-5.14.7-3c8e228.tar.gz 2ca8e47c3ab989c3b9842c3f230aca1c6f74a767797a24584c578dee264080a1' \
  'n83-cadical-818c956.tar.gz beaeb17a751db88d5384b35d65be82501d94cf60f758b768b59bfb2b437cb34b' \
  'n83-cadiback-802e1f6.tar.gz e9b29931f4d8e12a18eb12b862054af462332cf747be174eb9f7bdfdf4658918'; do
  read -r name expected <<< "$item"
  test "$(shasum -a 256 "/Volumes/SSD990-2/tmp/$name" | awk '{print $1}')" = "$expected" || { echo "archive hash mismatch: $name" >&2; exit 96; }
done

mkdir -p "$home" "$inputs" "$output"
cp /Volumes/SSD990-2/tmp/n83-cms-5.14.7-3c8e228.tar.gz "$inputs/cryptominisat.tar.gz"
cp /Volumes/SSD990-2/tmp/n83-cadical-818c956.tar.gz "$inputs/cadical.tar.gz"
cp /Volumes/SSD990-2/tmp/n83-cadiback-802e1f6.tar.gz "$inputs/cadiback.tar.gz"
test "$(shasum -a 256 "$inputs/cryptominisat.tar.gz" | awk '{print $1}')" = 2ca8e47c3ab989c3b9842c3f230aca1c6f74a767797a24584c578dee264080a1
test "$(shasum -a 256 "$inputs/cadical.tar.gz" | awk '{print $1}')" = beaeb17a751db88d5384b35d65be82501d94cf60f758b768b59bfb2b437cb34b
test "$(shasum -a 256 "$inputs/cadiback.tar.gz" | awk '{print $1}')" = e9b29931f4d8e12a18eb12b862054af462332cf747be174eb9f7bdfdf4658918

wrapper="$home/run-with-receipt.sh"
git show "$source_sha":research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-validation-wrapper-20261115/run-with-receipt.sh > "$wrapper"
test "$(shasum -a 256 "$wrapper" | awk '{print $1}')" = 090a3020c0ec2862735af0b1c0ab10258199fd4bcefc11c1b55daa0113c48bb3 || { echo 'wrapper hash mismatch' >&2; exit 97; }

bash "$wrapper" /private/tmp/n83-colima-host-20261115.log /private/tmp/n83-colima-host-20261115.receipt -- \
  /usr/bin/env COLIMA_HOME="$home" gtimeout 900 colima start n83-arm64 \
  --cpus 4 --memory 16 --disk 64 --root-disk 32 --arch aarch64 \
  --runtime docker --vm-type vz --mount-type virtiofs --activate=false \
  --mount /Volumes/SSD990-2/tmp/n83-native-rank-query-wt \
  --mount /Volumes/SSD990/crypto/.git \
  --mount "$inputs" --mount "$output:w"
