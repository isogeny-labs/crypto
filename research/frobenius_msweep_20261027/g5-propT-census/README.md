# Cell G5: Proposition T / Sigma census and the A2-prime overlap census at m = 2..8

Status: **prepared, not run** (revision 2, 2026-10-27, after an independent review of revision 1). No experiment, sweep or
data-producing binary has been executed. The 29 unit and integration tests (25 in `tests/identities.rs`, 4 in
`tests/pipeline.rs`, all passing under `cargo test --release --locked` and in the debug profile) use tiny exact fixtures (n = 11,
13, 19, and n = 17 for the stable subspaces), planted instances and naive reference enumerators. Two tests drive code that
writes or computes: the driver smoke test writes to cargo's per-target tmp directory and asserts schemas and exact instrument
identities only; the operating-characteristic test simulates synthetic Poisson counts through the frozen decision rule
(a design-stage power table, no experiment data). Their output is discarded or pasted into PROTOCOL section 6 as a design
calculation; it is never read for findings.

Question: EXP-CERTBIN-a14350 (Proposition T / Sigma) and H-BINSTD-6428a3 (Lemma A2-prime) were specified and priced at m = 2 and
never run (audit claim C4). Does the "n conjugates are n attempts" identity, the same-instance sporadic-overlap law
(probability about (2^m - 2)/r per solution per lag) and the "conjugates add no rank in the orbit basis" statement survive, exactly
and by measurement, for m = 2..8 on toy Koblitz curves, against a random-multiplier null? Notation: n is the field degree
(AGENTS.md 8a calls it m), m is the number of legs.

Files: `PROTOCOL.md` (design, claim classes, arms, controls, decision rule, power table, priors, runtime estimate, revision log),
`RUN_CARD.md` (token command, frozen route with argv array and env, session-side checkpoint gate, preconditions, run commands, outputs, after-run steps), `run_cells.sh` (driver, needs the
session's `G5_CHECKPOINT_STAMP`, never calls the gate itself, archives and uploads the run on exit: clean-tree check, tests, build, provenance, pre-flight timing, Proposition T cells and fail-fast gate, then rank and overlap
cells, then analyze), `src/` (std-only native Rust: `field`, `curve`, `chain`, `bases`, `census`, `rank`, `proptt`, `rng`, `run`,
`analyze`, `main`), `tests/`. No file outside this directory was touched.

Headline of the design (priors, not results): the identities are certain; the interesting measurement is the paired ratio D of
Frobenius to random-multiplier coincidences on identical tuples, expected near 1 on subspace and random bases, now judged by a
declared +-10 percent equivalence margin (TOST) with an A/A gate, a null-vs-null calibration and a planted positive control. One
derived, proved-here statement (PROTOCOL 3.4, not in the ledger): Frobenius coincidences are impossible on single-orbit tuples whose
spread and lag satisfy m^2 2^s (2^(j/2)+1)^2 < r; it is checked exhaustively by unit tests and, in the run, as an exact zero that
is informative only in the few cell-arms where the null is expected to hit at least 5 times. The cell cannot show an improvement
over the index-calculus baseline and reports no distance to rho. Nothing here discharges the m = 83 gate of AGENTS.md 8a.

Not done in preparation (scope was this directory only): Conductor check, session-log entry in `docs/session-logs/`,
`.resume/RESUME.md` fill and S3 push (no AWS profile and no upload token in this container; see RUN_CARD section 0), protocol freeze
(`protocol.json` and `tools/experiment_protocol.py freeze`), submission to isolab-eks, comparison of the re-implemented chain
against the SAT encoder in `src/cryptanalysis/binary_semaev_chain_sat.rs` (S3 and wiring compared by reading only).

Standalone crate (own `[workspace]` stub, no dependencies): `target/` is ignored by Git and the driver builds outside the cell by
default; delete any `target/` after use.

Revision 3 (2026-10-27): trend-rule inversion fixed, B4 now voids before classing, Cargo.lock is tracked via the cell .gitignore (commit it), see PROTOCOL 13.
