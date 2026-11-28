#!/usr/bin/env bash
# Assemble the analysis input (regenerated, not committed; PROTOCOL.md part B "References"):
# D2S/D2S23/D2Sall records (this study); D-S records (#5290, arm DS only);
# D2 records (#5291, arm D2 only); rho: #5253 (n <= 33), #5261 (n = 36..48),
# n = 51 W = 128 (#5291's 4 cases + this study's 8), n = 54 W = 64 (#5290),
# n = 57, 60 (this study); #5290's n = 51 W = 64 rho relabelled RHO_W64 (sensitivity).
set -eu
cd "$(dirname "$0")"
P=../../binary-composite-rootfind-20261126/data
W=../../binary-weil-descent-20261126/data
S=../../binary-descent-sparse-la-20261127/data
L=../../binary-descent-2lp-20261127/data
{ cat runs_d2s.jsonl
  jq -c 'select(.arm=="DS")' $S/runs_ds.jsonl
  jq -c 'select(.arm=="D2")' $L/runs_d2.jsonl
  jq -c 'select(.arm=="RHO")' $P/runs.jsonl
  jq -c 'select(.arm=="RHO")' $W/runs_ext_ref.jsonl
  jq -c 'select(.arm=="RHO" and .case.n==51)' $L/runs_ext_rho.jsonl
  jq -c 'select(.arm=="RHO" and .case.n==54)' $S/runs_rho_new.jsonl
  jq -c 'select(.arm=="RHO" and .case.n==51) | .arm="RHO_W64"' $S/runs_rho_new.jsonl
  [ -f runs_rho.jsonl ] && cat runs_rho.jsonl; true; } > runs_primary.jsonl
wc -l runs_primary.jsonl
