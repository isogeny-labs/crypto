# Chart-complete Frobenius-sector polynomial system for Semaev relations

**2026-11-23 | PR #5058 | [pcc:unverified], producer-only.** This follows **merged** PR #5044 and is scoped solely to \(S_{m+1}(x_1,\ldots,x_m,X)=0\). No factor-base map \(L\), no complete ECDLP experiment, no unspecialized Gröbner or first-fall advantage is claimed.

## Exact construction over \(\mathbb F_p\)

Let \(p>3\) be prime and \(E:y^2=x^3+ax+b\) nonsingular. Choose nonsquare \(d\). For each of the \(N=m+1\) input abscissae \(x_i\), introduce \(y_i\) and Boolean \(s_i\). The degree-three leaf constraints are

\[
s_i(s_i-1)=0,\qquad
(1+(d-1)s_i)y_i^2-(x_i^3+ax_i+b)=0.
\]

At \(s_i=0\), the leaf is on \(E(\mathbb F_p)\); at \(s_i=1\), it lies on the rational twist chart \(d\,y_i^2=f(x_i)\), corresponding via \((x,y)\mapsto(x,\omega y)\), \(\omega^2=d\), to anti-Frobenius points in \(E(\mathbb F_{p^2})\). Rational \(2\)-torsion \(y=0\) belongs to either chart harmlessly.

For each sector use point variables \((x,y,z)\) with \(z\in\{0,1\}\), \(z=0\Leftrightarrow(x,y)=(0,0)\) denoting the identity. In addition to the Boolean and zero-state equations, enforce
\[
 z(\delta y^2-x^3-ax-b)=0,\quad \delta=1\text{ or }d.
\]
This has total degree four, including \(z\). Each of the two length-\(N\) point-sum chains uses **five disjoint guarded charts** for \(P+Q\):

1. \(P=\mathcal O\), return \(Q\).
2. \(P\ne\mathcal O\), \(Q=\mathcal O\), return \(P\).
3. Both finite with \(x_P\ne x_Q\): witness \(\lambda=(y_Q-y_P)/(x_Q-x_P)\), guard \((x_Q-x_P)u=1\).
4. Finite equal points with \(y_P\ne0\): witness \(\lambda=(3x_P^2+a)/(2\delta y_P)\), guard \(y_Pu=1\).
5. Finite inverse points with \(x_P=x_Q\), \(y_P+y_Q=0\): return \(\mathcal O\).

Exactly one branch selector equals one. Each branch polynomial is multiplied by its Boolean selector; all equations, **including inverse guards**, have total degree \(\le4\). Chord and tangent output formulas use \(x_R=\delta\lambda^2-x_P-x_Q\), \(y_R=\lambda(x_P-x_R)-y_P\).

Finally require sector sums \(A,B\) to meet at the identity or the **same** rational \(2\)-torsion point. This is expressed without enumerating torsion:

\[
z_A=z_B,\quad z_A(x_A-x_B)=0,\quad z_Ay_A=0,\quad z_By_B=0.
\]

Because \(\mathbb F_p\cap\omega\mathbb F_p=\{0\}\), these conditions express precisely \(A=B=T\in E(\mathbb F_p)[2]\). Applying Frobenius to \(A+B=\mathcal O\) gives \(A-B=\mathcal O\), hence this condition is necessary; it is sufficient since \(A+B=2T=\mathcal O\).

**Projected-root theorem (set-theoretic).** For \(x_i,X\in\mathbb F_p\), the polynomial circuit has an \(\mathbb F_p\)-valued witness iff \(S_{m+1}(x_1,\ldots,x_m,X)=0\). The five guards are exhaustive, ensure sound group addition, and permit all infinity and torsion branches. This is not an identity of the unaugmented elimination ideal with the principal Semaev ideal.

### Finite-field enforcement without high-degree generators

A Gröbner ideal over \(\overline{\mathbb F}_p\) must not treat the Boolean split as a statement about rational Frobenius eigenspaces without field membership conditions. The optional **field_lift=True** path adds \(x_i^p=x_i\) and \(y_i^p=y_i\), but expands each via a binary square-and-multiply chain of quadratic equations (roughly \(2\lfloor\log_2p\rfloor\) equations per leaf coordinate). Only the *leaf* coordinates need field lifts: group sums, inverse witnesses on active charts, and selectors are forced by rational inputs; inactive slope variables do not affect the projection. Consequently the **radicals of the projected finite-field zero-set ideals** coincide after adjoining input-field equations. Multiplicities and the original characteristic-zero/unsliced elimination scheme are not preserved by this assertion.

The total size is \(29m+9\) polynomial variables and \(100m+20\) generators without field lifts; the lift adds \(O(m\log p)\) variables and generators. The degree-four bound is a generator-degree invariant, **not** a first-fall, Macaulay, Castelnuovo–Mumford regularity, or F4-solving-degree bound. The Boolean/chart and Frobenius-lift overhead may make Gröbner solving much slower than the monolithic control.

## Producer-only exact gates

Reproduction:

\`\`\`bash
PYTHONPATH=. python -m unittest -q summation_split.test_sector_complete
\`\`\`

Local Python/SymPy fallback: **3 tests passed**; no Sage, Singular, FGb, msolve, EKS execution, independent replay, or signed PCC standing for this follow-up.

- Six point-law cells \((p,\delta)\) across \(p=5,7,11\), both base and twist: **514/514 ordered point pairs**, including identity, chord, tangent and inverse branches, gave the exact independently computed group sum and satisfied all chart equations.
- Four small \((p,a,b,m)=(5,0,1,3),(7,1,1,3),(11,1,1,3),(11,1,1,4)\) cells: **50/50 random specialized input slices per cell**; *all* \(X\in\mathbb F_p\) were compared by exact fixed-degree Semaev resultant against a signed witness search, with **228 positive full-circuit certificates** checked polynomial by polynomial. This is a semantic witness oracle, **not a Gröbner elimination run**.
- Six Prime31 cells: \(p=2^{31}-1\), \((a,b)=(1,1),(1,4)\), \(m=3,4,5\): **50/50 constructed mixed-sector Semaev-positive instances per cell**, including nonidentity rational \(2\)-torsion on \((1,1)\). The fixed-degree resultant at the final \(X\) was zero, and every generator of the field-lifted polynomial system evaluated to zero: **300/300 certificates**. These are *positive constructed fixtures*, not 50 random exact root-set recoveries; the latter were separately reported in #5044.

| \((p,a,b,m)\) | Field-lifted variables | Generators | Max generator degree | Measured \(d_{F4}\) |
|---|---:|---:|---:|---|
| \((5,0,1,3)\) | 112 | 344 | 4 | not measured |
| \((7,1,1,3)\) | 120 | 352 | 4 | not measured |
| \((11,1,1,3)\) | 128 | 360 | 4 | not measured |
| \((11,1,1,4)\) | 165 | 470 | 4 | not measured |
| \((2^{31}-1,1,1,3)\) | 568 | 800 | 4 | not measured |
| \((2^{31}-1,1,1,4)\) | 715 | 1,020 | 4 | not measured |
| \((2^{31}-1,1,1,5)\) | 862 | 1,240 | 4 | not measured |

## Falsifiers and next experiment

The smallest failure of the *incorrect O-only* sector join remains \(p=5\), \(E:y^2=x^3+1\), \(S_4(0,1,2,3)=0\); shared torsion must be \((4,0)\). Removing the nonzero-denominator guards permits spurious branches, while removing the identity/inversion chart loses valid points. The full guarded construction fixes those errors **at the level of finite-field point sets**.

The high cost of chart completeness is an immediate negative control: for Prime31 and \(m=5\), **862 unknowns** and **1,240 equations** replace one degree-\(16\)-per-coordinate summation polynomial. No Gröbner advantage follows from that trade. Next compare this *exact same unspecialized slicing* against monolithic \(S_{m+1}\), complete projective S3 chain, and symmetric and invertible-linear controls using Sage/Singular F4 or msolve, after \(50\) exact projected-set gates per configuration. Track chart overhead, first fall under a declared definition, maximum Macaulay degree, memory and full wall time; **do not make a solving-degree-vs-\(m\) plot from generator degree**.

## Prior art

- Semaev, [*Summation polynomials and the discrete logarithm problem*](https://eprint.iacr.org/2004/031), 2004: geometric zero-sum semantics and recursive resultant.
- Renes, Costello and Batina, [*Complete addition formulas for prime order elliptic curves*](https://eprint.iacr.org/2015/1060), EUROCRYPT 2016: complete short-Weierstrass additions predate this circuit. Their efficient formulas assume relevant completeness hypotheses; applying them to arbitrary quadratic-twist sector curves requires separate checking.
- Kosters and Yeo, [*Notes on summation polynomials*](https://arxiv.org/abs/1503.08001), 2015: caution against inferring solving degree from first fall.
- Existing repository findings: \`summation_split/FINDINGS.md\`, \`summation_split/SECTOR_FINDINGS.md\` (merged PRs #5023/#5044); internal \`research/pdp_split_dreg_20261011/README.md\`.

Research-KB service and repository wrapper unavailable in this runtime; fallback primary sources and the two existing PRs were read. This guarded circuit is an implementation of **classical** elliptic addition semantics plus the established Frobenius-sector invariant, not a novelty or complexity theorem.
