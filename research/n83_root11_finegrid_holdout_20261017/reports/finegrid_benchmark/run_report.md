# N83 root11 fine-grid screen - run report

Run date: 2026-10-17
Report generated: 2026-10-21
Scope: Local exploratory arm selection on 128 public N83 points. No new benchmark was run to prepare this report.

## Frozen procedure

Five fixed K arms (2752, 2816, 2880, 2944, 3072) ran sequentially on the same ordinary macOS ARM64 host. The sealed public batch contains 128 targets (SHA-256 72486b7d708023838db1104a4a5e75a04b915d6f6d4013057c934fb31000aa87). Each arm independently replayed its factor base, four-point relations, fixture and recovered scalars, target timing phases, and source/binary binding. Each arm exited successfully; full records are in [results](../../results/).

The selector was frozen as minimum geometric mean target-online interval across the 128-point local batch, with smaller K as the tie-break. See [RESULT.md](../../RESULT.md), [PROTOCOL.md](../../PROTOCOL.md), [full replay](../../results/batch_full_replay.json), [selection record](../../results/local_selection.json), and [host receipt](../../single_target/host.json) for complete provenance.

## Host and isolation

- Platform: macOS-26.6-arm64-arm-64bit-Mach-O; machine: arm64; RAM: 48 GiB.
- Isolation: shared ordinary workstation; no dedicated or cloud-host isolation claim.
- Arms ran sequentially, not concurrently. Process interval includes base construction, root-index and log preparation, all 128 target queries, and producer checks.
- This is an exploratory local selection screen, not production or Tier 1 performance qualification.

## Reproduction

Run from repository root with Python 3 and Matplotlib installed:

    python3 research/n83_root11_finegrid_holdout_20261017/reports/finegrid_benchmark/build_reports.py

The script reads committed per-arm JSON, selection, host receipt, and canonical stratum analysis. It writes this source report set and copies PDFs and graph into [the dated benchmark directory](../../../../benchmarks/20261017-n83-root11-finegrid-screen/). SVG and PNG use the same Matplotlib figure; PNG is exported at 150 dpi.