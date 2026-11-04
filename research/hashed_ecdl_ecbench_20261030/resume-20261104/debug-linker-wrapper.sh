#!/usr/bin/env bash
# Local development links only. Preserve executable code; omit debug sections.
exec /usr/bin/cc "$@" -Wl,--strip-debug
