# CT23 — exact rational-product control for the proposed (7,7) Jacobian hop

**2026-11-11; [pcc:unverified].** This is a finite rational-group control for CT20–CT22, not an actual genus-2 balanced-divisor evaluator, geometric identification of HLP maps, full benchmark, or attack.

## Conditional rational-group transfer theorem

Write A_i = E_(1,i) x E_(2,i). Let P_i:A_i -> J_i=A_i/G_i be the HLP degree-four gluing quotient and N_i:J_i -> A_i its dual norm, with N_i P_i=[2]. The old and two supported target elliptic products have #A_i(F_101)=#J_i(F_101)=109*113=12317, odd. Every geometric gluing kernel G_i has order four; hence P_i is injective on the odd-order rational points. Equal finite cardinalities make P_i a bijection **on rational-point groups**, not an isomorphism of abelian varieties.

If the actual geometric HLP diagrams satisfy Phi P_0=P_1 alpha for alpha=phi_1 x phi_2, their rational-point action is forced:

    Phi_rat(D) = P_1 alpha ([6159] N_0(D)),
    2*6159 = 1 (mod 12317).

This formula is **conditional** on constructing and identifying P_i,N_i,Phi geometrically. The regression below models the finite rational-product coordinates, *not* the proposed map on Mumford divisors.

The factors E_1(F_101)=C_109 and E_2(F_101)=C_113 have coprime orders. Any **rational-group homomorphism** from one factor to the other is zero: a generator image must be killed by both 109 and 113, hence by 1. This does **not** assert that all geometric elliptic-curve homomorphisms vanish over extension fields. It does show that, on these rational groups, the E109 challenge cannot acquire a nonzero E113 coordinate from a group homomorphism.

## Independent executable control

From repository root:

    python3 -S research/chained_jacobian_transfers_20261110/ct23_rational_product_transport.py
    python3 -m py_compile research/chained_jacobian_transfers_20261110/ct23_rational_product_transport.py

Standalone short-Weierstrass arithmetic, derivative of the degree-seven rational x-map, dual normalization, generator-translate checks, full-group enumeration, and cold BSGS. Does not import the CT19–CT22 engines. Source curves (a,b)=(1,46),(44,21); targets B+A=(26,58)x(13,42), B+B=(26,58)x(69,50). It checks 109 and 113 rational source/codomain points, all 12,317 distinct product images per chart, complete component forward/dual [7] identities, full-order CRT projectors, all 109 known-answer scalars, and exhaustive zero-cross-factor candidate images. A deliberately swapped second-factor codomain must fail the equation.

**Local development output (producer self-check):**

    B+A product=12317, distinct images=12317, forward/dual[7],
        CRT=all points, DLP=109/109, BSGS direct/transport
        additions=1685 1685, map evaluations=109
    B+B product=12317, distinct images=12317, forward/dual[7],
        CRT=all points, DLP=109/109, BSGS direct/transport
        additions=1685 1685, map evaluations=109

The frozen cold BSGS counts across 109 known-answer logs match: 1,685 elliptic additions for direct E109 versus 1,685 for the transported E109, plus 109 degree-seven map evaluations for the transported arm. Those numbers exclude field-operation cost, P_i/N_i, genus-2 Phi, elliptic-map setup and strongest rho/IC references. They are **not a matched full-cost benchmark**. All scientific standing remains unsigned.

## Source review, limitations and acceptance criteria

Reviewed crypto PRs [#3877](https://github.com/isogeny-labs/crypto/pull/3877), [#3920](https://github.com/isogeny-labs/crypto/pull/3920), [#3921](https://github.com/isogeny-labs/crypto/pull/3921), [#3922](https://github.com/isogeny-labs/crypto/pull/3922) and [#3926](https://github.com/isogeny-labs/crypto/pull/3926), cryptanalysis [#1483](https://github.com/isogeny-labs/cryptanalysis/pull/1483), Howe–Leprévost–Poonen [Section 3.2](https://math.mit.edu/~poonen/papers/large.pdf), Bruin–Doerksen [general Richelot theory](https://arxiv.org/abs/0902.3480). Research-KB MCP and CLI are not exposed in this execution environment; encrypted atlas was not accessed. No global novelty claim.

Next: check actual geometric P_1/N_1/Phi and dual on balanced divisors, all 12,317 rational Jacobian classes and 109 scalars; obtain independent signed PCC verification, then compare complete same-host costs including map setup/evaluation, direct elliptic controls, strongest rho and established index-calculus baselines under approved EKS Spot/S3 admission and live budget. Never merge frozen parent history while outstanding ledger/archive/CI/review requirements remain unmet. No paid compute launched. [pcc:unverified]
