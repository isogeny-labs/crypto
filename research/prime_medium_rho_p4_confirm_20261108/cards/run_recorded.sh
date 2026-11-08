#!/usr/bin/env bash
# Run COMMAND... under the study's open benchmark record (tools/benchrec.py run).
# The record path is fixed here so that cards need not pin a file that every run rewrites.
#   run_recorded.sh COMMAND [ARG...]
set -euo pipefail
[[ $# -ge 1 ]] || { echo 'usage: run_recorded.sh COMMAND [ARG...]' >&2; exit 2; }
exec python3 tools/benchrec.py run benchmarks/records/20261108T0512Z-prime-medium-rho-p4-confirm.json -- "$@"
