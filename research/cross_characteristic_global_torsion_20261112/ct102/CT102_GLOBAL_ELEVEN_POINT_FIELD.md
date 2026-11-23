# CT102 — Unconditional degree-120 global eleven-torsion point field

**2026-11-14. Scientific status: [pcc:unverified].** This gives an exact global algebraic certificate, independent of any full-Galois-image hypothesis. It is not an input-dependent cross-characteristic transfer or a DLP speedup.

## Novelty boundary and dependencies

The proof uses elementary Gauss/Rabin irreducibility, Sylvester resultants and field norms, not a new algebraic number theory theorem. It extends CT96's *conditional* \(\mathrm{GL}_2(\mathbb F_\ell)\)-orbit calculation by verifying the actual degree of the \(\ell=11\) point field on the fixed 11a1 curve, without the full-image assumption. CT101 previously established its explicit source p=59 and binary p=2 splitting and three binary degree-40 oriented point orbits.

Reviewed source files: repo AGENTS.md and latest Nov14 log, PRs #3877/#4102/#4074, CT94, CT96 (via its PR body), CT100 and CT101. External methods: [Magma division polynomials](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurves/function_field.html) and [Sage's finite-field Frobenius polynomial](https://sporadic.stanford.edu/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_finite_field.html). Research-KB MCP and repository wrapper are unavailable in this connector session; absence of a hit is never a novelty proof.

## Exact irreducibility certificate

Let \(F(X)=\psi_{11}(X)\in\mathbb Z[X]\) be the integral degree-60 division polynomial constructed independently by the CT101 Python and CT102 JavaScript recurrences for

\[
E:\quad Y^2+Y=X^3-X^2-10X-20.
\]

Its leading coefficient is 11, which is invertible modulo 13. The reduction \(\bar F\in\mathbb F_{13}[X]\) is **irreducible** by Rabin's criterion:

\[
X^{13^{60}}\equiv X\bmod\bar F,\quad
\gcd(\bar F,X^{13^{60/r}}-X)=1
\quad(r=2,3,5).
\]

Both executables compute the three gcds at exponents 30, 20 and 12 using actual polynomial modular arithmetic, not by trusting a factoring library. Thus \(F\) is irreducible over \(\mathbb Q\), so for any nonzero eleven-torsion point \(T\),

\[
L=\mathbb Q(x(T))\quad\text{has}\quad [L:\mathbb Q]=60.
\]

Here one irreducible division polynomial implies the same degree for **every** nonzero geometric 11-torsion x-coordinate. This is a verified arithmetic fact about the fixed curve, not a claim that its entire mod-11 Galois image is \(\mathrm{GL}_2(\mathbb F_{11})\).

## Exact negative norm gives the missing quadratic degree

Let \(\alpha=x(T)\) be a root of \(F\). To solve the global Weierstrass equation for the oriented ordinate \(Y\), the quadratic polynomial

\[
Y^2+Y-\bigl(\alpha^3-\alpha^2-10\alpha-20\bigr)
\]

has discriminant

\[
D(\alpha)=4\alpha^3-4\alpha^2-40\alpha-79.
\]

Using separate integer implementations of a 63-by-63 Sylvester determinant and exact fraction-free Bareiss elimination, Python and JavaScript obtain

\[
\begin{aligned}
\operatorname{Res}(F,D)
={}&-1617717835776189959423474389729474523259174290922026987531771334356336309518989285550030273833071044614659831525201714316224621866288174560213656800883169001.
\end{aligned}
\]

Since the leading coefficient of \(F\) is 11 and \(\deg D=3\),

\[
N_{L/\mathbb Q}(D(\alpha))
=\frac{\operatorname{Res}(F,D)}{11^3}<0.
\]

If \(D(\alpha)\) were a square in \(L\), its rational norm would also be a square and could not be negative. It follows that \(D(\alpha)\) is not square in \(L\). Therefore the oriented ordinate is genuinely quadratic over \(L\) and

\[
\boxed{[\mathbb Q(x(T),y(T)):\mathbb Q]=120.}
\]

This is unconditional for **each** nonzero geometric 11-torsion point of the fixed curve.

## Compatibility with CT101 and remaining obstruction

At the source \(p=59\), CT101 proves the x-field splitting \(1^5 5^{11}\) and displays five degree-one source x-roots. At the target \(p=2\), it proves the x-field splitting \(20^3\), and explicitly constructs three distinct oriented Frobenius cycles of length 40, matching the degree-120 global point field: \(40+40+40=120\).

The numerically correct global field degrees **do not supply a computationally cheap map** sending an arbitrary encoded source point to one of those 120 oriented binary points. A fixed compatible source place and binary place must be chosen in the *same global algebra*, and source-dependent root selection must be proved without a scalar-revealing representation. Selection by a known source scalar, an arbitrary matching of residue roots or a full subgroup lookup is not acceptable.

## Independent reproduction and standing

Two exact programs:

- `ct102_global_point_field.py` imports CT101's integral \(\psi_{11}\), checks irreducibility using repeated Frobenius/gcd over \(\mathbb F_{13}\), and computes the exact 63-dimensional Sylvester determinant by fraction-free elimination. Its checks remain active under `python3 -O`.
- `ct102_global_independent.js` independently reconstructs \(\psi_{11}\) with BigInt arrays, checks modular irreducibility using a distinct arithmetic implementation, and independently computes the same integer Sylvester determinant.

From repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct102
python3 -B "$D/ct102_global_point_field.py" > /tmp/ct102-py.json
python3 -O -B "$D/ct102_global_point_field.py" > /tmp/ct102-pyopt.json
node "$D/ct102_global_independent.js" > /tmp/ct102-js.json
node --no-opt "$D/ct102_global_independent.js" > /tmp/ct102-jsopt.json
cmp /tmp/ct102-py.json /tmp/ct102-pyopt.json
cmp /tmp/ct102-py.json /tmp/ct102-js.json
cmp /tmp/ct102-py.json /tmp/ct102-jsopt.json
sha256sum /tmp/ct102-py.json
```

Four local modes produced identical output SHA-256
`a3ef0ab024d4fc8982a56482df4223a3c822ebf2ee253ccc70e21eb333c35a1d`.
Current-head repository CI, separate independent scientific PCC review, Conductor admission and complete optimized rho/index-calculus costs are **not** established by this local certificate. No paid EKS, no protected merge, no fabricated global point lift. [pcc:unverified]

**Next concrete mathematical step:** construct the common global degree-120 oriented quotient with explicit, integrally admissible source/target residue maps and test whether a public \(Q\in E(\mathbb F_{59})[11]\) can be assigned a unique compatible global point without identifying its scalar. Charge polynomial setup, factorization, coefficients and precision, residue matching, binary \(\mathbb F_{2^{40}}\) arithmetic and target DLP in the final cold-cost evaluation.
