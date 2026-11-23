#!/bin/bash
set -u
cd /home/ec2-user/crypto-pencil-signed-graph-20261029
root=research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-exception-controls
out=$(mktemp -d "$root/check-XXXXXXXX")
python3 "$root/bind.py" > "$out/before.sha256"
date -u +%FT%TZ > "$out/start"
ulimit -v 3145728
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/var/tmp/signed-stream-20261102/target timeout 180 cargo test --locked --offline --manifest-path "$root/Cargo.toml" -- --test-threads=1 --nocapture > "$out/stdout" 2> "$out/stderr"
code=$?
printf '%s\n' "$code" > "$out/exit"
sha256sum -c "$out/before.sha256" > "$out/bindings.log"
binding_code=$?
printf '%s\n' "$binding_code" > "$out/bindings.exit"
date -u +%FT%TZ > "$out/end"
tail -14 "$out/stderr"
tail -4 "$out/stdout"
if [ "$code" -ne 0 ]; then exit "$code"; fi
exit "$binding_code"
