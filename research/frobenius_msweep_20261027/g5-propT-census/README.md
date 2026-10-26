# Cell G5: Proposition T / Sigma census and the A2-prime overlap census at m = 2..8

Status: **prepared, not run.** No experiment, sweep or data-producing binary has been executed. The 21 unit and integration tests
(19 in `tests/identities.rs`, 2 in `tests/pipeline.rs`, all passing under `cargo test --release`) use tiny exact fixtures (n = 11,
and n = 17 for the stable subspaces), planted instances and naive reference enumerators; the one smoke test that drives the
run functions writes to cargo's per-target tmp directory and asserts schemas and exact instrument identities only. Its output is
discarded and never read for findings.

Question: EXP-CERTBIN-a14350 (Proposition T / Sigma) and H-BINSTD-6428a3 (Lemma A2-prime) were specified and priced at m = 2 and
never run (audit claim C4). Does the "n conjugates are n attempts" identity, the same-instance sporadic-overlap law
(probability about (2^m - 2)/r per solution per lag) and the "conjugates add no rank in the orbit basis" statement survive, exactly
and by measurement, for m = 2..8 on toy Koblitz curves, against a random-multiplier null?

Files: `PROTOCOL.md` (frozen design, claim classes, arms, controls, decision rule, priors, resolution check, runtime estimate),
`RUN_CARD.md` (**the token command**, preconditions, run commands, outputs), `run_cells.sh` (gated driver), `src/` (std-only
native Rust: `field`, `curve`, `chain`, `bases`, `census`, `rank`, `proptt`, `rng`, `run`, `analyze`, `main`),
`tests/identities.rs`, `tests/pipeline.rs`. No file outside this directory was touched.

Headline of the design (priors, not results): the identities are certain; the interesting measurement is the paired ratio D of
Frobenius to random-multiplier coincidences on identical tuples, expected near 1 on subspace and random bases. One new derived
prediction (PROTOCOL section 3.4, my derivation, unverified): tuples whose legs all lie in one sigma-orbit cannot coincide
(Z[tau] valuation argument), so on sigma-closed bases (the orbit-union base of the campaign) the independence model overstates
the overlap; the stratum table measures the sign. The cell cannot show an improvement over the index-calculus baseline and
reports no distance to rho.

Not done in preparation (scope was this directory only): Conductor check, session-log entry in `docs/session-logs/`,
`.resume/RESUME.md` fill and S3 push (no AWS profile and no upload token in this container; see RUN_CARD section 0),
protocol freeze with `tools/experiment_protocol.py`, comparison of the re-implemented chain against the SAT encoder in
`src/cryptanalysis/binary_semaev_chain_sat.rs` (S3 and wiring compared by reading only).

Standalone crate (own `[workspace]` stub, no dependencies): delete `target/` after use.
