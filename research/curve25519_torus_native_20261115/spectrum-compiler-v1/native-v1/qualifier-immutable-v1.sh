#!/usr/bin/env bash
set -euo pipefail
cd "${1:?source root}"
sha256sum --check "${2:?inventory}"
printf '%s  %s\n' "${4:?expected hash}" "${3:?immutable executable}" | sha256sum --check -
