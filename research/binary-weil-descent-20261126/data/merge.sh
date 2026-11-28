#!/usr/bin/env bash
# Assemble the analysis input (regenerated, not committed): D records, the
# #5253 panel references (F and M2 from A2, RHO from the frozen run), and the
# extension references (RHO, M2) from the unchanged #5253 binary.
set -eu
cd "$(dirname "$0")"
P=../../binary-composite-rootfind-20261126/data
{ cat runs_d.jsonl
  jq -c 'select(.arm=="F" or .arm=="M2")' $P/runs_a2.jsonl
  jq -c 'select(.arm=="RHO")' $P/runs.jsonl
  cat runs_ext_ref.jsonl; } > runs_primary.jsonl
wc -l runs_primary.jsonl
