#!/bin/bash
set -eu
cd "$(dirname "$0")"
ulimit -v 1572864
ulimit -t 60
rustc +1.98 --edition=2021 --test harness.rs -L dependency=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps --extern serde=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps/libserde-cac565bd9dd1312a.rlib --extern serde_json=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps/libserde_json-10f254ba1c7de3e0.rlib --extern sha2=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps/libsha2-7e1fb15708cb6235.rlib --extern hex=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps/libhex-1a24ad8e658c37ff.rlib -o capsule-tests
./capsule-tests --test-threads=1
