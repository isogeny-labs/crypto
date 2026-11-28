#!/usr/bin/env bash
# Thin orchestration of the frozen run (PROTOCOL.md part B). Each rung and arm
# group is one process under an address-space cap and a wall budget; a killed
# process is recorded in data/run_manifest.txt as a resource limit and the
# run goes on with the next rung.
set -u
cd "$(dirname "$0")"
B=harness/target/release/b2lp_harness
AS=$((10 * 1024 * 1024 * 1024))
M=data/run_manifest.txt
echo "b2lp_harness sha256 $(sha256sum $B | cut -d' ' -f1) $(date -u +%FT%TZ)" >> $M
budget() { if [ "$1" -le 48 ]; then echo 7200; elif [ "$1" -le 51 ]; then echo 10800; elif [ "$1" -le 54 ]; then echo 18000; else echo 28800; fi; }
one() { # n arms-json threads out
  local n=$1 arms=$2 th=$3 out=$4 tmp; tmp=$(mktemp)
  jq --argjson n "$n" --argjson arms "$arms" '.rungs=[.rungs[]|select(.n==$n)|.run_arms=[.run_arms[]|select(. as $a|$arms|index($a))]]' protocol.json > "$tmp"
  echo "n=$n arms=$arms start $(date -u +%FT%TZ) threads=$th" >> $M
  prlimit --as=$AS timeout "$(budget "$n")" $B run "$tmp" "$out" "$th" 2>> data/run.log
  echo "n=$n arms=$arms exit=$? end $(date -u +%FT%TZ)" >> $M
  rm -f "$tmp"
}
case "${1:-}" in
  main) for n in 18 21 24 27 30 33 36 39 42 45 48 51 54 57; do one $n '["D2","D2e","D2all"]' 2 data/runs_d2.jsonl; done ;;
  rho)  for n in 51 54 57; do one $n '["RHO"]' 1 data/runs_ext_rho.jsonl; done ;;
  replay) # n=30 ci=0 ti=0, arm D2, from the frozen manifest
    tmp=$(mktemp); jq '.rungs=[.rungs[]|select(.n==30)|.curves=1|.targets=1|.run_arms=["D2"]]' protocol.json > "$tmp"
    $B run "$tmp" data/replay_d2.jsonl 1 2>> data/run.log; rm -f "$tmp" ;;
  *) echo "usage: run.sh main|rho|replay"; exit 2 ;;
esac
