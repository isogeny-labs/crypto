# Item 9 PROTOCOL: exact last fall degree of the Velu 2-isogeny tower vs the Kummer chain

Frozen 2026-11-14, after the Kummer parity gate (kummer_gate.log: passes) and BEFORE any Velu closure was run.
Everything below is [pcc:unverified] (assertion until a pcc claim is filed).

## Objects
- Engine: `tower_engine.py` (numpy research prototype, exact mod p; not landed Rust code). Normal forms = squarefree
  monomials in 2t variables (same column set as the Kummer chain). Generator = NF(S_3(x_{1,0}, x_{2,0}, target)).
  V_D = least subspace of span(M_<=D) containing g, closed under multiplication by each variable on weight < D.
- LFD(t) = first D with 1 in V_D. "1 not in V_D" is certified by a full sweep (`verify_bad = 0`: every low row times
  every variable reduces to 0 against the final basis). "1 in V_D" needs no certificate (a single element).
- Kummer rules: x_j^2 = x_{j+1}, x_{t-1}^2 = 1. Velu rules: (x_{j+1}-x_j)(x_j-xi_j) = tau_j, x_t = c, i.e.
  x_j^2 = x_j x_{j+1} - xi_j x_{j+1} + xi_j x_j - tau_j (x_t := c).

## Validation gates (must pass before any Velu number is cited)
G1. Kummer parity on the N8 main instance (p 12289, a 4021, b 9917, target 4408): dim V_3(k) = 2k+1 (k=4..9),
    dim V_4(k) = 163, 386, 605, 829 at k = 4..7 (also 1061, 1301 at k = 8, 9 from N8-growth.md 6.2).
G2. Velu rewrite rules: for every variable j, every squarefree monomial m and every point v of V (all 2^t points), the
    evaluation of the reduced product at (x_0..x_{t-1})(v) equals x_j(v) * m(v) (exact).
G3. Tower build checks: Q has order exactly 2^t; 2R' not in <Q>; |V| = 2^t distinct; the chain x_{j+1} = x_j + tau_j/(x_j - xi_j)
    maps every v in V to the same c with x_j != xi_j; images of Q_j, R'_j lie on the codomain curves.

## Cells and instance rules
- Prime: p = smallest prime >= 2^20 with p = 3 mod 4 (so v_2(p-1) = 1, the P-192 shape). One prime for all cells.
- t = 4, 5, 6, 7, 8; two instances per t (instance index i = 1, 2). RNG: Python `random.Random(1000*t + 10*i)`;
  draws are consumed in this order and the first acceptable one is kept; every rejection is counted.
- Auxiliary curve E'/F_p: random (a', b'), nonsingular, #E'(F_p) (naive Legendre count) with 2^t | #E'; a point Q of
  order exactly 2^t (cofactor clearing, then 2-power descent); R' random point with 2R' not in <Q>.
  Kernel points K_j = 2^{t-1-j} Q_j, xi_j = x(K_j), tau_j = 3 xi_j^2 + a'_j, Q_{j+1} = Velu image (y' = y(1 - tau/(x-xi)^2)),
  a'_{j+1} = a'_j - 5 tau_j, b'_{j+1} = b'_j - 7 xi_j tau_j; c = x(R'_t). V = x(R' + <Q>).
- Target curve E: random (a, b) nonsingular over the same p; target x-coordinate drawn until x^3+ax+b is a nonzero square
  (a rational target point). UNIT instance: S_3(v1, v2, target) != 0 for all (v1, v2) in V x V (enumerated, 4^t pairs).
  Draw (a, b, target) until unit.
- Kummer reference: the same (p, a, b, target) with Kummer rules at the same t (its own unit status is taken from the
  closure itself; if 1 never enters V_D within the budget the cell is reported as such).
- D schedule: D = 4, 5, 6, ... (D = 2, 3 are vacuous for Velu, see Derived). For each D: run with early exit if 1 enters V;
  stop at the first D with 1 in V (that is the LFD). A "no" at D is certified (verify_bad = 0) before moving to D+1.
- Budget: a single closure expected over 40 minutes is not started (t = 8 at D = 6, 14,893 columns, is the one at risk);
  it is reported as "LFD >= D+1 at t, D+1 not run" with the exact next command.

## Derived before data (not predictions about data)
- The Velu generator NF(x1^2 x2^2) has leading part x_{1,0} x_{1,1} x_{2,0} x_{2,1} (weight 4) while Kummer's has weight 2.
  Hence V_D (Velu) with D = 4 is span{g} (g itself has weight 4 = D, so it is not multiplied), dim 1, 1 not in V_4.
  So LFD_velu(t) >= 5 for every t; the Kummer values (4, 4, 5, 5, 5) are matched by offset, not by value.

## Predictions (frozen)
Kummer law on the same engine: LFD_K(t) = floor((t+14)/4) = 4, 4, 5, 5, 5 at t = 4..8 (one step, between t = 5 and 6).
Let L_V(t) be the Velu LFD (if the two instances disagree both are reported and the larger is used for the reading).
- H_same: L_V has exactly one step in t = 4..8 and it is between t = 5 and t = 6 (offsets allowed).
- H_flat: L_V has no step in t = 4..8, or exactly one step and it is at t = 7 or t = 8 (plateau longer).
- H_steep: L_V steps at t <= 5, or has two or more steps in t = 4..8 (including a step at t = 6 plus another).
- "other": non-monotone in t, or an incomplete column.
Registered probabilities: H_flat 0.50, H_same 0.20, H_steep 0.20, other 0.10.
Reason: the F4 data of the encyclopedia section 0.2 show isogeny D = 5 flat at N = 4..18 while Kummer steps 4 -> 5; with the
derived bound LFD >= 5 this suggests L_V = 5 flat (an upper bound only under the F4 containment, which is not re-checked here).
Point prediction under H_flat: L_V = 5 at t = 4..8. Secondary: dim V_4 = 1 at all t (derived); V_5 profile is not predicted.

## Falsifiers
- G1, G2 or G3 failing means no Velu LFD is reported.
- H_flat is falsified by any step at t <= 6 in L_V; H_same by a plateau through t = 8 or a step elsewhere; H_steep by L_V <= 5 flat.
- A cell where the two instances give different L_V is reported as such and is not averaged.

## Not claimed
Relabelling, leaderboard or benchmarks rows; any statement about p ~ 2^192 or about solving cost. Only the closure LFD
at the listed cells.

## Amendment 2026-11-14 (after the t = 4..8 data; exploratory, not part of the frozen cells)
All Velu cells t = 4..7 (and t = 8 instance 1) returned L_V = 5 = the derived floor. The frozen cells therefore cannot
separate "pinned at the floor" from "flat". One extension cell is added, post hoc and labelled exploratory: Velu, t = 9,
instance 1 (same RNG rule), D = 5 only with early exit (12,616 columns, about 20-30 minutes). Reading rule: 1 in V_5 repeats
the plateau; 1 not in V_5 (certified) is the first Velu step. No Kummer arm is run at t = 9 (law: 5; V_4(9) = 1301 gate value).
t = 10 (21,700 columns, dense basis about 3.8 GB) is not run: out of the host memory and time budget.
