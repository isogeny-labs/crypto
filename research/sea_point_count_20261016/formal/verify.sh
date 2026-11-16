#!/usr/bin/env bash
# Thin tool orchestration. Arithmetic, measurements and verification are native.
set -euo pipefail
study=$(cd "$(dirname "$0")/.." && pwd)
jar=${TLA_JAR:-${RUNNER_TEMP:-/tmp}/sea-tla2tools-1.7.1.jar}
logs=${FORMAL_LOG_DIR:-${RUNNER_TEMP:-/tmp}/sea-formal-replay}
mkdir -p "$logs"
if [ ! -f "$jar" ]; then
  # v1.8.0 is a rolling prerelease whose asset bytes change under one URL.
  # Use the stable 1.7.1 release and verify its published SHA-1 and our SHA-256.
  curl --proto '=https' --tlsv1.2 -fsSL \
    https://github.com/tlaplus/tlaplus/releases/download/v1.7.1/tla2tools.jar -o "$jar"
fi
printf '9416f74257aa50f250776db34964db7ec99e9883  %s\n' "$jar" | sha1sum -c -
printf 'd532ba31aafe17afba1130f92410d9257454ff7393d1eb2fe032f0c07f352da5  %s\n' "$jar" | sha256sum -c -
cd "$study/formal"
check() {
  local model=$1 config=$2 expected=$3 invariant=${4:-}
  local code=0
  timeout 300 java -XX:+UseParallelGC -Xmx2g -cp "$jar" tlc2.TLC \
    -workers 1 -seed 1 -fp 0 -metadir "$logs/states-$config" \
    -config "$config.cfg" "$model.tla" > "$logs/$config.txt" 2>&1 || code=$?
  printf '%s\n' "$code" > "$logs/$config.exit"
  if [ "$code" -ne "$expected" ]; then cat "$logs/$config.txt"; exit 1; fi
  if [ "$expected" -eq 0 ]; then
    grep -q 'Model checking completed. No error has been found.' "$logs/$config.txt"
  else
    grep -q "Invariant $invariant is violated" "$logs/$config.txt"
  fi
  printf '%s: expected exit %s verified\n' "$config" "$expected"
}
check Recombination Fixed 0
check Recombination Wide 0
check Recombination Legacy 12 NontrivialTrueSurvives
check Certification Certification 0
check Certification SampleOnly 12 NontrivialSampleSound
cd "$study/lean"
lake build > "$logs/lean-build.txt" 2>&1
lake env lean SeaProofs.lean > "$logs/lean-axioms.txt" 2>&1
lake env lean RegistryOrderUniqueness.lean > "$logs/registry-order-axioms.txt" 2>&1
if grep -Eq 'sorryAx|^error:' "$logs/lean-axioms.txt" "$logs/registry-order-axioms.txt"; then
  cat "$logs/lean-axioms.txt" "$logs/registry-order-axioms.txt"
  exit 1
fi
printf 'Lean build and axiom inspection passed\n'
