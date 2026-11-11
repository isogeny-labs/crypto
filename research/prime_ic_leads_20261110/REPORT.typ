#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 14pt, weight: "bold")[Prime-field index calculus: the distance to rho in the lab's own observable, the equation budget every new lead must meet, and two cheap tests]
  #v(2pt)
  2026-11-10 · `research/prime_ic_leads_20261110/` · crypto `main` f14b66786b · desk study, derivations and a development check; no S, no cost ratio, no leaderboard row; every measurement here is `[pcc:unverified]`
]

= Answer

On the generic track parity with rho is impossible and the gap widens like $r^(1\/6)$; its value is measured constant gains. On the algebraic track the only exponent lane on record, the PKM tower family, is converging to closure: the last fall degree $d_F$ grows with the tower height at one degree per four levels (C-step holds at $t = 11, 12$; $t = 13$ reads $>= 6$) and with the arity at half a degree per unknown bit (stage C). Section 3 turns that growth into the distance to parity in the lab's own observable; section 4 names the mechanism as an _equation budget_: one cross-summand equation per relation over $F_p$, every further one costing about $log_2 p$ unknown bits, against one per bit for Weil descent over $F_(2^n)$. Every idea examined today fails that budget (section 5); the one family that passes it for free, the $[k]$-multiple consequences of the relation, is implemented as a presentation control with pre-stated predictions (section 6). Nothing here reaches parity and nothing on record can; the note states exactly what a new lead would have to supply.

= Where the prime-field program stands (measured, by path)

#table(
  columns: (auto, auto, auto),
  inset: 4pt,
  stroke: 0.4pt,
  [*item*], [*value*], [*source*],
  [Generic track, Table B, `icv1-fp10`…`fp24`], [IC / rho-negation 5.47× to 13.5×], [`docs/ic/LEADERBOARD.md` Table B],
  [20-bit factor base $N = 32 -> 45$], [gap 9.47× → 7.75× [6.92, 8.70]; second-host replay 1,008/1,008], [`research/prime_base_oracle_tuning_20261029/`],
  [Algebraic-track prime baseline], [empty (R1 Option C)], [`research/prime_ic_program_20261029/DECISIONS.md`],
  [`pkm-semaev` smoke, Curve25519 miniature $k = 20$, $t = 2$], [S = 1027 / 49,513 / 485,893 at $w$ = 0 / 0.1 / 1 vs 9.05 / 9.46 / 13.16 generic], [`problems/curve25519-family-algebraic-ic-tower.md`],
  [PKM Kummer $m = 2$, $P_1$, $d_F$ by $t$], [3, 4, 4, 4, 4, 5, 5, 5, 5, 6 ($t <= 10$); 6 ($t = 11, 12$); $>= 6$ ($t = 13, 14$)], [`research/pkm_falldegree_unblock_20261107/RESULTS.md`, `RESULTS_T12_BRACKET.md`],
  [Stage C, fixed $t$, growing $m$], [$t = 2$: 4, 5, 6, 6, $>= 7$; $t = 3$: 4, 6, 7, $>= 8$, $>= 8$ at $m = 2..6$; "grows in m" on all six ladders], [`RESULTS_STAGE_C.md`],
  [Exponent study, symmetrised subgroup base], [$m = 2$: $s = 2.77$ [2.31, 3.23] at $k = 20$; $m = 3, 4$ undetermined (censored)], [`research/curve25519_exponent_study_20261108/`],
  [Decisive cells], [$t = 14$ cap 7 queued; $m = 4$, $N = 20$ closure not launched (112 GiB, 49 h, Spot)], [`docs/session-logs/2026-11-10.md`],
)

= What parity requires (derived)

With base size $B$, arity $m$, per-target cost $T = B^s$ and a sparse solve $approx m B^2$, the total is about $r^(2\/(m+1-s))$, below $r^(1\/2)$ iff $s < m - 3$ (program note §2.4; audit `semaev_alternatives_audit_20261104`). A dense closure or Macaulay elimination reaching degree $d = alpha N$ in $N = m log_2 B$ one-bit unknowns meets $M(N, d) = sum_(k <= d) binom(N, k) approx 2^(N H(alpha))$ columns, so $s >= m H(alpha) omega'$ and the lane can reach parity only if $H(alpha) < (1 - 3\/m)\/omega'$:

#table(
  columns: 4, inset: 4pt, stroke: 0.4pt, align: right,
  [*m*], [*$alpha^*$, $omega' = 1$*], [*$alpha^*$, $omega' = 2$*], [*$alpha^*$, $omega' = 3$*],
  [4], [0.042], [0.017], [0.010],
  [5], [0.079], [0.031], [0.019],
  [6], [0.110], [0.042], [0.025],
  [8], [0.156], [0.056], [0.033],
  [10], [0.189], [0.066], [0.038],
)

Measured slopes: $1\/8 = 0.125$ per bit in $t$ (C-step, $m = 2$) and $0.5$ per bit in $m$ (stage C, $t = 2$ and $t = 3$). Both exceed every threshold at $omega' >= 2$; the $m$-slope exceeds every threshold at $omega' = 1$ too. Recorded `max_cols` = 16,572 at $t = 13$, cap 5, against $M(26, 5) = 83,682$: F4's matrices are about 5× sparser than the dense count, a constant that does not move $s$. This is a lower bound for dense degree-$d$ elimination and a model for F4; it assumes the slope persists beyond $N <= 28$, which the $m = 4$, $N = 20$ closure tests.

#figure(image("fig_slope.svg", width: 100%), caption: [Recorded $d_F$ ladders (lower medians of 2 draws; triangles are censored lower bounds) against $N$, with the C-step prediction and the $alpha^*$ parity slopes of section 3. Data: `research/pkm_falldegree_unblock_20261107` RESULTS tables; thresholds derived here.])

= The cross-summand equation budget (derived)

Weil descent over $F_(2^n)$: one relation gives $n$ equations of degree 2 in $m n' approx n$ bit unknowns. A PKM tower over $F_p$: $t = log_2 B$ one-bit unknowns per summand and *one* equation, with no Galois group to split it. A second equation from the same relation costs about $log_2 p$ unknown bits in every route: a fixed rational image $phi(P)$ in its own tower ($|V| dot |V'| = B p$), the extension $F_(p^k)$ (Gaudry cost $p^(2 - 2\/k)$), a $p$-adic digit (the torsion lift's second digit has degree about $N^2$), or a jet component. The only free family is $[k] R = sum [k] P_i$, which enters the Kummer quotient at degree about $2 log_2 k + 2$ (6 for $k = 2$). Hence a tower system is a one-generator system in a $2^N$-point quotient ring and its fall degree grows. The filter for any new lead: equations per unknown bit after the membership equations are removed; whether every extra equation brings a new $F_p$ unknown or shrinks the base; the degree at which consequence equations enter.

#figure(image("fig_budget.svg", width: 100%), caption: [The equation budget: Weil descent (balance), the PKM tower (one generator), any extra equation over $F_p$ (ratio $1 \/ log_2 p$).])

= Examined today and closed on paper

2-torsion ($T_0$) symmetrised tower on Montgomery-form curves: the 2-isogeny with kernel $(0, 0)$ has $x$-map $x |-> x + A + B\/x$, so a tower on the invariant is a plain tower on $E \/ ⟨T_0⟩$, same one-generator system (constant column gain only; Kummer unavailable on Curve25519, $v_2(p - 1) = 2$). Translate-pair and endomorphism-image bases: $|V| dot |V'| = p B$, the second equation costs $log_2 p$ bits. Wagner $k$-tree filtering on tower sets: needs $|F| >= sqrt(2p)$. $p$-adic second digit of the torsion lift: degree $N^2$, or a free $F_p$ unknown per summand. For $j = 0$, the $omega$-conjugate equation is the original up to relabelling ($S_3(zeta x) = zeta S_3(x)$). Atlas matches are listed in the README.

= The multiples presentation control

`examples/pkm_tower_pilot.rs --controls multiples` appends to the tower system the cleared $S_(m+1)(x([2]P_1), …, x([2]P_m), x([2]R)) = 0$, with $x([2]P) = N(x)\/D(x)$, $N = x^4 - 2 a x^2 - 8 b x + a^2$, $D = 4 (x^3 + a x + b)$ (for chains, doubled partial-sum unknowns linked by $D(u_k) u'_k - N(u_k) = 0$). Same cell seed, same tower, curve and targets as the tower control; every appended equation vanishes at planted solutions (asserted; 12/12 unit tests). It enters at multilinear degree 6, so at $m = 2$ the two presentations must agree wherever $d_F <= 5$ (P1, a built-in control); at cells with $d_F ("tower") >= 7$ the prediction was $d_F ("multiples") in {d_F - 1, d_F}$ with the slopes unchanged (P2). Falsifier: a drop of at least 2 at a cell with $d_F ("tower") >= 8$, or an $m$-ladder that reads flat.

*Development check* (this host, closure engine, Kummer at $P_1$, seed `0x504B4D54`, two random targets per cell, both arms on the same targets; no isolation; every value `[pcc:unverified]`; rows and table in `devcheck/`). The tower arm reproduces stage A's medians at $t = 2..9$ and stage C's cells, the instrument check.

#table(
  columns: 5, inset: 4pt, stroke: 0.4pt,
  [*cell*], [*$d_F$ tower*], [*$d_F$ multiples*], [*Δ*], [*falls tower / multiples*],
  [$m = 2$, $t = 2..9$ (16 rows)], [4, 4, 4, 4, 5, 5, 5, 5], [identical on every row], [0], [identical],
  [$m = 2$, $t = 10$], [> 6 (budget-censored)], [> 6 (same)], [—], [[4, 5] / [4, 5]],
  [$m = 3$, $t = 2$ ($N = 6$)], [5, 5], [5, 5], [0], [[4, 5] / [4, 5]],
  [$m = 3$, $t = 3$ ($N = 9$)], [6, 6], [5, 5], [*−1*], [[4, 5, 6] / [4, 5]],
  [$m = 4$, $t = 2$ ($N = 8$)], [6, 6], [5, 5], [*−1*], [[4, 5, 6] / [4, 5]],
  [$m = 4$, $t = 3$ ($N = 12$)], [7, 7], [6, 6], [*−1*], [[4, 5, 6, 7] / [4, 5, 6]],
  [$m = 5$, $t = 2$ ($N = 10$)], [6, 6], [6, 6], [0], [[4, 5, 6] / [4, 5, 6]],
  [$m = 5$, $t = 3$ ($N = 15$)], [> 7 both draws ($c = 8$ budget-censored)], [> 6 both draws ($c = 7$ censored by the entry cap at 399,850 columns)], [inconclusive], [[4, 5, 6, 7] / [4, 5, 6]],
)

P1 holds (identical at $m = 2$, draw by draw). P2 holds and nothing reaches the falsifier: at $m >= 3$ the drop is exactly one degree on six of ten completed cells and zero on four, and both slopes are unchanged ($m = 4$: 6 → 7 against 5 → 6 from $t = 2$ to 3; $t = 3$: 6 → 7 against 5 → 6 from $m = 3$ to 4). The one-degree offset at $m >= 3$ arrives with the degree-4 doubling links and the extra unknowns $u'_k$, a presentation shift, not a change in growth. The $m = 5$, $t = 3$ cell is censored on both arms (tower > 7, multiples > 6) and is inconclusive on this host. Section 4 item 3 stays closed as far as this check can test it; not tested: $[3]$- and $[4]$-multiples together, larger $N$, an F4 run on the augmented systems.

= Null-support slope (lead 3, development check, 2026-11-11)

Pre-registered before the rows existed. The `null` control replaces every summation equation by a random polynomial on a chosen support, with the same tower and targets as the `tower` arm: $S_3$'s own support, or the full exponent box $0..W$ (`--null-width`). Every value `[pcc:unverified]`; rows in `devcheck/`.

#table(
  columns: 4, inset: 4pt, stroke: 0.4pt,
  [*support*], [*$m = 2$, $t = 2..8$*], [*$m = 3$, $t = 2, 3$*], [*$m = 4$, $t = 2, 3$*],
  [tower ($S_3$, the curve)], [4, 4, 4, 4, 5, 5, 5], [5, 6], [6, 7],
  [null on $S_3$'s support], [4, 4, 4, 4, 5, 5, 5], [5, 6], [6, 7],
  [box $W = 1$], [3, 3, 4, 4, 4, 4, 5], [4, 4], [4, 5],
  [box $W = 2$], [4, 4, 4, 4, 5, 5, 5], [5, 6], [6, 7],
  [box $W = 3$], [5, 5, 5, 5, 5, 5, 5], [8, 8], [> 8],
  [box $W = 4$], [5, 5, 5, 5, 5, 5, 5], [> 8], [> 8],
)

Shape-driven, as predicted: a random polynomial on $S_3$'s support or on the $W = 2$ box has the tower's $d_F$ on all 40 completed cells, draw by draw, with the same fall sequences. The degree is monotone in the support width (at $m >= 3$ the $W >= 3$ boxes also raise the degree in the chain's free unknowns, so those rows are reported, not read). The curve contributes nothing the support does not: the growth is a property of the carry-graded one-generator closure, the mechanism of section 4, and the remaining step on this lead is the derivation of the $d_F (W, N)$ law, a computation without any curve.

= Leads, ranked

1. Finish the decisive PKM cells: $t = 14$ cap 7; the $m = 4$, $N = 20$ closure (Amendment 8 readings).
2. The multiples presentation probe: done (section 6): P1 and P2 hold, the falsifier is not reached, one cell inconclusive.
3. The null-support slope: measured (section above): curve-independent on all 40 cells; what remains is the derivation of $d_F (W, N)$.
4. The first counted algebraic-track row: specification written (`algebraic_row_spec/`: 3 Table B curves × 8 targets × 14 arms on the isogeny base, protocol draft validated, derived 46 vCPU-hour ceiling inside the approved budget); the owner decides the submission route; it needs an EKS identity and an image rebuild.
5. Generic-track constants: the prefetch Design A patch is obsolete (reconciled 2026-11-11: `layout=lines` is already on `main` and measured 12 % slower at fp28 in the v6 tournament, `invalid_measurement`); what stays open is the software-prefetch step on the landed layout and $N > 45$ at 20 bits: the only place a measured improvement over the current baseline is available.
6. Honest non-uniform accounting ($S T^2$ per row; preprocessing-rho reference).

What would change the list: an equation source that passes the filter of section 4. None is known. On 2026-11-11 the jet and $Z\/p^k$ exits were derived and measured against the filter (`jet_exit/`): dual numbers add only free $F_p$ unknowns, and the torsion section's $p$-adic digits are generic on Kummer bases (popcount degree about $t\/2$, equal to random, on nine cells at $t = 6, 8, 10$), so all three bigger-ring exits are closed at toy scale.

= Sources checked

Knowledge base unavailable (`research-kb` MCP absent; `scripts/research-kb status`: no executable); fallback: the lead atlas (`crypto-autoresearcher`, 176 leads, 30 in `ic-prime/`, `--match` run per idea) and the repository files listed in README §0. Problem record: `problems/prime-field-cross-summand-equation-budget.md`.
