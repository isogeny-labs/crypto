#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c RUNS.sha256
repacked=$(mktemp)
trap 'rm -f -- "$repacked"' EXIT
tar --sort=name --mtime='2026-10-29 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > "$repacked"
cmp runs.tar.xz "$repacked"
