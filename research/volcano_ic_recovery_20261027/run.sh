#!/usr/bin/env bash
set -euo pipefail
here=$(cd -- "$(dirname -- "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
[[ $# == 2 && -d "$1" && ! -e "$2" ]] || { echo 'usage: run.sh VERIFIED_DEVELOPMENT_RECEIPT_DIRECTORY NEW_PRIVATE_OUTPUT_DIRECTORY' >&2; exit 1; }
receipt=$(cd -- "$1" && pwd)
[[ -f "$receipt/READY_FOR_DIAGNOSTIC" ]] || { echo 'development controls did not complete' >&2; exit 1; }
cd "$root"
sha256sum -c "$receipt/source_manifest.sha256"
(cd "$receipt" && sha256sum -c binaries.sha256)
# Repository-wide durable checkpoint policy. Do not bypass this gate or
# manufacture .last-push. A failure means no experiment starts.
bash scripts/checkpoint.sh gate
sha256sum -c "$here/SOURCES.sha256"
exec "$receipt/build/runner" run "$here/public_source_case.json" "$here/pilot_config.json" "$2" --private-output --explicit
