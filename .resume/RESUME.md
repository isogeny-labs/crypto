# Resume: crypto @ claude/curve25519-ic-autolab-loop (2026-11-04)

## Goal
An automated optimisation loop for index calculus on the Curve25519-family
miniatures, built on ecbench and the ic-autolab contract: a ledger of
incumbents per track and size, a candidate panel, a frozen protocol per round,
a train screen, one challenger per size confirmed on a fresh holdout, the §5a
band, the success-criterion classification and the ratchet. Native driver
`src/bin/ic_autolab.rs`; home `research/curve25519_autolab_20261104/`.

## Lead / hypothesis
Round 3 (generic, k = 20 and 24). k = 24: the `ksum:m=4` neighbourhood of the new
incumbent N = 12 (N = 6, 8, 17, 24), plus exploration `rootfind-mitm:m=2` and
`rootfind-pairs:m=3`. k = 20: cross-size transfer of `ksum:m=4` at the k = 24
incumbent's N / N_a = 0.148 (N = 6, 7, 10), `mitm2-n16`, and the same two rootfind
explorations. Prediction: if the m = 4 optimum is a fixed fraction of N_a, k = 20
gains nothing (N = 5 was 3.9x worse); if it needs a minimum absolute N for the
4-sum table to cover, N = 7-10 can beat ksum3-n11.

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
0. Round 3 done (no challenger). The k = 20/24 generic search is near saturated for existing
   oracles; next lever: k = 28, 32 rounds (incumbents there are still m = 2), then source changes.
1. Open the PR, subscribe, land it (artifact-autofix moves records.jsonl to S3).
2. Round 3: `ic_autolab loop --rounds 1 --sizes 20,24` (ksum4 neighbourhood at k = 24, ksum5 at N >= 8, rootfind families); then k = 28, 32.
3. Second-host replay of the k = 24 ratchet before any leaderboard mention; the algebraic track with `--train-targets 6`.

## Runs in flight
Round 4 (goal: keep running the loop toward rho parity): `ic_autolab loop --rounds 1 --track generic --sizes 28,32 --max-candidates 9 --cpus auto --allow-busy --replay 8`.
Panel: mitm2 N-neighbourhood, ksum4 cross-size transfer (k28 N = 21/30/42, k32 N = 53/76/106), rootfind-subtract, subtract.
If interrupted: `ic_autolab run --round 4 --stage train` adopts complete sessions; then select/run holdout/assess/ratchet/report.

## Stop and success conditions
- Success: every step of round 1 runs end to end with every session verified; the classification and ledger update follow the frozen rule whatever the outcome (improvement, null or regression are all valid).
- Stop: a session that fails to verify is kept as evidence and the round ends as inadmissible; no rerun of a session directory.
