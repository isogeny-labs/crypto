# Resume: crypto @ claude/curve25519-ic-autolab-loop (2026-11-04)

## Goal
An automated optimisation loop for index calculus on the Curve25519-family
miniatures, built on ecbench and the ic-autolab contract: a ledger of
incumbents per track and size, a candidate panel, a frozen protocol per round,
a train screen, one challenger per size confirmed on a fresh holdout, the §5a
band, the success-criterion classification and the ratchet. Native driver
`src/bin/ic_autolab.rs`; home `research/curve25519_autolab_20261104/`.

## Lead / hypothesis
Round 1 (generic track, k = 20 and 24): the incumbents are `ksum:m=3` at
N = 11 and N = 20 (m = 3 study). Candidates: the ksum3 base-size
neighbourhood (0.5, 0.7, 1.4, 2.0 x N) and two exploration families
(`mitm2` at N_a, `ksum4` at 0.15 N_a). Prediction: the ksum3 optimum is near
the grid boundary found by the m = 3 study, so a neighbour may win at k = 24;
ksum4 is the first m = 4 arm on this family and has no prior.

## Spec
`research/curve25519_autolab_20261104/round-001/{PROTOCOL.md,specs/train-k20.json,specs/train-k24.json,MANIFEST.json}`.
Train 16 targets x 1 round, holdout 32 x 2, timeout 1800 s, counted units,
local route (`ic_autolab run --cpus auto --allow-busy`), `verify --replay 8`.

## State
- Binary builds; 9 unit tests; ledger seeded (`ledger.seed.json` -> `ledger.json`).
- Round 1 frozen (see MANIFEST.json for seeds and sha256).
- Nothing committed yet on this branch.

## Next steps
1. `ic_autolab run --round 1 --stage train`, then `select`, `run --stage holdout`, `assess`, `ratchet`, `report`, `scripts/render_pdf.sh`.
2. Emit a task card (`card`) and lint it with `tools/task_card.py lint`.
3. Session log entry, ic-autolab skill pointer, workflow, commit, PR.

## Runs in flight
None yet.

## Stop and success conditions
- Success: every step of round 1 runs end to end with every session verified; the classification and ledger update follow the frozen rule whatever the outcome (improvement, null or regression are all valid).
- Stop: a session that fails to verify is kept as evidence and the round ends as inadmissible; no rerun of a session directory.
