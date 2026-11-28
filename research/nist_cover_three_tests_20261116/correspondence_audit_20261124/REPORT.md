---
title: "P-192: separate correspondence and elliptic norm replay"
date: "2026-11-24"
geometry: margin=0.65in
fontsize: 10pt
header-includes: |
  \makeatletter
  \renewcommand{\maketitle}{\begin{center}\Large\@title\par\small\@date\end{center}\vspace{0.5em}}
  \makeatother
---

The separate native backend now **reconstructs both correspondence directions
and the elliptic norm** for the frozen six-fixture P-192 panel. All five
nonidentity recorded quartic cycles bind to the actual forward tensor algebra;
their independently reconstructed transpose traces, after reference subtraction,
equal the triple pullbacks and recorded source duals. The computed norms equal
both the recorded norms and $[6]P$ for all six fixtures. Five corruption controls
reject. PCC-20261124-18d97f binds the code, inputs, native result and costs.
[pcc:unverified]

| Fixed replay obligation | Result |
|---|---:|
| Complete forward tensor-algebra bindings / power-basis rank | 5 / 5, rank 9 each |
| Reconstructed transpose/reference comparisons | 5 / 5 |
| Computed norm equals recorded norm and $[6]P$ | 6 / 6 |
| Raw degrees of the three reference fibers | 8, 8, 8 |
| Raw degree required on every generic closed-point fiber | 9 |
| Frozen mutation controls rejected | 5 / 5 |
| Build / replay / complete-wrapper exits | 0 / 0 / 0 |

![The new executable reconstructs each solid-arrow step. The exact correspondence and reference convention are retained inputs; their derivation and general projective qualification are separate.](chain.pdf){width=97%}

## Binding the recorded forward cycle

Let $D=(u,v)$ be the independently computed reduced pullback. Each nonzero
fixture has degree three. Evaluate every coefficient $(a+bY)/c$ of the retained
global $U(T),V(T)$ in $A=\mathbb F_p[S]/u$, with $Y=v(S)$, rejecting any
nonunit denominator. The monic cubic $U$ defines the rank-nine algebra
$B=A[T]/U(T)$. Put $x=T$, $y=V(T)$ and $z=x+\lambda y$.

For the recorded cycle $(s(z),x_0(z),y_0(z))$, the new code checks in $B$

$$s(z)=0,\qquad x_0(z)=x,\qquad y_0(z)=y,$$

and requires the nine prime-field coordinate vectors of
$1,z,\ldots,z^8$ to have rank nine. Thus the recorded length-nine support and
coordinates describe the complete image algebra; mere incidence on the quartic
cannot pass this test. The full quartic equation and coordinate identity are
also checked modulo the entire support, preserving multiplicity.
[pcc:unverified]

\clearpage

## Reconstructing the transpose and reference subtraction

The checker factors each target support over $\mathbb F_p$, replays its full
factor product and verifies every factor's irreducibility by Frobenius tests.
Over each residue field it substitutes the target coordinates in both graph
equations. Clearing their common denominator gives
$A_0+B_0Y=0$ and $A_1+B_1Y=0$ on $Y^2=h(S)$.
[pcc:unverified]

It computes the gcd of
$A_0B_1-A_1B_0$, $A_0^2-B_0^2h$ and $A_1^2-B_1^2h$, saturates the retained
denominator factors, recovers $Y$ by polynomial Bezout identities and checks
both graph equations and source incidence. Every resulting generic fiber has
the required finite degree nine. Cantor reduction and all Frobenius-conjugate
sums produce a prime-field source class. Repeated factors retain multiplicity.
[pcc:unverified]

The three reference points retain indices 225, 224 and 223. Their independently
reconstructed affine fibers each have degree eight, matching the qualified
reference convention. Let $R$ be the sum of their reduced source classes.
For each forward cycle $E$, the replay computes

$$\operatorname{Tr}(\Gamma^t(E))-\deg(u)R=[3]D.$$

This is an executed comparison of actual reconstructed classes. The earlier
audit only recomputed the expected right-hand side. The raw result retains
$R$ and every final source dual. [pcc:unverified]

| Fixture | Target closed-point factor degrees |
|---|---|
| $O$ | Explicit identity |
| $G$ | 1, 2, 2, 4 |
| $-G$ | 1, 2, 6 |
| $2G$ | 1, 2, 2, 4 |
| $Q$ | 1, 1, 2, 2, 3 |
| $-2G$ | 1, 2, 6 |

For the norm, the new executable independently factors the reduced source
support and evaluates the original cover map in each residue field. It adds
all elliptic Frobenius conjugates, checks prime-field descent and compares
with a freshly computed $[6]P$. Identity and the $-2G$ input whose old shift
has $P+2G=O$ are checked explicitly. No scalar recovery is used.
[pcc:unverified]

The five controls alter cycle y, replace a source dual by its valid negative,
change an elliptic norm by adding $G$, omit a nonzero reference summand and
alter a global graph coefficient. All reject. The reference control requires
an actual nonzero omitted contribution, so it cannot succeed by changing a
term that vanishes in this panel. [pcc:unverified]

\clearpage

## Cost, reproducibility and scope

| Local interval | Wall (s) | User (s) | System (s) |
|---|---:|---:|---:|
| Complete replay wrapper | 75.269 | 74.432 | 0.540 |
| Nested strict native build | 7.145 | 6.668 | 0.456 |
| Nested native replay | 68.070 | 67.748 | 0.049 |

Nested costs are included in the wrapper. The native result also records
setup, reference reconstruction, each complete fixture and controls as separate
wall-clock phases. These are additional diagnostics to the original local
relation run and preceding divisor audit; no speed comparison is claimed.
One original allowed CPU, 300-second build and 600-second replay caps, and
1GiB address-space limits were retained. No cap was reached or widened; no
scientific retry occurred. Peak RSS is unmeasured. [pcc:unverified]

```bash
study=research/nist_cover_three_tests_20261116/correspondence_audit_20261124
bash "$study/run.sh" evidence/replay-new-01
```

The [native result](evidence/attempt-v1/result.json), frozen inputs, complete
source snapshots, pre/post digests, compiler/OpenSSL versions, exits and clocks
are retained. Only the typed JSON parser and the preceding separate OpenSSL
arithmetic are reused; no original producer arithmetic is imported. Exact
path-bound claim/replay records stay loose. Compiled binaries are local-only,
with their hashes and build sources published.

H-T holds for this panel **under the retained affine/saturated correspondence
and reference convention**. Arbitrary projective charts, a general subgroup
identity, independent signed standing and original map-synthesis costs remain
outside this result. It does not change the relation collection's zero added
old-base rank. Earlier proof subjects and their false geometric-verification
flags remain immutable. [pcc:unverified]

Sources: exact parent complete/closed-transpose/transfer protocols and native
sources; the retained polarized-quotient argument; Galbraith's corrected
[chapter 10](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch10.pdf),
Theorem 10.3.14, equation 10.11 and Theorem 10.4.1. KB and Conductor remain
unavailable as recorded in [PROTOCOL.md](PROTOCOL.md). Standard divisor and
finite-algebra algorithms are used; no novelty or signed proof is claimed.
