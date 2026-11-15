# Resume: crypto @ aburan28/inspiring-dijkstra-ay05qi

## Goal
Owner (2026-11-15): "Attack the algebraic gap and 1" — single-target IC toward rho parity: (1) the C = 1 arm toward its 1.5x generic floor; (A) the algebraic gap.

## Spec
(1) Rerun the frozen C = 1 confirmation spec research/curve25519_autolab_20261104/c1/spec-confirm-k32.json unchanged (copied to research/ic_c1_headroom_20261115/spec-k32.json) with `ecbench run --cpus none --allow-busy` (L0), to get per-phase GAE locally (the original records are in the artifact store, unreachable without AWS). Then split C = 1 S into setup vs walk vs floor 1.329 and compare with rho-neg's split.
(A) Finding so far (no run): the algebraic screen's best row (m = 2 root-find per base point) is the generic rootfind-subtract sweep; on the real algebraic base (L13) even a free oracle is ~17x behind rho at w = 0 (coverage-limited).

## State
Complete: rerun reproduced C = 1 S = 1.545; phase split and algebraic analysis written in research/ic_c1_headroom_20261115/README.md; record closed.

## Next
Phase split, README + graph, close record, PR; report to owner.
