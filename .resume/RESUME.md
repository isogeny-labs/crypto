# Resume: crypto @ aburan28/inspiring-dijkstra-ay05qi

## Goal
Owner (2026-11-14): "run the interleaved rho vs BSGS comparison" — tuned multi-target rho (PR #4144) vs folded multi-target BSGS on j=0 curves, same targets, interleaved.

## Spec
research/ic_e222_m221_20261106/interleaved_rho_bsgs/PROTOCOL.md (frozen before data); runner run.sh; rows in rows/.
examples/prime_fast_j0_bsgs_multi.rs gained INST0.

## State
26-30 complete (PR #4169). Addendum A (32/34 bits) frozen in e8c0fd72da; run started under record j0-multirho-vs-bsgs-interleaved-32-34.

## Next
Run under benchrec run, analyse (geomean ratio + bootstrap, A/A), README + graph, close record, PR.
