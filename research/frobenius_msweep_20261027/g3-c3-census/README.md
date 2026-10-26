# Cell G3: the C3 census (H-BINSTD-c72284 Stage 2 / EXP-BINSTD-372679)

**Status: prepared, not run.** No experiment, sweep, benchmark or
data-producing run has been executed in this cell. The only executions were the
unit tests (exact identities and planted instances on n = 7 and n = 17 toys) and
a build-only size check. No resume checkpoint could be uploaded from the
preparation container, so the run gate stays closed.

Question: on a linear sigma-stable factor base V = ker g(sigma), do sound
Frobenius orbit encodings (A: representative-and-shift; B: gauge-fixed one-hot
orbit system) cost at least 0.95x the plain single-instance search (C), at
n = 17, 23, 31 (m = 2) and n = 31 (m = 3), against a size-matched generic
control (G), an A/A arm and two positive controls? The audit
(`/home/user/crypto-work/frobenius_audit_20261026.json`, claim C3) found no data
for it.

| file | what |
|:--|:--|
| [PROTOCOL.md](PROTOCOL.md) | frozen-at-run design: claims, arms, controls, metric, decision rule, sizes and resolution, runtime estimate, limits |
| [AMENDMENT.md](AMENDMENT.md) | where the spec is not executable as written (12 items) and what replaces it |
| [RUN_CARD.md](RUN_CARD.md) | exact commands, gates, expected wall times, output paths under `reports/run-<UTC>/` |
| `src/gf.rs` | GF(2^n), normal and polynomial bases, stable subspaces V = ker g(sigma) |
| `src/curve.rs` | binary curve arithmetic, planted instances, exact solver-free decomposition oracle |
| `src/enc.rs` | the eight encodings (C, Cpoly, Cnobreak, A, B, Bnogauge, G, U) and certificate verification |
| `src/run.rs` | CLI: `run`, `sizes`, `targets`, `summarize`, `gate`, `manifest` |
| `src/stats.rs` | paired cluster-bootstrap ratios, A/A, controls, decision rule, pricing band, SVG |
| `src/tests_enc.rs` | correctness fixtures |

Solver: the repository's native CDCL with native XOR rows
(`src/cryptanalysis/sat.rs`), compiled in by path; WDSat and CaDiCaL are not
native Rust and are not used. std-only crate, no root `Cargo.toml` change.

Tests (cell directory): `cargo test --release -- --skip satlib` gives 30
passed, 0 failed. `satlib` is the repository solver's own suite, skipped here.

What the tests establish (exact, no statistics): the Semaev S3 formula equals
the group law; the exact oracle equals brute force; arms C, Cpoly, Cnobreak, A,
B, Bnogauge and G return the oracle's verdict with valid certificates on all 127
targets at n = 7 (m = 2, l = 3 and 4), on all 127 targets at m = 3, and on six n = 17
targets; the unsound arm U flips satisfiable targets and B is caught as unsound
on an ordinary curve; the domain of each arm has the model count of an
independent predicate.

What is not known: any conflict count, ratio or runtime. The runtime table in
PROTOCOL.md section 8 is a code-reasoning estimate with a factor-ten
uncertainty.

Clean-up: build with `cargo build --release` in this directory; delete
`target/` when finished (`rm -rf target`).
