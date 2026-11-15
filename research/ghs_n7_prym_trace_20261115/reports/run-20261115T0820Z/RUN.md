# Failed first q=2 trace run

- Checkpoint before run: `20261115T081941Z-f97af36fbed3` in `s3://setec-astronomy-artifacts/checkpoints/crypto/codex__ghs-n7-natural-quotient-20261115/`.
- Command: `rustc -O -o /tmp/ghs-n7-prym-trace research/ghs_n7_prym_trace_20261115/src/main.rs`; then `/tmp/ghs-n7-prym-trace > curves.csv 2> summary.txt`.
- Exit: 101 at signed parity assertion `t % 2 == 1`. Negative odd t has Rust remainder -1. `curves.csv` is incomplete and must not be interpreted as a census. Preserved error in `summary.txt`; correction in `../../AMENDMENT_1.md`.
