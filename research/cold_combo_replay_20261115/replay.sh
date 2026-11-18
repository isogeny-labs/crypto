#!/usr/bin/env bash
# Thin external-tool adapter; no field or curve arithmetic in this script.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
input=${1:?usage: replay.sh NUMERIC_REPLAY_FILE}
gp=${GP_BIN:-gp}
test -f "$input"
test "$(wc -c < "$input")" -le 16384
test "$(wc -c < "$input")" -gt 0
# Never evaluate producer-supplied GP code, strings, escapes or identifiers.
test -z "$(LC_ALL=C tr -d '0-9,[] \n' < "$input")"
export PARI_REPLAY_INPUT
PARI_REPLAY_INPUT=$(realpath "$input")
output=$("$gp" -fq "$here/replay.gp" 2>&1) || { printf '%s\n' "$output" >&2; exit 1; }
test "$output" = PARI_REPLAY_OK || { printf '%s\n' "$output" >&2; exit 1; }
printf '%s\n' "$output"
