#!/usr/bin/env bash
set -euo pipefail
cd "${1:?source root}"
sha256sum --check "${2:?source inventory}"
printf '%s  %s\n' "${4:?expected binary hash}" "${3:?immutable binary}" | sha256sum --check -
