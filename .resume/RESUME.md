# Resume: crypto @ claude/dreg-lfd-theory-scaling

## Goal
User request (2026-11-05): find ways to reduce the degree of regularity / total solve time of the
descended Semaev system. Three phases, chosen by the user: (1) theory and scaling survey, done
(PR #3512); (2) the l = 9 guess-saturation test; (3) a multilinear closure engine prototype.

## Lead / hypothesis
LEAD-ICE-descent-closure-degree-at-scale / LEAD-ICE-hybrid-guess-determine-l7. The recorded k*
(guesses that hold degree 4) is 0, 1-2 and 4 at l = 6, 7 and 8. K_marginal predicts 7 at l = 9.
Saturation (k* <= 5) would revive the guess route.

## Spec
research/dreg_guess_scaling_20261106/PREREGISTRATION.md (frozen at commit 37129b4253).

## State
examples/dreg_linear_guess.rs gained `--engine pe`. Release build in progress. No runs yet.

## Next steps
1. Gate A: run dreg_linear_guess --engine pe on the A1/A2 replays (commands in PREREGISTRATION.md).
2. Generate draws: target/release/examples/dreg_ladder --cells 25:9:4 --m 3 --ffd-max 3 --seed 20261106 --max-draws 64 > research/dreg_guess_scaling_20261106/draws/m3-25-9.jsonl
3. Run P and the S5 branches with fail-fast, under prlimit 15 GB and timeout 7200.

## Runs in flight
none

## Stop and success conditions
About 12 h of container time at most. The decision rule is in the PREREGISTRATION.

## Open questions
Whether pe_closure fits about 196k columns in 15 GB.
