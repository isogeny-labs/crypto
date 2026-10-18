#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.")
#show link: underline

#align(center)[
  #text(size: 16pt, weight: "bold")[Non-generic ECDLP leads for prime, binary and Koblitz curves]
  #v(2pt)
  Search round, 2026-10-18 · study `research/ecdlp_leads_20261018` · repository `aburan28/crypto`
]

#block(fill: luma(240), inset: 8pt, radius: 4pt)[
  *Status.* Proposals and derivations only. Nothing in this report is a measurement, and no security
  claim about any deployed curve is made or withdrawn. Labels: _sourced_ (cited fact), _derived_
  (follows by the argument shown; prose, no `IDC1` certificate yet), _computed_ (shell arithmetic of a
  stated formula; data in `cover-genus-budget.csv`), _proposed_ (open). The Markdown `README.md`
  beside this PDF is the full text.
]

= Question and scope

The owner asked for 10 to 33 new leads for _non-generic_ ECDLP algorithms over prime-field,
binary and Koblitz curves. A lead is admitted only if it passes three tests. It is not already on the
repository's list. It names a mechanism that the Corrigan-Gibbs–Kogan bound for generic algorithms
does not cover. It has a first experiment and a kill criterion. Ten leads passed. Ten ideas were
closed on paper while drafting.

= Bottom line

+ *Prime fields: one real lever, and it is curve-specific.* On curves with small integer
  coefficients the summation polynomial $S_3$ has small integer coefficients. Its small-root lattice
  collapses from 6 unknown monomials to 3. With projective (Farey) factor bases the 3 unknowns are
  also balanced in size. The level-1 reach for $m = 2$ rises from $p^(1\/12)$ to $p^(1\/9)$ to
  $p^(1\/6)$ (derived). This applies to secp256k1, Curve25519, Curve448, BN254 and BLS12-381 G1. The
  NIST P-curves ($a = -3$) get $p^(1\/10)$. Random-coefficient curves get nothing. Since $m = 2$
  needs $1\/2$ and $m = 4$ needs about $1\/5$, this is an oracle improvement, not a threat.
+ *Koblitz curves: the cover route is not dead.* GHS is degenerate on Koblitz curves. Two facts
  reopen it. First, the Klein quartic over $FF_2$ has $L(X) = 1 + 5X^3 + 8X^6$, which equals
  $L(op("Res")_(FF_8\/FF_2) E_(a=1))$. Second, Eichler–Shimura applied to the CM newform of 49a1
  twisted by a character $chi$ of order $n$ with $chi(2) != 1$ gives a reduction isogenous to the
  trace-zero variety $T_n$, for every prime $n$. Class-group index calculus over $FF_2$ is
  subexponential. A covering curve of genus at most $c(n) dot n$ beats rho, where $c(131) approx 1.9$
  and $c(571) approx 6.1$ (computed).
+ *Random-b binary curves: only the cover question is new here.* A random-b curve does not lift to
  a Q-curve, so the Koblitz mechanisms do not transfer (derived sketch).

= The ten leads

#table(
  columns: (auto, auto, 1.9fr, auto, 2fr, 1.3fr),
  stroke: 0.4pt + luma(170),
  inset: 4pt,
  align: left,
  [*\#*], [*family*], [*lead*], [*status*], [*first experiment*], [*kill criterion*],
  [L8], [Koblitz], [Étale route: $D\/FF_2$ with an order-$n$ automorphism and an $FF_2$-map to $E$; its twist contains $E times T_n$, genus $1 + n(g_0 - 1)$], [proposed], [enumerate genus-2 and genus-3 $B\/FF_2$ covering $E$ with $n | \#J_B(FF_2)$; test $L(B, chi, X)$ for the factor $L_E(X)$], [no hit for $g_0 <= 3$],
  [L6], [Koblitz], [Modular twists: $(f_E ⊗ chi) mod 2 ∼ T_n$ for every $n$], [derived], [smallest curve containing $A_(f ⊗ chi)$ for $n = 3, 5, 7, 11$], [genus grows faster than $c(n) n$],
  [L10], [binary], [Genus-$n$ Jacobians realising $op("Res")(E)$, beyond GHS], [proposed], [all genus-3 curves over $FF_2$, which traces $t_3$; genus-5 search for point counts $3, 5, 9, 17, 33 - 5 t_5$], [no genus-5 realisation],
  [L9], [Koblitz and binary], [Certified lower bounds on the cover genus], [derived (partial)], [Howe's test on $T_n$; manypoints (32, 5); point-count positivity], [(produces bounds)],
  [L1], [prime], [Coefficient collapse plus Farey bases], [derived], [Probes B/C with the collapsed and Farey lattices, $m = 2, 3, 4$, against random-coefficient controls], [gain under $1.25 times$ at $m = 4$],
  [L2], [prime], [Eisenstein–Farey bases for $j = 0$], [derived], [same harness, secp256k1-shaped toys], [invariance lost under shifts],
  [L7], [Koblitz], [Klein family: $ZZ\/n$ covers of $E$ ramified at one place of degree $op("ord")_n (2)$], [derived ($n = 3$)], [$L(E, chi, X)$ for $n = 7, 31, 73, 127$], [no hit at 7 or 31],
  [L3], [prime], [Algebraic-exposure ledger, 176 prime curves], [proposed], [table of super-monomial count × model × shape], [none (audit)],
  [L4], [prime], [Audit four unchecked prime-field IC papers], [proposed], [native re-implementation under matched rho], [none below baseline],
  [L5], [prime], [Global twist-collision relations], [proposed], [squarefree twist classes of $f(x)$, $x <= B$; ranks], [yield below $B^0.6$],
)

#figure(image("lead-map.svg", width: 100%), caption: [The ten leads by family with status colours, and the closures C1–C7. A diagram, not data.])

= Key derivations

*L1, level-1 reach.* $S_3(x_1, x_2, x_R) equiv 0 mod p$ is a linear relation among unknown integer
polynomials in the small summands. Reach is the largest factor base for which the product of their
size bounds stays below $p$.

#table(
  columns: (2.6fr, 1fr, 1fr),
  stroke: 0.4pt + luma(170), inset: 4pt,
  [*curve class*], [*interval base*], [*Farey base*],
  [generic $a, b$ (5 unknowns plus a constant; Farey adds $s_1^2 s_2^2$)], [$1\/12$], [$1\/12$],
  [$a = -3$, random $b$ (P-256, P-384, P-521)], [$1\/10$], [$1\/10$],
  [small $a, b$ or small Montgomery $A$ (3 unknowns $c_2, c_1, c_0$)], [$1\/9$], [$1\/6$],
  [needed for decompositions to exist at $m = 2, 3, 4$], [$1\/2, 1\/3, 1\/4$], [],
)

For the collapsible class, $S_3 = c_2 x_R^2 - 2 c_1 x_R + c_0$ with $c_2 = (x_1 - x_2)^2$,
$c_1 = (x_1 + x_2) x_1 x_2 + 2b$ and $c_0 = (x_1 x_2)^2 - 4b(x_1 + x_2)$, all small integers. The
Montgomery form is analogous with $A$. With $x = r\/s$, each $c_k$ becomes bihomogeneous of bidegree
$(2, 2)$ and of size $H^4$. That gives $H^12 <= p$ and $|F| approx H^2 <= p^(1\/6)$.

*L6, Eichler–Shimura.* $f_E ⊗ chi$ has level $49 ell^2$, character $chi^2$ and
$a_2 = chi(2) t$. At $p = 2$, $"Frob"^2 - chi(2) t "Frob" + 2 chi(2)^2 = 0$, so
$"Frob" = chi(2) pi_E$. Over the conjugates these are exactly the eigenvalues of $T_n$. By Tate,
$A_(f ⊗ chi) mod 2 ∼ T_n$.

*L7, Klein.* The point counts are 3, 5, 24 over $FF_2, FF_4, FF_8$, so
$L_"Klein" = 1 + 5X^3 + 8X^6$. For the $a = 1$ Koblitz curve $t_3 = -5$, so
$L(op("Res")) = 1 - t_3 X^3 + 8X^6$. These are equal, so by Tate the two varieties are isogenous.

*L8, L9, closure C7.* Tame $ZZ\/n$ ramification over $FF_2$ needs branch places of degree at least
$op("ord")_n (2)$. Riemann–Hurwitz then leaves three options: quotient genus 0 or 1 (genus at least
about $op("ord")_n (2) (n - 1)\/2$), or étale over a quotient of genus $g_0 >= 2$ (genus
$1 + n(g_0 - 1)$, needing $n | \#J_B(FF_2)$). Genus exactly $n$ forces $n = 3$. If $J_C ∼ T_n$
then $\#C(FF_(2^k)) = 2^k + 1 + t_k$ for $n ∤ k$. For $a = 1$ that gives 4 points over
$FF_2$ but 2 over $FF_4$, which is impossible.

#figure(image("cover-genus-budget.svg", width: 100%), caption: [Derived, not measured. For each deployed Koblitz size: the L(1/2) genus budget $g_max (n)$ (Enge–Gaudry constant $sqrt(2)$, o(1) dropped), the étale-route genus $1 + n(g_0 - 1)$ at the smallest $g_0$ the Weil bound allows (optimistic), and the ramified Klein-family genus $1 + op("ord")_n (2) (n-1)\/2$. Data: `cover-genus-budget.csv`.])

= Closed on paper while drafting

#table(
  columns: (auto, 2fr, 3fr),
  stroke: 0.4pt + luma(170), inset: 4pt,
  [C1], [Farey bases on generic curves], [the constant becomes a sixth unknown; reach stays $1\/12$],
  [C2], [Special-prime quadratic orders (2#super[255] − 19, secp256k1, P-521)], [equal reach with the collapse, worse without it; no automorphism gain],
  [C3], [Linked expansions by $1 + tau$, $sqrt(-3)$, halving], [equivalent to re-randomising the target; towers raise the solving degree],
  [C4], [K-233 invariant subspaces (8 cosets of 29)], [linear algebra $approx 2^102$ is under rho $2^110.9$, but the per-solve budget $2^59$ is below a degree-3 Macaulay solve ($approx 2^73$)],
  [C5], [K-571 and B-571 with $m = 5$ chained systems], [needs solving degree at most 6 to 9 (the Semaev 2015 regime); the measured D growth contradicts it],
  [C6], [Oakley groups 3 and 4 by direct decomposition], [per-trial Gröbner cost dominates; GHS stays the better route],
  [C7], [Genus-$n$ covers with an $FF_2$-automorphism of order $n$], [only $n = 3$ (Klein)],
  [C8], [Norm-residue map $P |-> f_P$ on Koblitz trace-zero points], [trivial by the Hasse norm theorem],
  [C9], [Characteristic-2 Farey bases], [the descended degree rises to 4],
  [C10], [Characters, Legendre symbols, pairings modulo small $ell$], [these see only cofactors],
)

= Unresolved checks

- `identity.certificate/v1` records are owed for four claims: the L1/L2 counts, the Klein
  L-polynomial identity, the Eichler–Shimura twist, and the $a = 1$ obstruction.
- LMFDB queries for dimension 3, 5 and 6 classes were blocked by a captcha. The four classes that
  returned data were $T_3$ and $T_5$ for $a = 0$ and $a = 1$: all simple, none principally
  polarisable, none a Jacobian.
- The manypoints bound for $(q, g) = (32, 5)$ is unchecked. It decides whether $(a = 0, n = 5)$ can
  be realised at genus 5.
- The novelty audit against the cover-attack literature (Diem, Scholten, Hess, Ekedahl–Serre,
  Elkies) is owed before any of L6–L10 is called new.
- The L(1/2) budget is optimistic: it uses the hyperelliptic constant and drops o(1). General curves
  need slower class-group algorithms.
- Graphs checked and left unchanged: the IC scoreboard, the progress timeline, the IC leaderboard
  and the ECDLP attack matrix. Nothing in this round is measured or certified.
