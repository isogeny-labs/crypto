#!/usr/bin/env bash
set -euo pipefail
image_repo=$(git rev-parse --show-toplevel)
image_study="$image_repo/research/twist_index_calculus_20261020/endomorphism_image_base"
image_binary="$image_study/native/target/release/twist-endomorphism-image-study"
image_cpus=$(cat /sys/devices/system/cpu/cpu1/topology/thread_siblings_list)
image_label=${1:?new execution label}
image_output="$image_study/runs/$image_label"
test ! -e "$image_output"
image_metadata="$image_output.metadata"
mkdir "$image_metadata"
sudo -n docker --host=unix:///var/run/docker.sock image inspect twist-image-msolve:0.11.0-precision > "$image_metadata/image-inspect.json"
cp /home/ec2-user/scratch/twist-image-msolve-build.log "$image_metadata/base-build.log"
cp /home/ec2-user/scratch/twist-image-msolve-precision-build.log "$image_metadata/precision-build.log"
grep 'FROM.*debian' "$image_metadata/base-build.log" > "$image_metadata/base-image.txt"
sudo -n docker --host=unix:///var/run/docker.sock run --rm twist-image-msolve:0.11.0-precision sh -c 'cat /opt/msolve-source-commit.txt; sha256sum /usr/local/bin/msolve /usr/local/lib/libmsolve.so.4.0.0; gcc --version | head -1; git -C /opt/msolve diff --stat' > "$image_metadata/versions.txt"
sha256sum "$image_binary" "$image_study/msolve-precision.patch" > "$image_metadata/native-and-patch.sha256"
# Linux protected_regular can refuse root's O_CREAT on an existing user-owned
# /tmp lock. Open that same inode as its owner and expose the existing
# descriptor through procfs. The repository wrapper still takes its normal
# exclusive flock on the original shared lock; no replacement lock is used.
exec 3<>/tmp/crypto-bench.lock
image_lock="/proc/$BASHPID/fd/3"
test "$(stat -Lc '%d:%i' "$image_lock")" = "$(stat -Lc '%d:%i' /tmp/crypto-bench.lock)"
sudo -n python3 "$image_repo/tools/isolated_bench.py" run --wait --lock "$image_lock" --cpus "$image_cpus" --out "$image_output.isolation.json" -- \
  docker --host=unix:///var/run/docker.sock run --rm --user "$(id -u):$(id -g)" --cpuset-cpus "$image_cpus" --memory=8g --env RAYON_NUM_THREADS=1 \
  --mount "type=bind,src=$image_study/runs,dst=/evidence" \
  --mount "type=bind,src=$image_binary,dst=/native/twist-image,readonly" \
  twist-image-msolve:0.11.0-precision /native/twist-image --image-external-benchmark /evidence/external-inputs /usr/local/bin/msolve "/evidence/$image_label"
