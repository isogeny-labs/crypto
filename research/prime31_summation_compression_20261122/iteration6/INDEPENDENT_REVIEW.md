# Independent review: iteration 6 oriented quadratic S3 system

Date: 2026-11-23 UTC. Reviewer: the indexing/literature worker, independently of the mathematical-note author. Written mathematical review only; no code, experiment, point count, remote mutation, Lean build, or signed PCC verification was performed.

Reviewed file: `higher_arity_math_v5.md` (its title correctly identifies iteration 6).

Reviewed SHA-256: `90717bb468073046ee712bd986600c2788db7bb8cc6de7b9126e7833b245a4dd`.

## Disposition

**Accept the representation and completeness statements at their stated scope, after the incorporated boundary clarifications. [pcc:unverified]** I found no counterexample to all-node rational purity under the actual assumptions: nonsingular short Weierstrass curve in characteristic greater than three, odd full rational group order, a specified nonzero rational root point, finite auxiliary abscissae in the ground field, and a connected binary S3 tree.

This is a written proof audit, not certification of a solver, implementation, asymptotic bound, or prior-art novelty.

## 1. Local lift and induction

Writing child abscissae as \(r+u,r+v\), parent ordinate as \(s\), and \(f(r)=s^2\), I independently checked

\[
S_3(r+u,r+v,r)
=\bigl(uv-(3r^2+a)\bigr)^2-4s^2(u+v+3r).
\]

Thus \(\lambda=(uv-(3r^2+a))/(2s)\) satisfies \(\lambda^2=u+v+3r\), and

\[
f(r+T)-(\lambda T-s)^2=T(T-u)(T-v).
\]

This proves the proposed child ordinates, the sum to the oriented parent, and repeated-root/tangent cases without dividing by child-coordinate differences. The slope is unique since \(s\ne0\). Odd rational group order prevents a recovered finite rational child from having ordinate zero, allowing induction at every internal node. Consequently the full-fibre/liftable-fibre equality is valid as an equality of **ground-field assignments**, not as an unrestricted projected higher-summation-polynomial identity. **[pcc:unverified]**

Q1 and Q2 are precisely the two coefficient conditions in this factorization; Q3 propagates the already determined ordinate. No separate cubic equations are needed for internal curve membership. On ground-field solutions, the quadratic augmentation and the original rational S3 tree are bijective. **[pcc:unverified]**

## 2. Variable and equation counts

For \(m\ge2\), \(k\ge1\), I independently obtain:

| Component | Variables | Equations |
|---|---:|---:|
| Nonroot abscissae | \(2m-2\) | 0 |
| Nonroot internal ordinates | \(m-2\) | \(m-2\) |
| Internal slopes | \(m-1\) | \(2m-2\) |
| Dickson ladders | \(m(k-1)\) | \(mk\) |
| Total | \(mk+3m-5\) | \(mk+3m-4\) |

The earlier generic display needed explicit base cases. The author has incorporated them: \(m=1\) is direct target membership, and \(m=2\) has only \(S_3(x_1,x_2,r)=0\). The count formulas are now expressly restricted to \(m\ge2\). **[pcc:unverified]**

## 3. Rational-closure guards

The Mersenne guard satisfies \(X^{2^{31}}-X^2=X(X^p-X)\). Its algebraic-closure zero set is exactly \(\mathbb F_p\); zero has multiplicity two. The proposed circuit adds 30 variables and 31 quadratic equations per internal abscissa, giving totals

\[
mk+33m-65,\qquad mk+34m-66.
\]

The alternative squarefree guard also checks: starting with \(a_1=X\), the recurrence \(a_{j+1}=a_j^2X\) gives \(a_j=X^{2^j-1}\). Thirty square variables and 29 intermediate \(a_j\) variables, with \(a_{31}=X\) substituted, give 59 variables and 60 equations. Its eliminated polynomial is exactly \(X^p-X\). **[pcc:unverified]**

Guarding only the \(m-2\) internal abscissae is sufficient: full splitting places leaf abscissae and Dickson intermediates in the ground field; Q2 and top-down propagation then force slopes and ordinates into that field. No division by a possibly zero child abscissa is used. The double-zero Mersenne guard supports zero-set equivalence only; scheme length or raw multiplicity is not a witness count. **[pcc:unverified]**

## 4. Completeness: chain versus fixed topology

The backward-peeling proof is valid for \(m\le n\): failure at a remaining size \(t\ge2\) would mean every remaining point equals its nonzero sum \(S\), so \((t-1)S=O\), contradicting prime order and \(t-1<n\). The resulting permutation has no zero prefix. This supplies completeness for an affine **left chain with free leaf order**, including repeated and cancelling points. **[pcc:unverified]**

I supplied a counterexample to a broader fixed-topology claim: for prime \(n\ge5\), the multiset \(P,P,P,-P\) totals \(2P\ne O\), but every balanced two-pair tree has a cancelling child. The author now includes this counterexample and restricts the completeness theorem accordingly. A fixed sorted order and an arbitrary fixed balanced shape cannot inherit the chain guarantee. Identity-cut branches would need a separate representation and completeness proof. **[pcc:unverified]**

The bound is sharp as a uniform guarantee: \(n+1\) copies of a nonzero point total that point, but every ordering has a zero prefix at length \(n\). **[pcc:unverified]**

## 5. Output and accounting checks

The integer DP recurrence and its mass recurrence check using \(A=-A\). The explicit-witness \(\Theta(L^j)\) statement is correctly restricted to fixed \(j\); it does not lower-bound an aggregate state graph. Integer-count bit lengths must remain charged when arity grows. The revised meet-in-the-middle paragraph correctly retains suffixes whose standalone prefixes vanish, since the global chain prefix is offset by the first block's sum. **[pcc:unverified]**

The per-node extraction tally \(I+4M+S+6A+2C\) matches direct evaluation of the displayed formula. The rational/anti-rational decomposition and the stated \(C_3,C_4\) count identities also check under odd rational group order. They count signed point tuples, not x-solutions or independent linear relations; cancellation and duplicate relation rows still require accounting. **[pcc:unverified]**

## 6. Prior art and access receipt

Semaev's [2015 primary paper](https://arxiv.org/pdf/1504.01175), §3 equation (4) and §4.1 Lemma 2, already uses ground-field S3 chains and rational auxiliary coordinates, with minimal-arity and nonidentity qualifications. Its Frobenius argument identifies the nonrational subtotal through rational 2-torsion. The present local purity proof and explicit quadratic bookkeeping are applications/refinements of this established construction; this review makes no literature-wide originality claim. Repository native orientation and Dickson-ladder precedents are identified in the mathematical note and sibling source audit.

KB receipt: no KB tools were exposed in the bounded tool inventory, and no local wrapper appeared in the materialized workspace. Review used the parent lane scope, the mathematical note, the sibling-source pointers, and the primary Semaev paper already inspected. Lack of KB access is not evidence of absence of prior art.
