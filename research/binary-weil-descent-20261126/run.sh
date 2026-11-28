#!/usr/bin/env bash
# Thin orchestration of the frozen run (PROTOCOL.md). Each rung is one
# process under an address-space cap and a wall budget; a killed process is
# recorded in data/run_manifest.txt as a resource limit and the run goes on.
set -u
cd "$(dirname "$0")"
D=harness/target/release/bwd_harness
R=${BCR_BIN:?path to the unchanged #5253 bcr_harness binary}
AS=$((10 * 1024 * 1024 * 1024))
M=data/run_manifest.txt
echo "bwd_harness sha256 $(sha256sum $D | cut -d' ' -f1)" >> $M
echo "bcr_harness sha256 $(sha256sum $R | cut -d' ' -f1)" >> $M
budget() { if [ "$1" -le 42 ]; then echo 7200; elif [ "$1" -le 45 ]; then echo 10800; else echo 14400; fi; }
run_d() { # n threads
  local n=$1 th=$2 tmp; tmp=$(mktemp)
  jq --argjson n "$n" '.rungs=[.rungs[]|select(.n==$n)]' protocol.json > "$tmp"
  echo "D n=$n start $(date -u +%FT%TZ) threads=$th" >> $M
  prlimit --as=$AS timeout "$(budget "$n")" $D run "$tmp" data/runs_d.jsonl "$th" 2>> data/run_d.log
  echo "D n=$n exit=$? end $(date -u +%FT%TZ)" >> $M
  rm -f "$tmp"
}
run_ref() { # n arms-json threads
  local n=$1 arms=$2 th=$3 tmp; tmp=$(mktemp)
  jq --argjson n "$n" --argjson arms "$arms" '{curves_per_rung, targets_per_curve, rho_seeds_per_target, target_walk, linear_algebra, ic_arms: $arms, rungs: [.rungs[]|select(.n==$n)|{n, np, rho_lanes} + (if .curves then {curves, targets} else {} end)]}' protocol.json > "$tmp"
  echo "REF n=$n arms=$arms start $(date -u +%FT%TZ)" >> $M
  prlimit --as=$AS timeout "$(budget "$n")" $R run "$tmp" data/runs_ext_ref.jsonl "$th" 2>> data/run_ref.log
  echo "REF n=$n exit=$? end $(date -u +%FT%TZ)" >> $M
  rm -f "$tmp"
}
case "${1:-}" in
  panel) for n in 18 21 24 27 30 33; do run_d $n 2; done ;;
  ext-d) for n in 36 39 42; do run_d $n 2; done; for n in 45 48; do run_d $n 1; done ;;
  ext-ref) for n in 36 39; do run_ref $n '["M2"]' 2; done; for n in 42 45 48; do run_ref $n '[]' 2; done ;;
  replay) # n=30 ci=0 ti=0 from the frozen manifest
    tmp=$(mktemp); jq '.rungs=[.rungs[]|select(.n==30)|.curves=1|.targets=1]' protocol.json > "$tmp"
    $D run "$tmp" data/replay_d.jsonl 1 2>> data/run_d.log; rm -f "$tmp" ;;
  *) echo "usage: run.sh panel|ext-d|ext-ref|replay"; exit 2 ;;
esac
