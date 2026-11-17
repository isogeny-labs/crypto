#!/bin/bash
set -u
cd /home/ec2-user/crypto-pencil-signed-graph-20261029
out=research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/target-stream-implementation/root-regression-16g
sha256sum Cargo.toml Cargo.lock research/pencil_large_prime_20261029/Cargo.toml research/pencil_large_prime_20261029/src/{lib,relation_stream,native_solver}.rs > "$out/before.sha256"
date -u +%FT%TZ > "$out/start"
ulimit -v 16777216
CARGO_BUILD_JOBS=1 timeout 600 cargo test --release --lib --locked --offline -- --test-threads=1 > "$out/stdout" 2> "$out/stderr"
code=$?
printf '%s\n' "$code" > "$out/exit"
sha256sum -c "$out/before.sha256" > "$out/bindings.stdout"
printf '%s\n' "$?" > "$out/bindings.exit"
date -u +%FT%TZ > "$out/end"
tail -25 "$out/stderr"
tail -10 "$out/stdout"
exit "$code"
