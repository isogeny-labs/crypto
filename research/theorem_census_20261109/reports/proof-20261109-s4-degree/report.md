---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# S4: generic degree four, specialized degree can drop

Lean checks the canonical S4 Sylvester resultant as an actual polynomial
in each input position, over every field and in every characteristic.
When the other three inputs remain independent formal variables, the
degree in the chosen input is 4. After specializing those other inputs,
the degree can drop. This preserves the qualified generic degree statement
and refutes its unrestricted specialization strengthening. [pcc:unverified]

Let $T$ occupy any one of the four input positions, with the other three
coordinates denoted $x,y,z$. The checked statements are

$$\deg_T S_4\le4,\qquad [T^4]S_4=S_3(x,y,z)^2,$$
$$\deg_T S_4=4\quad\Longleftrightarrow\quad S_3(x,y,z)\ne0.$$

The definitions compute the actual nested-polynomial Sylvester determinant
over the coefficient ring. Explicit map/evaluation lemmas connect every
input position to the previously checked scalar S4. Polynomial equality
is not inferred from finite-field pointwise agreement. [pcc:unverified]

![Canonical determinant, generic lower bound, and specialization control.](proof.pdf){width=100%}

Each of the four symbolic presentations uses $K[x_1][x_2][x_3][T]$, with $T$
in the selected source input position. The formal S3 is nonzero because
its coefficient of the third indeterminate squared is $(x_1-x_2)^2\ne0$.
Pair swaps and the explicit symmetric S3 identity give the same leading
coefficient criterion in every position. [pcc:unverified]

\newpage

# A smooth-curve specialization control

On the actual nonsingular curve $y^2=x^3-x$ over $\mathbf F_5$, Lean proves
the following canonical identity. [pcc:unverified]

$$S_4(0,1,4,T)=1,\qquad \deg_T S_4(0,1,4,T)=0.$$

This is equality of canonical polynomials, not merely equality of their
values on the five-element field. It disproves the assertion that fixing
other inputs always preserves degree 4. It does not refute Semaev's generic
individual-variable degree theorem. [pcc:unverified]

| Situation | Checked conclusion |
|---|---|
| Three other coordinates formal and independent | Degree4 in each chosen position |
| Three coordinates specialized with S3 nonzero | Degree4 |
| Three coordinates specialized with S3 zero | Degree below4 |
| Smooth F5 model, last-variable triple(0,1,4) | Nonzero constant 1, degree 0 |

Source: ../../lean/S4Degree.lean. Main bindings include
`positionPolynomial_curve_eval`, `positionPolynomial_coeff_four`,
`positionPolynomial_curve_degree_iff`, `all_four_symbolic_degrees`,
`actual_specialization_constant`, and `s4_degree_audit_certificate`.
[pcc:unverified]

Whole census axiom gate: 259 declarations, 0 failing, 0 trusted-native exceptions;
the count includes generated declarations, not 259 new results. Packet
PCC-20261109-b93977 pins the explicit combined theorem. Its full local check
passes17:28:09Z; standing remains unsigned.
No custom axiom, proof hole or native_decide exception is accepted.
[pcc:unverified]

Supplement IDC1h9d53585cac843775 checks only the scalar leading-term rewrite
$\operatorname{Res}_{2,2}(aX^2+bX+c,(X-z)^2)=(az^2+bz+c)^2$:
32 derived and11 extra points over $2^{61}-1$,4 variables,degree bound 6.
Local source commit 3d1dd96b preceded issue, and the proof-file digest matches.
These one-host finite checks do not establish coefficient extraction,
generic degrees, other characteristics or independent/signed standing;
those universal mathematical conclusions rest on Lean. [pcc:unverified]

Primary source: [Semaev2015](https://arxiv.org/pdf/1504.01175), Section 2,
pp2--3; PDF p3 visually checked for $2^{m-2}$ and algebraic-closure coordinates.
[Kosters--Yeo2015](https://arxiv.org/pdf/1503.08001), Section 2/p3 supplies
the general invariant formula/recursion. Exact read scopes and hashes:
../../s4-degree-sources.json. Original Semaev 2004 remains unread after
403/404 responses. These are not full-paper or all-fourteen audits.
[pcc:unverified]

Scope is $m=4$. General all-$m$ recursion/degree, irreducibility and the
remaining requested obligations stay open. No solving-degree, regularity
or algorithm-complexity conclusion follows from this polynomial degree.
No new native curve population, benchmark, frozen-data change or paid
compute was run. All 11 standalone attempt logs and the final gate are retained.
[pcc:unverified]
