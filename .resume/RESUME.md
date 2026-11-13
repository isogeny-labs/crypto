# Resume: crypto @ aburan28/laughing-darwin-3qdnk2

## Goal
Owner (2026-11-13): "CPU-only with a layout tag" for the partition-local presence filter (FilterLayout::Top), then a proper A/B.

## Spec
research/ic_tool_program/explorations/filter-layout-20261113/ab/PROTOCOL.md (frozen at 39affc25bf, before any row).
Runner: ab/run.sh with BIN=/dev/shm/fl/ic (SHA-256 in ab/runs/arms.sha256; built from 39affc25bf with --features scan-probes)
and ISO=/tmp/claude-0/rulev9/bin/isolated_bench. 42 rounds x 2 rows x labels a q t. Bench record: benchmarks/records/20261113T1855Z-ic-filter-layout-ab.json.

## State
2026-11-13 20:05Z: complete. 42 rounds; build gain established at both sizes; Top kept as the CPU default. Record closed; PR open.

## Next
Summarise (lever 2's summarise.sh with a/t), report and PDF, close the record, apply the decision rule, open a PR.
