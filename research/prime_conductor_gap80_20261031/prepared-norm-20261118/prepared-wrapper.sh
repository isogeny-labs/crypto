#!/bin/sh
set -eu
exec "${PREPARED_BINARY:?supply pinned native binary}" "$@" --prepared-norm
