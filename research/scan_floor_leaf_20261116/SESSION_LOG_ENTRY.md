## 2026-11-16T02:45Z — Scan-floor leaf pilot: leads B and C (research/scan_floor_leaf_20261116)

### What was done (commands, files, features)
- `python3 tools/lessons.py brief --scope research-pilot` read. Read `examples/ic_algebraic_floor/linear21.rs` in full and proposals R4-B, R4-C.
- `cargo build --release --example ic_algebraic_floor` tried once: failed, `error[E0308]` at `src/cryptanalysis/koblitz_index_calculus.rs:1646` (HEAD 5f0dbd38f1). Target directory removed afterwards.
- Wrote and committed `PROTOCOL.md` alone (commit d2a43e435e) before any measured run.
- Wrote `scan_leaf.rs` (standalone Rust, rustc -O -C target-cpu=native): port of the lin21-ht leaf, arms B1, B2, C, stage arms, gates, counters, bootstrap, JSON and Markdown output.
- Runs (verified from their output): debugging run on 17:5, 19:6, 21:6; `--mode breakdown` (44 s); `--mode full` (110 s) on 23:7 … 47:15, 128 + 16 targets per cell. All gates pass.
- Outputs: `breakdown.json`, `breakdown_tables.md`, `results.json`, `tables.md`, `README.md`.

### Decisions made (and why)
- Baseline = standalone port, because the repository library does not compile; recorded in the protocol before data.
- Under the frozen rules: B fails (B1 counted 0.423, B2 1.062 at n = 47); C inconclusive (t-stage counted 1.276 at n = 47, threshold 1.3; t-stage wall 2.28). [pcc:unverified]
- No benchmark record, leaderboard or benchmarks/README row: local pilot.

### Open threads (unfinished, pick up next time)
- The library compile error at koblitz_index_calculus.rs:1646 blocks the repository arm; C would need a rerun against the real `add_many` and 4-lane `batch_inv` before any statement about lin21-ht.
- About 0.9 ms per target of C's whole-leaf wall difference at n = 47 is not explained by the t stage; not investigated.
- Only PROTOCOL.md is committed; the other files in the directory are untracked.

### Errors (what went wrong, how it was handled)
- Repository build failure, handled by the pre-declared fallback (standalone port).
- A/A arm biased at n = 23 (1.117) from rotation-slot effects on a 25 µs scan; reported, not a decision cell.
