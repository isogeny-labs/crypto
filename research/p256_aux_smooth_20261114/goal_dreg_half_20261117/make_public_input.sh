#!/usr/bin/env bash
set -euo pipefail
[[ $# == 2 && -f "$1" && ! -e "$2" ]] || { echo 'Usage: make_public_input.sh FROZEN_INPUT FRESH_OUTPUT' >&2; exit 2; }
[[ $(grep -c '^Alarm(20);$' "$1") == 1 ]]
sed '/^Alarm(20);$/d' "$1" > "$2"
# Reinsert the single exact line and compare every original byte.
reconstruction=$(mktemp)
trap 'rm -f "$reconstruction"' EXIT
sed '1a Alarm(20);' "$2" > "$reconstruction"
cmp "$1" "$reconstruction"
