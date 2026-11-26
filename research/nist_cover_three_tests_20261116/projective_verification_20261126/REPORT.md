---
title: "P-192: projective map evaluation and KM infinity normalization"
date: "2026-11-26"
geometry: margin=0.65in
fontsize: 10pt
header-includes: |
  \makeatletter
  \renewcommand{\maketitle}{\begin{center}\Large\@title\par\small\@date\end{center}\vspace{0.5em}}
  \makeatother
---

The separate arithmetic implementation now evaluates the retained curve map
at every infinity point and at the affine denominator-zero base point. A
projective principal relation passes actual target-Jacobian arithmetic and
separate elliptic transport. A new class-input normalizer removes infinity
support from valid rational degree-12 KM representations, including nonzero
classes. The executable also accepts an external class space and returns its
equivalent finite representation. Results are unregistered and unsigned.
[pcc:unverified]

| Completed check | Native outcome |
|---|---|
| Global rational map equation | Exact identity modulo the quartic |
| Homogeneous base-divisor dimensions / curve-map degree | 12, 12 / 6 |
| All geometric points at infinity | One closed orbit of degree 4 |
| Their elliptic images | Finite, represented over $\mathbb F_{p^4}$ |
| Retained affine pole | Maps to $O$, orders $-2,-3$ |
| Original GMP series comparisons | 45 source-y, 24 scaled-X coefficients |
| Projective relation | Actual KM zero and elliptic difference $O$ |
| KM normalization fixtures | Four principal and two nonzero classes |
| Opaque external class | Infinity multiplicity 1 removed; class preserved |

![The normalizer consumes a valid KM class space. The principal relation and separate transport checks bind its infinity replacement to the actual fixed curve.](normalization.pdf){width=97%}

## Exact curve-map boundary evaluation

Write the retained rational morphism as $X_E=A/B$, $Y_E=C/D$. The new
OpenSSL BIGNUM implementation checks

$$C^2B^3=D^2(A^3-3AB^2+bB^3)\pmod F.$$

It independently reconstructs the degree-six certificate from the homogeneous
ideal $(F,A_6,B_6)$ in degrees 14 and 15. Both quotient dimensions are 12.
The original smooth-quartic/base-divisor argument then gives
$\deg(X_E)=24-12=12$ and $\deg(f)=6$. The existing smoothness/Riemann--Roch
assumptions remain explicit; this does not supply an independent signed proof
of the original full Jacobian isogeny. [pcc:unverified]

The infinity polynomial $F(1,v,0)$ is irreducible of degree four. Thus this
plane model has no $\mathbb F_p$-rational point at infinity; all four geometric
points occur in one Frobenius orbit. Their local chart is $X=1$, with $Z$ a
uniformizer and $Y$ solved by Newton lifting. All elliptic coordinate valuations
are zero. A fresh lift at the conjugate root agrees with the Frobenius image.
Full coefficients and endpoints are in the [native result](evidence/attempt-v2/result.json).
[pcc:unverified]

The precision is fixed at $4\max(\deg A,\deg B,\deg C,\deg D)+1=45$.
Bezout bounds on nonzero polynomial functions guarantee a leading coefficient
within this interval. The affine point uses its own unramified chart; its
Laurent orders and retained series agree with the original GMP computation.
Five changed-polynomial/factor/jet/order controls reject. [pcc:unverified]

\clearpage

## A principal relation containing the infinity orbit

Let $H_\infty=C\cap\{Z=0\}$ and let $H_{56}$ be the original first split
vertical fiber, containing points 42, 43, 44 and 45. The exact line ratio gives

$$\operatorname{div}_C\left(\frac{Z}{X-56Z}\right)=H_\infty-H_{56}.$$

The separate OpenSSL worker reconstructs all infinity images, takes their
complete Frobenius sum, freshly evaluates all four finite reference images
and checks each against its original recorded image. It also checks the full
factor products of both hyperplane intersections. The two elliptic sums are
equal and nonzero; their difference is $O$. Dropping an infinity conjugate or
a reference intersection factor rejects. [pcc:unverified]

This relation is also checked in the **actual GMP target KM arithmetic**.
With $D_0=3H_{56}$, encode the degree-12 divisor
$D=H_\infty+2H_{56}$. Its 22-dimensional space inside $L(3D_0)$ is generated
by homogeneous sections

$$Z(X-56Z)^2p_6,$$

where $p_6$ ranges over the complete degree-six section basis. The implementation
uses the same 85-point evaluation model and accepts its zero class. Adding a
known nonzero target class rejects. The complete class space is retained in
[km.stdout](evidence/transport-v1/km.stdout), while
[openssl.stdout](evidence/transport-v1/openssl.stdout) retains the two actual
elliptic sums. [pcc:unverified]

This adds a checked projective relation to the implementation evidence. It
does not change the earlier fixed collection matrix or its zero additional
old-base rank. The relation is used here to remove exceptional representation
support. [pcc:unverified]

## Why this permits rational infinity removal

The entire infinity intersection is one irreducible degree-four closed point.
Consequently an $\mathbb F_p$-rational divisor has infinity part $mH_\infty$.
The same principal line ratio permits replacing that part by $mH_{56}$.
For a valid effective degree-12 KM representative, $m$ lies in $0,1,2,3$.
This uses the fixed model and rational-input contract; it is not a statement
about arbitrary individual infinity points over every extension field.
[pcc:unverified]

The stored affine numerators $A,B$ have degree five, although their original
section degree is six: $A_6=ZA_5$, $B_6=ZB_5$. Resolving the common factor
at infinity is part of evaluating this representation correctly.

\clearpage

## An actual class-space normalization API

The input is the valid 22-dimensional space $W=L(9H_{56}-D)$, given only
through its 85 evaluation coordinates per row. The new function reconstructs
each row's unique numerator in the original 34-monomial $V_3$ basis and checks
every reconstructed evaluation. It does not consume the class's generating
scalar or a point-support decomposition. [pcc:unverified]

The reduced basis has $x$ exponent at most three. Let $P$ be a nonzero
numerator with total degree $d\le9$. At infinity, its highest homogeneous
part evaluated at $(1,v)$ is a nonzero cubic times a power of $v$ (or has
degree below four). Because $v$ has degree four and is nonzero, this leading
part cannot vanish. The denominator $(X-56Z)^9$ is a unit there. Hence

$$\operatorname{ord}_{H_\infty}(P/(X-56Z)^9)=9-d.$$

The line bundle $9H_{56}-D$ has degree 24 and is base-point-free. The common
minimum of these orders therefore equals the infinity multiplicity of $D$:

$$m=9-\max\{\deg P:\ P\text{ is a reconstructed basis numerator}\}.$$

Multiply the evaluated input functions by $(x-56)^m$. The returned space
represents $D-mH_\infty+mH_{56}$, has no infinity support and has the same
Jacobian class. The principal-function witness is
$((X-56Z)/Z)^m$. This derivation assumes the valid effective-degree-12 RR
input contract; arbitrary 22-dimensional subspaces are not automatically valid
Jacobian encodings. [pcc:unverified]

| Actual input family | Detected infinity multiplicities |
|---|---|
| $mH_\infty+(3-m)H_{56}$ | 0, 1, 2, 3 |
| $mH_\infty+(3-m)H_{56}-Q_0+P$, nonzero class | 1, 2 |

Every normalized canonical space equals the expected finite space and passes
actual KM equality. A changed evaluation entry outside $V_3$ rejects. GMP
also checks the degree-four infinity irreducibility independently of OpenSSL.
The [six-case result](evidence/normalization-v1/result.json) retains these
checks. [pcc:unverified]

The executable accepts an optional external `class.json`, containing a
`class_space` array with 22 rows of 85 canonical field coefficients. That
mode directly normalizes the supplied space and outputs the finite
`class_space`. The [external-input call](evidence/normalization-v2/result.json)
consumes the actual projective KM output from the previous phase, removes
one infinity copy and preserves its class. No stored support metadata is read.
[pcc:unverified]

\clearpage

## Costs, reproduction and remaining scope

All rows are separate local wrapper calls on one allowed CPU. Nested build
and worker intervals are included and must not be added again. The initial
rejected build is retained. The five nonoverlapping wrappers total 76.558
wall seconds; these are diagnostics, not a matched performance comparison.
[pcc:unverified]

| Call | Wrapper wall seconds |
|---|---:|
| Rejected strict build, attempt-v1 | 8.089 |
| Boundary evaluator, attempt-v2 | 21.650 |
| Projective relation, transport-v1 | 32.667 |
| Six-case normalizer, normalization-v1 | 8.852 |
| External-input normalizer, normalization-v2 | 5.300 |

| Nested interval | Build / worker wall seconds |
|---|---|
| Rejected build | 8.041 / not run |
| Boundary evaluator | 7.452 / 14.150 |
| Transport, OpenSSL | 7.644 / 17.997 |
| Transport, target KM | 3.640 / 3.013 |
| Six-case normalizer | 4.535 / 4.274 |
| External-input normalizer | 4.525 / 0.739 |

Native phase clocks, wall/user/system process clocks, CPU selection, exits,
source/input/binary digests and unchanged-byte checks accompany the results.
The sole rejected build was misleading serialization indentation, corrected
before any mathematical worker ran. No precision or resource cap was widened.
Strict builds retain 300 seconds, workers 600 seconds and 1GiB address space.
Peak RSS, reporting, coordination and publication costs are outside these
wrapper diagnostics. Original map synthesis remains unpriced.
[pcc:unverified]

From the repository root, use fresh output names:

```bash
study=research/nist_cover_three_tests_20261116/projective_verification_20261126
bash "$study/run.sh" evidence/replay-endpoints-01
bash "$study/check-transport.sh" evidence/replay-transport-01
bash "$study/check-normalization.sh" evidence/replay-normalization-01
```

The normalizer's three-argument mode runs the six-case qualification; its
fourth argument selects external-input mode. All exact input paths are
tracked, and the source snapshots preserve both versions. Generated binaries
stay local; their hashes and complete build sources are retained. Loose raw
records are required by the frozen manifests and replay input paths, including
the KM output consumed by the external normalizer.

The user explicitly waived coordinate-screen and matched-rho requirements
for this task while retaining execution limits. No new PCC registration or
performance comparison was made. This component covers the declared curve-map
boundary panel and rational KM infinity normalization. A general arbitrary
projective individual-point transpose, independent signed standing and original
construction accounting remain separate. Earlier evidence stays unchanged.

Sources: exact parent backmap/KM protocols and sources; Galbraith's corrected
[chapter 7](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch7.pdf),
uniformizers/valuations and Lemma 7.3.6, and chapter 9 section 9.2. KB remained
unavailable as recorded in [PROTOCOL.md](PROTOCOL.md). Standard morphism and
RR methods are used; no novelty claim is made. [pcc:unverified]
