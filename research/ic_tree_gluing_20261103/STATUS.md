# Status of the tree-gluing line (rounds 1–6), one page

2026-11-04 · consolidated from README.md and round2–6. Every result here is an
unverified assertion [pcc:unverified]. The round-5 statement claims are
PCC-20261104-3d6b6c, -785e08, -5d4597 and -e899a0. Rounds 1–4 and 6 have no
claim records.

## Question
Find a factor-base symmetry, or a solver, for prime-field ECDLP index calculus
(point decomposition, the PDP) whose F4 solving degree D stays small enough for
a subexponential, rho-beating relation search.

## What was measured
msolve 0.6.5, p = 786433, Kummer base g·μ_{2^t}, planted on-curve targets
[pcc:unverified].

| encoding | idea | outcome |
|---|---|---|
| T, the cherry tree (round 1) | pair the summands, encode each pair by its symmetric functions, glue the pairs by S₃; inputs of degree ≤ 4 at every m | beats the caterpillar chain in every completed cell; loses to full symmetrisation on cost at m ≤ 4 |
| S, separators fixed (round 2) | T with every partial sum given | **D = 4 in all 10 cells up to m = 6**: all excess degree comes from coupling the pairs |
| F, full symmetric (rounds 2 and 4) | S_{m+1} in e₁..e_m | input degree 2^{m−1}; lowest cost at m = 4, impossible to build beyond that |
| G, glued (round 3) | the separators eliminated by resultants | input degree 16; dominated, and still dominated under natural weights (round 4) |
| swap-adapted T (round 4) | Z/2-graded coordinates | same D, 3.7× wider matrices |
| H, character branch (round 5) | branch on whether X₁ is a square | no-op: over F̄_p every value is a square |
| L, fat leaf (round 5) | one symmetric triple, so one coupling | D ≥ 8 > 7; the node equation has degree 8 |
| Y, chord coupling (round 5) | the partial sum as a function, using y and the slope | same D at t = 4, steeper in t, 5–20× the cost |

**Coupling excess E = D_T − 4 at t = 4–5** [pcc:unverified]:
- m = 2, 3, 4: 0, 2, 3 (measured);
- m = 5: E ≥ 5 (cut off at 2,400 s at 13.5 GiB, then memory exhausted at 14.5 GiB, round 6);
- m = 6: E ≥ 6.

The depth slope is about +0.5 per level at m = 3.

## What was derived
All [pcc:unverified]:
- **Ritt dichotomy** (THEORY.md §1): the Kummer, Dickson and Lattès towers are
  Ritt's three families. Only Lattès towers are compatible with the group law,
  and they degenerate to IC on an isogenous curve.
- **Counting bound** (round5/SOLVERS.md §2a): every enumeration-type solver
  (SAT, meet-in-the-middle, dynamic programming, exact tensor networks) pays
  ≥ √p.
- **Kronecker solver**: intermediate degree ≳ p^{1−1/m}.
- **Cost law** (round4/CROSSOVER.md, round5/SOLVERS.md §1): dense or sparse
  Macaulay cost from the measured D laws.
  - It is subexponential asymptotically if D = O(m).
  - At 2^256 it costs at least 2^{156} even with the most optimistic laws,
    and 2^{298}–2^{399} with the measured depth slope.
  - The crossover with rho lies roughly between 2^{500} and 2^{6000}.

## Where it stands
- **Closed at cryptographic sizes** under the measured laws [pcc:unverified].
  No encoding, symmetry, branching or solver change tried in this line moves
  either slope: about +1.5 per summand and about +0.5 per level.
- **Open asymptotically:** whether E grows by at most 1 per summand (P2). The
  deciding cells, m5 t4 and m6 t5, need more than 15 GB; see round4/BIGHOST.md.
- **Not to be repeated as new:** factor-base symmetries of the separator type;
  tree, glued, fat-leaf and chord encodings; character branching of
  separators; sparse or faster linear algebra as the lever.
- **Remaining routes:**
  - a node relation of degree < 4, which x-only coordinates cannot give.
    Round 7 found that Edwards nodes alone do not lower D, but the 2-torsion
    quotient in the Edwards v-coordinate lowers the coupling excess by 1–2 at
    m = 3, 4 [pcc:unverified] (`round7/README.md`);
  - a memory-lean F4 to decide P2 (round 7);
  - directions outside summation polynomials (round 7).
