## Goal
Prove the exact chord encoding needed by the user's twist useful-yield/source-DLP program. The user explicitly approved checkpoint uploads for continued work; routine start/finish/plan checkpoints are authorized, with no paid resources or arbitrary result uploads.
## Lead / hypothesis
The accepted target-aware chord proposal, merged PR3450, needs faithful distinct-root and diagonal algebra before any native solver admission. Sources and prior work are linked in research/quadratic_twist_chord_preflight_20261105/ALGORITHM.md.
## Spec
research/quadratic_twist_chord_proof_20261105/PROTOCOL.md and Chord.lean. Mathlib d13f23b723b8a846827a245b89c10fc7d3f11612; Lean4.34.1. Existing LOCAL resources only, two workers max,600s stage caps,3600s iteration.
## State
New branch codex/twist-chord-proof from current main4268660fa364601cbb6c42f4099677209aa35213. Five proposed algebra declarations written; ZERO proof builds/scientific runs on this branch so far. Prior merged worktrees/evidence untouched. No new measured rank/degree/cost or DLP result.
## Next steps
Commit exact source, pass current official checkpoint gate, then run pinned lake env lean on Chord.lean with600s cap; retain raw output and failed sources. Review axioms and declaration fidelity. Complete group coverage/native census only in later explicitly frozen stages.
## Runs in flight
None. Next proof command has not launched. Logs will live in /workspace/twist-chord-proof-20261105 outside the checkout and be preserved for publication.
## Stop and success conditions
Stop each stage at600s and the focused scientific window at3600s. No paid compute or token fallback. Kernel errors are retained failures. All five declarations plus allowed-axiom audit complete this algebra component only; point-group coverage, native solver census, useful-yield/source-scalar/regularity objectives remain open.
