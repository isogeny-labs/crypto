#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
# Preserve original study-relative paths, including diagnostic/failed attempts.
# No executable build products, credentials, or transient locks enter the archive.
tar --sort=name --mtime='2026-10-18 00:00:00Z' --owner=0 --group=0 \
 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - \
 sessions run-logs replay-receipts reports session-reports.pdf session-reports.typ \
 exhaustion-replays exhaustion-replays-attempt1 instruction-AA-session \
 diagnostic-results-v1 interim-results-v2 interim-results-v3 interim-results-v4 \
 coverage-fixtures-v1 coverage-fixtures-v2 results/counts.csv results/cost-cells.csv \
 | xz -6 -T1 > runs.tar.xz
sha256sum runs.tar.xz > runs.tar.xz.sha256
