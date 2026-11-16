#!/usr/bin/env bash
set -euo pipefail
task_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
task_repo=$(git -C "$task_dir" rev-parse --show-toplevel)
task_out=${1:-/tmp/secant-fixture-proof-receipts}
mkdir -p -- "$task_out"
task_out=$(cd -- "$task_out" && pwd)
task_stage=inputs
trap 'task_exit=$?; printf "{\"stage\":\"%s\",\"exit_code\":%d}\n" "$task_stage" "$task_exit" > "$task_out/status.json"' EXIT
cd -- "$task_dir"
sha256sum -c verification/inputs.sha256 > "$task_out/inputs.log" 2>&1
task_stage=build
lake --no-cache build > "$task_out/build.log" 2>&1
task_stage=warnings
lake env lean -DwarningAsError=true FixtureGroupDegree.lean > "$task_out/warnings.log" 2>&1
task_stage=axioms
lake env lean --run "$task_repo/tools/lean/AuditAxioms.lean" -- \
  CanonicalFixture DegreeBridge SecantFixture RelationDegree SecantIdeal \
  SecantIdentity SecantTopForms DenseTopForms SecantQuadratic SecantGraphTop \
  TopDegree SecantGroupLaw SplitFactorBase FixtureGroupDegree \
  > "$task_out/axioms.log" 2>&1
task_stage=complete
