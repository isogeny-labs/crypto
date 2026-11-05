#!/usr/bin/env bash
# Writes an isolab.eks.command-status/v1 request for one Job name to stdout.
set -euo pipefail
jq -n --arg job "${1:?job name}" --argjson lines "${2:-200}" \
  '{schema:"isolab.eks.command-status/v1",job_name:$job,log_tail_lines:$lines}'
