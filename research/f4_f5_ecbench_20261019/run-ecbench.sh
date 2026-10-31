#!/usr/bin/env bash
set -euo pipefail
out=${EPHEMERAL_OUT:?ephemeral output directory required}
study=research/f4_f5_ecbench_20261019
mkdir -p "$out/source"
cp "$study/PROTOCOL.md" "$study/ECBENCH_PROTOCOL.md" "$study/run-ecbench.sh" "$study"/ecbench-n*.json "$study/specs.sha256" "$out/"
git rev-parse HEAD > "$out/revision.txt"
git diff --binary > "$out/local.patch"
rustc --version > "$out/compiler.txt"
lscpu > "$out/cpu.txt"
free -b > "$out/memory.txt"
files=(src/cryptanalysis/koblitz_groebner.rs src/cryptanalysis/gf2_elim.rs src/bin/ecbench/pdp3_koblitz.rs src/cryptanalysis/ecbench/methods.rs tools/generalized-rootfinder/src/solver.rs src/cryptanalysis/ic_framework/rootfind.rs src/cryptanalysis/ic_boundary.rs Cargo.lock)
for file in "${files[@]}"; do mkdir -p "$out/source/$(dirname "$file")"; cp "$file" "$out/source/$file"; done
sha256sum "${files[@]}" > "$out/source.sha256"
sha256sum -c "$study/specs.sha256" > "$out/spec-verification.log"
cargo test --locked --release --test ecbench pdp3_matrix_runtime_and_generalized_recover_and_replay -- --exact > "$out/integration-test.log" 2>&1
cargo build --locked --release --bin ecbench > "$out/build.log" 2>&1
bin=target/release/ecbench
cp "$bin" "$out/ecbench-binary"
sha256sum "$bin" > "$out/binary.sha256"
"$bin" host > "$out/ecbench-host.json"
sleep 60
for n in 17 23; do
  "$bin" plan --spec "$study/ecbench-n$n.json" > "$out/plan-n$n.txt"
  # A session is never overwritten or edited. An incomplete run still keeps
  # its records, audit, comparison and resources; the other curve can run.
  set +e
  "$bin" run --spec "$study/ecbench-n$n.json" --out "$out/session-n$n" --wait --quiet > "$out/run-n$n.log" 2>&1
  code=$?
  set -e
  printf '%s\n' "$code" > "$out/run-n$n.exit"
  if [ ! -f "$out/session-n$n/records.jsonl" ]; then continue; fi
  set +e
  "$bin" verify --dir "$out/session-n$n" --replay-all --out "$out/audit-n$n.json" --exit-code > "$out/audit-n$n.log" 2>&1
  code=$?
  set -e
  printf '%s\n' "$code" > "$out/audit-n$n.exit"
  for method in f4 f5; do
    "$bin" compare --dir "$out/session-n$n" --a "$method-baseline" --b "$method-tail" --save --json > "$out/compare-n$n-$method.json"
  done
  for method in f4-tail f5-tail f6-native generalized-typed semaev-batched semaev-full-table; do
    "$bin" compare --dir "$out/session-n$n" --a rho-strong --b "$method" --save --json > "$out/compare-n$n-rho-$method.json"
  done
  "$bin" table --dir "$out/session-n$n" > "$out/table-n$n.txt"
  "$bin" db sql "$out/session-n$n" > "$out/session-n$n.sql"
done
printf '%s\n' complete > "$out/ecbench-phase.status"
