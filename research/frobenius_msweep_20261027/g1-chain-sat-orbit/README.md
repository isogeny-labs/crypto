# Cell G1: chained-S3 SAT, plain x-domain against exact Frobenius-orbit x-domain (m = 4, 5, 6; n = 17, 23, 31)

Status: **prepared, not run.** No experiment, sweep or data-producing binary was executed. The 14 unit tests of the driver use tiny
fixtures (n = 7 exact identities, a planted instance, a proven UNSAT coset target, tamper checks, a synthetic known-ratio analysis) and
their output was discarded.

Question: audit claim C3 (orbit encodings >= 0.95x plain) was never tested at m >= 3. This cell runs four arms (plain P, orbit O,
plain+ordering R, orbit+ordering Q) and a plain-on-sigma-conjugate A/A through one shadow copy of the production chained-S3 loop, against an
exact solver-free oracle and an independent certificate check, on 9 (n, m) cells.

Files: `PROTOCOL.md` (frozen design, controls, resolution, decision rule, runtime reasoning, what does not count), `RUN_CARD.md` (token
command, stages, cap rule, row schema), `run_stage.sh` (one stage), and, outside this directory, `examples/frobenius_msweep_g1_chain_sat_orbit.rs`
(driver plus tests; auto-discovered, no Cargo.toml edit). No standalone crate, so there is no private `target/` to delete.

Headline of the design (priors, not results): arms P and O have the same solution set, so this tests an encoding, not a symmetry
reduction; at N = 80 per cell the design resolves 1.25x to 1.6x differences but not the 5% C3 margin unless the paired sd is under about
0.13, so "unresolved" is the likely grade for C3. The instances are dense (yield near 1, 2 to 8 orbits) and do not transfer to ECC2K-130.
Conductor control plane was unreachable during preparation (`conductor check` could not connect); no scope reservation exists.
