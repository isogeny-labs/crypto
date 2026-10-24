# DRAFT protocol: N83 K1107 panel, rho arms under canonicalisation control

**Status: DRAFT, not frozen, not run.** Companion to `README.md` in this directory. Items marked TBD must be fixed before the card is promoted to `ready`.

## Question

On the sixteen public N83 points of the K1103/K1107 cold-base panel, does the rho arm's cost depend on which orbit canonicalisation it uses? Two arms are compared on identical points, seeds and configuration:

- **N (normal basis, existing).** `examples/n83_rho_optimization_eval.rs` with `StrongRhoG::canonicalize` (`src/cryptanalysis/koblitz_strong_rho.rs`, normal-basis least rotation with sign by y). This is the arm the panel already ran; no port is needed.
- **P (polynomial basis, control).** The same walk with the polynomial-basis orbit minimum of `src/cryptanalysis/koblitz_index_calculus.rs` `canonicalize_signed_rho` (line 11244), or an equivalent arm written for it. TBD: the arm is not written yet; writing it is the only code work.

Decision value: the cryptanalysis claim (4.1x in `papers/frobenius-orbits/sec-benchmarks.tex`, table `tab:rho-ladder`, R1 to R2) is measured at n=53. This control measures the same ratio at n=83 on the panel's own points. It does not change the leaderboard; it tells the reader how much any polynomial-basis-referenced IC/rho ratio is inflated.

## Frozen inputs (to be recorded before the run)

- Points: `targets.jsonl` (16 lines, each a JSON array of two decimal strings) in `research/n83_cold_base_confirm_20261021/` at commit `753b622349` (ci branch). Each line is written to `point_<i>.jsonl` (one array per file) as `run.py` `point_inputs` does. TBD: record the SHA-256 of each point file.
- Seeds: `RHO_SEED_BASE + point_index` with `RHO_SEED_BASE = 2026102100` (`run.py` line 21), point index 1 to 16.
- Configuration: `research/n83_dense_root_20261013/rho_config.json` (`fold`, 64 lanes, `dp_bits` 6, `escape_fruitless` false, `step_cap_factor` 2000). The arm P must use the same lanes, `dp_bits` and step cap; if its config schema differs, record the difference and do not claim equal policy.
- Binary: built with `cargo build --release --locked --example n83_rho_optimization_eval` from the source commit that carries the arm. TBD: record source and executed-binary SHA-256. The panel's binary is `4ed1335f55240ff98470caf0096b9c0cbb3da0a01b157ce88bc83cf886f761e9` (`results/source.json`); arm N should reproduce its receipts' policy fields.

## Execution

- 16 points, each run as one fresh process per arm, arm order alternating by point (N first on odd points, P first on even points).
- Cap 120 s per rho arm (`RHO_CAP_SECONDS = 120`). Keep every failed, timeout and censored row.
- Replay each recovered scalar against the public point with the generic library curve. An unverified scalar is a failure, not a timing.
- Host: wall time needs isolation level L2 (`docs/ecbench/README.md` section 6), with a reserved cpuset and a quiet preflight. Operation counts (Callgrind Ir, the rho receipt's step and charge fields) stand at L0 and are the primary figure if the host is not quiet. Record `uptime`, `/proc/pressure` and the isolation level in the receipt.

## Analysis

- Per point, the paired ratio P/N of online time and of operation counts.
- Mean, sample deviation and two-sided 95% Student-t interval of the paired log-ratio over the 16 points, reported with the isolation level of each figure.
- Count of verified recoveries per arm. A ratio is reported only if both arms recovered on the same points.

## Decision rule (proposed)

- If the 95% interval of P/N for operation counts lies above 1, the polynomial-basis IC/rho ratios cited elsewhere in the repository are inflated by the factor reported, and the note in `research/rho_canonicalisation_audit_20261024/README.md` section 2 is updated.
- If it contains 1, the canonicalisation does not change the N83 rho figure at n=83, and the 2,335 ms figure stands with its basis recorded.
- No wall-clock claim, no leaderboard row, and no speed claim follows from this run under any outcome.

## Not in scope

No change to the IC arm, the 16 IC runs of the panel, or the N83 K1103/K1107 verdict. No new rho algorithm. No solver is run by this draft.
