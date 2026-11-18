# Rust Vieta native NTT study

[Results](RESULTS.md) · [PDF](report.pdf) · [Frozen first cohort](protocol-primary-v2.json) · [Fresh8b cohort](protocol-budget8b.json)

The explicit `fiber-half` arm uses native Montgomery arithmetic, NTT polynomial
products, the existing half-GCD, and exact linear/quadratic roots. `school`,
`newton`, `half`, field-only and NTT-only arms remain references. `lookup-fiber`
uses the same optimized construction as the candidate and its required index.
All cryptographic computation, statistics, control checks and scalar replay
are Rust. Python utilities only map metadata/hashes or render stored statistics.

The experimental crates compile the literal owner Vieta/field sources through
small asserted build-time seams. Earlier source/data/protocol files remain
unchanged. Separate facades expose the local32768 size and8b operation budget;
they do not change the original library's defaults or resource guards.

The fully pinned8b source is2595478e2ac478055c7526f19af6dfc1fe604f8f.
From that commit, build/test `fiber-budget/Cargo.toml`; `replay` reads the saved
`runs/primary-budget8b/0-word62-coset-m32768` directory. See RESULTS.md for exact
commands. A new measurement needs a new protocol and output directory; these
completed matrices are immutable and no measurement remains active.

Reported timings are local producer observations, below L2 and unverified.
Optimized lookup remains cheaper. The general degree bound is still unverified.
No protected leaderboard/index promotion or S3 availability is asserted.
