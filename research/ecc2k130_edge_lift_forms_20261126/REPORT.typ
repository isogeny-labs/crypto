#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#set par(justify: true)

= ECC2K-130 low-solving-degree forms: support-aware edge lifts

2026-11-26. Public challenge curve $y^2 + x y = x^3 + 1$ over $"GF"(2^131)$.
This report presents an exact matched formulation census and a next experiment;
it does not report a solving-degree reduction or attack improvement. All claims
are unsigned and unverified.

== Result

Prior repository quadratisation lifted only $w = x_a x_b$ in each cubic binary
$S_3$ link. Lifting either mixed edge, $w = x_a x_c$ or $w = x_b x_c$, keeps
the same Boolean-variable count, field-equation count and generator degree two,
but changes the descended monomial graph because leaves and intermediates have
different coordinate spaces.

An exact native census covered 72 matched planted cells: $n in {11,13,17,19}$,
$a in {0,1}$, $b=1$, $m in {3,4,5}$ and $ell in {3,4,5}$. All 216 systems
have degree two and satisfy their planted relation.

#table(
  columns: (1.0fr, 2.1fr, 2.1fr, 1fr),
  inset: 5pt,
  align: (left, center, center, center),
  [*arm*], [*distinct monomials / ab*], [*term occurrences / ab*], [*lower support*],
  [lift ac], [0.479 / 0.677 / 0.814], [0.701 / 0.965 / 1.164], [72/72],
  [lift bc], [0.479 / 0.847 / 0.938], [0.884 / 0.997 / 1.164], [72/72],
)

Each triple is minimum / upper median / maximum. The $a c$ arm has fewer term
occurrences in 47/72 cells and $b c$ in 38/72. Raw SHA-256:
`9af7c84b4767330fbfc4c2c89c9567862b3e49f29fd0207bc12e27245f113cd7`.
These are support statistics, not closure or solving degree.

#figure(
  image("figures/support-ratios.svg", width: 94%),
  caption: [Matched support ratios over all 72 cells. Bars show the observed range and dots the upper median; `ab` is 1.00.],
)

#figure(
  image("figures/edge-lift-map.svg", width: 84%),
  caption: [The matched presentation family and its next falsification gate.],
)

== Exact family

For any lifted edge $i j$ with remaining vertex $k$, introduce $w_(i j)$:

$ Q_(i j) = w_(i j) + x_i x_j = 0, $

$ F_(i j) = w_(i j)^2 + (x_i x_k)^2 + (x_j x_k)^2 + w_(i j) x_k + b = 0. $

Squaring is $F_2$-linear after Weil descent, so both equations have Boolean
degree at most two. Substitution recovers the original characteristic-two
link. The three forms are symmetric over unrestricted fields but not after
descent: $x_b$ is a new $ell$-bit leaf, $x_a$ becomes a full-field running sum,
and $x_c$ is normally a full-field output. The observed support difference is
therefore a domain effect, not a lower generator degree.

The substituted identities are sealed as `IDC1h02b24a926e268f4d` (ac) and
`IDC1h943d3d2cc5f015d2` (bc), 32 derived points each. They certify the displayed
integer-polynomial substitutions, not solving degree or usefulness.

== Candidate ledger

*C1: mixed-edge `ac` left chain.* Advance to the matched closure scan. It is
the only new arm with lower distinct support in every toy cell and the largest
median reduction. Hypothesis: lower support reduces closure fill or refutation
degree relative to `ab`.

*C2: support-aware tree and edge schedule.* For $m >= 4$, enumerate merge trees
and `{ab,ac,bc}` choices, scoring the union of descended monomials. Advance only
if the selected form reduces measured closure work, not merely source terms.

*C3: sum-exposed mixed lift.* At a leaf pair, use $u=x_1$, $s=x_1+x_2$ and
$z=u e$:

$ z + u e = 0, $

$ u^4 + u^2 s^2 + s^2 e^2 + u^2 e + s z + b = 0. $

This is quadratic and exposes the pair-sum coordinate that helped earlier
invariant guessing. Since it is an invertible leaf-coordinate change plus an
auxiliary choice, it receives no regularity credit without a matched result.

*C4: half-trace reciprocal quotient.* For $s=x+y$, $p=x y$, $h=e^(-1)$ and
$t=p/e$, the nonzero chart gives $t^2+t=s^2+b h^2$. Half trace is already known
and is not novel. Exact pair-image membership is the obstacle: reducing the
linearised subspace polynomial modulo $T^2+s T+p$ yields
$A_(j+1)=s A_j^2$ and $B_(j+1)=p A_j^2+B_j^2$, growing ordinary degree unless
a large auxiliary ladder is restored. Retain only as a boundary test.

== Prior-work filters

The round screened the existing chained and direct summation-polynomial work,
line-norm quadratisation, pair invariants, half trace, Riemann--Roch/Nagao
encodings, irreducibility, high-arity and Frobenius-slot records. Adding
$p=x_1x_2$ without replacing leaves had no lower measured degree at $(22,6)$
and 5.1--5.5 times the degree-four columns. Direct/symmetrised higher $S_m$
does not lower generator degree. Riemann--Roch's exact search remained the same
order as pair enumeration. Frobenius decorrelation is an existing orthogonal
arm, not a new form.

== Next decision

The frozen protocol compares cubic chain, `quad_ab`, `quad_ac`, `quad_bc` and
line-norm arms on the inherited exact draws. A degree effect requires `quad_ac`
to have smaller closure refutation degree on at least 12 of 16 unsatisfiable
draws in every $m>=4$ cell, with no reverse cell. Equal degree with fewer built
nonzeros is support-only evidence. Any false refutation, failed planted
assignment, unmatched size or system-digest mismatch voids a cell.

The closure run was not launched: checkpoint upload was unavailable. No
ECC2K-130 cost row moves until a form also preserves natural relation yield and
wins on cold total cost in compatible units.

== Sources and limits

Grounding used the repository's ECC2K-130 status/degree notes, representation
census, irreducibility audit and Riemann--Roch panel, plus Semaev (2004),
Petit--Quisquater (2012), Huang--Kosters--Yeo (2015), and Karabina (2015).
The research-KB executable/service was unavailable; novelty is a direct-source
search result, not a proof. The v1 identity records check the small-coefficient
substituted integer identities over an odd prime; native planted evaluation
separately checks the characteristic-two systems. Neither proves a closure win.
