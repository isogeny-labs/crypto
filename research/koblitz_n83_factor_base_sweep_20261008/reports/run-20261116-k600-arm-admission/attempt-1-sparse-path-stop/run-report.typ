#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)
= N83 K600 arm admission attempt 1: sparse path stop

At 2026-11-16 08:40:51 UTC, the first host-native arm build exited 101 before compiling because the sparse H2 checkout lacked the tracked Cargo path dependency `crates/ecdl-tensor-ic/Cargo.toml`. The minimal profile materialization completed; H2 remained clean. No arm binary, read-only S3 stage, relation search or solver run followed.

#image("diagram.svg", width: 100%)

The frozen launcher and protocol are under `verification/k600-arm-admission-20261116/`. The raw Cargo stderr, exit, start/end timestamps, compiler versions and sparse materialization output are adjacent to `REPORT.md`. Attempt 2 has a separately committed sparse-path amendment and new output paths; this failed setup interval remains charged in the cold ledger.
