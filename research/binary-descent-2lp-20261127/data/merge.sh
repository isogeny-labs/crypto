#!/usr/bin/env bash
# Assemble the analysis input (regenerated, not committed): D2/D2e/D2all
# records, #5261's D records, rho references from #5253 (panel), #5261
# (n = 36..48) and this study (n = 51..57).
set -eu
cd "$(dirname "$0")"
P=../../binary-composite-rootfind-20261126/data
W=../../binary-weil-descent-20261126/data
{ cat runs_d2.jsonl
  jq -c 'select(.arm=="D")' $W/runs_d.jsonl
  jq -c 'select(.arm=="RHO")' $P/runs.jsonl
  jq -c 'select(.arm=="RHO")' $W/runs_ext_ref.jsonl
  cat runs_ext_rho.jsonl; } > runs_primary.jsonl
wc -l runs_primary.jsonl
