#!/usr/bin/env bash
# Exercise cold-clone path dependency discovery without network or Rust builds.
set -euo pipefail
helper=${1:-$(cd "$(dirname "$0")" && pwd)/sparse-checkout.sh}
fixture=$(mktemp -d)
trap 'rm -rf "$fixture"' EXIT
cd "$fixture"
git init -q
git config user.name 'Sparse checkout test'
git config user.email 'sparse-test@example.invalid'
mkdir -p src crates/direct/src crates/transitive/native crates/transitive/fixtures research/note
cat > Cargo.toml <<'MANIFEST'
[package]
name = "fixture"
version = "0.1.0"
[dependencies]
direct = { path = "crates/direct" }
MANIFEST
cat > crates/direct/Cargo.toml <<'MANIFEST'
[package]
name = "direct"
version = "0.1.0"
[dependencies]
transitive = { path = "../transitive" }
MANIFEST
cat > crates/transitive/Cargo.toml <<'MANIFEST'
[package]
name = "transitive"
version = "0.1.0"
[lib]
path = "native/lib.rs"
MANIFEST
cat > src/lib.rs <<'SOURCE'
#[path = "extra.rs"] mod extra;
const NOTE: &str = include_str!("../research/note/fixture.txt");
SOURCE
: > src/extra.rs
: > research/note/fixture.txt
: > crates/direct/src/lib.rs
cat > crates/transitive/native/lib.rs <<'SOURCE'
const FIXTURE: &str = include_str!("../fixtures/data.txt");
SOURCE
: > crates/transitive/fixtures/data.txt
git add .
git commit -qm 'Fixture with transitive path dependencies'
# Start with only the root manifest and source on disk.
printf '/*\n!/*/\n/src/\n' | git sparse-checkout set --no-cone --stdin
test ! -f crates/direct/Cargo.toml
test ! -f crates/transitive/Cargo.toml
test ! -f crates/transitive/fixtures/data.txt
bash "$helper" patterns --profile minimal > "$fixture/patterns"
for rule in /crates/direct/Cargo.toml /crates/direct/src/ /crates/transitive/Cargo.toml /crates/transitive/native/; do
  grep -qxF "$rule" "$fixture/patterns"
done
bash "$helper" apply --profile minimal > "$fixture/status"
test -f crates/direct/src/lib.rs
test -f crates/transitive/native/lib.rs
test -f crates/transitive/fixtures/data.txt
# Read the working manifest when present: uncommitted dependency changes count.
mkdir -p crates/working/src
cat > crates/working/Cargo.toml <<'MANIFEST'
[package]
name = "working"
version = "0.1.0"
MANIFEST
printf 'working = { path = "../working" }\n' >> crates/direct/Cargo.toml
bash "$helper" patterns --profile minimal > "$fixture/working-patterns"
grep -qxF '/crates/working/Cargo.toml' "$fixture/working-patterns"
echo 'PASS: cold sparse dependencies, transitive targets, compile-time fixtures, and working manifests'
