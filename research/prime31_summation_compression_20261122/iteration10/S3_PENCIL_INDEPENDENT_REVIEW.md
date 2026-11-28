# Independent algebraic review of the reusable S3 pencil

**Disposition.** The proposed matrix identity, norm identity, exact formal degree, leading coefficient, and arbitrary-subset span statement are correct under the hypotheses below. No off-by-one error was found. The dense-table and linear-span observations do not establish an algorithmic lower bound, and the pencil has not supplied the required faster exact query algorithm. This review is a mathematical derivation; no code experiment or remote mutation was performed. [pcc:unverified]

**Reviewed input.** This review binds `S3_PENCIL_CONTRACTION.md`, 19,184 bytes, SHA-256 `9e9e64e76208202d05957794e7bd9371f9e427b2a5dded9f88aceb3ca40588ed`, including its discriminant claim and full parameter inequalities. The initial draft's rearranged exponent interval needed an explicit positive-denominator condition; the bound input includes that correction. No further mathematical correction is requested.

## 1. Hypotheses and rank-three identity

Let `p > 3`, let `X` be a set of `D >= 1` distinct elements of `Fp`, and put

\[
f(T)=T^3+aT+b,\qquad H(V)=\prod_{u\in X}(V-u),\qquad
R_X=\mathbb F_p[V]/(H).
\]

Assume `f(u) != 0` for every `u in X`. Rational liftability is not needed for the degree calculation; it is needed for the rational-point interpretation in section 5. The usual short-Weierstrass summation polynomial is

\[
S_3(u,V,r)=(u-V)^2r^2
-2\big((u+V)(uV+a)+2b\big)r
+(uV-a)^2-4b(u+V).
\]

With `w(t)=(1,t,t^2)^T`, direct expansion gives

\[
S_3(u,V,r)=w(u)^T C_r w(V),\quad
C_r=\begin{pmatrix}
a^2-4br&-2ar-4b&r^2\\
-2ar-4b&-2(r^2+a)&-2r\\
r^2&-2r&1
\end{pmatrix}.
\]

Thus the matrix of values indexed by `u,v in X` has rank at most 3. This is a statement about a linear matrix factorization. Taking the product of each column is a different, degree-`D` operation; the rank bound does not by itself compute those products cheaply.

## 2. Norm identity and exact formal degree

Define

\[
\Phi_X(z_0,z_1,z_2)=\prod_{u\in X}(z_0+uz_1+u^2z_2),
\qquad F(r,V)=\prod_{u\in X}S_3(u,V,r),
\qquad K(r,V)=F(r,V)\bmod H(V).
\]

Then `F(r,V)=Phi_X(C_r w(V))`. Equivalently, it is the norm of `S3(U,V,r)` in the finite free algebra defined by the monic polynomial `H(U)`, over `Fp[r,V]`. Reduction modulo `H(V)` gives the stated quotient-ring identity for `K`.

For each `v in X`, the diagonal factor is

\[
S_3(v,v,r)=-4f(v)r+(v^2-a)^2-8bv,
\]

of degree exactly 1. Each of the other `D-1` factors has degree exactly 2 in `r`, with leading coefficient `(v-u)^2`. Consequently,

\[
\deg_r K(r,v)=2D-1,\qquad
[r^{2D-1}]K(r,v)=-4f(v)\prod_{u\ne v}(v-u)^2
=-4f(v)H'(v)^2.
\]

Since `H` is split and squarefree, the Chinese remainder isomorphism

\[
R_X\simeq\prod_{v\in X}\mathbb F_p
\]

proves

\[
\boxed{\deg_r K=2D-1,\qquad
[r^{2D-1}]K=-4f(V)H'(V)^2\pmod H.}
\]

The coefficient is a unit of `R_X`, because none of its evaluations vanishes. This includes `D=1`: the quotient product is exactly the displayed linear diagonal factor.

The degree is **formal degree in `R_X[r]`**. If only values at `r in Fp` matter, reduction modulo `r^p-r` may give a different function representation. No lower bound follows from insisting on the unreduced formal degree.

## 3. What the dense table counts

The canonical representative has `2D` possible powers of `r` and `D` possible powers of `V`. A fully materialized rectangular monomial table therefore allocates **`2D^2` field-element slots**. This is a format capacity, not a proof that every slot is nonzero, independent, or necessary to store. In particular, the leading coefficient already has the compact formula above, and `Phi_X` has an `O(D)`-factor product representation.

When `D=Theta(B^2)`, explicitly allocating this dense rectangle costs `Theta(B^4)` slots. This rules out that materialized implementation under a `B^sigma` setup budget with `sigma<=2`; it does not rule out implicit representations, specialized recurrences, nonlinear algorithms, or an `X`-dependent data structure.

## 4. Exact scope of the arbitrary-subset span statement

Assume `p >= D+2`. As `X` ranges over all `D`-element subsets of `Fp`, the forms `Phi_X` span the entire space of homogeneous degree-`D` ternary polynomials, of dimension

\[
\binom{D+2}{2}.
\]

A direct interpolation proof avoids any factorial or characteristic issue. Choose a set `T` of `D+2` distinct field elements. For each pair `{a_0,b_0}` in `T`, take the form associated with `X=T\setminus\{a_0,b_0\}` and the evaluation point

\[
z_{a_0b_0}=(a_0b_0,-a_0-b_0,1).
\]

For every `u`,

\[
(1,u,u^2)\cdot z_{a_0b_0}=(u-a_0)(u-b_0).
\]

Among the `D`-subsets of `T`, only the form with omitted pair `{a_0,b_0}` is nonzero at that point. Every other such subset contains `a_0` or `b_0`. The square evaluation matrix is therefore diagonal with nonzero diagonal entries. These `binom(D+2,2)` forms are linearly independent and constitute a basis.

This establishes the dimension required by a **fixed universal linear basis containing all those formal forms**. It does not establish a lower bound for the actual Dickson-derived pair-spectrum subclass, a circuit chosen from `X`, or the specialized expression `Phi_X(C_r w(V)) mod H_X`. In the last expression the quotient polynomial itself depends on `X`, so the universal-span statement cannot simply be transferred to it. The interpolation argument is elementary prior-art territory; no novelty claim is attached to it.

## 5. Exact translated-divisor interpretation

Now assume `E: y^2=f(x)` is nonsingular, every element of `X` has two rational lifts, and `U` is the set of all `2D` oriented lifts. Let `R` be an affine point with `r=x(R)` and require **`r notin X`**. Then all points `R-T`, for `T in U`, are affine, and the addition law gives

\[
S_3(u,V,r)=(r-u)^2
\bigl(V-x(R-P_u)\bigr)\bigl(V-x(R+P_u)\bigr).
\]

Multiplication yields the exact polynomial identity

\[
\boxed{F(r,V)=H(r)^2\prod_{T\in U}\bigl(V-x(R-T)\bigr).}
\]

Roots are counted with multiplicity; coincident translated x-coordinates do not invalidate the formula. An affine rational 2-torsion target also causes no algebraic problem when `r notin X`, although the odd-order application excludes such targets.

The displayed affine formula excludes `R=O` and `r in X`. In the latter case one translated point is `O`, and the corresponding quadratic factor becomes linear, with coefficient `-4f(r)`; substituting `H(r)=0` into the affine product formula is invalid. Those cases require separate handling or a projective divisor formulation.

For the stated nonexceptional input, `gcd(H(V),F(r,V))` is nonconstant exactly when some translated point has its x-coordinate in `X`. Because `U` contains both signs, this is equivalent to a relation `R=T_1+T_2` with `T_1,T_2 in U`.

## 6. Discriminant exhaustion on the specified source

For fixed `v,r`, the coefficient of `U^2` in `S3(U,v,r)` is `(v-r)^2`, and direct expansion gives

\[
\operatorname{disc}_U S_3(U,v,r)=16f(v)f(r).
\]

One way to check the expansion is to set `s=v+r`, `t=vr`. One quarter of the discriminant equals

\[
\bigl(s(t+a)+2b\bigr)^2-(s^2-4t)\bigl((t-a)^2-4bs\bigr)
=4f(v)f(r).
\]

In the complete pencil note, both `v` and `r` come from nonzero points of an odd-order rational subgroup. Thus `f(v)` and `f(r)` are nonzero squares. If `v!=r`, the coefficient of `U^2` and the discriminant are nonzero, and the two roots are distinct and rational. If `v=r`, the polynomial is linear with nonzero coefficient `-4f(r)`. Therefore a nonsquare-discriminant filter eliminates none of these source components. The group-law formulas in section 6 of the bound note have the correct signs and recover the two nondegenerate roots.

## 7. Full exponent-interval check

In the stated `D=Theta(B^2)` regime, setup `O_tilde(D^s)` after a `B^2` source chart gives `sigma=max(2,2s)`, a query exponent `theta` in `D` gives `tau=2theta`, and `alpha=max(sigma,2)`. Under the explicitly conditional uniform-target and sufficient-independent-row model, beating the square-root exponent requires

\[
\alpha\beta<\tfrac12,\qquad
1-\beta(3-2\theta)<\tfrac12.
\]

For `beta>0`, feasibility of the second inequality first requires `3-2theta>0`. Only on that domain may it be rearranged to

\[
\frac1{2(3-2\theta)}<\beta<\frac1{2\alpha}.
\]

The interval is nonempty exactly when `alpha<3-2theta`, before intersecting with the actually permitted source-family range. For `s<=1`, one has `alpha=2`, so the condition becomes precisely `theta<1/2`, equivalently `tau<1`. The amended note states the required positive-denominator condition. The remaining source, relation-yield, rank, and complete setup-cost qualifications are retained correctly.

## 8. Remaining algorithmic lemma and certificate scope

In the regime `D=Theta(B^2)`, the stated four-summand budget with setup exponent at most 2 needs a reusable exact query method whose cost is `O_tilde(D^theta)` with a fixed `theta<1/2`. The useful missing lemma is an algorithm for the **actual structured pair spectrum**, with charged target-independent setup, memory, exact witness recovery or complete miss handling, and that query bound. The identities above do not supply it.

A miss can be certified algebraically by a Bezout identity, or by the inverse of `K(r,V)` modulo `H`. If the chosen contract emits a dense length-`D` coefficient vector for that inverse, merely writing that output costs `Omega(D)` field-element writes. The same observation applies to an explicitly materialized dense Bezout certificate. This is a restriction on that output contract, not a lower bound for a Boolean decision, a differently represented proof, or every exact algorithm. No unconditional relation-solving lower bound or speedup is established here.
