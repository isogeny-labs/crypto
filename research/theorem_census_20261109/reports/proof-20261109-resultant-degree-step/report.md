---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# General resultant degree step; canonical S5 application

Lean proves an arbitrary-size coefficient and degree theorem for the
actual fixed-size Sylvester determinant. Let $f\in R[X]$, with $R$ a
commutative ring and $\deg f\le D$. With two fresh independent variables
$Z,T$, define the step by the actual resultant. [pcc:unverified]

$$F(Z,T)=\operatorname{Res}_{D,2}\bigl(f(X),S_3(X,Z,T)\bigr).$$
$$[T^{2D}]F=f(Z)^2,\qquad\deg_T F\le2D.$$

If $R$ is a domain and $f\ne0$, then $\deg_T F=2D$. The first Sylvester size
is the declared $D$, retained even if specializing $f$ lowers its actual
degree. These conclusions hold in every characteristic, without finite-field
interpolation or an assumed generic determinant formula. [pcc:unverified]

The proof bounds each determinant column separately. Its top coefficient
matrix is exactly the Sylvester matrix of $f$ and $(X-Z)^2$. Mathlib's
actual monic-square resultant lemma gives $f(Z)^2$; Lean discharges the
even sign explicitly. [pcc:unverified]

![Proof dependencies and the reset-of-size counterexample.](proof.pdf){width=100%}

For the canonical S5 application, the incoming canonical S4 has degree 4
over a faithful coefficient ring with three independent formal variables.
Two further polynomial layers provide $Z,T$. Hence the actual canonical
S5 has last-input degree 8 over every field. This proves one S5 input;
it does not prove all five positions or the all-m family theorem.
[pcc:unverified]

\newpage

# A failed specialization shortcut and exact checks

Lean disproves changing the declared size after specializing the incoming
polynomial to 1. Over every domain, and for any invariant parameters,
the following actual resultants differ. [pcc:unverified]

$$\deg_T\operatorname{step}_{2}(1)=4,\qquad
\deg_T\operatorname{step}_{0}(1)=0.$$

This is a counterexample to resetting the operator's size, not to the
qualified generic degree theorem. The previously checked smooth F5
example $S_4(0,1,4,T)=1$ separately shows an actual curve specialization
that drops degree. [pcc:unverified]

Source: ../../lean/ResultantDegreeStep.lean. Bindings include
`resultantStep_coeff_top`, `resultantStep_natDegree`,
`resetting_declared_degree_changes_result`, `symbolicS5_last_natDegree`,
and `resultant_degree_step_audit_certificate`. The whole tracked census
gate passes 291 declarations, 0 failing and 0 trusted-native exceptions;
counts include generated declarations. Only propext, Classical.choice
and Quot.sound are allowed. [pcc:unverified]

Local source commit ddc05a59 preceded the packet and native supplement.
Packet PCC-20261109-621d56 pins the combined theorem. Its check receipt
is ../../evidence/pcc-resultant-degree-step-check.json; standing remains
unsigned until a separate signed verdict. [pcc:unverified]

Supplement IDC1h7dd754d70e343323 samples only this scalar identity for
fixed $D=4$, using the actual six-by-six determinant AST. [pcc:unverified]

$$\operatorname{Res}_{4,2}\left(\sum_{i=0}^4a_iX^i,(X-z)^2\right)
=\left(\sum_{i=0}^4a_iz^i\right)^2.$$

The unchanged Rust checker accepts 32 derived points plus 11 extra points
over $2^{61}-1$, with 6 variables and degree bound 10. The source digest
matches. This finite supplement establishes no unboundedD, coefficient,
degree, other-characteristic or independent-host conclusion; the universal
mathematical conclusions rest on Lean. [pcc:unverified]

Primary sources: [Semaev2015](https://arxiv.org/pdf/1504.01175), §2/pp2--3,
and [Kosters--Yeo2015](https://arxiv.org/pdf/1503.08001), §2/p3. Exact scopes,
versions and hashes are in ../../resultant-degree-step-sources.json.
Original Semaev2004 remains unread after 403/404 responses. These related
sections do not constitute full-paper or all-fourteen audits. [pcc:unverified]

All 12 standalone build attempts are retained: 3 component/unbundled passes
and 9 failed intermediate attempts; the final bundled tracked gate passes.
No proof hole, custom axiom or native trust exception was accepted.
The general all-m canonical family, every-input degrees, symmetry,
irreducibility and remaining original obligations stay open. No new curve
population, statistical study, benchmark, frozen-data change, paid compute
or separate-host run was made. Existing performance/curve scoreboard graphs
were checked for relevance and retain their prior values. [pcc:unverified]
