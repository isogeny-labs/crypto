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
| [AMENDMENT.md](AMENDMENT.md) | where the spec is not executable as written (A1-A18) and what replaces it |
| [RUN_CARD.md](RUN_CARD.md) | exact commands, gates, expected wall times, output paths under `reports/run-<UTC>/` |
| `src/gf.rs` | GF(2^n), normal and polynomial bases, stable subspaces V = ker g(sigma) |
| `src/curve.rs` | binary curve arithmetic, planted instances, exact solver-free decomposition oracle |
| `src/enc.rs` | the nine encodings (C, Cpoly, Cnobreak, A, B, Bnogauge, G, U, Chalf) and certificate verification |
| `src/run.rs` | CLI: `run`, `sizes`, `targets`, `summarize`, `gate`, `manifest` |
| `src/stats.rs` | paired cluster-bootstrap ratios, A/A, controls, decision rule, pricing band, SVG |
| `src/tests_enc.rs` | correctness fixtures |

Solver: the repository's native CDCL with native XOR rows
(`src/cryptanalysis/sat.rs`), compiled in by path; WDSat and CaDiCaL are not
native Rust and are not used. std-only crate, no root `Cargo.toml` change.

Revision 2 (after independent review, still preparation only): arm G made
sound at m = 3, target keys include `xr`, a B-only m = 3 label (O-BOUNDED), arm
Chalf and control P3 (P1 and P2 demoted to informative), memory and
re-estimate gates, a hard-capped pilot with a disjoint target stream, 1%
censoring tolerance, a freeze procedure that commits the frozen set and writes
`FREEZE.txt`. See PROTOCOL.md section 14.

Tests (cell directory): `CARGO_TARGET_DIR=<outside the tree> cargo test --release -- --skip satlib` gives 37
passed, 0 failed. `satlib` is the repository solver's own suite, skipped here.

What the tests establish (exact, no statistics): the Semaev S3 formula equals
the group law; the exact oracle equals brute force; arms C, Cpoly, Cnobreak, A,
B, Bnogauge and G return the oracle's verdict with valid certificates on all 127
targets at n = 7 for m = 2 (l = 3 and 4) and, with the same seven arms, for
m = 3 (l = 3); on six n = 17 targets only C, Cpoly, A, B and G were checked;
G admits every sorted V-triple at (7,3,3) and 120 sampled triples at (31,5,3)
with the slots and chain variable pinned; Chalf (all-slot pin) never invents a solution; the unsound arm U flips satisfiable targets and B is caught as unsound
on an ordinary curve; the domain of each arm equals an independent Rust predicate
(PROTOCOL claim S2, "up to the tie sliver", is checked only as that equality
plus a weak lower bound on B's domain, not numerically against |V^m/S_m|).

What is not known: any conflict count, ratio or runtime. The runtime table in
PROTOCOL.md section 8 is a code-reasoning estimate with a factor-ten
uncertainty.

Clean-up: build with `CARGO_TARGET_DIR=<dir outside the tree> cargo build --release`
(the freeze requires an empty `git status`; tests write to the OS temp dir);
delete that directory when finished. Revision 3 (PROTOCOL section 15): P3 re-derived,
O-STRUCTURE fails closed on missing controls, freeze commands made exact.
Revision 4 (PROTOCOL section 16, 2026-10-27): O-RELABEL now requires the P3 positive control; Cargo.lock no longer git-added in the freeze (ignored; hashed only). 38 tests.
