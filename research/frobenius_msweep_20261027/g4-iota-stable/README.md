# Cell G4: iota-stable non-subspace factor bases

Status: **prepared, not run.** No experiment, sweep or data-producing binary has been executed. The unit tests (24, all
passing under `cargo test --release`) use tiny exact fixtures and planted decompositions; their output is discarded.

Question: does the same-instance 2-torsion stabiliser of order 2^(m-1) on iota-stable (x -> 1/x, non-subspace) bases W
union iota(W) change relation statistics or relation-collection economics at m = 2..6, and what does its membership
system cost? Exact enumeration on toy Koblitz curves (a in {0, 1}, n in {11, 13, 17, 19}), all targets at once.

Files: `PROTOCOL.md` (frozen design, decision rule, priors, resolution check, runtime estimate), `RUN_CARD.md` (token
command, preconditions, run commands, outputs), `run_cells.sh` (gated driver), `src/` (std-only native Rust: `field`,
`curve`, `bases`, `enumr`, `run`, `analyze`, `main`), `tests/identities.rs`. No file outside this directory was touched.

Revision 2 (2026-10-27, still unrun): revised after an independent review (PROTOCOL.md section 12a): exact per-replicate item-count
matching and a paired bootstrap, a plan-reconciling analysis that never classifies a partial cell, a registered total-cost
estimand next to the per-relation one, fractional-target accounting, a construction null and an A/A false-alarm gate, MITM
orientation chosen per arm, and an explicit scope statement (nothing here addresses m = 31, the m = 83 gate or ECC2K-130).

Headline of the design (priors, not results): the stabiliser is real and exact and clusters relations into orbits of
2^(m-1) identical rows. Under a meet-in-the-middle model the cost per independent relation is expected at parity (m = 2) and
worse for m >= 3, while the total cost to 90 percent rank, which sees the halved column count, is expected to favour the stable
base about 2x at m = 2, be at parity at m = 3, 4 and lose at m >= 5. The membership indicator of W union iota(W) is expected to
have the degree of an unstructured set. The run exists to confirm or break these priors cheaply before any solver-level cell
is built; it cannot establish any solver effect or any improvement over the index-calculus baseline.
Standalone crate: delete `target/` after use (`.gitignore` lists it).
