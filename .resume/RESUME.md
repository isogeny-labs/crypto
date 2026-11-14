# Resume: crypto @ aburan28/inspiring-dijkstra-ay05qi

## Goal
Owner (2026-11-14): "run the interleaved rho vs BSGS comparison" — tuned multi-target rho (PR #4144) vs folded multi-target BSGS on j=0 curves, same targets, interleaved.

## Spec
research/ic_e222_m221_20261106/interleaved_rho_bsgs/PROTOCOL.md (frozen before data); runner run.sh; rows in rows/.
examples/prime_fast_j0_bsgs_multi.rs gained INST0.

## State
Complete: BSGS faster at 26-28 bits, tied at 30. Record closed; PR next.

## Next
Run under benchrec run, analyse (geomean ratio + bootstrap, A/A), README + graph, close record, PR.
