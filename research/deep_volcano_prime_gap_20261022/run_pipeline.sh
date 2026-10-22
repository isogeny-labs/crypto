#!/usr/bin/env bash
# Thin orchestration for research/deep_volcano_prime_gap_20261022 (PROTOCOL.md).
# Every computation is in the native binaries; this only calls them in order.
#
#   run_pipeline.sh build OUT BITS DEPTH MAXH SEED MODELS TARGETS AA_TARGETS BSGS_TARGETS
#   run_pipeline.sh run   OUT        (the three ecbench sessions, concurrently, one lock each)
#   run_pipeline.sh audit OUT REPLAY (ecbench verify --replay REPLAY on every session)
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
bin=target/release
cmd=$1; out=$2
case "$cmd" in
build)
  bits=$3 depth=$4 maxh=$5 seed=$6 models=$7 targets=$8 aa=$9 bsgs=${10}
  mkdir -p "$out"
  $bin/volcano-gap construct --bits "$bits" --depth "$depth" --d0 11 --max-cofactor "$maxh" \
    --seed "$seed" --out "$out/pair.json" 2> "$out/construct.log"
  p=$(jq -r .class.p "$out/pair.json"); ord=$(jq -r .class.order "$out/pair.json")
  for side in s1 s2; do
    a=$(jq -r .${side}_root.a "$out/pair.json"); b=$(jq -r .${side}_root.b "$out/pair.json")
    g=$(jq -r ".${side}_root.generator | map(tostring) | join(\",\")" "$out/pair.json")
    $bin/volcano-rho --curve "$p,$a,$b" --order "$ord" --generator "$g" --ell 2 --seed "$seed" \
      --per-level 100000 --models "$models" --targets "$targets" --ecbench "$out/export-$side" \
      2> "$out/export-$side.log" > /dev/null
  done
  $bin/volcano-gap certify --pair "$out/pair.json" --seed "$seed" \
    --curves "$out/export-s1/curves.jsonl,$out/export-s2/curves.jsonl" --out "$out/certificates.json" \
    2> "$out/certify.log"
  $bin/volcano-gap spec --pair "$out/pair.json" --seed "$seed" --targets "$targets" \
    --aa-targets "$aa" --bsgs-targets "$bsgs" \
    --curves "$out/export-s1/curves.jsonl,$out/export-s2/curves.jsonl" --out "$out/specs" 2> "$out/spec.log"
  for s in spec spec-aa spec-bsgs; do $bin/ecbench plan --spec "$out/specs/$s.json" > "$out/specs/$s.plan.txt"; done
  ;;
run)
  mkdir -p "$out/sessions"
  pids=()
  for s in spec spec-aa spec-bsgs; do
    $bin/ecbench run --spec "$out/specs/$s.json" --out "$out/sessions/$s" --cpus none --allow-busy \
      --lock "/tmp/crypto-bench-$s.lock" --quiet > "$out/sessions/$s.log" 2>&1 &
    pids+=($!)
  done
  rc=0; for p in "${pids[@]}"; do wait "$p" || rc=1; done; exit $rc
  ;;
audit)
  replay=$3
  for s in spec spec-aa spec-bsgs; do
    $bin/ecbench verify --dir "$out/sessions/$s" --replay "$replay" --exit-code --out "$out/sessions/$s.audit.json" > "$out/sessions/$s.audit.log" 2>&1 || {
      echo "audit failed: $s"; exit 1; }
  done
  ;;
*) echo "unknown command $cmd" >&2; exit 2 ;;
esac
