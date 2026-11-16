#!/bin/bash
set -u
cd /home/ec2-user/crypto-pencil-signed-graph-20261029
root=research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-stream
out=$(mktemp -d "$root/native-check-XXXXXXXX")
python3 "$root/bind.py" > "$out/before.sha256"
date -u +%FT%TZ > "$out/start"
ulimit -v 3145728
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/var/tmp/signed-stream-20261102/target timeout 180 cargo test --locked --offline --manifest-path "$root/native-manifest/Cargo.toml" --lib three_leads_followup::tests -- --test-threads=1 > "$out/stdout" 2> "$out/stderr"
code=$?
printf '%s\n' "$code" > "$out/exit"
sha256sum -c "$out/before.sha256" > "$out/bindings.log"
printf '%s\n' "$?" > "$out/bindings.exit"
date -u +%FT%TZ > "$out/end"
cat "$out/exit"
tail -14 "$out/stderr"
tail -14 "$out/stdout"
exit "$code"
