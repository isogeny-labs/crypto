# Isogeny-edge certification gap and an independent checker

Date: 2026-11-13. Scope: certification of recorded Vélu edges only. This is
**not** an ECDLP speedup and changes no measurement, row or leaderboard.
All statements below are `[pcc:unverified]` assertions until a pcc claim is
filed; the test command and its output are given so they can be replayed.

## The gap (as found in the code today)

`src/bin/volcano_rho.rs` (used by the `isogeny-volcano` skill) checks each
new edge with three **sampled** tests on the two instance points P, Q:
image on the codomain, `φ(P+Q) = φ(P)+φ(Q)`, and `n·φ(P) = O`. Then:

1. **Kernel never checked against the division polynomial at check time.**
   The kernel abscissa comes from the producer's own root finder of ψ_ℓ;
   the checks never re-test `ψ_ℓ(x0) = 0`. Witness (test
   `test_degenerate_non_kernel_passes_sampled_checks_but_is_rejected`):
   on `y² = x³ + x + 3` over F_31, `x0 = 17` satisfies `3x0² + a = 0`, is not
   a 2-kernel, and the ℓ=2 Vélu formulas give `t = u = 0`, the identity map
   E → E. It passes all three sampled checks. (In the walk itself such an
   edge would be deduplicated by j, so this is a checker weakness, not an
   observed wrong result.)
2. **Checks are sampled, not exact**: two points, not the rational map.
3. **Records are not self-certifying.** volcano-rho `results.json` stores
   only `{ell, kernel_x}` per edge and a count `edges_verified`;
   `research/isogeny-volcano-cost-v2_20261017/classes/*/curves.jsonl`
   stores only `parent`, and in every ℓ=2 class 4 sampled vertices' parents
   are absent from the file, so those edges cannot be re-derived from the
   record at all.
4. **Twist vs. isomorphism** of a recorded codomain was not distinguished
   by any re-check (only j is compared in the walk).
5. ℓ > 3 kernels (kernel polynomial h | ψ_ℓ, Kohel) have no certifier.
   This is where torsion-aware division-polynomial machinery (the Stange
   lead) could matter; for ℓ ∈ {2,3} the plain ψ_ℓ check suffices, so the
   lead was **not** needed for the change below and its claimed closure of
   a gap is not assessed here.

## The change

`tools/velu_edge_certify.py` (stdlib only, shares no code with the Rust
producer) certifies an edge `(p,a,b) --(ℓ,x0)--> (p,a2,b2)` exactly:
`ψ_ℓ(x0) = 0` (and not 2-torsion for ℓ=3); the Vélu map satisfies
`Y² = X³ + AX + B` as a polynomial identity in F_p[x] after clearing
`(x−x0)^6` and substituting `y²`; x-map degree = ℓ; recorded codomain is
F_p-isomorphic (`a2 = w²A, b2 = w³B`, w a square), twists rejected. Modes:
`--results-json` (volcano-rho runs, uses recorded kernel_x) and
`--curves-jsonl --ell L` (searches the parent's rational ψ_ℓ roots).

Tests: `tools/test_velu_edge_certify.py` — brute-force cross-check on all
rational 2-/3-kernels of small curves over F_31, F_101, F_103 (every point
image on E', #E = #E'); fast root finder vs brute force; existing records
(4 volcano-rho runs: 79, 322, 29, 29 edges, equal to recorded
`edges_verified`; volcano-cost-v2 classes l2/l3 s1001–s1003); negative
controls: non-kernel x0, wrong ℓ, quadratic twist, tampered b2, altered
kernel_x on a real record.

    python3 -m pytest -q tools/test_velu_edge_certify.py   # 827 passed

CLI over all 28 cost-v2 classes: 0 failures; ℓ=2 classes each report
`parent_absent: 4`.
