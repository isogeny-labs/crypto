# Target intersection of triple-sum divisors: exact matrices and size wall

Research note, 2026-11-26. The pair-sum quotient and triple-sum resultant `D₂,G,D₃` are already described in [pair_sum_quotient.md](pair_sum_quotient.md) and an earlier working memo. This note adds the **target-intersection operator** and audits the proposed displacement-rank shortcut. It makes no new solver claim.

## Triple-level operator

Let `A(X)` be the squarefree product over the distinct finite x-coordinates of signed triple sums, with either sign allowed for each of three distinct base columns. Write `m=deg A`; the random-support model gives `m=Θ(B³)`. For nonzero target `R` with `w=x(R)`, define

`C_R(Y)=Res_X(A(X),F₃(X,w,Y))=∏_{u:A(u)=0}F₃(u,w,Y)`.

Here `F₃` is the usual three-point summation polynomial. A root of `C_R` is the x-coordinate of `R±P` for some triple sum `P`. Thus a signed six-term decomposition of `R` exists **iff** `gcd(A(Y),C_R(Y))` has positive degree, allowing the two three-column halves to overlap. A globally distinct six-column witness requires an additional index-disjointness check. If `A` is squarefree, this gcd degree counts distinct matched x-classes (not signed witness multiplicity); back-substitution through the two triple witnesses resolves signs. Every factor `F₃(u,w,Y)` is quadratic except possibly when `u=w`, where one addition branch is infinity, so `2m−1≤deg C_R≤2m` on this odd-order curve.

Equivalently, let `C` be the `m×m` companion matrix of `A`. The target operator `C_R(C)` on `F_p[Y]/(A)` is singular exactly when the six-sum intersection is nonempty. This is an exact algebraic criterion, but it already has `m` state coordinates. The multiset resultant `D₃` from the earlier memo can replace `A` without changing the zero/nonzero criterion; it has proven degree `2B³−B²≤deg D₃≤2B³`, so it makes the state larger.

## Displacement rank: genuinely small, still insufficient

The Sylvester matrix `S(A,C_R)` has `m+deg C_R≈3m` rows. It consists of two rectangular Toeplitz convolution blocks. With the down-shift `Z_n` and a block-diagonal column shift `diag(Z_deg(C_R),Z_m)`, each block's displacement has nonzero entries only in its first row and first column. Therefore

`rank(S − Z_(m+deg C_R) · S · diag(Z_deg(C_R),Z_m)^T) ≤ 4`.

This **constant displacement rank is real but generic to polynomial resultants**; it does not arise from the root-of-unity factor base. A structured resultant or subresultant algorithm can exploit it to work quasi-linearly in the *degree* once dense coefficients are supplied. That is still `Ω(m)=Ω(B³)` input/state work. Moreover, generating the dense coefficients of `A` and `C_R` from the compact `D₂,G` circuit is itself a `Θ(B³)`-degree expansion. An unproved, stronger algorithm would have to operate directly on the compact circuit and output a common root without ever materializing those coefficient vectors. Low Sylvester displacement rank by itself gives no such operation.

At `B=2447`, `B²=5,987,809` and the proven multiset `D₃` degree is at least `29,298,349,437`. Merely storing its coefficients as `u64` needs more than 234 GB (decimal). A Sylvester matrix need not be stored densely, but its standard structured generators still have `Θ(B³)` scalar entries. These figures are size bounds for the direct polynomial route, not lower bounds for every implicit algorithm.

## Pair-level operator and the lost separability

Let `n=deg D₂=B²`, `A₂=F_p[U]/D₂(U)`. For two pair-sum x-roots `u,v`, and base columns `k,l`, set

`K_{k,l,w}(U,V)=Res_X(F₃(U,x_k,X), Res_Y(F₃(V,x_l,Y),F₃(X,Y,w)))`.

All eliminated polynomials have constant degree, so one `K_{k,l,w}` has constant degree in `U,V`. Every genuine finite six-term witness makes the relevant `K_{k,l,w}(u,v)` vanish. The converse is secure only after saturating degree-drop and infinity factors and checking signs and indices by direct group addition: for example, `u=x_k` or `v=x_l` can lower a quadratic's degree and create a projective common root in an unsaturated resultant. The product `K_w(U,V)=∏_{k,l}K_{k,l,w}(U,V)` is therefore a **necessary candidate filter**, not an unqualified witness oracle. Multiplication by `K_w` in `A₂⊗A₂` has **`n²=B⁴` states**, or `35,853,856,620,481` at the selected `B`, before any dense matrix storage. This is already worse than direct `Θ(B³)` triple enumeration, so the pair-level operator is only a structural accounting device, not an algorithmic improvement. The product over `B²` base-column pairs also destroys the constant-factor separability of an individual `F₃` kernel; no bounded displacement rank for this full product follows from its low-degree factors.

There is a small positive structural fact: `F₃(X,Y,w)` has bidegree `(2,2)` and hence separable tensor rank at most 3 in the monomial bases `1,X,X²` and `1,Y,Y²`. The multiplication operator on `F_p[X,Y]/(A(X),A(Y))` is therefore a sum of at most three Kronecker products. Yet that tensor space has `m²=Θ(B⁶)` coordinates, and a low Kronecker rank of the **polynomial** does not imply a low-rank zero-indicator or a fast null-vector locator. The pair-level elimination above reduces the base algebra to `B⁴` states, still beyond `B²`.

## Cost conclusion and falsifiable lead

Precomputing the pair roots and the compact `D₂,G` circuit costs `O~(B²)` time and space. Using the proven-degree **multiset** `D₃`, the explicit polynomial target operator needs `Θ(B³)` state; its squarefree replacement `A` has that size under the random-support model unless a massive collision collapse is demonstrated. The pair-product operator has `Θ(B⁴)` state unconditionally from the counted-multiplicity `D₂`, and loses even to direct triple enumeration. The only credible continuation is a **circuit-aware** intersection algorithm that exploits more than ordinary Toeplitz displacement rank: it must accept `(D₂,G,w)`, return signed triple witnesses, and use `O~(B²)` total target work and memory. A toy implementation should record its largest materialized object and arithmetic count as `B` grows; producing a `Θ(B³)` coefficient vector or `Θ(B⁴)` grid falsifies this route at the desired exponent.
