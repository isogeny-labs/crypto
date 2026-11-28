#!/usr/bin/env bash
set -euo pipefail

repo=$(git rev-parse --show-toplevel)
study=research/koblitz_n83_factor_base_sweep_20261008
source_commit=cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff
source_dir=/Volumes/SSD990-2/tmp/n83-colima-inputs/n83-v5-exporter-source-20261116
output=/Volumes/SSD990-2/tmp/n83-colima-output/n83-v5-exporter-build-20261116
colima_home=/Volumes/SSD990-2/tmp/n83-colima-home
wrapper="$colima_home/run-with-receipt.sh"
image=sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573
container=n83-v5-exporter-build-20261116
export DOCKER_HOST="unix://$colima_home/n83-arm64/docker.sock"

free_kib() { df -Pk "$1" | awk 'NR == 2 {print $4}'; }
cd "$repo"
test -z "$(git status --porcelain)" || { echo 'checkout not clean' >&2; exit 90; }
git diff --quiet "$source_commit" HEAD -- src examples crates gpu tools Cargo.toml Cargo.lock \
  "$study/primary_adapter.rs" "$study/size-frontier-v2.json" \
  "$study/paired-frontier-v3.json" "$study/paired-frontier-v4.json" \
  "$study/paired-frontier-v5.json" || { echo 'pinned code/design changed' >&2; exit 91; }
test ! -e "$source_dir" && test ! -e "$output" || { echo 'attempt path exists' >&2; exit 92; }
test "$(free_kib /)" -ge 2097152 || { echo 'boot below 2 GiB' >&2; exit 93; }
test "$(free_kib /Volumes/SSD990-2)" -ge 4194304 || { echo 'SSD below 4 GiB' >&2; exit 94; }
test "$(COLIMA_HOME="$colima_home" colima ssh -p n83-arm64 -- df -Pk / | awk 'NR == 2 {print $4}')" -ge 10485760 || { echo 'VM root below 10 GiB' >&2; exit 95; }
test "$(docker image inspect "$image" --format '{{.Id}} {{.Os}}/{{.Architecture}}')" = "$image linux/arm64" || { echo 'image mismatch' >&2; exit 96; }
test "$(shasum -a 256 "$wrapper" | awk '{print $1}')" = 090a3020c0ec2862735af0b1c0ab10258199fd4bcefc11c1b55daa0113c48bb3 || { echo 'receipt wrapper changed' >&2; exit 97; }
test -z "$(docker ps -q)" && test -z "$(docker ps -aq --filter "name=^/${container}$")" || { echo 'container overlap' >&2; exit 98; }

mkdir "$source_dir"
rsync -a --exclude='.git' --exclude='target' "$repo/" "$source_dir/"
for path in examples/koblitz_n83_factor_base_export.rs examples/n83_factor_base_sweep_v5.rs \
  "$study/primary_adapter.rs" "$study/size-frontier-v2.json" \
  "$study/paired-frontier-v3.json" "$study/paired-frontier-v4.json" \
  "$study/paired-frontier-v5.json" src/cryptanalysis/binary_semaev_chain_sat.rs \
  src/cryptanalysis/sat.rs src/cryptanalysis/koblitz_index_calculus.rs; do
  cmp "$repo/$path" "$source_dir/$path" || { echo "snapshot drift: $path" >&2; exit 99; }
done
printf '{"schema":"n83.chain-s3-source-attestation/v1","source_commit":"%s","status_clean":true}\n' \
  "$source_commit" > "$source_dir/attestation.json"
mkdir "$output" "$output/tmp"
printf '%s\n' "$source_commit" > "$output/source.commit"
date -u +%Y-%m-%dT%H:%M:%SZ > "$output/start.utc"
printf 'utc\tboot_available_kib\tssd2_available_kib\n' > "$output/disk.tsv"
(
  while :; do
    now=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    boot=$(free_kib /)
    ssd=$(free_kib /Volumes/SSD990-2)
    printf '%s\t%s\t%s\n' "$now" "$boot" "$ssd" >> "$output/disk.tsv"
    if test "$boot" -lt 2097152 || test "$ssd" -lt 2097152; then
      printf 'disk_floor boot_kib=%s ssd2_kib=%s\n' "$boot" "$ssd" > "$output/disk.stop"
      docker stop -t 5 "$container" >/dev/null 2>&1 || true
      break
    fi
    sleep 10
  done
) &
watcher=$!
set +e
bash "$wrapper" "$output/build.log" "$output/child.receipt" -- \
  gtimeout 1800 docker run --pull=never --name "$container" --platform linux/arm64 \
  --memory=12g --memory-swap=12g --cpus=4 --pids-limit=512 \
  --network none --cap-drop=ALL --security-opt=no-new-privileges \
  --tmpfs /tmp:rw,size=512m \
  --env CARGO_TARGET_DIR="$output/target" --env CARGO_INCREMENTAL=0 \
  --env TMPDIR="$output/tmp" \
  --mount "type=bind,src=$source_dir,dst=$source_dir,readonly" \
  --mount type=bind,src=/Volumes/SSD990-2/tmp/n83-colima-output,dst=/Volumes/SSD990-2/tmp/n83-colima-output \
  --workdir "$source_dir" "$image" sh -ec '
    test "$(uname -m)" = aarch64
    test "$(cat /sys/fs/cgroup/memory.max)" = 12884901888
    test "$(cat /sys/fs/cgroup/memory.swap.max)" = 0
    test "$(cat /sys/fs/cgroup/cpu.max)" = "400000 100000"
    test "$(cat /sys/fs/cgroup/pids.max)" = 512
    test -f attestation.json
    cargo build --release --locked --offline -j 1 --example koblitz_n83_factor_base_export
  '
child_exit=$?
kill "$watcher" >/dev/null 2>&1 || true
wait "$watcher" >/dev/null 2>&1 || true
docker inspect "$container" > "$output/container.inspect.json"
inspect_exit=$?
docker rm -f "$container" >/dev/null 2>&1
cleanup_exit=$?
set -e
date -u +%Y-%m-%dT%H:%M:%SZ > "$output/end.utc"
binary="$output/target/release/examples/koblitz_n83_factor_base_export"
if test -f "$binary"; then
  file "$binary" > "$output/binary.file"
  shasum -a 256 "$binary" > "$output/binary.sha256"
  wc -c "$binary" > "$output/binary.bytes"
fi
printf 'schema=n83.v5-linux-exporter-build-outer/v1\nsource_commit=%s\nchild_exit_code=%s\ninspect_exit_code=%s\ncleanup_exit_code=%s\ndisk_stop=%s\n' \
  "$source_commit" "$child_exit" "$inspect_exit" "$cleanup_exit" "$(test -f "$output/disk.stop" && echo true || echo false)" > "$output/outer.receipt"
test "$child_exit" -eq 0 && test "$inspect_exit" -eq 0 && test "$cleanup_exit" -eq 0
test ! -f "$output/disk.stop"
test -f "$binary"
rg -q 'ELF 64-bit.*ARM aarch64' "$output/binary.file"
test "$(docker ps -aq --filter "name=^/${container}$")" = ''
