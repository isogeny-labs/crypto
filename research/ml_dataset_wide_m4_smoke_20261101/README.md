# Wide-prime m4 native CLI smoke

Frozen functional check on two n47 prime-curve twists. The exact protocol bytes are SHA-256 bound by `plan.json`; the repository commit precedes data. Source method context: `research/notes/index-calculus/RESEARCH_FFD_MEASUREMENT.md`, `docs/ecbench/README.md`, and `src/ml_dataset/solvers.rs`. These identify existing measurement and backend conventions; this smoke makes no novelty claim.

Run with the exact-head native CLI, `dataset generate --max-cases 2 --seconds-per-case 30 --memory-mib 2048 --trace-mib 64`, then `dataset verify` and `dataset report`. Preserve every terminal state. The protocol checker referenced by the experiment-protocol skill is absent from this checkout, so native `verify_protocol` is the available freeze/hash check.
