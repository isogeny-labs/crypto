#set page(paper: "a4", margin: 2cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Representation leads for binary point decomposition

Campaign `research/representation_leads_20261109/`, 2026-11-09. Exact derivations in native
Rust (`derive/`), no timings, no experiment runs. *Every number is an unverified assertion:* no
pcc claim was minted, and the `research-kb` server was unreachable, so novelty rests on the lead
atlas `--match` and one literature search.

== Why these representations

An atlas finding on ECC2K-130 (KN-FIND-aa2efc, unreviewed) gives the product law relations
× targets × attempts = m·2#super[131]. With free attempts the floor is 2#super[68.58] at m = 3
(above rho's 2#super[60.81]), 2#super[56.40] at m = 4, 2#super[48.44] at m = 5 and 2#super[42.85]
at m = 6. A representation therefore matters only if it makes one attempt fit roughly
2#super[12] (m = 5) to 2#super[18] (m = 6) rho-step equivalents: the leads aim at m ≥ 5.

== R1′: quadratised chains (line norm form)

For a merge P#sub[a] + P#sub[b] = U on $y^2 + x y = x^3 + a x^2 + b$, the line through
P#sub[a], P#sub[b] and −U gives $e_1 = lambda^2 + lambda + a$, $e_2 = mu$, $e_3 = mu^2 + b$
(unit-tested on 1000 random pairs). Keeping $mu = e_2$ replaces the cubic
$S_3(x_a, x_b, x_U)$ by two quadratics: $x_a x_b + x_U (x_a + x_b) = mu$ and
$mu x_U + x_U^2 (x_a + x_b) + mu^2 + b = 0$. The descended chain then has degree 2 at every m.

*Size (exact).* Against the confound control (w = x#sub[a]x#sub[b] on the same merges),
$mu = e_2$ has the same variable count and the same set of distinct monomials, and 30–40 % more
terms (figure below). On written size it is a relabelled generic degree reduction.

#figure(image("figures/sizes.svg", width: 100%),
  caption: [Exact sizes at n = 19, l = 5 (Koblitz a = 0, b = 1). All 288 built systems satisfy
  their planted relation; single-bit flips are detected in ≥ 31/32 cases per system.])

*The slope λ.* Over GF(2#super[n]), $lambda^2 + lambda = c$ is solvable exactly when Tr(c) = 0.
An exhaustive census of every root triple of $S_3(x_1, x_2, X) = 0$ (n = 11, 13; four curves
each) finds only two classes: all three abscissae on E with λ in the field, or all three on the
twist with none. Mixed triples never occur, and every sampled on-E triple lifts to a true sum.
In a chain the merges share abscissae and the last contains $x_R$ on E, so the trace equations
are *implied*; the atlas already records the trace linear equation (KR-IC-f5c584). λ is
eliminable (one linear equation per merge).

#figure(image("figures/census.svg", width: 85%),
  caption: [Exhaustive census of S#sub[3] root triples (results/census.jsonl).])

*Prior art for λ.* The trace map behind this is the Lenstra–Kosters homomorphism
$E(k) -> FF_2$, $P |-> "Tr"((x(P) + a_2)/a_1^2)$ with kernel $2E(k)$ (2013), derived from the same
line identity $x_1 + x_2 + x_3 = lambda^2 + a_1 lambda + a_2$; it is the known explanation of the
first fall degree 2 of $S_3$ systems (Kosters, ECC 2015 slides; atlas KR-IC-f5c584). The census is
an exhaustive confirmation at n = 11, 13, not a new fact.

*Open question (gated).* Whether $mu = e_2$, or the explicit implied trace equations, lowers the
closure refutation degree relative to its matched control. `PROTOCOL.md` (frozen 2026-11-14) fixes
four arms in two matched pairs with identical variable sets, eight cells, 16 exhaustively
unsatisfiable and 4 planted draws per cell, and the instrument `examples/replead_closure.rs`
(the routines of `examples/dreg_closure.rs`). The 640 systems are pinned by sha256; running needs the
checkpoint token.

== R3: odd power-sum coordinates

In characteristic 2, $p_k = sum_i x_i^k$ is slot-separable on a subspace. Exact linear algebra
over GF(2#super[13]) and GF(2#super[17]), three (b, x#sub[R]) draws each: $p_1 S_3$ lies in
$F[p_1, p_3, p_5, ...]$, which is the generic pair-table form; $p_1^j S_4$ does not, for any
j ≤ 8, on all six draws. $S_4$ is in the elementary-symmetric span, and degree-12 positive controls
in three variables are recovered (j = 0), so the method sees such elements. Closed in scope.

== R2: membership as a Frobenius chain modulo N

$N = product (X - x_i)$ divides the subspace polynomial $L_V$ iff all roots lie in V; tracking
$R_k = X^(2^k) mod N$ with $R_k = R_(k-1)^2 mod N$ is bilinear, so membership is degree 2 for all
m at l·m·n auxiliary bits.

*Closed: dominated inside the Huang–Kosters–Yang–Yeo framework* (arXiv:1505.02532, Theorem 1.1,
read 2026-11-14). For a zero-dimensional radical big-field system $cal(F)$ in M variables with at
most s solutions over the closure, the last fall degree of its full Weil descent is at most
$max(tau(max(d_(cal(F)), deg cal(F), (M+1)s, 1), q, M), M dot tau(2s, q, 1), q)$ with
$tau(r, c, t) = max(floor(2t(c-1)(log_c(r/(2t)) + 1)), 0)$; their Section 4.4 says, without proof,
that the same should hold with added constraints of Frobenius weight at most $tau(2s, q, 1)$,
"such as linear subspace constraints". $L_V$ is linearized (weight 1), so the standard system
($S_(m+1)$ plus $L_V (x_i) = 0$, M = m) already sits in that scope; the squaring chain lowers only
the ordinary degree, which the bound does not use, and adds l·m big-field variables, which enter it
linearly. Scope: the subspace case is an unproved remark there, and $d_(cal(F))$ is bounded for
neither system.

== R4 and R5

R4 (uncorrelated slots for the n = 83, m = 10 gate) is cell G6 of PR \#2679 and waits for the
checkpoint token. R5 (mixed chains) is the R1′ construction itself: auxiliaries only on merges
whose S#sub[3] is cubic.

== Scope and limits

- Sizes are at toy n (11–19); no solving degree, timing or cost is measured.
- Census and power-sum results are exact on the stated fields and draws only.
- Nothing here changes a leaderboard, benchmark or validation row.
