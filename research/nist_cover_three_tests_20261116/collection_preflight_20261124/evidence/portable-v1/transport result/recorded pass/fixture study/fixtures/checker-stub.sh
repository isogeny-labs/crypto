#!/bin/bash
set -euo pipefail
test "$#" -eq 7
test "$1" = --check
test -f "$6"
printf '%s\n' '{"scope":"checker-sentinel-only","cryptographic_verification_executed":false}' > "$7"
printf '%s\n' 'sentinel entered; no scientific code' > "$7.entered"
