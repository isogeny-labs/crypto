#!/usr/bin/env bash
# Thin orchestration of the frozen run (PROTOCOL.md part B). Each rung and arm
# group is one process under an address-space cap and a wall budget; a killed
# process is recorded in data/run_manifest.txt as a resource limit and the
# run goes on with the next rung.
set -u
cd "$(dirname "$0")"
B=harness/target/release/b2lsp_harness
AS=$((12 * 1024 * 1024 * 1024))
M=data/run_manifest.txt
echo "b2lsp_harness sha256 $(sha256sum $B | cut -d' ' -f1) $(date -u +%FT%TZ) $*" >> $M
budget() { if [ "$1" -le 48 ]; then echo 7200; elif [ "$1" -le 51 ]; then echo 10800; elif [ "$1" -le 54 ]; then echo 14400; elif [ "$1" -le 57 ]; then echo 21600; else echo 28800; fi; }
one() { # n arms-json threads out [rungs-key]
  local n=$1 arms=$2 th=$3 out=$4 key=${5:-rungs} tmp; tmp=$(mktemp)
  jq --argjson n "$n" --argjson arms "$arms" --arg key "$key" '.rungs=[.[$key][]|select(.n==$n)|.run_arms=[.run_arms[]|select(. as $a|$arms|index($a))]|select(.run_arms|length>0)]' protocol.json > "$tmp"
  if [ "$(jq '.rungs|length' "$tmp")" = 0 ]; then rm -f "$tmp"; return; fi
  echo "n=$n arms=$arms start $(date -u +%FT%TZ) threads=$th" >> $M
  prlimit --as=$AS timeout "$(budget "$n")" $B run "$tmp" "$out" "$th" 2>> data/run.log
  echo "n=$n arms=$arms exit=$? end $(date -u +%FT%TZ)" >> $M
  rm -f "$tmp"
}
case "${1:-}" in
  main) for n in 18 21 24 27 30 33 36 39 42 45 48 51 54 57 60; do one $n '["D2S","D2S23","D2Sall"]' 3 data/runs_d2s.jsonl; done ;;
  rho)  for n in 51 57 60; do one $n '["RHO"]' 3 data/runs_rho.jsonl rho_rungs; done ;;
  replay) # n=30 ci=0 ti=0, arm D2S, from the frozen manifest
    tmp=$(mktemp); jq '.rungs=[.rungs[]|select(.n==30)|.curves=1|.targets=1|.run_arms=["D2S"]]' protocol.json > "$tmp"
    $B run "$tmp" data/replay_d2s.jsonl 1 2>> data/run.log; rm -f "$tmp" ;;
  *) echo "usage: run.sh main|rho|replay"; exit 2 ;;
esac
