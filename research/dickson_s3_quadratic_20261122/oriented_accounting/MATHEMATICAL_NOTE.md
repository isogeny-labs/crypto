# Root-oriented Dickson–S3 chains: specification and predictions

Date: 2026-11-23. Parent task: [PR #4950](https://github.com/isogeny-labs/crypto/pull/4950).

**Standing:** written derivation and **unexecuted test predictions**,
`[pcc:unverified]`. This note contains no observed solver, timing, test-pass,
or recovered-scalar result. Execution records must establish what the native
implementation actually does; this note is not a replacement for those records.

## 1. Source and notation

The formulation, odd-order rational-tree argument, and free-order completeness
lemma come from merged [PR #4928](https://github.com/isogeny-labs/crypto/pull/4928),
read at published commit `fc184b03d47e13ec922c9b489f3b3f8a976a8bd9`:

- [Iteration-six mathematical note](https://github.com/isogeny-labs/crypto/blob/fc184b03d47e13ec922c9b489f3b3f8a976a8bd9/research/prime31_summation_compression_20261122/iteration6/MATHEMATICAL_NOTE.md),
  especially §§3–6; source blob `9120eb9d367fbc54e04fc7b2f53497f770e35d4c`.
- [Rational-chain guard](https://github.com/isogeny-labs/crypto/blob/fc184b03d47e13ec922c9b489f3b3f8a976a8bd9/examples/prime31_rational_chain_guard.rs),
  the supplied-certificate recovery implementation. Its verification scope is
  distinct from finding roots of the coupled equations.

Current implementation references are the [four-arm builder](../../../examples/dickson_s3_quadratic.rs),
[extraction/accounting module](../../../examples/dickson_s3_quadratic/accounting.rs),
[separate exhaustive control](../../../examples/dickson_s3_quadratic/controls.rs),
and [production visitor](../../../src/cryptanalysis/f4_fp.rs). The
[development protocol](PROTOCOL.md) owns the fixed execution scope. No novelty
or solving-cost improvement follows from reusing the oriented construction.

Let

\[
E/\mathbf F_p:\quad y^2=f(x)=x^3+Ax+B,
\qquad p>3,\qquad 4A^3+27B^2\ne0.
\]

The root is the actual finite signed point \(R=(r,s)\), including \(s=0\)
when discussing the general chart. The matched admitted odd-order experiment
has additional group hypotheses; they are not needed for the polynomial
identity below. All equations are equations over the declared field.

Sources checked (KB): no callable KB tools were exposed; `scripts/research-kb
status` reported no executable. The fallback was the exact published #4928
sources above and the named current Rust files. This is not an exhaustive
literature search or an absence-of-prior-work claim.

## 2. Local identity and signed induction

For child abscissae \(x,z\) and parent \((r,s)\), introduce one slope \(\lambda\):

\[
\lambda^2=x+z+r,\qquad
2s\lambda=(x-r)(z-r)-(3r^2+A). \tag{O1–O2}
\]

Set \(u=x-r\), \(v=z-r\). Since \(f(r)=s^2\), direct expansion gives

\[
\begin{aligned}
f(r+T)-(\lambda T-s)^2
 &=T^3+(3r-\lambda^2)T^2+(3r^2+A+2s\lambda)T\\
 &=T(T-u)(T-v).
\end{aligned}
\]

Thus the line \(y=\lambda(X-r)-s\) intersects the curve at \(-R\) and

\[
P=(x,\lambda(x-r)-s),\qquad
Q=(z,\lambda(z-r)-s),
\]

with the displayed intersection multiplicities. The chord-and-tangent law
gives \(P+Q=R\). This includes repeated coordinates and equality of a child
abscissa with the parent's. There is no division by \(s\), \(x-z\), or
\(x-r\) in this implication. It holds for every stated nonsingular curve in
characteristic \(p>3\). [pcc:unverified]

For a left chain, write \(S_j=P_1+\cdots+P_j=(X_j,Y_j)\), with
\((X_m,Y_m)=R\). The node for \(j=2\) has children \(x_1,x_2\); the node
for \(j>2\) has children \(X_{j-1},x_j\). Apply O1–O2 at every node and add

\[
Y_{j-1}=\lambda_j(X_{j-1}-X_j)-Y_j,
\qquad 3\le j\le m. \tag{O3}
\]

Starting at the signed root and descending, the identity proves that every
shared internal ordinate and recovered leaf ordinate belongs to its curve
point, and every child pair sums to its parent. Separate cubic internal
curve equations are unnecessary. Conversely, any ordered signed tuple with
finite nonzero prefixes determines its prefix points and unique nonvertical
chord/tangent slopes, hence an O assignment. This gives an exact correspondence
between O assignments and finite signed chains, including chains with finite
2-torsion points. It does **not** assert uniqueness after forgetting the signs.
[pcc:unverified]

For constructing a known signed witness, use the actual child chord slope
\((y_Q-y_P)/(x_Q-x_P)\), or the tangent slope
\((3x_P^2+A)/(2y_P)\) when \(P=Q\) and \(y_P\ne0\). A vertical chord or
a 2-torsion tangent has parent \(O\) and is outside this finite chart. This
construction avoids dividing by a possibly zero **parent** ordinate.

## 3. Zero ordinates, domains, and chart boundaries

When \(s\ne0\), O2 uniquely determines the slope from the three abscissae.
When \(s=0\), O2 instead requires its numerator to vanish, and O1 may have
zero, one, or two field slopes. Different slopes can represent different
signed child pairs with the same parent and abscissae. O consumes its returned
slopes; a decoder for an x-only arm must retain every compatible sign branch.
[pcc:unverified]

A concrete zero-parent example is \(y^2=x^3+3x+3\) over \(\mathbf F_7\):
\(T=(1,0)\), \(P=(3,2)\), \(Q=(4,3)\). Both \(P+Q=T\) and
\(-P-Q=T\), with slopes \(1\) and \(6\). Also, the local abscissa triple
\((2,0,1)\) has S3 equal to zero but O1 requires \(\lambda^2=3\), a
nonsquare. Both child curve right-hand sides are also \(3\). This explains
why rational-root uniqueness and purity cannot silently extend across
2-torsion parents. These examples are hand-derived predictions here.
[pcc:unverified]

On an **odd-order** source curve, every finite rational point has nonzero
ordinate. The published #4928 induction then gives a unique signed lift of
each all-\(\mathbf F_p\) S3 abscissa tree with finite rational root. Q has the
same projected roots as quartic S3, and L does not obtain an additional global
twist filter on that domain. Isolated twist-only examples do not demonstrate
a matched M31 filtering benefit. [pcc:unverified]

The exact exclusions and domain requirements are:

- The root and leaves are finite; \(R=O\) and identity leaves need separate
  charts. A chain omits an ordered tuple whenever a nonroot prefix is \(O\),
  even if its final group sum is the correct nonzero target.
- Tangencies, horizontal slopes, child/parent x equality, and zero ordinates
  are not automatically excluded. The equations and exact group replay decide
  them. In particular, \(\lambda=0\) does not justify dividing by a slope.
- Every solver variable ranges over \(\mathbf F_p\). Internal abscissae,
  ordinates, slopes, and invariant auxiliaries must never inherit a factor-base
  domain. Coefficients in \(\mathbf F_p\) do not constrain algebraic-closure
  roots to \(\mathbf F_p\). The reduced field-guard argument in #4928 relies
  on its odd-order hypotheses; with 2-torsion, rational x alone can still leave
  a nonsquare slope over an extension.
- For prime group order \(n\), \(m\le n\), nonidentity leaves, and nonzero
  total, #4928 proves that some ordering of each signed multiset has no zero
  prefix. This supports free-order multiset completeness, not completeness
  for every ordered tuple, a fixed sorted order, or every balanced topology.
  The multiset \(P,P,P,-P\) retains its balanced-tree obstruction.

All admitted subgroup and curve-order hypotheses require their own checks;
they do not follow from the O equations. [pcc:unverified]

## 4. Selector and system sizes

Each leaf retains \(D_{2^\ell}(x,1)=0\), with \(\ell\ge1\), as
\(z_{j+1}=z_j^2-2\) and a final \(z_{\ell-1}^2-2=0\).
This contributes \(m\ell\) variables, counting the leaves, and \(m\ell\)
equations. It is the fixed power-of-two zero selector, not an arbitrary-index
Dickson implementation.

O adds \(m-2\) prefix abscissae, \(m-2\) shared ordinates, and \(m-1\)
slopes; its coupling has \(2(m-1)\) node equations and \(m-2\) propagation
equations. Before optional algebraic-closure field guards:

| Representation | Variables | Equations | Maximum input degree |
|---|---:|---:|---:|
| Quartic S3 | \(m\ell+m-2\) | \(m\ell+m-1\) | 4 |
| Invariant Q / line L | \(m\ell+4m-5\) | \(m\ell+4m-4\) | 2 |
| Parent-oriented O | \(m\ell+3m-5\) | \(m\ell+3m-4\) | 2 |

For m=2/3/4, O's variable counts are respectively \(2\ell+1\),
\(3\ell+4\), and \(4\ell+7\), with one additional equation in each case.
Smaller input counts do not establish a smaller solving degree, fewer extracted
roots, or lower complete work. [pcc:unverified]

## 5. F7 prediction: roots, signs, and duplicate rows

**Everything in this section is a pre-execution prediction, not an observed
outcome.** Use \(E:y^2=x^3+3\) over \(\mathbf F_7\), depth one, and

\[
U=(3,3),\qquad V=(4,2),\qquad R=(1,2)=U+V.
\]

The selector roots are \(3,4\), and the signed base is \(\{\pm U,\pm V\}\).
For each nonzero x, \(x^3\) is \(1\) or \(6\), so \(x^3+3\) is the
nonzero square \(4\) or \(2\); at x=0 it is the nonsquare \(3\). Thus the
curve has \(12+1=13\) points. Direct doubling gives \(U=2V\), so
\(R=3V\). These facts derive the control expectations; known base logs are
not supplied to root extraction. [pcc:unverified]

Represent signed leaves by weights \(\{2,-2,1,-1\}\) relative to V.
For m=2,3,4 their integer sums lie between -8 and 8, so a sum equal to
\(3\bmod13\) must be the integer 3. The multiset possibilities are:

| m | Signed multisets totaling R | Finite ordered witnesses |
|---|---|---:|
| 2 | \(\{U,V\}\) | 2 |
| 3 | \(\{U,U,-V\}\), \(\{V,V,V\}\) | 3 + 1 |
| 4 | \(\{U,V,V,-V\}\), \(\{U,U,-U,V\}\) | 8 + 8 |

Each m4 multiset has 12 orders. Four begin with an opposite pair and are
excluded. No length-three prefix can vanish, because the remaining last point
would have to equal R, which is outside this base. Therefore eight of the
24 m4 group matches are outside the chart. [pcc:unverified]

For this odd-order fixture each full abscissa chain has one compatible signed
lift. Q adds unique invariant auxiliaries. L has two simultaneous line-sign
choices at every node, independently; \(\lambda=\nu=0\) cannot occur on an
odd-order curve. Its raw assignment multiplicity is therefore \(2^{m-1}\).

| Predicted quantity | m=2 | m=3 | m=4 |
|---|---:|---:|---:|
| Raw quartic / Q / O assignments, each | 2 | 4 | 16 |
| Raw L assignments | 4 | 16 | 128 |
| Accepted ordered signed tuples | 2 | 4 | 16 |
| Complete leaf-and-prefix x chains | 2 | 4 | 16 |
| Ordered leaf-x tuples alone | 2 | 4 | 8 |
| Signed multisets | 1 | 2 | 2 |
| Distinct effective coefficient rows | 1 | 2 | 1 |

These multiplicities count field assignments, not algebraic root multiplicity.
A complete native run must establish the prediction under its actual stopping
and domain contract. [pcc:unverified]

The leaf-x tuple \((3,4,4,4)\) alone does not determine the signs:

| Signed tuple | Prefix x coordinates \((X_2,X_3)\) | Slopes \((\lambda_2,\lambda_3,\lambda_4)\) |
|---|---|---|
| \(U,V,V,-V\) | \((1,2)\) | \((6,0,0)\) |
| \(U,V,-V,V\) | \((1,3)\) | \((6,1,6)\) |
| \(U,-V,V,V\) | \((4,3)\) | \((2,5,6)\) |

Deduplicating by leaf x before signed recovery would lose valid witnesses.
Replaying only the final sum also fails to establish that a witness matches
the supplied prefix assignment. [pcc:unverified]

## 6. Formal row and rank prediction

Use canonical base columns U,V and the public target column Q=R, with G=V.
For a public query \(R=aG+bQ\), signed multiplicities give the row
\((c_U,c_V,-b)\) with right-hand side a over the **scalar** field
\(\mathbf F_{13}\), not the coordinate field \(\mathbf F_7\).
Deduplication keys contain the coefficients and right-hand side.

For the fixed schedule \((a,b)=(0,1)\), the predicted rows are

\[
\begin{array}{c|c}
m=2 & r_1=(1,1,-1)\\
m=3 & r_2=(2,-1,-1),\quad r_3=(0,3,-1)\\
m=4 & r_1=(1,1,-1).
\end{array}
\]

Both m4 multisets cancel to U+V. Also \(r_2+r_3=2r_1\), so the three
distinct relation rows together have rank two. Per-cell row ranks are 1,2,1.
With the legitimate public anchor \(\log_G V=1\), the predicted cumulative
ranks after m=2,3,4 are 2,3,3: two independent relation rows beyond the seed.
Solving those symbolic equations predicts \(\log_G U=2\) and
\(\log_G Q=3\). This is a derived expected answer, **not an observed scalar
recovery or validation result**. [pcc:unverified]

The extraction contract must therefore keep raw roots, decoded occurrences,
ordered signed tuples, signed multisets, full rows, and incremental rank
separate. The final recovered scalar, if produced by an executed run, still
requires the independently charged original-curve replay specified in the
protocol. Constructor size, duplicate counts, and finite evaluation identities
provide no performance conclusion.
