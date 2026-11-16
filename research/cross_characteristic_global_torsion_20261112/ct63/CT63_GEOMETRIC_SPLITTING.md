# CT63 — the CT62 order-31 Jacobian splits after an exact quintic extension

**Standing:** [pcc:unverified]. This is a structural extension of CT62, **not** a global cross-characteristic point transfer or an ECDLP speedup.

## Exact Frobenius computation and two-sided Hom gate

Retain the CT62 smooth curve over \(k=\mathbb F_4=\mathbb F_2(\omega)\), \(\omega^2+\omega+1=0\):
\[
 C:\quad y^2+y=x^5+\omega x^3,\qquad J=\operatorname{Jac}(C).
\]
CT62 and the new independent counters give \(N_1=\#C(\mathbb F_4)=7\), \(N_2=\#C(\mathbb F_{16})=21\).
For \(q=4\), the Frobenius power sums are \(s_1=q+1-N_1=-2\), \(s_2=q^2+1-N_2=-4\).
Hence the middle coefficient is \((s_1^2-s_2)/2=4\), and the reciprocal genus-two Weil polynomial is
\[
 \boxed{F_J(X)=X^4+2X^3+4X^2+8X+16=16\Phi_5(X/2).}
\]
Its roots are \(2\zeta_5^j\), \(j=1,2,3,4\), with \(\zeta_5\) primitive. Consequently,
\[
 F_{J,n}(X)=
 \begin{cases}
 2^{4n}\Phi_5(X/2^n),&5\nmid n,\\
 (X-2^n)^4,&5\mid n,
 \end{cases}
\]
for the base change to \(\mathbb F_{4^n}\). The first polynomial is irreducible over \(\mathbb Q\) when \(5\nmid n\). By Honda–Tate/Poincaré reducibility, \(J_{\mathbb F_{4^n}}\) is simple in these extensions and
\[
 \boxed{\operatorname{Hom}_{\mathbb F_{4^n}}(E,J)=
 \operatorname{Hom}_{\mathbb F_{4^n}}(J,E)=0\quad(5\nmid n)}
\]
for **every** elliptic \(E/\mathbb F_{4^n}\).

Conversely, the explicitly defined elliptic curve
\[
 E_0/\mathbb F_4:\quad y^2+y=x^3+\omega
\]
has exactly one rational point and thus Frobenius characteristic polynomial \((X-2)^2\), so its \(4\)-Frobenius equals multiplication by \(2\). For \(5\mid n\), \(E_{0,\mathbb F_{4^n}}^2\) and \(J_{\mathbb F_{4^n}}\) have the same Weil polynomial \((X-2^n)^4\); **Tate's isogeny theorem supplies a nonzero isogeny over \(\mathbb F_{4^n}\)**. Both cross-factor Hom groups are therefore nonzero in those extensions. The **first** extension where elliptic factors and nonzero cross-factor maps can appear is degree \(5\) over \(\mathbb F_4\), i.e. the field \(\mathbb F_{1024}\). This existence result does **not** present an explicit isogeny formula or its computational cost.

The characteristic polynomial identity also gives \(F_J^5=[32]\) in \(\operatorname{End}(J)\) (use Cayley–Hamilton for Frobenius). Thus all geometric \(31\)-torsion is rational over \(\mathbb F_{1024}\):
\[
 \boxed{J(\mathbb F_{1024})=J[31]\cong(\mathbb Z/31\mathbb Z)^4,\quad
 \#J(\mathbb F_{1024})=31^4=923521.}
\]
In contrast the original \(J(\mathbb F_4)\) is cyclic of order 31. Extension changes rank, not just point count. A transfer must specify which oriented 31-line it carries, and no source input lifting is supplied by this new splitting fact.

## Independently counted controls

Separate Python and JavaScript programs directly enumerate all \(x\in\mathbb F_{4^n}\), \(n=1,\dots,5\). Python computes the absolute trace of each Artin–Schreier right-hand side; JavaScript independently builds the solution-image table \(y\mapsto y^2+y\). The Python program separately checks irreducibility of the five field moduli; both validate the \(\mathbb F_4\) embedding.

| \(n\) | \(4^n\) | \(\#C(\mathbb F_{4^n})\) | \(\#E_0(\mathbb F_{4^n})\) | \(\#J(\mathbb F_{4^n})\) |
|---:|---:|---:|---:|---:|
| 1 | 4 | 7 | 1 | 31 |
| 2 | 16 | 21 | 9 | 341 |
| 3 | 64 | 73 | 49 | 4681 |
| 4 | 256 | 273 | 225 | 69905 |
| 5 | 1024 | 897 | 961 | 923521 |

All four local execution modes passed with identical JSON, output SHA256 **23b20c8202a9ce29a42abea314b569193d3bb86c4f40e1f7f613970ccdee10ae**. The CI workflow independently repeats these exact counts and mode comparisons.

Run:
~~~sh
D=research/cross_characteristic_global_torsion_20261112/ct63
python3 -B "$D/ct63_frobenius.py" > /tmp/ct63-py.json
python3 -O -B "$D/ct63_frobenius.py" > /tmp/ct63-py-O.json
node "$D/ct63_independent.js" > /tmp/ct63-js.json
node --no-opt "$D/ct63_independent.js" > /tmp/ct63-js-noopt.json
cmp /tmp/ct63-py.json /tmp/ct63-py-O.json
cmp /tmp/ct63-js.json /tmp/ct63-js-noopt.json
cmp /tmp/ct63-py.json /tmp/ct63-js.json
~~~

## Prior art and strict limits

This is a consequence of classical Frobenius, Honda–Tate and Tate isogeny theory, not a new theorem. Compare CT56/CT62 source witnesses, CT61 variable binary fields, active #4015 and root #3877. Primary contextual sources: Maisner–Nart–Howe, *Abelian Surfaces over Finite Fields as Jacobians* (2002), https://doi.org/10.1080/10586458.2002.10504478 ; Howe–Zhu, *On the Existence of Absolutely Simple Abelian Varieties* (2002), https://doi.org/10.1006/jnth.2001.2697 ; CT62's Waterhouse/Bröker–Howe–Lauter–Stevenhagen references. Research-KB MCP and checkout-based wrapper were not accessible in this runtime; fallback GitHub searches across crypto, cryptanalysis and crypto-autoresearcher found no duplicate exact Φ5 computation, but that is **not** a novelty certificate.

**Next computational gate:** extract an explicit F1024 isogeny \(E_0^2\leftrightarrow J\) with its 31-torsion matrix, identify whether the original rational 31-line maps into a single elliptic factor, and measure full field/isogeny/DLP cost. Independently, obtain a uniform, compatible global lift of arbitrary encoded source \(P,Q\), their prime ideals and orientations across characteristics. The local splitting does not provide that lift, an efficient attack, or any cold-cost improvement.

No paid compute, protected merge, or edits to concurrently authored native Ciani/Cantor code. [pcc:unverified]
