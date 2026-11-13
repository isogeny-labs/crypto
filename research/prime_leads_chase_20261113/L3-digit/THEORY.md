# Theory: a digit-count lower bound on the polynomial-calculus solving degree (L3-digit, 2026-11-13)

Status: paper proof by this lab, not machine-checked, not adversarially checked. `[pcc:unverified]`.

## Setting
A = F[a_1..a_n]/(a_i^2 - a_i), F a field of characteristic p (or any extension of F_p). Degree = multilinear
degree. For a set of axioms F ⊂ A, the degree-c PC closure V_c(F) is the smallest subspace of A_{<=c}
containing F ∩ A_{<=c} and closed under: g ∈ V_c, deg g < c  ⇒  a_i g ∈ V_c (reduced in A). The PC
solving (refutation) degree is D(F) = min{c : 1 ∈ V_c(F)}. Every degree-c-bounded PC/F4/XL-with-mutants
derivation stays inside V_c, so D lower-bounds their maximal degree (in the quotient-ring convention).

## Lemma R (restriction)
Let ρ fix a subset J of the variables to values in {0,1}. Then ρ: A → A' is an algebra map that does not
raise degree, and ρ(V_c(F)) ⊆ V_c(ρF). Hence D(F) ≥ D(ρF).
*Proof.* Induction on the closure: ρ(F) ⊆ ρF; linear combinations commute with ρ; if g ∈ V_c with deg g < c
then ρ(a_i g) = ρ(a_i)ρ(g) is a scalar multiple of ρ(g) (i ∈ J) or a_i ρ(g) with deg ρ(g) ≤ deg g < c (i ∉ J),
which lies in V_c(ρF). ∎ (Same proof for any finite digit sets.)

## Lemma M (monotonicity / scalar extension)
If F ⊆ V_c(G) then V_c(F) ⊆ V_c(G). Extending scalars to F' ⊇ F maps V_c(F) into V_c^{F'}(F). ∎

## Lemma U (no top-degree fall below the middle)
Let L = Σ_i w_i a_i − ρ with every w_i ≠ 0 in F, k variables, and c ≤ (k+1)/2, p > c. Put
X_c = {tL : deg t ≤ c − 1}. Then every g ∈ X_c with deg g < c is tL with deg t ≤ c − 2; hence X_c is
PC-closed and V_c(L) ⊆ X_c.
*Proof.* For t with homogeneous top part t_{c−1}, the degree-c part of tL is φ(t_{c−1}) where
φ(a_S) = Σ_{i∉S} w_i a_{S∪{i}} (terms with i ∈ S and the −ρ term have degree ≤ c − 1). Rescaling
a_S ↦ (Π_{i∈S} w_i) a_S conjugates φ to the unweighted up-operator U_{c−1→c} of the Boolean lattice,
whose matrix is the inclusion matrix W_{c−1,c}(k). By Wilson's rank theorem its F_p-rank is
Σ_{i ≤ c−1, p ∤ C(c−i, c−1−i)} (C(k,i) − C(k,i−1)); here C(c−i, c−1−i) = c − i ∈ [1, c] is prime to p, so the
rank is C(k, c−1): U is injective when c − 1 ≤ k − c. So deg(tL) < c forces t_{c−1} = 0. ∎
(Wilson: R. M. Wilson, "A diagonal form for the incidence matrices of t-subsets vs. k-subsets",
Europ. J. Combin. 1990 — cited from memory, not re-read here.)

## Theorem L (single digit-linear equation)
If L = Σ w_i a_i − ρ (all w_i ≠ 0) has no zero on {0,1}^k and p > k/2 + 1, then D(L) ≥ ⌈k/2⌉ + 1.
*Proof.* L is a unit of A (it vanishes nowhere), with inverse h = 1/L. If deg h ≤ (k−1)/2, apply the
top-degree argument of Lemma U to hL = 1 at c = deg h + 1 ≤ (k+1)/2: the degree-c part φ(h_top) of hL
must vanish, so h_top = 0, a contradiction. So deg h ≥ k/2. For c ≤ (k+1)/2, V_c(L) ⊆ X_c, and 1 ∈ X_c
means 1 = tL with deg t ≤ c − 1 ≤ (k−1)/2, i.e. t = h of degree < k/2: impossible. Hence 1 ∉ V_c for every
c ≤ ⌊(k+1)/2⌋, i.e. D ≥ ⌊(k+1)/2⌋ + 1 = ⌈k/2⌉ + 1. ∎
This contains the Impagliazzo–Pudlák–Sgall bound for Σ x_i = r (unit weights) as a special case and holds
for binary weights 2^i (which are nonzero mod p) and for random nonzero weights.

## Theorem S (Semaev systems with Boolean digits)
Let f = S_{m+1}(ℓ_1, …, ℓ_m, r) with ℓ_j = Σ_{i<k} 2^i a_{j,i} (k Boolean digits per block, p > 2^k), and
suppose f has no zero on the grid and some assignment λ of blocks 2..m makes q(x) = S_{m+1}(x, λ_2, …, λ_m, r)
a non-constant polynomial in x (for m = 2: λ_2 ≠ r suffices, the x^2-coefficient being (λ_2 − r)^2).
Then D(f) ≥ ⌈k/2⌉ + 1, i.e. D ≥ n/(2m) + 1 with n = mk.
*Proof.* By Lemma R, D(f) ≥ D(q(ℓ_1)). Over F' = splitting field of q, q = c Π_j (ℓ_1 − ρ_j) with every ρ_j
outside ℓ_1({0,1}^k) (q has no zero on the cube). For c ≥ deg_x q, q is derivable from L = ℓ_1 − ρ_1 inside
degree c (multiply L by the monomials of the cofactor; each product has degree ≤ c), so by Lemma M
V_c(q) ⊆ V_c(L). If c < deg_A q then V_c(q) = {0}. Either way 1 ∈ V_c(q) needs 1 ∈ V_c(L), so by Theorem L
D(f) ≥ ⌈k/2⌉ + 1. ∎

## Generalisation to b-ary digits (conditional)
The same proof works for digits in {0..b−1} (A = ⊗ F[d]/(Π_c(d − c))) with the Boolean lattice replaced by
the product of chains [b]^k and Wilson's theorem replaced by injectivity of the unit-weight raising
operator of [b]^k below its middle rank over F_p. That injectivity is classical over Q for products of chains
(Peck / Lefschetz property; cited from memory) and is not proved here over F_p. Conditional on it,
D(ℓ − ρ) ≥ ⌈k(b−1)/2⌉ + 1. The measured `linb3` values D = k + 1 (k = 2..8, 14 draws) agree exactly.

## What this does and does not give
- It proves that no Boolean-digit presentation has bounded PC solving degree: D grows at least like n/(2m),
  for every m, for every non-degenerate unsatisfiable target. With the measured values at m = 2
  (D = 4 + ⌊k/2⌋) the slope 1/2 per digit is **exactly tight**; only the additive constant is not captured
  (measured +3 at m = 2; the cited first-fall closed form suggests m·2^{m−1} − 2^{m−2} at general m).
- It is a degree bound, not a cost bound. A cost bound needs the dimension of what must be computed at
  degree D; in the full-rank (XL/Macaulay) charge the cost is at least dim A_{≤D} ≈ 2^{n H(1/(2m))} =
  B^{m H(1/(2m))}. That exponent is **below** the rho requirement for large m (θ = ω m H(1/(2m)) < m − 3 for
  m ≥ 6 at ω = 1 and m ≥ 9 at ω = 2), so the lower bound alone does not close the lever asymptotically.
- Relation to Hypothesis G clause (ii) (closure note §3.3): Theorem S says that, for Boolean digits, **no**
  unsatisfiable non-degenerate target has refutation degree ≤ ⌈k/2⌉; low-degree unsatisfiable targets are
  confined to the degenerate set (empty at m = 2 whenever the block-2 image has ≥ 2 values ≠ r).
  Clause (ii) concerns decomposable (satisfiable) targets detected at low degree; the restriction argument
  does not by itself bound their detection degree, so clause (ii) is tested empirically (census).

---

# Addendum (00:55Z, same day): the exact first-fall lower bound, valid for every target

Found after the first m = 2/3 rows showed V_c = f·A_{<=c-deg f} (rank_{D-1} = dim A_{<=D-1-deg f})
right up to D. This supersedes Theorem S as the main statement (Theorem S stays as the weaker, simpler bound).

## Setting
Presentation with m blocks; block algebra A_blk = ⊗_{i<k} F[d_i]/(Π_{c∈X_i}(d - c)), A = A_blk^{⊗m};
gr(A) = ⊗ F[d]/(d^{|X|}) (associated graded for the degree filtration). ℓ_j = Σ w_i d_i with all w_i ≠ 0.
f = S_{m+1}(ℓ_1..ℓ_m, r). Let e = 2^{m-1}. Fact (checked symbolically for m = 2, 3: the x-top part of S_3 is
x_1^2 x_2^2, of S_4 is x_1^4 x_2^4 x_3^4, coefficient 1, independent of r): f_top = (ℓ_1 ⋯ ℓ_m)^e in gr(A),
the same for every target r.

## Lemma F (no fall below j*)
Let j*_blk = min{ j : multiplication by ℓ^e : gr_j(A_blk) → gr_{j+e}(A_blk) is not injective }.
Multiplication by f_top on gr_j(A) is injective for j < j*_blk (it is a tensor product of the per-block maps on
each multidegree component, injective iff every factor is), and not injective at j = j*_blk (component
(j*,0,..,0)).
Hence for c < ffd := m·e + j*_blk:  V_c(f) = X_c := { t f : deg t <= c - m e }, and every nonzero element of
V_c has degree >= m e.
*Proof.* X_c ∋ f, X_c ⊆ A_{<=c}. If g = t f with deg t = c - me then deg_c(g) = t_top·f_top ≠ 0 (injectivity at
degree c - me < j*), so g of degree < c has deg t <= c - me - 1 and a_i g = (a_i t) f ∈ X_c: X_c is PC-closed,
so V_c ⊆ X_c (and ⊇ trivially). A nonzero t f has degree deg t + me by the same injectivity. ∎

## Theorem F
For every target r (satisfiable or not): no nonzero polynomial of degree < m e (in particular no constant and
no linear form) lies in V_c(f) for c < ffd. So the PC refutation degree of an unsatisfiable target and the PC
detection degree (first degree with a linear consequence) of a satisfiable one are both >= ffd = m e + j*_blk.
For unsatisfiable targets this is the statement 1 ∉ V_c; it uses no property of r.

## Values of j*_blk
- Boolean digits, k bits, p > k: ℓ^e acts on the squarefree algebra; rescaling makes ℓ = Σ a_i, and
  W_{j,j+e}(k) has full rank C(k,j) over F_p (Wilson, p > k) iff C(k,j) <= C(k,j+e), i.e. j <= (k-e)/2.
  So j* = ⌊(k-e)/2⌋ + 1 and **D >= m 2^{m-1} + ⌊(k - 2^{m-1})/2⌋ + 1** (k >= e) — exactly the ledger's
  first-fall closed form, now a proved lower bound on the solving degree (for p > k).
- b-ary / mixed radix digits: gr(A_blk) = monomial complete intersection; if ℓ^e has maximal rank
  (strong Lefschetz; classical in characteristic 0, assumed here for p > top degree, not proved) then
  j* = min{ j : h_j > h_{j+e} } for the symmetric Hilbert function h of the block; for equal radix b,
  j* = ⌊(k(b-1) - e)/2⌋ + 1.
- Univariate base of size B (one variable per block, any B-set, interval or random): gr = F[x]/(x^B),
  ℓ^e = x^e is injective on degree j iff j + e <= B - 1, so j* = B - e and **D >= B + (m-1)·2^{m-1}**
  (B + 2 at m = 2, B + 8 at m = 3). Fully proved (no Lefschetz input). This is the F4-side complement of
  Theorem B: the closure-note checker's msolve degrees d+2 (m=2) and d+8 (m=3) on univariate bases are
  exactly this bound, now shown to hold for every degree-bounded PC/F4/XL-with-mutants strategy.

## Consequence for cost (model-relative)
At c = D the measured closures have rank ≈ dim A_{<=D} (full) and below D the rank is dim A_{<=c-me}. In the
full-rank charge T_dec >= dim A_{<=ffd}. With D ≈ (block top degree)/2 + me, dim A_{<=D}/B^m →
P(J_1+..+J_m <= N_blk/2 + me) for iid block degrees J_i: a constant (univariate: ≈ 1/(2^m m!) up to lower
order, exponent 1) or B^{-Ω(1)} (many small digits). See project.py / project.txt for the numbers.
