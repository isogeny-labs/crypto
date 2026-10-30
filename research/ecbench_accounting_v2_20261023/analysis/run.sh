#!/bin/bash
# Run every session of the study, one after the other, from the repository root.
# Usage: research/ecbench_accounting_v2_20261023/analysis/run.sh   (sessions must not exist)
set -u
ROOT=$(git rev-parse --show-toplevel); cd "$ROOT"
S=research/ecbench_accounting_v2_20261023
BR=./target/release/ecbench
MAIN=${MAIN_BIN:?set MAIN_BIN to the main-branch ecbench binary}
run() { # binary spec name
  echo "== $3 $(date -u +%FT%TZ)"; "$1" run --wait --allow-busy --cpus auto --quiet --spec "$2" --out "$S/sessions/$3" 2>&1 | tail -2
}
run $BR docs/ecbench/specs/check-prime-j0-floor-v2.json j0-v2
run $BR $S/specs/frozen-j0.json j0-frozen-branch
run $MAIN $S/specs/frozen-j0.json j0-frozen-main
run $BR $S/specs/pairs-prime.json pairs-prime
run $BR $S/specs/frozen-prime.json frozen-prime-branch
run $MAIN $S/specs/frozen-prime.json frozen-prime-main
run $BR $S/specs/pairs-koblitz.json pairs-koblitz
run $BR $S/specs/frozen-koblitz.json frozen-koblitz-branch
run $MAIN $S/specs/frozen-koblitz.json frozen-koblitz-main
run $BR $S/specs/pairs-large-prime.json pairs-large-prime
run $BR docs/ecbench/specs/smoke-prime-v2.json smoke-prime-v2
run $BR docs/ecbench/specs/smoke-koblitz-v2.json smoke-koblitz-v2
run $BR $S/specs/strong-n37.json strong-n37
run $BR docs/ecbench/specs/claim-koblitz-strong-v2-n13.json claim-n13
run $BR docs/ecbench/specs/claim-koblitz-strong-v2-n19.json claim-n19
echo "== done $(date -u +%FT%TZ)"
