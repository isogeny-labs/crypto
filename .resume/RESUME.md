# Resume: crypto @ claude/prime-field-index-calculus-atxvot

## Goal
User request (2026-11-10): prime-field index-calculus leads; "stuck, cannot reach rho parity".
Deliverable: research/prime_ic_leads_20261110/ (note, figures, PDF), a problem record
(problems/prime-field-cross-summand-equation-budget.md), a new presentation control in
examples/pkm_tower_pilot.rs (`--controls multiples`), and a local development-check ladder.
Success rule: every verified measured improvement counts; report distance to rho; no S, no row here.

## Lead / hypothesis
Equation budget (note §4): one cross-summand equation per relation over F_p; the [k]-multiple
consequences are the only free extra equations and enter at degree 6. Predictions for the
multiples control: P1 identical d_F where d_F(tower) <= 5; P2 at d_F(tower) >= 7, d_F(multiples)
in {d_F - 1, d_F}; falsifier: a drop >= 2 at a cell with d_F(tower) >= 8 or a flat m-ladder.

## Spec
Pilot cells: Kummer, P1 = 2013265921, seed 0x504B4D54 (1347106132), --planted 0 --random 2,
--engine hky, --controls tower,multiples; m = 2 at t = 2..10 (cmax 7); m = 3, 4 at t = 2, 3
(cmax 8); m = 5 at t = 2, 3 (cmax 9, budget 1800). Binary: target/release/examples/pkm_tower_pilot
built from this worktree (release build 10m58s). Rows to research/prime_ic_leads_20261110/devcheck/.

## State
Worktree /home/user/crypto on the designated branch; nothing pushed yet. No AWS identity here
(no EKS, no S3 checkpoint push); KB, ecdb, cairn, Conductor unavailable. A Sonnet agent implemented
the control (examples/pkm_tower_pilot.rs modified, smoke rows in the scratchpad).

## Next steps
1. Collect the agent's report; run the devcheck ladder; tabulate rows into the note §6 / devcheck/.
2. Session-log entries; commit; push; open the PR (ready for review, auto-merge); subscribe.
