#!/bin/bash
set -u
cd /home/ec2-user/crypto-pencil-signed-graph-20261029
out=research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/target-stream-implementation/producer-regression
sha256sum research/pencil_large_prime_20261029/Cargo.{toml,lock} research/pencil_large_prime_20261029/src/{lib,relation_stream,native_solver}.rs research/pencil_large_prime_20261029/src/bin/pencil_symbolic.rs > "$out/before.sha256"
date -u +%FT%TZ > "$out/start"
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/var/tmp/pencil-target-stream-producer-20261102/target timeout 180 bash -c 'ulimit -v 3145728; cargo test --offline --locked --manifest-path research/pencil_large_prime_20261029/Cargo.toml --features producer --lib -- --test-threads=1' > "$out/stdout" 2> "$out/stderr"
code=$?
printf '%s\n' "$code" > "$out/exit"
sha256sum -c "$out/before.sha256" > "$out/bindings.stdout"
printf '%s\n' "$?" > "$out/bindings.exit"
date -u +%FT%TZ > "$out/end"
tail -15 "$out/stdout"
exit "$code"
