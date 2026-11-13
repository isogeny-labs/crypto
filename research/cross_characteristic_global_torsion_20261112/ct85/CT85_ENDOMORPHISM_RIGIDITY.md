# CT85 — Non-CM endomorphism rigidity and the hidden-log diagnostic

**Standing:** [pcc:unverified]. This is a fixed-curve algebraic obstruction for a restricted class of reconstructions, **not** a general lower bound for the selective global-lift problem and **not** a claim of ECDLP speedup.

## Question and result

CT84 reconstructed an input-conditioned rational function (R(X)) from 37-adic source coordinates. Its exact global certificate identified (R(X)=x([2]T)). Does such recovery move the log computation exclusively to the binary destination? **No, when the recovered rational function is certified as the x-map of a globally defined elliptic-curve endomorphism.** The rational function itself exposes the multiplication magnitude.

For the CT80/84 model (E/mathbb{Q}: y^2+y=x^3-x^2-10x-20), its general-Weierstrass invariants are
[
 c_4=496,qquad Delta=-161051=-11^5,qquad
 j(E)= -122023936/161051.
]
As (11
mid496), this rational (j) is nonintegral. A complex-multiplication elliptic curve has algebraic-integral (j); therefore (E) has no CM and
[
 operatorname{End}_{\overline{mathbb Q}}(E)=mathbb Z.
]
Consequently every nonzero global **group endomorphism** (f:E\to E) in characteristic zero is (f=[m]) for some nonzero integer (m). For the local parameter (t=-x/y) at the identity, ([m]^*t=mt+O(t^2)) while (x=t^{-2}+O(t^{-1})). Thus
[
 oxed{x([m]T)= X/m^2+O(1)quad (X=x(T)\to\infty).}
]
If a certified endomorphism's reduced x-map is (A(X)/B(X)) with (deg A=deg B+1), its coefficient at infinity satisfies (operatorname{lc}(A)/operatorname{lc}(B)=1/m^2). It therefore reveals (|m|), and the oriented source ordinate separates (m) from (-m). On an order-seven subgroup this recovers (m\bmod7) before evaluating the map at the binary destination.

**Scope:** This says *if* an input-conditioned recognizer returns an actual **global endomorphism** satisfying (f(P)=Q), it has already produced a representation encoding the source DLP. It does not forbid such a recognizer from being faster than rho, nor does it rule out more general algebraic-number expressions or point-specific functions that are **not** endomorphisms. The claim applies to maps of the entire elliptic curve, not merely assignments to seven torsion points.

## Two exact witnesses, independent of logarithm labels

Set
[
 D=4X^3-4X^2-40X-79,quad
 psi_3=3X^4-4X^3-60X^2-237X-21,
]
and
[
 f_4=2X^6-4X^5-100X^4-790X^3-210X^2-1496X-5821.
]
The exact multiplication-formula identities give
[
egin{aligned}
x([2]T)&=\frac{(3X^2-2X-10)^2+(1-2X)D}{D},\\
x([3]T)&=\frac{X\psi_3^2-Df_4}{\psi_3^2}.
end{aligned}
]

| Certified map | Numerator degree | Denominator degree | Leading ratio | Source x at 9 | Binary x |
|---|---:|---:|---:|---:|---:|
| [1] | 1 | 0 | 1 | 9 | 2 |
| [2] | 4 | 3 | 1/4 | 27 | 17 |
| [3] | 9 | 8 | 1/9 | 28 | 1562 |

Both independent programs reconstruct the integer coefficients and then test reductions in F37 and the *same* CT80 binary residue representation F_(2^12), using the irreducible polynomial (Z^{12}+Z^9+Z^5+Z^2+1). The F_(2^24) ordinate and full seven-target group tests remain covered by CT80; the CT85 scripts check x only, not all oriented binary ordinates. A negative point or an incorrect target embedding can have the same x, so **the x ratio alone cannot certify signed discrete logs**. CT80's fixed-place oriented group-law certificate is required for the ordinate/sign check.

This expands the independently checked global x-map degree control from CT84's [2] case to [3], the remaining nonzero x-coordinate class in the order-seven source subgroup. **It is not Padé/LLL recognition of [3] from a public target**, and the second map was explicitly constructed using classical division polynomials. It supplies a falsifiable diagnostic for future selective-recognition output.

## Cross-factor implication for multi-hop chains (classical conditional gate)

For a *globally* split abelian variety (J\sim E\times B) over a characteristic-zero field, with (B) containing no abelian subvariety isogenous to (E), (operatorname{Hom}(E,B)=0). Every composite (E\to J\to B) vanishes. A nonzero component into (B) requires (B) to contain an (E)-isogenous factor (or a failure of the claimed global splitting/field of definition). This **does not follow from a merely finite-field split reduction** and does not prohibit improvements inside a genuine isogenous factor. It is a filter for proposed genus-2/3 transfers, not new enumeration of all explicit covers.

## Reproduction / observed checks

```sh
D=research/cross_characteristic_global_torsion_20261112/ct85
python3 -B "$D/ct85_rigidity.py" >/tmp/ct85-py.json
python3 -O -B "$D/ct85_rigidity.py" >/tmp/ct85-pyopt.json
node "$D/ct85_rigidity_independent.js" >/tmp/ct85-js.json
node --no-opt "$D/ct85_rigidity_independent.js" >/tmp/ct85-jsopt.json
cmp /tmp/ct85-py.json /tmp/ct85-pyopt.json
cmp /tmp/ct85-py.json /tmp/ct85-js.json
cmp /tmp/ct85-py.json /tmp/ct85-jsopt.json
printf '%s  %s\n' c0e58ecce5818071114aead61c71114a59619ed60dbd4881bb604fb2cc55a148 /tmp/ct85-py.json | sha256sum -c
```

Producer-local Python normal/-O and Node normal/--no-opt completed and matched this SHA-256. The existing CT84 JavaScript syntax defect in the scoped CI was separately repaired in commit `034dfae0`, and its PR body PCC prose-lint citation was corrected; exact-head new workflow status and independent scientific signing are separate gates.

## Prior work and remaining tasks

- Repository CT80, CT82, CT83, CT84; root crypto PR #3877, cryptanalysis PR #1523, and cross-char PR #4074.
- Silverman, *The Arithmetic of Elliptic Curves*, III.4/III.7 (classical morphisms, division polynomials, endomorphisms and formal groups).
- [MathWorld j-invariant formulas](https://mathworld.wolfram.com/j-Invariant.html) and [CM j algebraic integrality](https://www.math.columbia.edu/~calebji/CM-Final.pdf).
- [Snowden, prime-to-characteristic torsion specialization](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html).
- [CT84 Padé/LLL and explicit height limitation](../ct84/CT84_SELECTIVE_PADE.md).
- **KB unavailable:** shared research-KB MCP is absent from this runtime and a checked-out wrapper is unavailable; the repo reports, primary references and GitHub PRs are the fallback. No novelty claim for standard End=Z or multiplication identities.

**Next discriminating task:** reconstruct an arbitrary input's global **point** in the degree-48 field without assuming it comes from a low-degree multiplication endomorphism, and independently validate its minimal polynomial / source residue / binary residue at one fixed pair of primes. Record the search for degree and height bounds and all rejected short relations. On any candidate with a globally certified endomorphism, extract the leading coefficient before charging a separate binary DLP step, to avoid counting an already-recovered log as a speedup. No EKS or other paid compute was used.
