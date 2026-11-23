#!/usr/bin/env bash
set -euo pipefail
R=/tmp/ecdlp-port-mass-20261123
export CARGO_TARGET_DIR="$R/native-target"
cargo fmt --manifest-path "$R/crate-source-v2/Cargo.toml" -- --check
cargo test --release --offline --locked --lib --no-run --message-format=json --manifest-path "$R/crate-source-v2/Cargo.toml" > "$R/cargo-test-artifacts-v2.jsonl"
cargo clippy --release --offline --locked --all-targets --manifest-path "$R/crate-source-v2/Cargo.toml" -- -D warnings
node <<'NODE'
const fs=require('fs'),r='/tmp/ecdlp-port-mass-20261123/';const candidates=fs.readFileSync(r+'cargo-test-artifacts-v2.jsonl','utf8').trim().split('\n').map(x=>JSON.parse(x)).filter(x=>x.reason==='compiler-artifact'&&x.profile.test&&x.target.kind.includes('lib')&&x.executable).map(x=>x.executable);if(candidates.length!==1)throw Error('ambiguous native test binary');fs.writeFileSync(r+'test-binary-path-v2.txt',candidates[0]+'\n');console.log('One library test binary selected from Cargo metadata.');
NODE
sha256sum -c "$R/crate-source-v2-before.sha256"
