# Structured selector circuit for the B677 RR5 chart

Date: 2026-11-23 UTC. Continuation: PR #4967. This note derives an exact public-input formulation and records its proof obligations and implementation limits. It does not record a new solver execution, establish a practical elimination bound, or assert novelty. The arguments are written mathematics and same-team source review: no Lean-kernel proof, signed independent verification, or new evaluated certificate covers them. [pcc:unverified]

## 1. Scope and admission conditions

Work over an odd prime field \(K=\mathbb F_p\) on a nonsingular curve
\[
E:y^2=f(x)=x^3+Ax+B.
\]
Require that the **full group** \(E(K)\) have odd order. An odd-order subgroup alone is insufficient. For the existing PRIME31 fixture, the original [`check_curve`](../../prime31_ic_20261115/native/lib.rs) checks a prime-order nonidentity generator and a Hasse-bound condition establishing that its order is the full curve order. The protocol must retain the actual fixture and admission evidence.

Let \(S\subset K\) be the admitted factor-base abscissae, with one supplied nonzero ordinate \(y_x\) for each \(x\in S\). Both signs are allowed; put
\[
h(X)=\prod_{x\in S}(X-x).
\]
The intended canonical B677 fixture is the existing depth-ten Dickson base of 509 on-curve abscissae together with its frozen 168 extra abscissae. It is not the first 677 points of a natural-x scan. Counts, constants, ordering and file hashes belong to the fixture receipt; this note does not replace that check.

The structured selector is
\[
H(X)=\bigl(D_{1024}(X)-\tau\bigr)G\bigl(f(X)^7\bigr),
\qquad G(Z)=\prod_{i=1}^{8}(Z-c_i^2),
\]
where \(D_1=X\) and \(D_{2m}=D_m^2-2\). The intended degrees are 1024 and \(8\cdot21=168\), so \(H\) has degree 1192. Before using the equivalence below, establish all of the following:

1. \(H\) is monic, squarefree and split completely over \(K\).
2. Its roots with a nonzero curve ordinate are **exactly** \(S\):
   \[
   S=\{x\in K:H(x)=0,\ f(x)\in(K^\times)^2\}.
   \]
3. The supplied base has distinct abscissae and valid chosen ordinates; no additional root with \(f(x)=0\) is silently admitted. Full odd curve order in particular excludes rational two-torsion.
4. The public target \(R=(r,s)\) is finite, lies on \(E\), and satisfies \(H(r)\ne0\).

For the intended factorization, the original Dickson builder checks that its raw fibre has exactly \(2^{10}\) distinct field roots and excludes terminal values \(\pm2\). Each extra factor must have its full 21 distinct roots, and the eight fibres and Dickson fibre must be disjoint. Distinct \(c_i^2\), nonzero constants and degrees alone are not substitutes for the complete split/squarefree check. The identity \(\chi(f(x))^7=\chi(c_i^2)=1\) shows that any field root of a nonzero extra fibre has a square curve value; it does not establish that all 21 roots lie in the field.

Under the exact root-set condition, a valid finite target outside the signed base has \(H(r)\ne0\): a field root with that target ordinate would already belong to \(S\). Keeping the explicit selector evaluation is useful admission evidence. Failed selector assumptions are invalid/unsupported inputs, not empty relation sets. Public base construction and its verification are setup work; they do not authorize tuple enumeration during relation extraction.

## 2. RR5 norm and denominator

Use the original three coefficient parameters \(b,c,d\), and define
\[
V=X+b,\qquad L=cX+d,\qquad D_R=X^2+rX+r^2+A,
\]
\[
\ell=(X-r)L+(s+y)V,\qquad
U=D_RV^2-(X-r)L^2-2sLV.
\]
The coefficient of \(xy\) in \(\ell\) is one, so its pole divisor is exactly \(5(O)\). It vanishes at \(-R\). Since \(s^2=f(r)\),
\[
fV^2-\bigl((X-r)L+sV\bigr)^2=(X-r)U. \tag{1}
\]
The polynomial \(U=X^4+u_3X^3+u_2X^2+u_1X+u_0\) is monic, with
\[
\begin{aligned}
u_3&=2b+r-c^2,\\
u_2&=b^2+2rb+r^2+A+rc^2-2cd-2sc,\\
u_1&=rb^2+2b(r^2+A)+2rcd-d^2-2s(cb+d),\\
u_0&=(r^2+A)b^2+rd^2-2sbd.
\end{aligned}
\]

**Denominator lemma.** Over an algebraic closure, every solution of \(U\mid H\) satisfies \(\gcd(U,V)=1\).

Indeed,
\[
U(-b)=(b+r)(d-bc)^2.
\]
If \(-b\) were a root of \(U\), either \(b=-r\), contradicting \(H(r)\ne0\), or \(d=bc\). In the latter case \(L=cV\), and
\[
U=V^2\bigl(D_R-(X-r)c^2-2sc\bigr),
\]
contradicting squarefreeness of a divisor of \(H\). No saturation variable is required under the admitted conditions.

Consequently, the four distinct roots \(z_j\in K\) of \(U\) have well-defined lifts
\[
t_j=-s-\frac{(z_j-r)L(z_j)}{V(z_j)}.
\]
Equation (1) gives \(t_j^2=f(z_j)\). The points \(P_j=(z_j,t_j)\), together with \(-R\), are five distinct finite zeros of \(\ell\), accounting for its whole zero divisor. The elliptic divisor/group-law relation gives \(P_1+\cdots+P_4=R\). This argument initially takes place over the algebraic closure; rationality requires the next lemma.

Conversely, a signed four-point decomposition with four distinct abscissae in \(S\) determines the principal divisor \(\sum_j(P_j)+(-R)-5(O)\). Its function is unique up to scalar. Its pole has order five, so normalizing the nonzero \(xy\) coefficient to one gives a unique \((b,c,d)\) above. This is the same RR5 chart and divisor argument as [the parent mathematical note](../MATHEMATICS.md).

## 3. Four leaves exclude anti-Frobenius points

Let \(F\) be the \(p\)-power Frobenius and define
\[
E^+=\{P:F(P)=P\}=E(K),\qquad
E^-=\{P:F(P)=-P\}\subset E(\mathbb F_{p^2}).
\]
These are subgroups. Their intersection is \(E(K)[2]=\{O\}\), because the full rational group has odd order.

Every finite point whose abscissa is in \(K\) has ordinate either in \(K\), or in its quadratic extension with \(t^p=-t\). Since rational two-torsion is absent, a nonidentity leaf belongs to exactly one of \(E^+\) and \(E^-\).

**Four-leaf lemma.** Suppose four finite points have distinct abscissae among the roots of \(H\), and sum to a nonidentity \(R\in E^+\) with \(H(r)\ne0\). Then all four points belong to \(E^+\).

To prove this, let \(T\) be the sum of the anti-Frobenius leaves. It lies in \(E^-\). Subtracting the rational leaves from \(R\) also puts \(T\) in \(E^+\), so \(T=O\). If the number of anti-Frobenius leaves is: [pcc:unverified]

- One: that leaf would be \(O\), impossible for a finite leaf.
- Two: they must be negatives of one another, giving the same abscissa.
- Three: the remaining rational leaf must equal \(R\), contradicting \(H(r)\ne0\).
- Four: the complete sum is \(O\), contradicting the target assumption.

Thus the number is zero. The exact on-curve root-set condition then places every leaf in the canonical signed base. The normalized principal function is fixed by Frobenius, since its divisor is rational and its \(xy\) coefficient is one; hence \(b,c,d\in K\).

This proves the assertion for **all geometric coefficient solutions**, not just solutions already assumed to have field-valued parameters. It relies on four leaves, distinct abscissae, full odd rational-group order, and the target exclusion. It must not be transferred to a longer RR construction: three anti-Frobenius leaves can sum to zero while several rational leaves sum to the target. [pcc:unverified]

## 4. Radical ideals and equality of schemes

For \(P=h\) or \(P=H\), let \(I_P\subset K[b,c,d]\) be generated by the four coefficients of \(P\bmod U\). The preceding arguments give the same geometric zero set for \(I_h\) and \(I_H\). To claim more than equality of points, reducedness must also be established.

Fix a geometric solution and write \(P=UW\). Squarefreeness gives \(\gcd(U,W)=1\). An infinitesimal parameter variation preserving the four remainder equations obeys
\[
\delta U\,W+U\,\delta W=0.
\]
Reducing modulo \(U\) forces \(\delta U=0\), because \(W\) is invertible in the quotient and \(\deg(\delta U)\le3\). Differentiating (1) and evaluating at a leaf gives
\[
0=2t_jV(z_j)\,\delta\ell(P_j).
\]
Both displayed factors are nonzero. Therefore \(\delta\ell\) vanishes at the four leaves and at \(-R\). Its normalized \(xy\) coefficient cannot vary, so \(\delta\ell\in L(4O)\); five distinct finite zeros force it to be zero. Comparing its \(y,x^2,x\) coefficients yields \(\delta b=\delta c=\delta d=0\).

There are finitely many possible signed four-point divisors, and their normalized functions are unique. The parameter schemes are therefore zero-dimensional and have zero tangent space at every geometric point. Their local Artinian rings are fields, so both ideals are geometrically radical. Equality of their geometric zero sets now implies
\[
I_H\overline K[b,c,d]=I_h\overline K[b,c,d],
\qquad I_H=I_h.
\]
The last equality follows by field-extension contraction. Thus replacing the degree-677 filtered polynomial by this degree-1192 structured selector preserves the affine scheme on the admitted chart. Repeated selector roots would invalidate the reducedness argument; merely filtering bad decoded assignments would not restore this proof.

## 5. Exact quadratic circuit lift

Introduce four variables \(z_i\) and equations \(z_i-u_i(b,c,d)=0\). These equations have degree at most two. Write
\[
\widehat U=X^4+z_3X^3+z_2X^2+z_1X+z_0.
\]
Every gate operates on residues of degree at most three. For a modular multiplication \(A B\bmod\widehat U\), introduce
\[
Q=q_0+q_1X+q_2X^2,\qquad C=c_0+c_1X+c_2X^2+c_3X^3
\]
and impose **all seven** coefficient equations of
\[
AB-\widehat UQ-C=0. \tag{2}
\]
For coefficient index \(k=0,\ldots,6\), explicitly impose
\[
\sum_{i+j=k}a_i b_j-
\sum_{i+j=k}z_iq_j-c_k=0,
\]
where \(z_4=1\), out-of-range \(z_i,q_j\) are zero, and \(c_k=0\) for \(k>3\). A generic gate adds seven variables and seven equations. If input coefficients are affine expressions in existing variables, every equation is at most quadratic.

The high coefficients determine \(q_2,q_1,q_0\) triangularly with unit pivots; the remaining four determine \(C\). Monic division works over any coefficient algebra, including one with nilpotents. Therefore each acyclic gate is a polynomial graph extension, not just an existential relaxation over fields. Degree drops at special inputs cause no problem: quotient coefficients may vanish, and no gate-input leading coefficient is inverted.

Starting from \(K[b,c,d]\), adding the four coefficient lifts and every complete gate preserves the coordinate ring as a graph. Requiring the final four selector-residue coefficients to be zero gives a ring isomorphic to \(K[b,c,d]/I_H\), hence to the original B677 chart algebra. Omitting even one coefficient of (2), allowing a higher-degree quotient, or dropping final residual coefficients removes this guarantee.

### Structured evaluation schedule

All operations below occur modulo \(\widehat U\); additions and scalar combinations can be inlined.

- **Dickson selector:** \(D_2=X^2-2\) is already a residue, and
  \[
  D_4\bmod\widehat U=(2-z_0)-z_1X-(z_2+4)X^2-z_3X^3
  \]
  is affine in the lifted coefficients. Continue by modular squaring and subtracting two to obtain \(D_{1024}-\tau\).
- **Seventh power:** \(f\) is a cubic residue. An addition chain \(f^2,f^3,f^6,f^7\) computes the required power using modular multiplications.
- **Degree-eight polynomial:** write \(G(Y)=\sum_{i=0}^8g_iY^i\), with \(g_8=1\). Compute \(Y^2,Y^3\) and use
  \[
  G(Y)=\bigl((Y^2+g_7Y+g_6)Y^3
      +(g_5Y^2+g_4Y+g_3)\bigr)Y^3
      +(g_2Y^2+g_1Y+g_0).
  \]
  This explicit blocked identity uses four nonscalar modular multiplications including the two powers. It is a specialization of the Paterson–Stockmeyer evaluation method [PS].
- **Union:** multiply the Dickson residue by \(G(f^7)\) and constrain all four coefficients of its remainder to zero.

This specifies algebraic dependencies, not a promised implementation count. Constant folding, shared expressions, inlined initial residues and final-zero outputs change allocation counts. The builder must emit its actual gate, variable, equation, term and degree counts before a solver invocation. The count of circuit multiplications is not the cost of solving the lifted equations.

## 6. Extraction contract and downstream APIs

The solver receives only public curve/base/selector data, the target and frozen limits. It must preserve complete original gate equations through specialization, branch only on algebraically derived coefficient univariates, and evaluate every terminal **full assignment** before projecting to \((b,c,d)\). Quartic factorization, signed-base membership, and literal original-curve sum replay remain mandatory.

A proved nonzero constant or an exhausted algebraic root cover can justify emptiness. Missing elimination, incomplete basis construction, an output limit, resource interruption, failed gate replay, or rejected signed lift must remain incomplete. A failure to find a separating coordinate is not an empty result. Factoring the public selector and trying each base root as a chosen leaf is a domain scan, even if presented through a polynomial-factorization API. [pcc:unverified]

Existing code supplies useful components, with important boundaries:

| API | Actual requirement and limitation |
| --- | --- |
| [`f4_fp::solve_visit_traced`](../../../src/cryptanalysis/f4_fp.rs) | Retains incoming generators, exposes branch events, and checks full original assignments. Root-only refusal can preserve completeness without a fully finished basis, provided every branch is covered. Native allocation limits remain a separate implementation obligation. |
| [`zero_dim_la::fast_stage`](../../../src/cryptanalysis/zero_dim_la.rs) | Requires an already valid monic grevlex zero-dimensional basis. Runtime admission is 2–8 variables, prime below \(2^{32}\), leading exponents below 120, and staircase at most 65,536. It returns `None` on unsupported shape, short minimal polynomial, zero read-out denominator or failed original replay. |
| [`jv_cover::linear_stage_from_basis`](../../../src/cryptanalysis/jv_cover.rs) | Offers sparse/dense downstream recovery, but its dense implementation uses eight-bit-per-variable packing in a `u64` and a 4,096-element staircase cap. Do not pass a many-variable circuit into this fallback. It has incomplete/nonseparating outcomes that callers must retain. |
| [`f4_fp::certified::try_quotient_basis`](../../../src/cryptanalysis/f4_fp/certified.rs) and compact variant | These recognize supported certified root/coordinate graphs, rather than solve arbitrary circuits. Adapter admission caps variables at 16; the quotient core additionally requires a small supported input and a recognized certified univariate vertex. No applicable RR5 recognition has been established. |

Sparse FGLM changes order or reads roots after obtaining the finite quotient and its multiplication action; it does not solve the present front-end elimination problem [FM]. In particular, original-input point replay alone cannot validate the completeness of an uncertified basis supplied to a linear stage. A future FGLM route should first produce a certified basis or equivalent quotient representation on the original three parameters.

## 7. Scaling and a possible structured elimination path

The direct monic cofactor at B677 has 676 variables. Expanding its unit-pivot elimination back into three parameters merely recovers the four remainder polynomials, with degree bound \(2(677-3)\). A dense three-variable polynomial at that degree has \(\binom{2(677-3)+3}{3}\) possible monomials. Keeping a linear recurrence or a circuit avoids storing that expansion; it does not prove cheap elimination. [pcc:unverified]

The selector circuit exposes small local multiplication blocks and removes the need for one variable per coefficient of the degree-673 cofactor. Nevertheless, if its actual variable count is \(n\), a generic total-degree-\(d\) monomial universe has \(\binom{n+d}{d}\) elements. No claim that those monomials all occur, or that a small fixed \(d\) suffices, is made. Raw homogenized graph equations can have auxiliary directions at infinity. Native construction, matrix, coefficient-storage and wall limits must apply before the corresponding allocations; a trace emitted after an F4 matrix has been built cannot prevent that allocation.

One exact next architecture is block elimination with a parameter-only border basis. Keep gate equations as sparse blocks with exact derivation provenance; eliminate monic local quotient variables using their unit pivots without expanding all global parameter expressions. Seek an order ideal of monomials in \(b,c,d\), and preserve membership certificates for each proposed border rewrite. No currently inspected production API implements this entire front end, and no bounded-width or degree guarantee follows from the circuit graph.

For a proposed finite border algebra, completeness requires both ideal containments:

1. Every border rewrite must have an exact membership certificate in the original lifted gate ideal, hence in its parameter elimination ideal.
2. Border reduction must be closed and consistent; its parameter multiplication matrices must commute, and the **entire original selector circuit** must vanish in that quotient.

The second condition alone is insufficient: it permits a proper quotient that retains only some solutions. Commuting matrices and successful candidate replay cannot exclude that failure. With both containments, the full chart algebra is established and downstream univariate extraction becomes justified. Matrix evaluation of the circuit avoids expanding its high-degree parameter polynomials, but its matrix work and storage must be charged.

Let \(\delta=\dim_K K[b,c,d]/I_H\). Reducedness identifies \(\delta\) with the number of signed chart rows. The parent bound \(16\binom{|S|}{4}\) is valid; a tighter elementary bound is \(2\binom{|S|}{3}\), since a signed distinct-three subset determines at most one fourth point, and each four-row is counted four times. This is a counting bound, not permission to enumerate triples. Neither bound supplies a small practical quotient. Three dense parameter multiplication matrices already require \(O(\delta^2)\) field entries; classical dense elimination has cubic dependence on \(\delta\). A small observed output or a heuristic expected yield cannot replace a certified quotient-dimension bound.

## 8. Small discriminating correctness controls

Before interpreting a B677 solve, separate constructor correctness from extraction success. A narrow native constructor check can compare the modular circuit with independent scalar monic division of the explicit selector at fixed coefficient triples, then mutate every quotient/remainder coefficient and require rejection by the full equations. Use canonical selector constants and retain this as a construction check only.

A small control specifically for the new nonrational-leaf argument uses the existing \(\mathbb F_{11}\) curve \(y^2=x^3+x+6\), its four base abscissae \(\{2,3,5,7\}\), and target \((8,3)\). Adjoin nonresidue abscissae 0 and 1 to a squarefree split test selector. In \(\mathbb F_{121}=\mathbb F_{11}[i]/(i^2-2)\), their ordinates can be taken as \(5i\) and \(2i\). A test-only independent group-law oracle checks all \(16\binom64\) raw signed quartets, including the anti-Frobenius points, and compares their target-summing set with the original four-point signed oracle. The expected equality exercises the precise exclusion that a field-only decoder cannot test. The candidate receives no oracle tuple or coefficient solution.

That test selector validates the algebraic extension lemma; it is not a substitute for checking the canonical Dickson/y7 selector data or for an actual circuit extraction. Target-in-selector and repeated-selector-factor mutations must fail admission. A bounded circuit solver must satisfy the same all-row equality and `Complete`/`Empty` requirements as existing controls; an inconclusive result remains a failed completion gate. These are proposed controls, not executed observations in this note.

## Sources and standing

- Parent [RR5 mathematical note](../MATHEMATICS.md), [original arithmetic/base builder](../../prime31_ic_20261115/native/lib.rs), and [RR9 projective note](../../prime31_ic_20261115/rr9/RR9_PROJECTIVE_D9_THEOREM_20261117.md) supply the inspected repository construction and chart context.
- **[PS]** M. S. Paterson and L. J. Stockmeyer, *On the Number of Nonscalar Multiplications Necessary to Evaluate Polynomials*, SIAM Journal on Computing 2(1), 1973, pp. 60–66, [DOI 10.1137/0202007](https://doi.org/10.1137/0202007). Only its blocked evaluation upper-bound method is used. No lower bound for this finite-field selector, no lower bound for relation extraction, and no elimination-complexity conclusion is imported.
- **[FM]** J.-C. Faugère and C. Mou, *Sparse FGLM algorithms*, [arXiv:1304.1238](https://arxiv.org/abs/1304.1238), particularly the zero-dimensional quotient/multiplication-matrix assumptions in Sections 1–3. Its order-change results do not supply the missing RR5 basis construction.
- Prior context remains Semaev [ePrint 2004/031](https://eprint.iacr.org/2004/031) and Faugère–Huot–Joux–Renault–Vitse [EUROCRYPT 2014](https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf), with the access limitations already recorded in the parent protocol. KB tools are unavailable and the repository wrapper returned 127; this is the authorized named-source fallback, not an exhaustive novelty review.

All new geometric, ideal-theoretic and circuit-equivalence arguments remain written, unsigned and unverified. The earlier evaluated RR5 norm record does not certify this selector replacement, radicality proof, gate implementation, or any scaling claim. [pcc:unverified]
