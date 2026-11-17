#!/usr/bin/env bash
set -euo pipefail
if [ "$#" -ne 1 ]; then echo 'usage: prepare-qualifier.sh FRESH_RECEIPT_DIRECTORY' >&2; exit 2; fi
study="$(cd "$(dirname "$0")" && pwd)"
mkdir "$1"
receipt="$(realpath "$1")"
cp "$study/qualify-template.sh" "$receipt/qualifier.sh"
chmod a-w "$receipt/qualifier.sh"
(cd "$receipt" && sha256sum qualifier.sh > qualifier.sha256)
printf 'Frozen qualifier prepared at %s; no native call was started.\n' "$receipt"
