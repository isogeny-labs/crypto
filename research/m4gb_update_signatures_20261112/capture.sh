#!/usr/bin/env bash
# Capture the exact M4GB inputs the decomposition oracle sends, for every
# target of the descent ladder: `ic descent --solver m4gb-f2` through wrapper
# binaries that copy each `-i` file before running the real solver.  The
# targets are seeded, so the corpus is reproducible from the same ic build.
#
#   M4GB_BINARY=<dir of solver_m4gb_n<N>_gf2> research/m4gb_update_signatures_20261112/capture.sh <corpus-dir>
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="${1:?corpus dir}"; mkdir -p "$out"
: "${M4GB_BINARY:?set M4GB_BINARY to a directory of solver_m4gb_n<N>_gf2 binaries}"
TARGETS="${TARGETS:-6}"
CELLS="${CELLS:-K:11:6:2 R:11:6:2 K:13:7:2 R:13:7:2 K:15:8:2 R:15:8:2 K:17:9:2 R:17:9:2 K:19:10:2 R:19:10:2 R:21:11:2 K:23:12:2 R:23:12:2 K:11:4:3 R:11:4:3}"
wrap="$(mktemp -d)"; trap 'rm -rf "$wrap"' EXIT
for b in "$M4GB_BINARY"/solver_m4gb_n*_gf2; do
  name=$(basename "$b")
  cat >"$wrap/$name" <<W
#!/bin/sh
prev=""; for a in "\$@"; do if [ "\$prev" = "-i" ]; then cp "\$a" "\$CAPTURE_TO/\$(printf %04d \$(ls "\$CAPTURE_TO" | wc -l)).in"; fi; prev="\$a"; done
exec "$b" "\$@"
W
  chmod +x "$wrap/$name"
done
for c in $CELLS; do
  fam=${c%%:*}; cell=${c#*:}; tag="$fam-$(echo "$cell" | tr ':' '-')"
  mkdir -p "$out/$tag"
  CAPTURE_TO="$out/$tag" M4GB_BINARY="$wrap" M4GB_THREADS=1 "$repo/target/release/ic" descent --json \
    --out "$out/$tag/descent.json" --families "$fam" --cells "$cell" --targets "$TARGETS" --repeats 1 \
    --budget-seconds 120 --solver m4gb-f2 --solver fes-f2 >"$out/$tag/descent.log" 2>&1
  echo "$tag: $(ls "$out/$tag" | grep -c '\.in$') systems"
done
