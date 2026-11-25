---
title: "P-192: separate checking of the recorded transfer divisors"
date: "2026-11-24"
geometry: margin=0.65in
fontsize: 10pt
header-includes: |
  \makeatletter
  \renewcommand{\maketitle}{\begin{center}\Large\@title\par\small\@date\end{center}\vspace{0.5em}}
  \makeatother
---

The new native checker accepts **all six recorded source-dual comparisons**
against freshly computed triple pullbacks, including the identity and the
exceptional $-2G$ fixture. It also checks every recorded forward cycle in the
full quartic coordinate algebra and rejects all eight mutation controls.
This closes a finite arithmetic-checking gap in the local run: the preceding
separate companion checked only its final elliptic source/norm pairs.
PCC-20261124-df0ddc binds the new result, code, inputs and costs.
[pcc:unverified]

| Fixed audit | Native outcome |
|---|---:|
| Recorded source dual equals freshly computed $[3]\pi^*(P)$ | 6 / 6 |
| Recorded length-nine cycle satisfies full quartic equation | 5 / 5 |
| Mutation controls rejected | 8 / 8 |
| Source smoothness, ramified anchor order two | Pass |
| Identity, inverse, doubling, fixed associativity controls | Pass |
| Strict build / checker / complete wrapper exit | 0 / 0 / 0 |

![Solid boxes show checks implemented by the new arithmetic executable. Dashed boxes show the producer construction whose independent reconstruction remains open.](audit.pdf){width=90%}

## Arithmetic checked and its assumptions

The prime, elliptic coefficient and cover parameter are pinned to the exact
P-192 d7 input. With $c=1024$, the checker independently expands

$$h(X)=X^8\widetilde h(1+1/(cX)),$$
$$\widetilde h(t)=(t^2-1)\big((t^2+7)^3
-3(t^2+7)(t^2-1)^2+b(t^2-1)^3\big).$$

It requires a monic degree-seven result and checks
$\gcd(h,h')=1$. This is the ramified genus-three model $Y^2=h(X)$, with a
single point at infinity. The ramified point
$R=(-1/(2c),0)$ supplies the second point in the fiber above elliptic infinity.

For a finite elliptic point $P=(x,y)$, its quadratic fiber has

$$u_P=X^2-\frac{x-1}{4c}X-\frac{x-1}{8c^2},\qquad
v_P=\frac{y(1+2cX)^2}{c^4}\pmod {u_P}.$$

These follow by eliminating $t$ from
$t^2=(x+7)/(x-1)$ and $t=1+1/(cX)$. The pullback class is the fiber class
minus $[R-\infty]$. The checker verifies the fiber and anchor equations,
computes this difference and its triple, and compares the resulting canonical
Mumford polynomials with the recorded `source_dual`. Identity is handled
explicitly. No elliptic scalar is recovered. [pcc:unverified]

## Separate implementation and exceptional input

The producer uses GMP and its retained arithmetic headers. The new checker
uses **OpenSSL BIGNUM**, its own prime-field operations, polynomial division,
extended Euclidean certificates and Cantor composition/reduction. The only
copied component is the typed JSON parser; it contains no curve arithmetic.
Every composition checks its three-polynomial Bezout identity, exact
divisions and source incidence. Every reduction must strictly decrease degree.

For the odd-degree model, reduced Mumford representatives are unique.
Consequently equality of the computed and recorded pairs checks equality of
these divisor classes. The algorithms and uniqueness assumption are standard:
Galbraith, *Mathematics of Public Key Cryptography*, extended/corrected
[chapter 10](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch10.pdf),
Theorem 10.3.14, equation (10.11), Theorem 10.4.1 and Exercise 10.4.2.
The exact parent transfer and cover protocols were also read; KB is unavailable
as recorded in [PROTOCOL.md](PROTOCOL.md). No novelty claim is made.

The fixed order is $O,G,-G,2G,Q,-2G$. The new executable checks the negative
and doubling bindings from the elliptic coordinates. Thus the final fixture
is actually $-2G$, the input for which the old shifted representation has
$P+2G=O$. The original producer used its complete direct representation for
that case; this audit checks its recorded source result and forward-cycle
incidence. It does not create a new exceptional-map implementation.
[pcc:unverified]

Cycle checking works modulo the **entire degree-nine support polynomial**,
rather than its radical. It verifies length, monicity, reduced coordinates,
the separating-coordinate identity and the quartic equation, including any
retained multiplicity. It does not establish which source class produced that
cycle. The source-dual and target-cycle checks remain separate obligations.
[pcc:unverified]

Mutation controls change a dual to its valid negative, move a dual off the
source curve, alter cycle y, shorten cycle support, omit the exceptional
fixture, duplicate $2G$ in its place, supply a noncanonical coefficient or
change the field pin. All reject. [pcc:unverified]

## Recorded costs and reproduction

| Interval | Wall (s) | User (s) | System (s) |
|---|---:|---:|---:|
| Complete audit wrapper | 5.720 | 5.174 | 0.519 |
| Nested strict build | 5.632 | 5.118 | 0.486 |
| Nested checker process | 0.042 | 0.033 | 0.008 |

The native checker interval is 33,689 microseconds. Nested entries are
included in the wrapper and must not be summed again. These costs are
additional to the earlier 119.375-second relation run. One original allowed
CPU is used; strict build and worker limits are 300 and 120 seconds,
respectively, with 1GiB address space each. Neither limit was reached.
Peak RSS is unmeasured. Metadata, reporting and publication costs are outside
the wrapper; original map synthesis remains unpriced. [pcc:unverified]

```bash
study=research/nist_cover_three_tests_20261116/separate_divisor_20261124
bash "$study/run.sh" evidence/<fresh-name>
```

The [raw result](evidence/attempt-v1/result.json), costs, source/input copies,
pre/post digests, compiler/OpenSSL versions and exits are retained. Executables
are ignored locally; their hashes and complete build sources are published.
Path-bound claim/replay files remain loose, each below the data-size threshold.

H-G holds for this frozen panel. **Independent reconstruction of the isogeny,
its transpose and reference cancellation is still open.** The original
`independent_geometric_verification=false` and unsigned standing remain valid.
This audit neither proves the map identity for all subgroup points nor changes
the collection's zero added old-base rank. [pcc:unverified]
