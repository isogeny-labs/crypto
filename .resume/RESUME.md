# Resume: crypto @ aburan28/laughing-darwin-3qdnk2 — IC lever 2 A/B, 2026-11-12

## Goal
Measure the paired-inverse change (one inverse per ± twin pair in the aimed scan's folding kernel, a31ae987a6/b06ee3ae7f) on the collect phase and set-up of `ic price --single-target`.

## Lead / hypothesis
LEAD-ICB-koblitz-setup-phase-cost. research/ic_tool_program/explorations/paired-inverse-20261112/ab/PROTOCOL.md: A/B a/c medians predicted collect [1.02,1.08] (k0n53), [1.05,1.13] (k0n61).

## Spec
- Protocol, run.sh, summarise.sh: research/ic_tool_program/explorations/paired-inverse-20261112/ab/ (commit 064334a304).
- Binary: /tmp/claude-0/l2ab/ic, SHA-256 in ab/runs/arms.sha256 (rebuild: CARGO_TARGET_DIR=/dev/shm/tgt-prof cargo build --release --bin ic at that commit).
- isolated_bench: /tmp/claude-0/rulev9/bin/isolated_bench (2f56b833…).
- Bench record: benchmarks/records/20261112T0607Z-ic-paired-inverse-ab.json (open).

## State
Protocol and pins committed; session starting.

## Next steps
1. `BIN=/tmp/claude-0/l2ab/ic ISO=/tmp/claude-0/rulev9/bin/isolated_bench bash research/ic_tool_program/explorations/paired-inverse-20261112/ab/run.sh` (resumes).
2. `bash .../ab/summarise.sh`; report/graph/PDF; validator; benchmarks row only if validated.

## Runs in flight
Local session of the A/B (252 processes), started 2026-11-12 ~06:10 UTC.

## Stop and success conditions
Identity mismatch voids the session. Decision per PROTOCOL.md (A/B lower > 1 and > A/A upper), applied once at 42 rounds.

## Open questions
Iso Lab blocked (CodeBuild quota 0); lever 3 dropped; lever 1 dropped.
