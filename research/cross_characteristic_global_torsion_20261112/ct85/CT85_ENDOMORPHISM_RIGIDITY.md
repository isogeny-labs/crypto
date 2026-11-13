# CT85 — Non-CM endomorphism rigidity and hidden-log diagnostic

**Standing:** [pcc:unverified]. This is a fixed-curve algebraic obstruction for a restricted class of reconstructions, **not** a general complexity lower bound for global lifting.

## Mathematical question and theorem

CT84 recovered a rational function \(R(X)\) from 37-adic source coordinates and certified \(R(X)=x([2]T)\). Does a recovered endomorphism x-map defer all DLP computation to the characteristic-two destination? **No:** after an exact global endomorphism certificate, the x-map's coefficient at infinity reveals the multiplication magnitude.

For the CT80–84 model
\[
E/\mathbb Q:\quad y^2+y=x^3-x^2-10x-20,
\]
the general-Weierstrass invariants are
\[
c_4=496,\qquad \Delta=-161051=-11^5,\qquad j(E)=-122023936/161051.
\]
Since \(11\nmid496\), this rational \(j\) is not an integer. Every CM elliptic curve has algebraic-integral \(j\), so \(E\) is non-CM and
\[
\operatorname{End}_{\overline{\mathbb Q}}(E)=\mathbb Z.
\]
Every global **group endomorphism** \(f:E\to E\) in characteristic zero therefore has the form \([m]\). Using the local parameter \(t=-x/y\) at the identity,
\[
[m]^*t=mt+O(t^2),\qquad x=t^{-2}+O(t^{-1}).
\]
It follows that, as \(X=x(T)\to\infty\),
\[
\boxed{x([m]T)=X/m^2+O(1).}
\]
Consequently, for a **certified endomorphism** x-map \(A(X)/B(X)\), reduced with \(\deg A=\deg B+1\), the leading coefficient ratio equals \(1/m^2\). This reveals \(|m|\), and the oriented source ordinate determines the sign. On \(E[7]\), reduction of \(m\) modulo seven is the source DLP.

This is a **conditional rigidity statement**, not a theorem that a general selective p-adic number-field reconstruction must be expensive or must yield an endomorphism. A different algebraic method may yet be more efficient than rho. What cannot be done is charge the recovered global [m] map as a "free" representation change and then count binary DLP recovery as the first discovery of \(m\).

## Two exact witnesses

Let
\[
D=4X^3-4X^2-40X-79,\quad
\psi_3=3X^4-4X^3-60X^2-237X-21
\]
and \(f_4=2X^6-4X^5-100X^4-790X^3-210X^2-1496X-5821\).
The classical division-polynomial formulas give
\[
\begin{aligned}
x([2]T)&=\frac{(3X^2-2X-10)^2+(1-2X)D}{D},\\
x([3]T)&=\frac{X\psi_3^2-Df_4}{\psi_3^2}.
\end{aligned}
\]

| Map | Numerator degree | Denominator degree | Leading ratio | Source x | Binary x |
|---|---:|---:|---:|---:|---:|
| \([1]\) | 1 | 0 | \(1\) | 9 | 2 |
| \([2]\) | 4 | 3 | \(1/4\) | 27 | 17 |
| \([3]\) | 9 | 8 | \(1/9\) | 28 | 1562 |

The independent Python and JavaScript programs construct these polynomials with separate integer-arithmetic code; both evaluate all three at the chosen source x-coordinate \(9\) over \(\mathbb F_{37}\), and at CT80's **same** binary residue of the global x-generator \(\alpha\) over \(\mathbb F_{2^{12}}\) (irreducible modulus \(Z^{12}+Z^9+Z^5+Z^2+1\)).

This expands the independently checked global x-map witness from CT84's doubling case to tripling, covering the remaining nonzero **x-coordinate class** in the order-seven source subgroup. It **does not** reconstruct tripling using LLL or prove an arbitrary-input recognizer. Full oriented binary \(y\)-coordinates and all seven recovered logs are tested separately by CT80; CT85's independent programs check x-coordinate relations only. Because \(x([m]T)=x([-m]T)\), the orientation/sign is **not** established by the leading-coefficient diagnostic alone.

## Cross-factor obstruction for multi-hop chains (classical conditional statement)

For a globally split abelian variety \(J\sim E\times B\) in characteristic zero, if \(B\) has no abelian factor isogenous to \(E\), then \(\operatorname{Hom}(E,B)=0\). Thus a composition from \(E\) into \(J\), followed by projection to the \(B\)-factor, vanishes. Nonzero cross-factor images require a shared isogeny factor, or else the purported global decomposition/field of definition is invalid. **A splitting observed only after reduction modulo a prime does not establish global splitting**. This is a mathematical filter, not a new computed classification of the native genus-two or genus-three maps in PR #3877.

## Reproduction

\`\`\`sh
D=research/cross_characteristic_global_torsion_20261112/ct85
python3 -B "$D/ct85_rigidity.py" >/tmp/ct85-py.json
python3 -O -B "$D/ct85_rigidity.py" >/tmp/ct85-pyopt.json
node "$D/ct85_rigidity_independent.js" >/tmp/ct85-js.json
node --no-opt "$D/ct85_rigidity_independent.js" >/tmp/ct85-jsopt.json
cmp /tmp/ct85-py.json /tmp/ct85-pyopt.json
cmp /tmp/ct85-py.json /tmp/ct85-js.json
cmp /tmp/ct85-py.json /tmp/ct85-jsopt.json
printf '%s  %s\n' c0e58ecce5818071114aead61c71114a59619ed60dbd4881bb604fb2cc55a148 /tmp/ct85-py.json | sha256sum -c
\`\`\`

All four producer-local modes completed and matched the pinned SHA256. These are locally run checks, **not** signed independent scientific PCC certification.

### Governance and sources

Prior repository reports: CT80–84; root [crypto PR #3877](https://github.com/isogeny-labs/crypto/pull/3877), [cryptanalysis PR #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), and [cross-characteristic PR #4074](https://github.com/isogeny-labs/crypto/pull/4074).
Classical background: Silverman, *Arithmetic of Elliptic Curves*, Chapter III; [Weierstrass j formula](https://mathworld.wolfram.com/j-Invariant.html); [CM j integrality](https://www.math.columbia.edu/~calebji/CM-Final.pdf); [prime-to-characteristic torsion reduction](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html). No novelty claim for endomorphism rigidity, CM-integrality or division-polynomial formulas.

**KB unavailable** in this connector runtime; no research-KB MCP function or checked-out wrapper was accessible. Prior repo reports and referenced primary sources substituted, but this is an incomplete cross-repo survey.

The CT84 scoped workflow initially failed because its independent JavaScript file had a syntax error at the binary source-curve assertion. Commit \`034dfae0\` repaired the error. Separately the prior PCC gate failed because one claim-bearing PR-body sentence lacked the mandated unverified tag; the sentence was corrected without weakening the lint. Exact-head CI/review and external scientific PCC verdict must be checked separately; no paid EKS or GPU compute, protected merge or speedup claim is made.

**Next accepted experiment:** reconstruct arbitrary input-conditioned \(K\)-rational torsion points without assuming a small multiplication-polynomial degree, validate source and binary specializations with the **same fixed pair of places**, and account for coefficient heights, field construction, root selection, evaluation, and one-target solving. Whenever a globally certified endomorphism is returned, extract \(|m|\) from its leading coefficient before separately charging a destination DLP.
