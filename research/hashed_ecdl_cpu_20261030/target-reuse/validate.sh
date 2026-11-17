#!/usr/bin/env bash
# Thin entry point for immutable, historical candidate validation.
set -euo pipefail
[[ $# == 2 ]] || { echo 'usage: validate.sh FRESH-SNAPSHOT TARGET-DIRECTORY' >&2; exit 64; }
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
exec bash "$here/../validate-snapshot.sh" "$here" "target-reuse.patch" "$@"
