# CT101 — Constructive eleven-torsion splitting and degree-40 binary tower

**2026-11-14. Scientific standing: [pcc:unverified].** This is a second non-rational torsion-prime arithmetic fixture, not a scalar-independent source-to-target DLP evaluation and not a speedup.

## Reviewed dependencies and hypotheses

Read repository `AGENTS.md`, the latest Nov. 14 session log, root [PR #3877](https://github.com/isogeny-labs/crypto/pull/3877), stacked [PR #4102](https://github.com/isogeny-labs/crypto/pull/4102), CT94's Frobenius gate and CT100's chord-capacity/rational-five control. Queried GitHub for a CT101 branch or PR (none as of this run), checked the current CT100 exact-head 12 workflow runs (all successful before new edits), and inspected the prior cryptanalysis Richelot and public-target work. Research-KB MCP and its repository wrapper were not available; no absence-of-prior-art or novelty claim follows. Primary documentation checked: [Magma division polynomial definition](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurves/function_field.html), [Sage finite-field Frobenius polynomial](https://sporadic.stanford.edu/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_finite_field.html). The classical formulas are not newly discovered here.

Fixed global curve

\[
E:\quad y^2+y=x^3-x^2-10x-20.
\]

The good source prime is \(p=59\); the fixed destination is the characteristic-two reduction. Both are prime to \(\ell=11\). No paid EKS work or private curves.

## Exact integral division polynomial and source splitting

Let

\[
\begin{aligned}
D(X)&=4X^3-4X^2-40X-79,\\
f_3(X)&=3X^4-4X^3-60X^2-237X-21,\\
f_4(X)&=2X^6-4X^5-100X^4-790X^3-210X^2-1496X-5821.
\end{aligned}
\]

Here \(\psi_{2m}=\psi_2 f_{2m}\), \(\psi_2^2=D\). Standard division-polynomial recursion gives

\[
\begin{aligned}
f_5&=D^2 f_4-f_3^3,\\
f_6&=f_3(f_5-f_4^2),\\
f_7&=f_5 f_3^3-D^2f_4^3,\\
\boxed{\psi_{11}}&=f_7 f_5^3-D^2 f_4 f_6^3.
\end{aligned}
\]

Both executables construct this polynomial from the small seed coefficients, rather than accepting a hard-coded degree-60 division polynomial. It has degree 60 and leading coefficient 11.

Exhaustive source curve counting gives \(\#E(\mathbb F_{59})=55\). The source point \(P=(19,22)\) has exact order 11, with five nonzero x-classes

\[
\boxed{\{19,20,22,43,46\}}.
\]

A squarefreeness check of \(\psi_{11}\bmod59\), together with

\[
\deg\gcd(\psi_{11},X^{59}-X)=5,\qquad
\deg\gcd(\psi_{11},X^{59^5}-X)=60,
\]

proves the complete **source** x-field factor pattern \(1^5\,5^{11}\). In particular, the degree-one source places are exactly those five roots; the other 55 x-points have degree five. The source Frobenius has trace 5 and determinant \(59\equiv4\pmod{11}\), hence eigenvalues \(1,4\), consistent with the factorization. These assertions concern finite-field reductions; they do not by themselves prove the global \(\ell\)-adic image is full \(\mathrm{GL}_2\).

## Explicit target factorization and oriented point construction

The **same integral polynomial** modulo two factors exactly as three monic irreducible degree-20 polynomials. Bit masks encode coefficients in increasing x-degree order:

| Factor mask | Explicit polynomial |
|---|---|
| `0x130027` | \(X^{20}+X^{17}+X^{16}+X^5+X^2+X+1\) |
| `0x131203` | \(X^{20}+X^{17}+X^{16}+X^{12}+X^9+X+1\) |
| `0x131211` | \(X^{20}+X^{17}+X^{16}+X^{12}+X^9+X^4+1\) |

The programs multiply the factors back to the exact mod-two division polynomial, and check each factor's irreducibility with Rabin's finite-field criterion: \(X^{2^{20}}\equiv X\) and the required gcd checks at exponents \(2^4\) and \(2^{10}\) give one. The factors are squarefree, so the complete binary x-field pattern is \(20^3\).

For each factor \(g_i\), form \(k_i=\mathbb F_2[\alpha]/(g_i)\cong\mathbb F_{2^{20}}\). Put \(r_i=\alpha^3+\alpha^2\), the right side of the binary curve equation. Direct field arithmetic gives

\[
\operatorname{Tr}_{k_i/\mathbb F_2}(r_i)=1
\]

for **all three** factors. Consequently \(Y^2+Y+r_i\) is irreducible over \(k_i\), and

\[
K_i=k_i[Y]/(Y^2+Y+r_i)\cong\mathbb F_{2^{40}}
\]

contains the explicit on-curve point \(T_i=(\alpha,Y)\). This constructs actual oriented coordinates, not just an abstract Frobenius matrix.

Independent Python and JavaScript implementations each verify \([11]T_i=\mathcal O\), all 121 ordered group-law cases on the eleven-element cyclic subgroup, and a **40-element** Frobenius orbit of each oriented \(T_i\), with the 20th Frobenius image equal to \(-T_i\). The three independent 40-orbits account for all \(11^2-1=120\) nonzero geometric eleven-torsion points. Thus the 40-degree target extension from CT94 is **constructively necessary and sufficient** for this fixed target curve and prime \(\ell=11\).

This does **not** compute the special target point corresponding to a particular public source input. Choosing one of the three target orbits and one of its 40 embeddings is not a certified common global specialization; no cross-characteristic curve morphism exists. There are 120 abstract nonzero homomorphisms from the selected cyclic source group to geometric \(E[11]\), but an efficiently evaluable compatible one is not yet provided.

## Independent reproducibility

Files:

- `ct101_eleven_torsion.py`: exact integer \(\psi_{11}\), modular polynomial gcd/Frobenius, carryless binary polynomials, \(\mathbb F_{2^{20}}\) arithmetic, quadratic oriented extension, 363 binary ordered group-law checks and source order-11 fixture.
- `ct101_eleven_independent.js`: independently coded F59 polynomial recurrence and group law; BigInt bit-polynomial Rabin tests; separate 20-bit carryless field arithmetic, trace, 363 ordered group laws and 120 Frobenius point controls. The two programs do not borrow one another's calculation routines.

From repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct101
python3 -B "$D/ct101_eleven_torsion.py" >/tmp/ct101-py.json
python3 -O -B "$D/ct101_eleven_torsion.py" >/tmp/ct101-pyopt.json
node "$D/ct101_eleven_independent.js" >/tmp/ct101-js.json
node --no-opt "$D/ct101_eleven_independent.js" >/tmp/ct101-jsopt.json
cmp /tmp/ct101-py.json /tmp/ct101-pyopt.json
cmp /tmp/ct101-py.json /tmp/ct101-js.json
cmp /tmp/ct101-py.json /tmp/ct101-jsopt.json
sha256sum /tmp/ct101-py.json
```

Locally, all four modes produced byte-identical JSON with SHA-256
`b02f900029347e23300a6871cde8dbe35f4a2f497bcdfd9024bad93abeedc082`.
The repository-scoped exact-head CI run is a *separate* required check after commit. These are independent finite-field implementations, **not** independent global number-field lifting or signed scientific PCC.

## Exact cost and remaining transfer obligations

The output is a finite-field representation feasibility certificate. No globally compatible oriented root is recognized from an arbitrary encoded source point. It charges no relation collection, coefficient-height search, number-field field-defining polynomial construction, or cold target DLP; therefore it is **not** a complete-cost benchmark. The four-way output explicitly sets `cross_characteristic_lift_evaluated=false`.

The next gate is to construct and certify the degree-60 global x-point algebra for \(\ell=11\), select a *fixed* degree-one prime above 59 and a *fixed* target prime over 2 in the same algebra, then build an **input-scalar-independent** exact global root witness and oriented reduction. Factor masks and 40-degree arithmetic supplied here are necessary target prerequisites, not a substitute. The full cold comparison must include those number-field costs and an actual same-input target DLP against optimized source BSGS/rho and the admitted index-calculus baseline.

No protected merge, paid compute, generic hardness claim, or ECDLP advantage. [pcc:unverified]
