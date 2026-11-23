#!/usr/bin/env bash
# Register one job's session records and local audit as raw files of the study's benchmark record.
#   record_raw.sh JOB     (JOB is PC or M1 .. M9; run from the repository root)
set -euo pipefail
[[ $# == 1 && "$1" =~ ^(PC|M[1-9])$ ]] || { echo 'usage: record_raw.sh PC|M1|M2|M3|M4|M5|M6|M7|M8|M9' >&2; exit 2; }
j=$1
D=research/prime_medium_rho_p4_confirm_20261108
exec python3 tools/benchrec.py add-raw benchmarks/records/20261108T0512Z-prime-medium-rho-p4-confirm.json \
  "$D/sessions/$j/records.jsonl" "$D/sessions/$j.audit.json"
