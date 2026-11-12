#!/usr/bin/env bash
# Exact commands used for LEAD-ICB-tableB-binary-known-log-claw (2026-11-12).
# Local container, L0 (counts only), single-threaded, private lock file.
set -euo pipefail
cd "$(dirname "$0")"
EC=${EC:-../../../../target/release/ecbench}   # built: cargo build --release --bin ecbench (sha256 85318bc7...f8ac)
LOCK=${LOCK:-/tmp/kl-claw-bench.lock}
python3 make_specs.py && sha256sum specs/*.json > specs/SHA256SUMS
for c in f2m15 f2m18 f2m21 f2m24 f2m27; do
  RAYON_NUM_THREADS=1 "$EC" run --spec specs/main-$c.json --out sessions/main-$c --cpus none --allow-busy --lock "$LOCK" --quiet
done
for c in positive negative; do
  RAYON_NUM_THREADS=1 "$EC" run --spec specs/control-$c-f2m15.json --out sessions/control-$c-f2m15 --cpus none --allow-busy --lock "$LOCK" --quiet
done
for s in sessions/*; do "$EC" verify --dir "$s" --replay-all --out audits/$(basename "$s").audit.json; done
for c in f2m15 f2m18 f2m21 f2m24 f2m27; do
  for pair in "rho-neg known-log" "rho-neg known-log-aa" "known-log known-log-aa"; do
    set -- $pair; "$EC" compare --dir sessions/main-$c --a "$1" --b "$2" --json > compare/$c.$1.$2.json
  done
  "$EC" table --dir sessions/main-$c > compare/$c.table.md
done
python3 analyse.py
