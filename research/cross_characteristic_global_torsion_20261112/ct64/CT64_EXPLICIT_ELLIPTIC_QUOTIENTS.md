# CT64 — explicit quintic elliptic quotients and encoded Mumford transfer

**Standing: [pcc:unverified].** This constructs a genuine same-characteristic isogeny and an executable transport for public toy genus-two inputs. It does **not** construct a prime-field-to-characteristic-two algebraization or a full-cost advantage.

## Starting point and exact model

Retain the independently checked CT62–63 genus-two curve over \(\mathbb F_4\):
\[
C:y^2+y=x^5+\omega x^3,\quad \omega^2+\omega+1=0.
\]
CT63 established its Weil polynomial \(16\Phi_5(X/2)\), the absence of elliptic factors over \(\mathbb F_{4^n}\) for \(5\nmid n\), and
\[
J(C)(\mathbb F_{1024})=J[31]\cong(\mathbb Z/31)^4.
\]
Use \(\mathbb F_{1024}=\mathbb F_2[z]/(z^{10}+z^3+1)\), \(\omega=236\) under the bit-polynomial basis. Integers below are **field-element encodings**, not decimal curve coefficients in \(\mathbb Z\).

## Explicit degree-two quotient formulas

For \(r\ne0\) put
\[
\sigma_r:(x,y)\mapsto(x+r,\ y+a x^2+b x+c).
\]
The curve equation is preserved if
\[
a^2=r,\qquad b^2+a=\omega r,\qquad b=r^4+\omega r^2,\qquad c^2+c=r^5+\omega r^3.
\]
The square of \(\sigma_r\) adds the constant \(ar^2+br\) to \(y\). Thus a lift with \(ar^2+br=0\) is a *genuine involution*; a nonzero constant produces an order-four lift, **not** an elliptic involution.

The bounded \(\mathbb F_{1024}\) search finds **15 distinct nonzero translation parameters** with field-rational lifts: five parameters yield order-two lifts and ten yield order-four lifts. Each parameter has **two** lifts, differing by the hyperelliptic involution \(y\mapsto y+1\); thus there are **10 actual order-two** and **20 actual order-four** automorphisms. The auditors choose the first Artin–Schreier root \(c\) as the canonical representative for each parameter. Two explicit involutions are

| \(\sigma_r\) | \(r\) | \(a\) | \(b\) | \(c\) | Quotient \((\alpha,\beta,\gamma)\) |
|---|---:|---:|---:|---:|---|
| first | 20 | 6 | 120 | 432 | (941, 20, 336) |
| second | 534 | 794 | 636 | 668 | (249, 534, 39) |

For each, set
\[
A=a/r,\qquad D=(c+A r^3)/r,\qquad
U=x^2+rx,\quad V=y+Ax^3+Dx.
\]
Both \(U,V\) are invariant under the corresponding involution. Direct coefficient comparison gives the **elliptic quotient**
\[
\boxed{E_r:\quad V^2+V=\alpha U^3+\beta U^2+\gamma U,\quad
(\alpha,\beta,\gamma)=(r^{-1},r,D/r).}
\]
The map \(\pi_r:C\to E_r\) has degree two because \(x\) solves \(x^2+rx=U\) over the invariant function field. In particular it induces an explicit homomorphism \((\pi_r)_*:J(C)\to E_r\) on divisor classes.

## Certified mod-31 isogeny and oriented line

Each elliptic quotient has 961 points over \(\mathbb F_{1024}\). Independent point enumeration and group law verify \(E_r(\mathbb F_{1024})\cong(\mathbb Z/31)^2\). The product
\[
\boxed{\Phi=((\pi_{20})_*,(\pi_{534})_*):J(C)\longrightarrow E_{20}\times E_{534}}
\]
is a **concrete isogeny over \(\mathbb F_{1024}\)**, not merely a Tate-theorem existence claim: the following four actual affine points \(P\in C(\mathbb F_{1024})\) have images of rank four in the product's \(\mathbb F_{31}\)-coordinates (two coordinates per elliptic factor):

| \(P=(x,y)\) | image in \(E_{20}[31]\times E_{534}[31]\) |
|---|---|
| (0,0) | (1,0,1,0) |
| (2,738) | (30,27,13,11) |
| (4,684) | (5,16,17,21) |
| (6,920) | (18,2,0,10) |

Their images span all \(31^4\) product points; CT63 gives exactly \(31^4\) source points. Therefore \(\Phi\) is an isomorphism **on rational 31-torsion**, and its geometric isogeny kernel has degree prime to 31. The independent JS verifier checks rank four for **all ten pairs** of the five involutions, not only the displayed pair.

The original generator \(G=[(0,0)-\infty]\in J(\mathbb F_4)\) is mapped to the nonidentity point \((0,0)\) in **each** \(E_r\). Thus a prescribed rational order-31 source line is transported with orientation and without determining its logarithm.

## Encoded-input divisor transport and tests

The Python implementation consumes an arbitrary **reduced Mumford pair** \((u(x),v(x))\) from the existing CT62 independent Cantor implementation. It pushes degree-one support directly, split quadratic support pointwise, repeated support with multiplicity two, and nonsplit quadratic support by arithmetic in
\[
\mathbb F_{1024}[Z]/(Z^2+u_1Z+u_0).
\]
The two conjugate images are added using an independently implemented elliptic addition law; the result is explicitly checked to descend to \(\mathbb F_{1024}\). No scalar label, discrete log, divisor lookup table or precomputed inverse specialization is accepted as transport input.

**Local checks:** 31 encoded classes: one zero, 6 linear, 12 split, 6 repeated and 6 nonsplit. Both quotient maps preserved **all 961 ordered source-group additions**; all **62 public known-answer target discrete logarithms** were recovered. Python normal and \`-O\` modes agreed. Independently authored JS enumerated 896 finite points on \(C(\mathbb F_{1024})\), 961 points per elliptic quotient, one representative for each of the five involutive translation parameters, ten order-four translation negative controls, and rank four for all ten quotient pairs; Node normal and \`--no-opt\` matched. This is bounded local verification, **not** third-party PCC certification.

Run from the repository root:
~~~sh
D=research/cross_characteristic_global_torsion_20261112/ct64
python3 -B "$D/ct64_mumford_transport.py"
python3 -O -B "$D/ct64_mumford_transport.py"
node "$D/ct64_independent.js"
node --no-opt "$D/ct64_independent.js"
~~~

The additive CT64 workflow checks each mode and compares the two auditors' common exact invariants.

## Proof boundaries and future work

The explicit isogeny is a **finite-field, same-characteristic** map from a genus-two Jacobian to two elliptic curves; it does not solve the missing global compatible lifts of arbitrary **prime-field** input points \(P,Q\), choose their oriented reductions at different primes, or furnish an end-to-end prime-to-binary reduction. The target field is 5 times larger in degree over \(\mathbb F_4\), and all groups still contain order-31 DLPs; any cold solving comparison must charge the extension, divisor transport and target DLP, and use the matched negation-map rho / admitted index-calculus baseline. No speedup is asserted.

**Background/prior art:** CT62–63, root [#3877](https://github.com/isogeny-labs/crypto/pull/3877), stacked [#4015](https://github.com/isogeny-labs/crypto/pull/4015), Kani [*Elliptic subcovers of hyperelliptic curves*](https://doi.org/10.1002/mana.201600530) (correspondence between genus-even elliptic subcovers and elliptic Jacobian subgroups), and Howe–Zhu [*Absolutely simple abelian varieties*](https://doi.org/10.1006/jnth.2001.2697). All existence/splitting principles are classical; CT64 supplies **explicit formulas and executable finite tests for this particular CT62 witness**, without claiming novelty beyond this repo's implementation. Research-KB MCP and repository checkout wrapper remained unavailable in the session; fallback repo and primary-literature checks used. Full root cargo tests, EKS runs, independent PCC signing and complete cold-cost benchmarks are **not completed by this scoped finite arithmetic stage**. No paid compute or protected merge.
