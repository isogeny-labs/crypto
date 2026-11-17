# Corrected q=2 trace census

- Frozen inputs: `../../PROTOCOL.md`; parity correction: `../../AMENDMENT_1.md`.
- Checkpoint before run: `20261115T082026Z-f97af36fbed3` in `s3://setec-astronomy-artifacts/checkpoints/crypto/codex__ghs-n7-natural-quotient-20261115/`.
- Source SHA-256: `9418a0d981e97e40de7b22a7070a6f84ac27d0ab2088a336903f0737e7081e79`.
- Toolchain: `rustc 1.93.1 (01f6ddf75 2026-02-11) (Homebrew)` on the local macOS host.
- Commands: `rustc -O -o /tmp/ghs-n7-prym-trace research/ghs_n7_prym_trace_20261115/src/main.rs`; `/tmp/ghs-n7-prym-trace > research/ghs_n7_prym_trace_20261115/reports/run-20261115T0821Z/curves.csv 2> research/ghs_n7_prym_trace_20261115/reports/run-20261115T0821Z/summary.txt`.
- Exit: 0. CSV has a header plus 112 exact curve rows. Assertions checked the field inverses, kernel sizes and intersection, 42 per-arm W1 coefficients, 56 union coefficients, 14 classical coefficients, Hasse interval, parity, and inclusion of the classical slice. `summary.txt` gives the trace classes and counts.
- Validation scope: producer assertions and raw CSV; no independent replay or PCC signed verdict yet. No attack timing was measured.
