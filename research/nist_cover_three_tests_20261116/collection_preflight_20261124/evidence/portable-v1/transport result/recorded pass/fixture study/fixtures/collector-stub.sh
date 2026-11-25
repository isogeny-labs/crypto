#!/bin/bash
set -euo pipefail
test "$#" -eq 5
test "$1" = --collect
printf '%s\n' '{"scope":"collector-sentinel-only","production_collection_executed":false}' > "$5"
printf '%s\n' 'sentinel entered; no scientific code' > "$5.entered"
