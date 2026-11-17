#!/usr/bin/env bash
set -euo pipefail
R=/var/tmp/pencil-target-stream-offline-20261102
out=$(mktemp -d "$R/check-XXXXXXXX")
cd /home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/target-stream
{ find src tests ../../../../pencil_large_prime_20261029/src ../relation-adapter/src ../../five-followups/rank-information/src -type f -name '*.rs' -print0; printf '%s\0' Cargo.toml Cargo.lock ../../../../pencil_large_prime_20261029/Cargo.toml ../relation-adapter/Cargo.toml ../../five-followups/rank-information/Cargo.toml; } | sort -z | xargs -0 sha256sum > "$out/before.sha256"
date -u +%FT%TZ > "$out/start"
set +e
(ulimit -v 3145728; CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR="$R/target" timeout 180 cargo test --offline -- --test-threads=1) > "$out/stdout" 2> "$out/stderr"
code=$?
set -e
unchanged=true
gate=$code
sha256sum -c "$out/before.sha256" > "$out/bindings.log" || { unchanged=false; gate=126; }
printf '{"native_exit":%s,"gate_exit":%s,"bindings_unchanged":%s,"timeout_seconds":180,"virtual_memory_KiB":3145728,"build_jobs":1}\n' "$code" "$gate" "$unchanged" > "$out/receipt.json"
date -u +%FT%TZ > "$out/end"
tail -45 "$out/stderr"
cat "$out/stdout"
echo "$out"
exit "$gate"
