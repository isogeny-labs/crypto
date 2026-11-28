# Dickson leaf norms and the actual pair spectrum

Date: 2026-11-23. Parent task: PR #4977, Prime31 summation compression.

**Status: all mathematical findings below are unverified producer derivations.** No cryptographic experiment, benchmark, certificate issuance, or remote mutation was performed. This bounded follow-up supplies one exact candidate identity and a counterexample to a particular automatic composition shortcut. It does not claim novelty, a general lower bound, or an asymptotic query improvement. [pcc:unverified]

## Question and conclusion

The existing pencil uses the actual pair support `X`, its squarefree polynomial `H_X`, and `K(r,V) = product_{u in X} S_3(u,V,r) mod H_X(V)`. Can the Dickson origin of the leaves furnish a quotient identity that removes its `D = |X|` component zero search? The derivation below connects the leaf selector to `H_X` exactly, including doubling and multiplicity. It also shows that an actual regular Dickson fibre can produce a pair support whose coordinate algebra does not inherit the first uniform degree-two Dickson quotient. The norm identity therefore identifies a concrete structure to exploit, but supplies no sub-square-root target query. [pcc:unverified]

The independent review and `ROOT_REVIEW.md` were read before this derivation. Their scope restrictions are retained: arbitrary-set span and dense-output counts do not prove a lower bound for this special source. [pcc:unverified]

## 1. An exact leaf-to-pair norm identity

Work over a field of characteristic `p > 3` with a nonsingular curve

\[
E:y^2=f(x)=x^3+ax+b.
\]

Let `P_1,...,P_B` be nonzero points in an odd-order rational subgroup, with distinct abscissae `x_i`. Put

\[
A=\{\pm P_i:1\le i\le B\},\qquad
M(Z)=\prod_i(Z-x_i),\qquad
X=x((A+A)\setminus\{O\}).
\]

The nonzero values `f(x_i)` are squares. Odd order excludes nonzero rational two-torsion. Distinct abscissae exclude `P_i = ±P_j` for `i != j`. Thus all points appearing in the following products are affine. [pcc:unverified]

Define the monic polynomial that retains all unordered pair-chart multiplicities:

\[
C_M(W)=
\prod_i\bigl(W-x(2P_i)\bigr)
\prod_{i<j}
\bigl(W-x(P_i+P_j)\bigr)
\bigl(W-x(P_i-P_j)\bigr).
\]

Changing a representative `P_i` to its negative permutes these roots. The degree is `B^2`, including repeated roots when different charts collide. Its set of roots is exactly `X`; hence

\[
H_X=\operatorname{rad}(C_M),
\]

where `rad` means the monic product of distinct roots, not an assumed derivative-only implementation in characteristic `p`. [pcc:unverified]

The doubling part is

\[
J_M(W)=\prod_i\bigl(W-x(2P_i)\bigr),\qquad
x(2P_i)=d(x_i),\qquad
d(U)=\frac{(U^2-a)^2-8bU}{4f(U)}.
\]

Because `f` is a unit modulo `M`, `J_M` is the characteristic polynomial of `d(U)` in `F_p[U]/(M)`. Define the double resultant

\[
\begin{split}
R_M(W)&=\operatorname{Res}_U\left(M(U),
                 \operatorname{Res}_V(M(V),S_3(U,V,W))\right)\\
      &=\prod_{i,j}S_3(x_i,x_j,W).
\end{split}
\]

Equivalently, `R_M` is the norm of `S_3(U,V,W)` in the dimension-`B^2` split algebra

\[
\mathcal B=\mathbb F_p[U,V]/(M(U),M(V)).
\]

These definitions give the candidate identity

\[
\boxed{R_M(W)\,J_M(W)=\kappa\,C_M(W)^2},
\qquad
\kappa=(-4)^B\prod_i f(x_i)\prod_{i<j}(x_i-x_j)^4\ne0.
\tag{1}
\]

In terms of `M`, the constant is `(-4)^B Res(M,f) Disc(M)^2`. The identity is an elementary factorization of the established third summation polynomial; no novelty is asserted. [pcc:unverified]

### Derivation, including the diagonal correction

For `i != j`, Semaev's characteristic-at-least-five formula and the group law give

\[
S_3(x_i,x_j,W)=(x_i-x_j)^2
 (W-x(P_i+P_j))(W-x(P_i-P_j)).
\tag{2}
\]

The two roots are distinct: equality would imply `2P_i=O` or `2P_j=O`. On the diagonal, the quadratic leading coefficient vanishes and

\[
S_3(x_i,x_i,W)
=-4f(x_i)W+(x_i^2-a)^2-8bx_i
=-4f(x_i)(W-x(2P_i)).
\tag{3}
\]

Every off-diagonal chart occurs twice in `R_M`, while each diagonal occurs once. Multiplication by `J_M` makes every chart occur twice, proving (1). In particular, `deg R_M = 2B^2-B`, `deg J_M = B`, and `rad(R_M)=H_X`. These are formal polynomial identities and degree counts, with no evaluation-algorithm lower bound attached. [pcc:unverified]

Semaev's paper, Section 2, supplies the formula and resultant framework used in (2)–(3). The norm/characteristic-polynomial language is standard; Bostan–Flajolet–Salvy–Schost, Section 4.1 and Lemma 7, describes the corresponding quotient-algebra interpretation for diamond products. [pcc:unverified]

### The actual leaf selector matters

Here `M` contains the actual rational, subgroup-filtered leaves. A full Dickson fibre polynomial can contain abscissae that fail the curve-lift or subgroup restriction. Replacing `M` by that full polynomial changes the source of the norm; (1) must then be interpreted for that enlarged source, with whatever nonrational or outside-subgroup points it includes. The identity is not a license to remove existing rationality or subgroup guards. This issue is separate from the composition failure below, whose entire fibre does lift into a prime-order rational group. [pcc:unverified]

## 2. A regular Dickson fibre whose pair quotient is not a Dickson tower

This is a handwritten finite-field counterexample to the universal assertion that forming actual elliptic pair sums automatically preserves the normalized Dickson degree-two tower. It is not an experiment on the frozen `p=2^31-1` source. [pcc:unverified]

Take

\[
p=7,\qquad E:y^2=x^3+3,\qquad D_2(Z)=Z^2-2,
\qquad M(Z)=D_2(Z)= (Z-3)(Z-4).
\]

The curve is nonsingular since `27·3^2 = 5 mod 7`. The squares in `F_7` are `{0,1,2,4}`. At `x=0` the right side is `3`, and at `x=1,...,6` it is respectively `4,4,2,4,2,2`. Consequently there are twelve affine points and `O`, giving a group of prime order thirteen. [pcc:unverified]

This is a regular norm-one Dickson fibre with `N=2`, `c=0`, and `4N=p+1=8`. For either trace `t=3` or `4`, the discriminant `t^2-4=5` is nonsquare, so the roots of `Z^2-tZ+1` lie in the norm-one torus. The fibre is regular because `c` is neither `2` nor `-2`. This is the smallest tower level; it does not purport to satisfy any separate asymptotic density theorem's lower-size hypotheses. [pcc:unverified]

Choose

\[
P=(3,3),\qquad Q=(4,2).
\]

Both points lie on `E(F_7)`. Direct substitution in the elliptic addition and doubling formulas gives:

| Pair chart | Slope in `F_7` | Resulting abscissa |
| --- | --- | --- |
| `2P` | `3·3^2/(2·3) = 1` | `2` |
| `2Q` | `3·4^2/(2·2) = 5` | `3` |
| `P+Q` | `(2-3)/(4-3) = 6` | `1` |
| `P-Q` | `(-2-3)/(4-3) = 2` | `4` |

Thus all four chart roots are distinct,

\[
X=\{1,2,3,4\},\qquad
H_X(W)=C_M(W)=W^4+4W^3+6W+3.
\tag{4}
\]

By contrast,

\[
D_4(W)-c'=W^4+3W^2+(2-c')\quad\text{in }\mathbb F_7[W].
\]

No value of `c'` makes this equal to (4), because its cubic coefficient is zero. More strongly, `H_X` is not a polynomial in `D_2(W)`, since every such polynomial is even and `H_X` is not. [pcc:unverified]

The quotient-algebra failure can be stated without relying solely on coefficient shape. In `F_7[W]/(H_X)`, the coordinate `D_2(W)` takes these values:

| `W` in `X` | `1` | `2` | `3` | `4` |
| --- | --- | --- | --- | --- |
| `D_2(W)` | `6` | `2` | `0` | `0` |

The subalgebra generated by `D_2(W)` has dimension three, with fibre sizes `1,1,2`. The dimension-four pair algebra is therefore not a free rank-two algebra over that subalgebra. The original uniform two-to-one tower step does not descend to this actual pair support. This rules out that precise automatic folding rule, including its direct use as a uniform norm recursion. It does not rule out other coordinates, nonuniform fibres, special curves, or a different identity specific to the Prime31 source. [pcc:unverified]

The example also checks the diagonal correction in (1) directly:

\[
\begin{split}
S_3(3,3,W)&=6(W-2),\\
S_3(4,4,W)&=5(W-3),\\
S_3(3,4,W)&=(W-1)(W-4),\\
R_M(W)&=2(W-2)(W-3)(W-1)^2(W-4)^2,\\
J_M(W)&=(W-2)(W-3),\qquad\kappa=2.
\end{split}
\]

These displayed factors satisfy `R_M J_M = 2 H_X^2`. The arithmetic is supplied as a reproducible written check, not as an independently replayed computational result. [pcc:unverified]

## 3. What this does and does not do for the query budget

There are two distinct gates. With comparable charged setup and output, a genuine component improvement over the present `~O(D)` exact empty-query baseline requires an exponent `theta < 1`. The stronger `theta < 1/2` condition is the conditional complete-pipeline exponent target in the parent's regime `sigma <= 2` and `D = Theta(B^2)`. The stronger condition is not necessary for every useful stage-level improvement. Neither gate is established by the identity alone. [pcc:unverified]

Identity (1) is an exact bridge from the leaf algebra to the pair selector. It retains the known `B^2` chart structure, with its swap symmetry, rather than assuming the pair selector is another Dickson fibre. Extracting a radical is also a real operation when charts collide. In a regime with `D=Theta(B^2)`, the identity is compatible with a source representation of order `D`; it does not by itself give an algorithm that meets that setup budget. [pcc:unverified]

The target query still has to decide whether some `u,v in X` satisfy `S_3(u,v,r)=0`, while treating `r in X` and `R=O` as required by the parent note. Replacing each pair-selector occurrence by the norm in (1) expresses the same constrained zero problem in leaf variables. Neither the norm identity nor the scalar rank-at-most-three factorization supplies an algorithm for evaluating that predicate in `o(sqrt(D))` field work after `~O(D)` setup. A new target-dependent shortcut would have to be analyzed separately, including witness recovery, certificate construction/verification, and all requested output. [pcc:unverified]

The cited fast-resultant paper does not fill this gap by invoking its near-linear theorem indiscriminately. Its Theorem 1(1) gives `O(M(mn))` for composed sums and products when characteristic is zero or exceeds `mn`. Its general diamond-product theorem has a different bound, `O(sqrt(mn)(M(mn)+(mn)^(omega/2)))`, with stated multiplicity assumptions. Those are polynomial-construction bounds, not a preprocessed sub-square-root target zero-query theorem. Translating the rational elliptic pair correspondence into the theorem's polynomial map also requires an actual reduction, including denominators and diagonals. [pcc:unverified]

**Bounded finding:** the actual leaf-to-pair norm has the correction (1); automatic normalized Dickson tower inheritance is false in the explicit regular-fibre example. Both the component-improvement query gate and the stronger conditional pipeline gate remain unresolved. No conclusion here is a lower bound for an alternative succinct representation or the actual Prime31 spectrum. [pcc:unverified]

## Sources and handoff scope

- Parent notes read: `iteration10/S3_PENCIL_INDEPENDENT_REVIEW.md`, `iteration10/ROOT_REVIEW.md`, and `iteration10/S3_PENCIL_CONTRACTION.md` under `research/prime31_summation_compression_20261122/`.
- Igor Semaev, [New algorithm for the discrete logarithm problem on elliptic curves](https://arxiv.org/pdf/1504.01175), Section 2 and equations for `S_3` and recursive resultants; Section 3 for the auxiliary-variable system. The paper's binary-field heuristic is not imported as a prime-field query guarantee.
- Alin Bostan, Philippe Flajolet, Bruno Salvy, Éric Schost, [Fast Computation of Special Resultants](https://cs.uwaterloo.ca/~eschost/publications/BoFlSaSc05.pdf), introduction, Theorem 1, Section 4.1/Lemma 7, and Section 4.2. The quotient-algebra machinery is established prior art; its algorithmic scope is retained.
- KB and Conductor unavailable in this runtime, as established in the parent task. The fallback here was the supplied local reviews and direct primary-source reading. No exhaustive novelty review is claimed.
- Only this scratch directory was written. The parent owns PR #4977, claims/ledger integration, and persistent publication. No shared state, frozen note, source code, ledger, or remote object was changed by this follow-up.

## Local task log

### What was done

Read the supplied reviews, checked the relevant primary sources, derived (1) by diagonal/off-diagonal factorization, and supplied the explicit `F_7` regular-fibre composition counterexample. [pcc:unverified]

### Decisions made

Retain the actual lift-filtered selector `M`; keep the norm identity separate from its computational cost; reject only automatic uniform `D_2` folding of pair support. [pcc:unverified]

### Open threads

Independent verification of this note and any actual Prime31-specific quotient identity remain with the parent. No algorithm meeting the stated query gate was derived. [pcc:unverified]

### Errors

No experiment or source edit was attempted. The first combined web output exceeded the response budget; targeted primary-source reads recovered the formula, theorem hypotheses, and quotient-algebra statements. [pcc:unverified]
