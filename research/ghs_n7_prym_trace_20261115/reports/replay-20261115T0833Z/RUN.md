# Direct equation point-count replay

- Frozen protocol: `../../REPLAY_PROTOCOL.md`.
- Pre-run checkpoint and gate: `20261115T083252Z-7d9786f2518a` in `s3://setec-astronomy-artifacts/checkpoints/crypto/codex__ghs-n7-natural-quotient-20261115/`.
- Input CSV SHA-256: `e20933ba06cb35d52cfcc4bd853171682bc5853c62124cfb0b036bc1be89c18f`.
- Replay source SHA-256: `094c349893be3d23dfe49829b56f4f85cb51224cb38100842d6a51808a23cf8e`.
- Toolchain: `rustc 1.93.1 (01f6ddf75 2026-02-11) (Homebrew)`.
- Commands: `rustc -O -o /tmp/ghs-n7-prym-replay research/ghs_n7_prym_trace_20261115/src/replay.rs`; `/tmp/ghs-n7-prym-replay > research/ghs_n7_prym_trace_20261115/reports/replay-20261115T0833Z/result.txt 2>&1`.
- Exit 0; `result.txt` says `direct_equation_replay_rows=112 all_orders_match=true`. The replay uses a different field-multiplication implementation and enumerates all (x,y) solutions rather than the producer's trace formula. It is an algorithmic cross-check by this session, not a signed third-party verdict. No runtime comparison is made.
