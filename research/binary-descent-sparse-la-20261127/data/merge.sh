#!/usr/bin/env bash
# Assemble the analysis input (regenerated, not committed): D-S/D-S-B records,
# #5261's D records, and the rho references (#5253 panel, #5261 n=36..48,
# new n=51/54 from the unchanged #5253 binary).
set -eu
cd "$(dirname "$0")"
W=../../binary-weil-descent-20261126/data
P=../../binary-composite-rootfind-20261126/data
{ cat runs_ds.jsonl
  cat $W/runs_d.jsonl
  jq -c 'select(.arm=="RHO")' $P/runs.jsonl
  jq -c 'select(.arm=="RHO")' $W/runs_ext_ref.jsonl
  [ -f runs_rho_new.jsonl ] && jq -c 'select(.arm=="RHO")' runs_rho_new.jsonl; true; } > runs_primary.jsonl
wc -l runs_primary.jsonl
