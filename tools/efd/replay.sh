#!/usr/bin/env bash
# Regenerate the pinned EFD catalogues from local fetch caches and compare,
# ignoring the retrieval date. Each section has its own cache directory because
# both sections serve an index.html at the cache root.
# Usage: tools/efd/replay.sh G1P_CACHE G12O_CACHE
set -euo pipefail
[ $# -eq 2 ] || { echo "usage: replay.sh G1P_CACHE G12O_CACHE" >&2; exit 2; }
root=$(cd "$(dirname "$0")/../.." && pwd)
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
run() {
  local section=$1 cache=$2 target=$3
  local out="$tmp/$section.json"
  cargo run --quiet --release --manifest-path "$root/tools/efd/Cargo.toml" -- \
    "$cache" "$out" "$section"
  python3 - "$out" "$target" <<'PY'
import json, sys
a = json.load(open(sys.argv[1])); b = json.load(open(sys.argv[2]))
a.pop("retrieved", None); b.pop("retrieved", None)
if a != b:
    print(f"catalogue drift: {sys.argv[1]} != {sys.argv[2]}", file=sys.stderr); sys.exit(1)
PY
  echo "$section matches"
}
run g1p "$1" "$root/src/prime_ecc/efd/catalogue.json"
run g12o "$2" "$root/src/binary_ecc/efd/catalogue.json"
