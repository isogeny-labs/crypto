# Resume: crypto @ claude/affectionate-bohr-19igxi

## Goal
F2 (user, 2026-11-10: "push on for both" open falsifiers of the 04:30Z survey; F1 closed without a run,
see docs/session-logs/2026-11-10.md 16:40Z). Measure the linear complexity over F_p of the p²-lift
defect f = tau_0 / r along the group, k -> f([k]G), on the H110 curves, fresh 12/14/16-bit prime-order
curves and two supersingular (m = 2) curves. Mechanism: sparse DFT over F_{p^m} would transfer the
ECDLP to a DLP in mu_r in F_{p^m}^* through the lift. Class: boundary diagnostic, no S, no rho row.

## Lead / hypothesis
Prediction (frozen): full DFT support everywhere (c = (r-1)/m), L in {r-1, r}; prior <= 0.02 that any
gated observable (O1 f_0, O6 torsion_u) breaks it. Kill/alive lines: PROTOCOL.md §7.

## Spec
research/p2_lift_linear_complexity_20261110/PROTOCOL.md (frozen; FREEZE.sha256 binds it and
tool/src/main.rs). Tool: std-only Rust crate at research/p2_lift_linear_complexity_20261110/tool/
(cargo build --release --manifest-path tool/Cargo.toml; cargo test). Run: the task card in cards/
(experiment-director compiles, executor-mechanical runs: python3 tools/task_card.py run <card>).
Output results/run.json. Reference: research/p2_lift_defect_20261101/run.py (H110).

## State
Branch claude/affectionate-bohr-19igxi restarted from origin/main after PR #3823; PR #3838 open
(F1 closure log, auto-merge). Tool being written by a Sonnet agent; not yet frozen or run.
Conductor, research-kb, ecdb, aws CLI unavailable in this session; no checkpoint upload token seen.

## Next steps
1. Review tool, run cargo test, freeze (FREEZE.sha256), commit.
2. experiment-director compiles the run card; executor-mechanical runs it; director assesses.
3. README.md with rule-7 reading, [pcc:unverified] labels; SVG graph; REPORT.pdf (research-visuals).
4. Try the isolab-eks GitHub route (isolab-experiment.yml, 40-char source_revision) for the §9a diff.
5. Session log entry; commit; PR (new PR if #3838 has merged).

## Stop and success conditions
Stop when the run is complete, every self-check passed, README/graph/PDF/log committed and the PR is
open. Success: a decided rule-7 line per curve with the exact (L, c) numbers; a failed prediction is
reported as failed.

## Runs in flight
None.
