#!/usr/bin/env bash
# Build analysis/summary.json from the committed comparisons and the derivation.
set -euo pipefail
cd "$(dirname "$0")/.."
a=$(for t in r24 r26 r28; do d=sessions/A-$t/comparisons; jq -n --arg tag $t \
  --slurpfile c0 $d/m100__m141__w0.json --slurpfile c1 $d/m100__m141__w1.json \
  --slurpfile rho0 $d/rho-neg__m100.json --slurpfile r1 $d/rho-neg__m100__w1.json --slurpfile r2 $d/rho-neg__m141__w1.json \
  '{tag:$tag, c0:$c0[0], c1:$c1[0], rho0:$rho0[0], rho1m100:$r1[0], rho1m141:$r2[0]}'; done | jq -s .)
b=$(for t in h21 h22 h24 h29; do d=sessions/B-$t/comparisons; jq -n --arg tag $t \
  --slurpfile c0 $d/v1__v2-m141__w0.json --slurpfile c1 $d/v1__v2-m141__w1.json \
  --slurpfile rho0 $d/rho-neg__v1.json --slurpfile r1 $d/rho-neg__v1__w1.json --slurpfile r2 $d/rho-neg__v2-m141__w1.json \
  '{tag:$tag, c0:$c0[0], c1:$c1[0], rho0:$rho0[0], rho1v1:$r1[0], rho1v2:$r2[0]}'; done | jq -s .)
jq -n --argjson a "$a" --argjson b "$b" -f scripts/summary.jq \
  --argjson deriv "$(cat analysis/derivation.json)" > analysis/summary.json
