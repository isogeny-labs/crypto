#!/usr/bin/env bash
# Native execution path of the packed-filter validator (TASK-20261029-654f73).
# Builds the standalone crate and runs every computing and verifying path into
# native/out/, assembles validator-report.json and replay-certificate.json
# there, and writes native/out/equivalence.jsonl against the committed Python
# outputs (validation/code/out/*.json and the two published files). It never
# writes the published validation/validator-report.json or
# validation/replay-certificate.json.
#
# usage: native/run.sh [--s3]
#   --s3  also re-run the read-only S3 checks (s3check, s3requests, c2); needs
#         the aws CLI with read access to the isolab results bucket. Without it
#         the S3 outputs already in native/out/ are reused.
# env:   CARGO_TARGET_DIR (default /dev/shm/pfv-target), JOBS (default 2)
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
vdir="$(dirname "$here")"
out="$here/out"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/dev/shm/pfv-target}"
s3=0
for a in "$@"; do
  case "$a" in
    --s3) s3=1 ;;
    *) echo "unknown argument $a" >&2; exit 64 ;;
  esac
done
cargo build --release -j "${JOBS:-2}" --manifest-path "$here/Cargo.toml"
pv="$CARGO_TARGET_DIR/release/pf-validator"
mkdir -p "$out"
"$pv" j1 32 "$out/j1-P32.json"
"$pv" j1 24 "$out/j1-P24.json"
"$pv" xcheck "$out/j1-inpod-crosscheck.json"
"$pv" j2 "$out/j2.json"
"$pv" j3 "$out/j3.json"
"$pv" j4 "$out/j4.json"
"$pv" scalars "$out/scalars.json"
"$pv" selftest "$out/selftest.json"
if [ "$s3" = 1 ]; then
  "$pv" s3check "$out/s3check.json"
  "$pv" s3requests "$out/s3check.json" "$out/s3requests.json"
  "$pv" c2 "$out/c2_identity.json"
else
  for f in s3check s3requests c2_identity; do
    [ -s "$out/$f.json" ] || { echo "STOP: $out/$f.json missing; run with --s3" >&2; exit 3; }
  done
fi
# replay receipts are ecbench's own output (replay.sh); the report reads them
"$pv" report "$out" "$vdir/code/out/replay" "$out"
"$pv" equivalence "$out" "$out/equivalence.jsonl"
