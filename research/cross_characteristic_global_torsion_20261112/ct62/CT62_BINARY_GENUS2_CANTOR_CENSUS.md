# CT62 — genus-two binary target census, independent Cantor law, and subgroup-field shortcut

**Standing:** [pcc:unverified]. These are bounded arithmetic controls, **not** an encoded-input cross-characteristic transfer or a full-cost DLP advantage. Theorems about finite-field curve point counts and Cantor's algorithm are established prior art.

## Scope and exact arithmetic

We enumerated two **specified coefficient grids** for characteristic-two genus-two curves
\[
C:y^2+h(x)y=f(x),\qquad \deg h\le3,\ \deg f\le6.
\]
For \(\mathbb F_2\) we include all coefficients \(h_0,\dots,h_3\), use \(f_5=1,f_6=0\) when \(h_3=0\), and allow \(f_0,\dots,f_5\) with \(f_6=0\) or \(1\) when \(h_3=1\). Those two \(f_6\) cases represent split versus nonsplit infinite-place behavior; \(\mathbb F_2\) has no further nonzero leading \(h_3\). For \(\mathbb F_4\) this run **only** enumerates \(h_3=f_6=0,\ f_5=1\), all \(h_0,h_1,h_2\) and \(f_0,\dots,f_4\). **Do not call the F4 grid all genus-two curves over F4.** No isomorphism-class deduplication was attempted.

Every curve is tested for affine smoothness by
\[
\gcd\big(h,\;f'(x)^2+h'(x)^2 f(x)\big)=1,
\]
and the specified leading-term/infinite-point models are smooth at infinity. Python counts \(N_1=\#C(\mathbb F_q)\) and \(N_2=\#C(\mathbb F_{q^2})\) through exact field-arithmetic solution tables and handles the correct 0/1/2 points at infinity. The genus-two Jacobian order follows from
\[
\#J(C)(\mathbb F_q)=\frac{N_1^2+N_2}{2}-q.
\]
Independently authored JavaScript directly enumerates \((x,y)\), with a different smoothness oracle: polynomial-bit gcd for F2 and finite-extension singular-root checks over F16 for the quadratic F4 h. The field embedding \(\mathbb F_4\hookrightarrow\mathbb F_{16}\) is verified against both multiplication tables.

| Grid | Coefficient models | Smooth | Distinct Jacobian orders | Range of observed orders | Largest observed odd prime factor |
|---|---:|---:|---:|---:|---:|
| F2, both infinity types | 1,280 | **640** | 15 | 1–19 | **19** |
| F4, odd-degree subfamily only | 65,536 | **49,152** | 30 | 5–49 | **31** |

For F2, smooth models divide as **128** with one rational infinity, **256** with two rational infinities, and **256** with nonsplit infinity. As a CT56 positive control, \(y^2+(x^3+x+1)y=x^5+x\) gives \(N_1=N_2=6\) and \(J=19\). The F4 grid omits curves whose two points at infinity are rational or conjugate, which is a **scope limitation**, not a null result.

For one representative of every order attained by the **odd-degree** F2 and F4 models, a separate Python enumeration of all reduced Mumford pairs \((u,v)\) of degree \(0,1,2\) agrees with the zeta order. The enumeration includes irreducible quadratic \(u\), repeated *unramified* \(u\), and disallows a doubled ramification point \(u=(x-r)^2\) with \(h(r)=0\). That is a second arithmetic method for this restricted family, not a blanket proof that the same representation handles split or nonsplit infinity for h3 nonzero.

## Explicit genus-two shortcut that genus-one targets cannot match

Put \(\mathbb F_4=\mathbb F_2(\omega)\) with \(\omega^2+\omega+1=0\). The explicit smooth curve
\[
\boxed{C:\ y^2+y=x^5+\omega x^3}
\]
has
\[
\#C(\mathbb F_4)=7,\qquad\#C(\mathbb F_{16})=21,\qquad
\boxed{\#J(C)(\mathbb F_4)=31}.
\]
Its rational Jacobian is therefore cyclic of prime order 31. Direct enumeration finds 31 distinct reduced divisor classes: identity 1, degree-one 6, split quadratic 12, irreducible/nonsplit quadratic 6, and repeated-unramified quadratic 6. No *unverified* norm-map or index-calculus performance claim follows from their existence.

An elliptic curve defined over \(\mathbb F_{2^k}\) cannot carry a rational 31-point subgroup for \(k\le4\) by Hasse. At \(k=5\), the only group order divisible by 31 in Hasse's interval is 31, requiring trace \(t=2\), which is **not** a permitted elliptic trace over \(\mathbb F_{32}\) by Waterhouse's characteristic-two classification. At \(k=6\), trace \(3\) over \(\mathbb F_{64}\) is ordinary and gives group order \(64+1-3=62\). Hence the smallest binary definition field in the elliptic case is degree **6**, while the displayed genus-two Jacobian carries the same prime subgroup in degree **2**.

A second **computational falsifier control** independently enumerates all 2,016 standard characteristic-two elliptic Weierstrass models over \(\mathbb F_{32}\): the 992 ordinary forms \(y^2+xy=x^3+a_2x^2+a_6,\ a_6\ne0\), and 1,024 supersingular forms \(y^2+y=x^3+a_4x+a_6\). Exactly the supersingular traces \(\{-8,0,8\}\) occur; the ordinary traces are odd. **No model has trace 2 or group order 31**, confirming the relevant Waterhouse step by exact point enumeration. Isomorphic duplicates are intentionally retained; this is a finite-parameter sweep, not a performance benchmark. [pcc:unverified]

Moreover, this \(J/\mathbb F_4\) **cannot split over F4 as a product of elliptic curves up to isogeny**: such a product would have rational point count \(31=\#E_1(\mathbb F_4)\#E_2(\mathbb F_4)\), but every elliptic factor has at most \(4+1+2\sqrt4=9\) points. This is a concrete local field-size shortcut in an F4-simple genus-two Jacobian, **not** evidence for a source-to-Jacobian homomorphism or cheaper discrete-log solving.

### Exact vanishing cross-factor maps over F4

There is a further **proved negative result** for this particular witness:
\[
\boxed{\operatorname{Hom}_{\mathbb F_4}(E,J)=0
\quad\text{and}\quad
\operatorname{Hom}_{\mathbb F_4}(J,E)=0}
\]
for **every** elliptic curve \(E/\mathbb F_4\), where \(J=J(C)\) has 31 rational points. A nonzero elliptic-to-Jacobian map has positive-dimensional elliptic image, contradicting F4-simplicity; a nonzero Jacobian-to-elliptic map has an elliptic quotient, contradicting F4-simplicity (equivalently, apply Poincaré complete reducibility). Thus this concrete low-field-degree target gives a **vanishing** cross-factor Hom test, not an undiscovered nonzero split-Jacobian transfer. The Hom statement is over **F4**, not automatically over its extensions or the algebraic closure. [pcc:unverified]

## Independent composition and toy DLP controls

The standalone CT62 Python Cantor implementation uses polynomial extended gcd, composition, and reduction for the two explicitly given odd-degree genus-two models over F4, **not** the concurrent native Rust/Ciani/Jacobian work in root PR #3877. It enumerates every rational divisor class, checks closure and inverse/identity/commutativity for every ordered pair and associativity for every ordered triple:

| Model | #J(F4) | Pairs | Triples | Additional checks |
|---|---:|---:|---:|---|
| \(y^2+y=x^5+\omega x^3\) | 31 | 961 | 29,791 | six nonsplit quadratic classes; all 30 nonzero classes have order 31 |
| \(y^2+xy=x^5+1\) | 16 | 256 | 4,096 | eight nonsplit quadratic classes; ramified class \((u,v)=(x,1)\) has exact order 2 |

The order-31 implementation also solves **930** independent *destination-only* public pair DLPs by baby-step/giant-step: all 30 nonidentity generators times all 31 target classes, recovering the logarithm for each and verifying the resulting scalar. A maximum of 11 baby/giant additions is counted **inside the toy search**; this excludes verification, model setup, enumeration, and any hypothetical source lift. These are correctness checks, not a rho speed comparison or total attack cost.

## Reproduce

From repository root:

~~~sh
D=research/cross_characteristic_global_torsion_20261112/ct62
python3 -B "$D/ct62_genus2_census.py" > /tmp/ct62-py.json
python3 -O -B "$D/ct62_genus2_census.py" > /tmp/ct62-py-O.json
python3 -B "$D/ct62_elliptic_f32_control.py" > /tmp/ct62-f32.txt
python3 -O -B "$D/ct62_elliptic_f32_control.py" > /tmp/ct62-f32-O.txt
cmp /tmp/ct62-f32.txt /tmp/ct62-f32-O.txt
python3 -B "$D/ct62_genus2_cantor.py" > /tmp/ct62-cantor.json
python3 -O -B "$D/ct62_genus2_cantor.py" > /tmp/ct62-cantor-O.json
node "$D/ct62_genus2_independent.js" > /tmp/ct62-js.json
node --no-opt "$D/ct62_genus2_independent.js" > /tmp/ct62-js-noopt.json
cmp /tmp/ct62-py.json /tmp/ct62-py-O.json
cmp /tmp/ct62-cantor.json /tmp/ct62-cantor-O.json
cmp /tmp/ct62-js.json /tmp/ct62-js-noopt.json
~~~

Compare both independently computed JSON results for each grid's smooth count, order distribution, and prime-divisor incidence (the exact-head CI job performs this). All Python checks use unconditional exceptions rather than optimizable-away bare asserts. **Local replay** passed with identical outputs within each of the three implementations, and independent Python/JS result summaries agree. Exact-head GitHub Actions and scientific PCC signing must be independently confirmed before promotion.

### Prior work and negative-scope ledger

Sources checked: internal CT55–61 notes and PR #4015, root #3877, F101 native genus-three work, ongoing cryptanalysis #1520; Waterhouse, *Abelian varieties over finite fields* (1969), Theorem 4.1, https://numdam.org/articles/10.24033/asens.1183/ ; Bröker–Howe–Lauter–Stevenhagen, *Genus-2 curves and Jacobians with a given number of points* (2015), https://arxiv.org/abs/1403.6911 ; and the internal CT56 genus-two size control. This is elementary bounded arithmetic based on established zeta and Cantor theory; no novelty claim.

**KB receipt:** research-KB MCP absent from this runtime; repository checkout wrapper unavailable. Reviewed primary GitHub notes, active PR scopes, existing session log, and the above public references instead. Do not infer absence of earlier work from the fallback search.

The local result **does not construct** a field-compatible lift or homomorphism carrying arbitrary encoded \(P,Q=[x]P\) from an odd-prime-field ECDLP instance. No nontrivial cross-factor map is asserted, and the F4 prime-order Jacobian is explicitly not F4-split. A future solver must charge construction of a global number-field lift, selection of places and point orientations, transfer verification, and a full cold DLP solve against optimized negation-map rho and existing index-calculus baselines. Do not compare bare field extension degrees as if they were attack runtimes.

No EKS Spot instances or paid compute were launched; these are small local correctness enumerations. Root native CI, review gates, and automatic merge policy remain separate. [pcc:unverified]
