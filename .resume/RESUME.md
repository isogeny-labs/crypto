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
- Binary builds; 10 unit tests; clippy and rustfmt clean on the bin; train card lints.
- Round 1 done: no challenger at k = 20, 24 (train sessions ECBS1h87338492e26d, ECBS1h7e23ac17a395).
- Round 2 done (through `loop --rounds 1`): k = 24 improvement, `ksum4-n12` replaces
  `ksum3-n20` (sel/inc 0.917 [0.879, 0.957] at w = 0, band sel < inc, holdout
  ECBS1h32d89e6cb3e5, 480/480 verified); k = 20 no challenger. Ledger: 19 trials,
  28 exposures, 1 ratchet.
- Committed and pushed on this branch; PR to open.

## Next steps
1. Open the PR, subscribe, land it (artifact-autofix moves records.jsonl to S3).
2. Round 3: `ic_autolab loop --rounds 1 --sizes 20,24` (ksum4 neighbourhood at k = 24, ksum5 at N >= 8, rootfind families); then k = 28, 32.
3. Second-host replay of the k = 24 ratchet before any leaderboard mention; the algebraic track with `--train-targets 6`.

## Runs in flight
None (rounds 1 and 2 complete).

## Stop and success conditions
- Success: every step of round 1 runs end to end with every session verified; the classification and ledger update follow the frozen rule whatever the outcome (improvement, null or regression are all valid).
- Stop: a session that fails to verify is kept as evidence and the round ends as inadmissible; no rerun of a session directory.
