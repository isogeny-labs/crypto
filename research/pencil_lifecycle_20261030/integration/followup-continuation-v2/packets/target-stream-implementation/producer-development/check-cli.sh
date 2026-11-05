#!/bin/bash
set -u
cd /home/ec2-user/crypto-pencil-signed-graph-20261029
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=/var/tmp/pencil-target-stream-producer-20261102/target
ulimit -v 3145728
timeout 180 cargo check --offline --locked --manifest-path research/pencil_large_prime_20261029/Cargo.toml --features producer --bin pencil-symbolic
