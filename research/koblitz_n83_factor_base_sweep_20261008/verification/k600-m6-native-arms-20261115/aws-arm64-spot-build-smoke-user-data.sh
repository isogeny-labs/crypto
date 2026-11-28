#!/bin/bash
set -euo pipefail

# Frozen source and container identities. This script grants no IAM access
# and sends a compressed execution log only to the EC2 serial console.
source_commit=0ae1150458da16d5becc3316d0cf9217c43b3209
source_branch=codex/n83-native-rank-query-census-20261115
image=rust@sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c
log=/var/log/n83-arm64-build-smoke.log

finish() {
    result=$?
    trap - EXIT
    set +e
    {
        printf 'N83_BUILD_RESULT_BEGIN exit=%s source=%s\n' "$result" "$source_commit"
        if test -f "$log"; then
            printf 'N83_LOG_BYTES=%s N83_LOG_SHA256=%s\n' \
                "$(wc -c < "$log")" "$(sha256sum "$log" | cut -d' ' -f1)"
            gzip -n -c "$log" | base64 -w0 > /var/tmp/n83-build-log.b64
            if test "$(wc -c < /var/tmp/n83-build-log.b64)" -le 50000; then
                printf 'N83_LOG_GZIP_BASE64_BEGIN\n'
                cat /var/tmp/n83-build-log.b64
                printf '\nN83_LOG_GZIP_BASE64_END\n'
            else
                printf 'N83_LOG_TOO_LARGE_FOR_CONSOLE\n'
                tail -c 16000 "$log"
            fi
        fi
        printf 'N83_BUILD_RESULT_END\n'
    } > /dev/console 2>&1
    /usr/bin/systemctl poweroff
}
trap finish EXIT
exec > "$log" 2>&1

date -u '+start_utc=%Y-%m-%dT%H:%M:%SZ'
systemd-run --unit=n83-build-safety --on-active=65m /usr/bin/systemctl poweroff
dnf -qy install git docker file
systemctl enable --now docker
docker info --format 'docker_os={{.OSType}} docker_arch={{.Architecture}} docker_mem={{.MemTotal}}'

git clone -q --filter=blob:none --no-checkout --depth 50 \
    --branch "$source_branch" --single-branch \
    https://github.com/isogeny-labs/crypto.git /opt/n83-crypto
git -C /opt/n83-crypto sparse-checkout init --cone
git -C /opt/n83-crypto sparse-checkout set \
    src examples crates tools isogeny_algos research/pencil_large_prime_20261029
git -C /opt/n83-crypto checkout -q "$source_commit"
test "$(git -C /opt/n83-crypto rev-parse HEAD)" = "$source_commit"
test -z "$(git -C /opt/n83-crypto status --porcelain)"
printf 'checked_source=%s\n' "$source_commit"

docker pull "$image"
docker image inspect "$image" --format 'image_id={{.Id}} image_os={{.Os}} image_arch={{.Architecture}}'
mkdir /opt/n83-build-target
docker run --rm --pull never --platform linux/arm64 \
    --memory 12g --memory-swap 12g --cpus 4 --pids-limit 512 \
    --env CARGO_TARGET_DIR=/build-target \
    --mount type=bind,src=/opt/n83-crypto,dst=/src,readonly \
    --mount type=bind,src=/opt/n83-build-target,dst=/build-target \
    --workdir /src "$image" sh -c \
    'timeout 2400 cargo build --quiet --release --locked --no-default-features -j 2 \
      --example n83_k600_m6_linux_build \
      --example n83_k600_m6_native_arms \
      --example n83_k600_m6_native_preflight \
      --example n83_k600_m6_budget \
      --example koblitz_n83_mixed_orbit_worker'

for name in n83_k600_m6_linux_build n83_k600_m6_native_arms \
    n83_k600_m6_native_preflight n83_k600_m6_budget \
    koblitz_n83_mixed_orbit_worker; do
    binary="/opt/n83-build-target/release/examples/$name"
    test -x "$binary"
    file "$binary"
    file -b "$binary" | grep -q 'ELF 64-bit LSB.*ARM aarch64'
    wc -c "$binary"
    sha256sum "$binary"
done
date -u '+end_utc=%Y-%m-%dT%H:%M:%SZ'
