#!/usr/bin/env bash
set -euo pipefail
cd "${1:?repository root required}"
sha256sum --check "${2:?source manifest required}"
printf '%s  %s\n' "${4:?binary hash required}" "${3:?retained binary required}" | sha256sum --check -
