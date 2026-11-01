# Cell G1: chained-S3 SAT, plain x-domain against exact Frobenius-orbit x-domain (m = 4, 5, 6; n = 19, 23, 31)

Status: **prepared, not run.** No experiment, sweep or data-producing binary was executed. The 25 unit tests of the driver use tiny
fixtures (n = 7 exact identities, a planted instance, a proven UNSAT coset target, tamper checks, a synthetic known-ratio analysis) and
their output was discarded.

Question: audit claim C3 (orbit encodings >= 0.95x plain) was never tested at m >= 3. This cell runs four arms (plain P, orbit O,
plain+ordering R, orbit+ordering Q) and a plain-on-sigma-conjugate A/A through one shadow copy of the production chained-S3 loop, against an
exact solver-free oracle and an independent certificate check, on 9 (n, m) cells.

Amendment 1 (2026-10-27, before any data; PROTOCOL section 11): n = 17 replaced by 19 (a = 0 has no usable subgroup at n = 17), group order and k recomputed from the real r (all cells saturated with k = 2), coset targets 8, the deciding statistic is cost per relation with per-arm censoring gates (the old survivors-only ratio could flatter an arm censored more), per-cell caps from a nine-cell pilot via `cap`, provenance written by `run_stage.sh`.

Amendment 2 (2026-10-27, before any data; PROTOCOL section 12): `contradicted` now needs the lower bound above the margin; the price-0 leg no longer decides C3; certificates (`cert_points`, `target_xy`) and a per-cell `manifest-<cell>.json` are written to the run output and re-verified by the native verifier `verify_certs/` (Amendment 3 replaced the Python script); the run card is executable block by block (`env.sh`), `cap` flags cells without data, and the checkpoint gate is a session-side gate.

Files: `PROTOCOL.md` (frozen design, amended before data, controls, resolution, decision rule, runtime reasoning, what does not count), `RUN_CARD.md` (token
command, stages, cap rule, row schema), `run_stage.sh` (one stage), `env.sh` (shell setup sourced by each card block), `verify_certs/` (independent native certificate verifier crate), and, outside this directory, `examples/frobenius_msweep_g1_chain_sat_orbit.rs`
(driver plus tests; auto-discovered, no Cargo.toml edit). No standalone crate, so there is no private `target/` to delete.

Headline of the design (priors, not results): arms P and O have the same solution set, so this tests an encoding, not a symmetry
reduction; the C3 strata have N = 40 per cell, where the design resolves about a 1.4x difference but not the 5% C3 margin unless the paired
sd is under about 0.166, so "unresolved" is the likely grade for C3. The price-0 GAE leg (lift additions only) is too noisy to decide C3 and is
excluded from it (Amendment 2); C3 rests on word_ops and the priced GAE at 0.1 and 1. The instances are dense (yield near 1, 2 to 8 orbits) and do not transfer to ECC2K-130.
Conductor control plane was unreachable during preparation (`conductor check` could not connect); no scope reservation exists.
