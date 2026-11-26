---
title: "P-192: the actual-map composition beyond the fixed panel"
date: "2026-11-26"
geometry: margin=0.65in
fontsize: 10pt
header-includes: |
  \makeatletter
  \renewcommand{\maketitle}{\begin{center}\Large\@title\par\small\@date\end{center}\vspace{0.5em}}
  \makeatother
---

Separate native arithmetic now checks an exact differential identity and a
degree bound for the actual retained maps. Together with the standard
Jacobian push/pull and separability theorems, these give

$$B\circ F=[6]:E\longrightarrow E,\qquad
F=\phi\pi^*,\quad B=f_*.$$

Here $\pi:H\to E$ is the degree-two P-192 cover, $\phi:J(H)\to J(C)$ is
induced by the retained cubic correspondence, and $f:C\to E$ is the actual
degree-six rational map. This is a homomorphism identity on the retained
smooth projective models, including geometric points over extensions. The
argument uses the actual equations and does not assume the original
polarized-quotient identity. Results have unsigned separate-arithmetic
standing. [pcc:unverified]

| Exact native obligation | Outcome |
|---|---|
| Actual $f$ equation, differential and degree certificate | Pass; degree 6 |
| Actual $\pi$ equation and differential | Pass; degree 2 |
| Generic quartic incidence and separability | Pass in the full cubic algebra |
| Differentiated quartic incidence | Exact function-field identity |
| Weighted correspondence trace | $3\pi^*\omega_E$ |
| Multiplication-matrix / Newton traces | Equal |
| Actual-equation bound on $\deg(BF)$ | At most 5184 |
| Bound on $\deg(BF-[6])$ | At most $6084<p$ |
| Source-infinity special fiber / software reference | All 3 points match |
| Changed-input controls | 5 composition + 3 reference rejections |

![The exact differential and actual-equation degree bound determine the composite endomorphism. The reference test binds the software subtraction convention to the geometric homomorphism.](composition.pdf){width=92%}

## The computed differential

Write $Y^2=h(S)$ on $H$ and let the actual graph be
$U(T)=0$, $W=V(T)$, where $U$ is monic cubic and $\deg V\le2$. The new
OpenSSL backend implements $\mathbb F_p(S,Y)$ and the full rank-three
algebra $\mathbb F_p(H)[T]/U$, checking every inverse by multiplication.
It checks $U_T$ is a unit and verifies $F_C(T,V)=0$ and its exact
derivative. No recorded differential matrix is substituted for the trace
calculation. [pcc:unverified]

For $\omega_E=dx/(2y)$, the exact sparse identity for the actual backmap
gives $f^*\omega_E=L(x,y)\,dx/F_{C,y}$. The checker computes

$$\operatorname{Tr}\left(
 \frac{L(T,V)}{F_{C,y}(T,V)}\frac{-U_S}{U_T}\right)
=3\frac{l(S)}{2Y},\qquad
l(S)=\frac{16S}{1024^2}+\frac{16S^2}{1024}.$$

The right side is three times the separately checked actual
$\pi^*\omega_E$. The trace is computed both from the diagonal of its
multiplication matrix and from the cubic Newton identities. The
[native result](evidence/attempt-v1/result.json) retains the full reduced
function coefficients and phase clocks. [pcc:unverified]

\clearpage

## Why the identity extends beyond the panel

On the retained smooth models, the graph's normalization defines a
correspondence and hence a Jacobian homomorphism $\phi$. The induced action
on regular differentials is the pullback followed by the field trace.
Therefore the checked weighted trace is
$\phi^*f^*\omega_E=3\pi^*\omega_E$. The differential of the Jacobian
pullback $\pi^*$ is the trace along $\pi$ under the usual identifications.
The trace of a pulled-back differential for this separable degree-two map
is twice the original differential. Consequently

$$(B\circ F)^*\omega_E=6\omega_E.$$

This alone leaves the possibility of a nonzero inseparable difference. The
following actual-equation bound removes it. [pcc:unverified]

Clear all four $U$ coefficient denominators with their exact polynomial
lcm $c(S)$, of degree 36. Write the resulting graph equation as
$A(S,T)+B(S,T)Y=0$. Its norm is

$$P(S,T)=A(S,T)^2-h(S)B(S,T)^2.$$

It is nonzero because its $T^6$ coefficient is $c(S)^2$. The native
degree bounds give

$$\deg_S P\le M=72.$$

Above a generic point of $C$, its $S$ coordinate satisfies this polynomial
and its $Y$ coordinate satisfies a quadratic. Thus the total projection
degree to $C$ of the nonconstant graph components is at most $2M=144$.
The total degree to $H$ is at most three. Constant-to-$C$ components induce
zero Jacobian maps. This deliberately loose bound avoids relying on the
producer's sharper projection-degree-nine assertion. [pcc:unverified]

For completeness, use the positive Rosati trace inner product on
$\operatorname{Hom}(E,J)_\mathbb R$. Pullback along a degree-$d$ curve map
has norm $\sqrt d$: its adjoint is pushforward and their composite is
$[d]$. Its adjoint has the same norm. A graph component of bidegree $(a,b)$
therefore induces a map of norm at most $\sqrt{ab}$. For several components,
Cauchy--Schwarz gives a total bound
$\sum\sqrt{a_i b_i}\le\sqrt{(\sum a_i)(\sum b_i)}$.
Applying these bounds to the actual chain yields

$$\deg(BF)\le 6\cdot3\cdot144\cdot2=5184.$$

On $\operatorname{End}(E)_\mathbb R$, $\sqrt{\deg}$ is the corresponding
norm (up to the fixed trace factor). The triangle inequality now gives

$$\deg(BF-[6])\le(\sqrt{5184}+6)^2=6084<p.$$

Put $\delta=BF-[6]$. Its differential is zero. A nonzero elliptic
endomorphism with zero differential is inseparable and has degree divisible
by $p$, hence at least $p$. The displayed bound rules that out, so
$\delta=0$. This conclusion is independent of how many public fixtures
were tested. [pcc:unverified]

The geometric ingredients here are the standard Jacobian trace/adjoint
identities, positive Rosati trace form, quadratic degree and separability
criterion. Smoothness of $H$ is rechecked; smoothness and the canonical
polarizations of the retained projective $C$ are existing model hypotheses.
This is a written geometric argument with exact native arithmetic support,
not a compiled proof or an independent signature. [pcc:unverified]

\clearpage

## Binding the software's actual reference divisor

The software sends a source divisor $P-\infty_H$ to
$E(P)-E_{\rm ref}$. Its reference consists of the three retained quartic
points indexed 225, 224 and 223. A separate native invocation computes the
complete specialization of the actual $U,V$ at $\infty_H$ and compares it
with those coordinates. [pcc:unverified]

Because the source has monic odd degree seven,
$\operatorname{ord}_{\infty_H}(S)=-2$ and
$\operatorname{ord}_{\infty_H}(Y)=-7$. Each of the seven actual coefficient
functions $(a+bY)/c$ satisfies

$$\deg a\le\deg c,\qquad b=0\ \text{or}\quad 2\deg b+7<2\deg c.$$

The $Y$ part therefore vanishes at infinity. Its limit is the leading
$a/c$ ratio when the degrees match, otherwise zero. The resulting monic
cubic is exactly

$$U_\infty(T)=(T-255)(T-253)(T-251).$$

The specialized $V$ takes precisely the three retained $y$ values at these
distinct roots. All three points satisfy the actual quartic. Since the
coefficient functions are regular, the monic cubic gives a finite free
degree-three family at the source DVR; the three distinct finite special
points exhaust its special fiber. Thus $E(\infty_H)=E_{\rm ref}$ and the
subtraction convention is the one inducing $\phi$. Exact $U_\infty$,
$V_\infty$ and all point coordinates are retained in the
[reference result](evidence/references-v2/result.json). [pcc:unverified]

Changing a limiting ordinate, dropping a reference factor or adding a
divergent $Y$ term rejects. The composition controls separately change the
backmap differential, the generic graph ordinate, the trace multiplier,
the denominator-clearing reconstruction and the strict characteristic
bound. All eight controls reach their intended comparisons. [pcc:unverified]

## Relation to the completed boundary work

[PR #5255](https://github.com/isogeny-labs/crypto/pull/5255) implements the
four geometric infinity images and retained affine pole of $f$, checks an
actual projective principal relation in target KM and separate elliptic
arithmetic, and removes infinity support from valid rational degree-12 KM
class spaces. That input contract remains explicit. This result adds the
general composite identity for $B=f_*$ and the original correspondence
forward map, together with its exact reference convention. Earlier relation,
rank and KM evidence remains unchanged. [pcc:unverified]

This does not globally identify every implementation of the older
affine/saturated $\Gamma^t$ transpose with $B$. Its projective point
interface, independent signing and original map-synthesis cost accounting
remain separate. No new PCC ID or performance comparison was made. The
user's coordinate-screen/matched-rho waiver applies; the execution limits
are unchanged.

\clearpage

## Executed costs and reproduction

All three rows are nonoverlapping complete local wrappers on one allowed
CPU. Their total is **149.116 diagnostic wall seconds**. The rejected strict
build is included; each successful build/worker/wrapper exits zero.
Nested intervals are already included in their wrappers. [pcc:unverified]

| Invocation | Wrapper wall (s) | Build / worker wall (s) |
|---|---:|---:|
| Generic composition, attempt-v1 | 140.258 | 7.923 / 132.288 |
| Rejected reference build, references-v1 | 1.389 | 1.352 / not run |
| Reference specialization, references-v2 | 7.469 | 7.412 / 0.008 |

| Generic native phase | Microseconds |
|---|---:|
| Elliptic map equation, differential and degree | 1,212,012 |
| Source cover and function-field controls | 4,154 |
| Actual correspondence degree envelope | 38,230 |
| Generic quartic incidence | 20,696,989 |
| Generic separability and $dT/dS$ | 11,858,368 |
| Differentiated generic incidence | 34,461,102 |
| Actual weighted trace | 43,398,028 |
| Five composition mutation controls | 20,612,423 |

The reference worker records 2929 native microseconds. Complete process
wall/user/system clocks, selected CPU, compiler/OpenSSL versions,
source/input/binary digests and unchanged-byte checks accompany each call.
The failed reference build used an unavailable polynomial inequality
operator. Replacing it with negated equality changed only that comparison;
all criteria and inputs stayed fixed, and the rejected source/build remains
in the record. No mathematical rejection, timeout or cap increase occurred.
[pcc:unverified]

Strict builds retain 300 seconds, native workers 600 seconds and each native
invocation 1 GiB address space on one allowed CPU. Peak RSS, metadata,
literature, reporting, coordination and publication costs lie outside these
wrapper diagnostics. Original construction/synthesis is not repriced by a
verification replay and remains unpriced here. [pcc:unverified]

From the repository root, use fresh output names:

```bash
study=research/nist_cover_three_tests_20261116/composition_verification_20261126
bash "$study/run.sh" evidence/replay-composition-01
bash "$study/check-references.sh" evidence/replay-references-01
bash "$study/build-report.sh"
```

Both wrappers snapshot the exact tracked inputs and build-only separate
arithmetic sources before calls. Source/input versions are fixed by their
manifests and recorded commits. Binaries stay local with published hashes.
Raw records remain loose because the replay/manifests name their exact
paths. The protocols precede the affected executions; the first successful
composition source is unchanged after its call.

Sources checked: exact parent global-correspondence, backmap and
polarized-quotient documents and sources, prior separate replays and #5255;
Milne, [*Abelian Varieties* v2.0](https://www.jmilne.org/math/CourseNotes/AV.pdf),
I.14 Theorem 14.3 and III.6; Snowden,
[Math 679 Lecture 2](https://websites.umich.edu/asnowden/teaching/2013/679/L02.html),
push/pull, separability and quadratic degree; Galbraith's corrected
[chapter 7](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch7.pdf),
uniformizers and extension of curve morphisms. The shared KB remains
unavailable as previously recorded. Established methods are used; no
novelty claim is made. [pcc:unverified]
