# Corrected q2/q4 natural quotient census

Frozen protocol: ../../PROTOCOL.md, with ../../AMENDMENT_1.md.
Command: `cargo run --release --manifest-path research/ghs_n7_natural_quotient_20261115/Cargo.toml` under `/usr/bin/time -l` on the local macOS host.
Checkpoint: `s3://setec-astronomy-artifacts/checkpoints/crypto/codex__ghs-n7-natural-quotient-20261115/20261115T074655Z-9d0666146ffc/`; gate passed before launch.
Status: exit 0. Raw stdout: result.txt. Raw stderr and time: timing.txt. Source hash: source.sha256. Compiler: rustc.txt.
The assertions check exact W1 order, expected per-arm and union counts, swap subset of classical locus, and equality of swap and classical coefficient sets.
