# Cell G4: iota-stable non-subspace factor bases

Status: **prepared, not run.** No experiment, sweep or data-producing binary has been executed. The unit tests (13, all
passing under `cargo test --release`) use tiny exact fixtures and planted decompositions; their output is discarded.

Question: does the same-instance 2-torsion stabiliser of order 2^(m-1) on iota-stable (x -> 1/x, non-subspace) bases W
union iota(W) change relation statistics or relation-collection economics at m = 2..6, and what does its membership
system cost? Exact enumeration on toy Koblitz curves (a in {0, 1}, n in {11, 13, 17, 19}), all targets at once.

Files: `PROTOCOL.md` (frozen design, decision rule, priors, resolution check, runtime estimate), `RUN_CARD.md` (token
command, preconditions, run commands, outputs), `run_cells.sh` (gated driver), `src/` (std-only native Rust: `field`,
`curve`, `bases`, `enumr`, `run`, `analyze`, `main`), `tests/identities.rs`. No file outside this directory was touched.

Headline of the design (priors, not results): the stabiliser is real and exact, but it clusters relations into orbits of
2^(m-1) identical rows, so under a meet-in-the-middle cost model the per-relation cost is expected to be at best parity
(m = 2, 3) and worse for m >= 4; the membership indicator of W union iota(W) is expected to have the degree of an
unstructured set. The run exists to confirm or break these priors cheaply before any solver-level cell is built.
Standalone crate: delete `target/` after use.
