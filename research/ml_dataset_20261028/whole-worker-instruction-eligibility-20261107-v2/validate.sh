#!/usr/bin/env bash
# Software-only synthetic policy tests. No perf, cloud, solver or dataset access.
set -euo pipefail
cd -- "$(dirname -- "$0")"
ulimit -t 30
ulimit -v 1048576
rustc --version > evidence/rustc-version.txt
sha256sum eligibility.rs > evidence/source-before-tests.sha256
timeout 45s rustc --edition 2021 --test -D warnings -C opt-level=0 \
  eligibility.rs -o evidence/eligibility-tests \
  > evidence/compile.stdout 2> evidence/compile.stderr
timeout 15s evidence/eligibility-tests --test-threads=1 > evidence/tests.log 2>&1
sha256sum --quiet -c evidence/source-before-tests.sha256
