# Resume: crypto @ aburan28/inspiring-dijkstra-ay05qi

## Goal
Owner (2026-11-15): "run it at 36 bits", then "run it at 38 bits", then "run it at 40 bits" — extend the interleaved multi-target rho vs folded BSGS study (merged in #4169, 26-34 bits) to 36 bits.

## Spec
research/ic_e222_m221_20261106/interleaved_rho_bsgs/PROTOCOL.md addendum B (frozen before data). Runner: run.sh "36 200" "36 1000" "36 3000", under bench record j0-multirho-vs-bsgs-interleaved-36. Binaries target/release/examples/prime_fast_j0_{rho,bsgs}_multi built from source unchanged since a8cc24a6fe.

## State
40 bits complete: rho faster in all cells (0.71/0.55/0.45), A/A pass. Record closed; pushed to PR #4237.

## Next
analyse.py, plot.py, README addendum B section, close record, session log 2026-11-15, PR.
