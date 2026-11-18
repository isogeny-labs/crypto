---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# General S3: every characteristic, actual elliptic group

Lean proves the S3 zero-sum equivalence on the general Weierstrass model
$y^2+a_1xy+a_3y=x^3+a_2x^2+a_4x+a_6$, over an algebraically closed field
of any characteristic with nonzero discriminant. No distinct-coordinate
or normal-form assumption remains. All points and sums belong to Mathlib's
actual nonsingular elliptic group. [pcc:unverified]

For elementary symmetric coordinates $s_1=x+y+z$,
$s_2=xy+xz+yz$ and $s_3=xyz$, the checked invariant formula is

$$S_3=s_2^2-(4s_1+b_2)s_3-b_4s_2-b_6s_1-b_8.$$

Its equality to the source quadratic expression is checked in Lean.
[pcc:unverified]

![Actual proof dependencies. Earlier bounded native censuses are not premises. All repeated-coordinate and vertical cases are included. [pcc:unverified]](proof.pdf){width=92%}

The tangent uses $A=3x^2+2a_2x+a_4-a_1y$ and $D=2y+a_1x+a_3$.
The repeated-coordinate polynomial is
$A^2+a_1AD-D^2(a_2+2x+z)$. If $D=0$, nonsingularity forces $A\ne0$;
otherwise this factors through the actual doubled point. No division by 2
or 3 enters. Algebraic closure supplies actual quadratic y-coordinate
lifts for the final converse. [pcc:unverified]

Whole census project gate: 167 declarations, 0 failing, 0 trusted-native
exceptions. Full declaration/axiom receipt and failed-build diagnostics are
retained under ../../evidence/general-s3-*. The count includes generated
declarations; it is not 167 new results. Packets 4d5cd7/b32b34 remain unsigned.
[pcc:unverified]

\clearpage

# Geometric controls and source limits

| Actual smooth model | Prescribed triple | Wrong-model polynomial | Actual geometric result |
|---|---|---|---|
| F2: $y^2+y=x^3$ | (1, 0, 0) | Ordinary binary expression 0 | General S3 nonzero; no zero-sum triple |
| F3: $y^2=x^3+x^2+1$ | (0, 0, 0) | Short expression 0 | General S3 nonzero; no zero-sum triple |

These are actual algebraic-closure point nonexistence statements, not
merely missing rational-coordinate lifts. They refute dropping the model
hypotheses, not either qualified formula. The normal-form specializations
are themselves checked identities. Source:
../../lean/GeneralS3Controls.lean. [pcc:unverified]

Two parametric actual-group controls also compile: characteristic 2,
nonsingular model with $a_1=0$ has trivial 2-torsion; characteristic 3,
nonsingular short model has trivial 3-torsion. The latter follows by
reducing the S3 diagonal to $a_4^2\ne0$. These statements apply over
algebraically closed fields as well. They do not assume or prove the
source trace/Hasse/classification or general embedding-degree bridges.
[pcc:unverified]

The required rewrite certificate is IDC1h392e1740fcb7cf05. The unchanged
repository native checker accepts 32 derived and 11 extra points over
$2^{61}-1$; 8 variables, syntactic degree bound 5. The proof-file digest
matches GeneralS3.lean. The source commit preceded issue. This is a
one-host supplement; the all-characteristic/group conclusion rests on
Lean, not finite samples. No new curve census or statistical inference.
[pcc:unverified]

Primary review: [Kosters–Yeo 2015, arXiv1503.08001v3](https://arxiv.org/pdf/1503.08001),§2/p3,
general invariant formula/Proposition2.1; page visually checked.
PDF SHA256 prefix b5ea5e4c5de7. [Semaev 2015, arXiv1504.01175v1](https://arxiv.org/pdf/1504.01175),§2/pp2–3,
normal-form assumptions; PDF SHA256 prefix 4db8433bdd7a. Original Semaev 2004
ePrint still 403/unread. Neither the general higher-resultant theorem nor
the first-fall/complexity parts of these papers are marked audited/proved.
[pcc:unverified]

Exact hashes/read scopes: ../../general-s3-sources.json. Main actual-group
source: ../../lean/GeneralS3.lean, declaration
generalS3_zero_iff_exists_points. Signed/independent-host standing is not
asserted. Frozen earlier native sources/protocols/raw data remain unchanged;
user-approved local checkpoints and zero paid spend persist. General
resultant/irreducibility/degree proofs and the other 14-item obligations
remain open. [pcc:unverified]
