# Resume: crypto @ aburan28/laughing-darwin-3qdnk2

## Goal
Owner (2026-11-12): "Keep optimizing" the index calculus. Then "1-2", answered as a proper A/B on both
null levers: (1) one inverse per ± twin pair in the fold build; (2) filter-probe lookahead 64 in the
aimed scan.

## Spec
research/ic_tool_program/explorations/build-pairing-lookahead-20261112/ab/PROTOCOL.md (frozen before any
row). Runner: ab/run.sh, with BIN=/dev/shm/blab/ic (SHA-256 in ab/runs/arms.sha256; built from 62f1f0a778
with --features scan-probes) and ISO=/tmp/claude-0/rulev9/bin/isolated_bench. 44 rounds × 2 rows × 4 labels.
Bench record: benchmarks/records/20261112T2321Z-ic-build-pairing-lookahead-ab.json.

## State
2026-11-12 23:2xZ: launching. run.sh resumes: it skips finished rows. If /dev/shm is lost, rebuild the
binary from 62f1f0a778; the hash will differ, so the session restarts with a new arms.sha256 (declare it).

## Next
summarise (adapt lever 2's ab/summarise.sh to the labels a q b l), report and PDF, close the record,
apply the decision rule (keep or revert each switch), and update PR aburan28/crypto#4014.
