#!/usr/bin/env bash
# Thin orchestration of the frozen run (PROTOCOL.md). Each rung is one
# process under an address-space cap and a wall budget; a killed process is
# recorded in data/run_manifest.txt as a resource limit and the run goes on.
set -u
cd "$(dirname "$0")"
D=harness/target/release/bdsl_harness
R=${BCR_BIN:?path to the unchanged #5253 bcr_harness binary}
AS=$((10 * 1024 * 1024 * 1024))
M=data/run_manifest.txt
echo "bdsl_harness sha256 $(sha256sum $D | cut -d' ' -f1)" >> $M
echo "bcr_harness sha256 $(sha256sum $R | cut -d' ' -f1)" >> $M
budget() { if [ "$1" -le 45 ]; then echo 7200; elif [ "$1" -le 51 ]; then echo 10800; else echo 14400; fi; }
run_ds() { # n threads
  local n=$1 th=$2 tmp; tmp=$(mktemp)
  jq --argjson n "$n" '.rungs=[.rungs[]|select(.n==$n)]' protocol.json > "$tmp"
  echo "DS n=$n start $(date -u +%FT%TZ) threads=$th" >> $M
  prlimit --as=$AS timeout "$(budget "$n")" $D run "$tmp" data/runs_ds.jsonl "$th" 2>> data/run_ds.log
  echo "DS n=$n exit=$? end $(date -u +%FT%TZ)" >> $M
  rm -f "$tmp"
}
run_ref() { # n lanes threads
  local n=$1 w=$2 th=$3 tmp; tmp=$(mktemp)
  jq --argjson n "$n" --argjson w "$w" '{curves_per_rung, targets_per_curve, rho_seeds_per_target, target_walk, linear_algebra, ic_arms, rungs: [.rungs[]|select(.n==$n)|{n, np, rho_lanes: $w} + (if .curves then {curves, targets} else {} end)]}' protocol.json > "$tmp"
  echo "REF n=$n W=$w start $(date -u +%FT%TZ)" >> $M
  prlimit --as=$AS timeout "$(budget "$n")" $R run "$tmp" data/runs_rho_new.jsonl "$th" 2>> data/run_ref.log
  echo "REF n=$n exit=$? end $(date -u +%FT%TZ)" >> $M
  rm -f "$tmp"
}
case "${1:-}" in
  panel) for n in 18 21 24 27 30 33; do run_ds $n 2; done ;;
  ext) for n in 36 39 42 45 48 51 54; do run_ds $n 2; done ;;
  ref) w=${2:?rho lanes from the n=51 pilot}; run_ref 51 "$w" ${3:-2}; run_ref 54 "$w" ${3:-2} ;;
  replay) # n=39 ci=0 ti=0 from the frozen manifest
    tmp=$(mktemp); jq '.rungs=[.rungs[]|select(.n==39)|.curves=1|.targets=1]' protocol.json > "$tmp"
    $D run "$tmp" data/replay_ds.jsonl 1 2>> data/run_ds.log; rm -f "$tmp" ;;
  *) echo "usage: run.sh panel|ext|ref <W>|replay"; exit 2 ;;
esac
