set -euo pipefail
jq -r '.sessions[] | "/" + .dir + "/", (select(.replay_receipt != null) | "/" + .replay_receipt)' docs/backend-leaderboard/manifest.json > /tmp/backend-sources.txt
git sparse-checkout add --stdin < /tmp/backend-sources.txt

cargo +1.99.1 fmt --manifest-path tools/backend-leaderboard/Cargo.toml --check
cargo +1.99.1 test --locked --manifest-path tools/backend-leaderboard/Cargo.toml
cargo +1.99.1 clippy --locked --manifest-path tools/backend-leaderboard/Cargo.toml --all-targets -- -D warnings

cargo +1.99.1 run --locked --manifest-path tools/backend-leaderboard/Cargo.toml -- --check
lock_sha=$(sha256sum docs/backend-leaderboard/reproduction.lock.json | cut -d ' ' -f 1)
cargo +1.99.1 run --locked --manifest-path tools/backend-leaderboard/Cargo.toml -- reproduce --expected-lock-sha256 "$lock_sha" > /tmp/backend-reproduction.json
cat /tmp/backend-reproduction.json
