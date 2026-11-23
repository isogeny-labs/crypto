# L1-pkm theory: what restriction arguments can and cannot give (desk proofs, `[pcc:unverified]`)

Setting. Tower PC: variables y_{i,j} (block i < m, level j < t), axioms y_{i,j+1} − y_{i,j}²,
1 − y_{i,t−1}², and the split-Semaev chain C = {S3(x0,x1,u0), S3(u_{k−1},x_{k+1},u_k),
S3(u_{m−3},x_{m−1},x_R)} with x_i = g·y_{i,0}. Modulo the tower axioms every polynomial has a
unique normal form (NF) in F_p[Z/2^t]^{⊗m}[u]; the block monomial y_{i,0}^e has degree
popcount(e). Deg_PC(F) = least D such that 1 is in the degree-D closure W_D (PROTOCOL.md).
Raw-presentation PC degree equals NF-closure degree for D ≥ 4 (both directions: NF(x·f) =
NF(x·NF(f)) and reduction never raises degree; the S3 generators are derivable in degree 4).

## Lemma M (monotonicity in t; proved)
Let the instance class allow a separate scale g_i per block (V_i = g_i·μ_{2^t}). For any
inconsistent instance I_t and any signs s_i, the restriction y_{i,t−1} := s_i followed by the
degree-preserving rescaling y_{i,j} = ω_i^{2^j}·y'_{i,j} (ω_i^{2^{t−1}} = s_i) is an inconsistent
instance I'_{t−1} of the class, with V'_i = g_i ω_i μ_{2^{t−1}}. Restriction and invertible
diagonal rescaling never raise PC degree, so Deg_PC(I_t) ≥ Deg_PC(I'_{t−1}).
Consequence: max over the class of Deg_PC is non-decreasing in t. (It does not say it grows.)

## Proposition R (one free block has t-independent degree; proved)
Fix m and let I be an inconsistent instance in which blocks 1..m−1 are restricted to points
(x_i = c_i constants), block 0 and all u_k free. Then Deg_PC(I) ≤ C(m) := max(c_1(m), 2m),
where c_1(m) depends on m (and the field) only, not on t.

Proof. (a) Elimination. Let h(X) = S_{m+1}(X, c_1, …, c_{m−1}, x_R) (Semaev; deg_X h ≤ 2^{m−1}).
The split identity (iterated resultant in u) puts h in the ideal of the chain in F_p[X, u], so
h = Σ a_k f_k with deg(a_k f_k) ≤ c_1(m). This identity does not involve t. Substituting
X = g·y_{0,0} and taking NF maps a raw degree-d line to NF degree ≤ d (popcount(k mod 2^t) ≤ k),
and maps the PC rules to PC rules, so h_0(y_{0,0}) := h(g·y_{0,0}) is derived in degree ≤ c_1(m).
(b) Graeffe root squaring inside the tower. Write h_j(Y) with deg_Y ≤ e = 2^{m−1} and define
h_{j+1} by h_j(Y)·h_j(−Y) = h_{j+1}(Y²). With Y = y_{0,j}: Y^k has NF degree
popcount(k·2^j mod 2^t) ≤ popcount(k) ≤ L := m (k ≤ 2^{m−1}). Multiplying the derived line h_j
(NF degree ≤ L) by the ≤ e+1 monomials of h_j(−Y), each a product of ≤ L variables, uses lines of
degree ≤ 2L, and the sum is h_{j+1}(y_{0,j+1}) in NF (degree ≤ L). After t steps
y_{0,t} := y_{0,t−1}² = 1, so h_t(1) = lc·Π_r (1 − r^{2^t}) over the roots r of h_0, which is a
nonzero constant iff no root lies in μ_{2^t}, i.e. iff I is inconsistent (every root of h gives a
u-chain over F̄_p). Hence 1 ∈ W_{max(c_1, 2L)}. ∎

Measured check (instrument 1, p = 40961, runs/restricted_*.txt): Deg_PC = 2, 3, 4, 5, 5 for
m = 2, 3, 4, 5, 6, identical at every t tested (t = 2…12 for m = 3, 4; t = 2, 6, 12 for
m = 2, 5, 6). Consistent with R; the Graeffe bound 2m is loose (measured ≈ m − 1 … m).

## Corollary (a barrier, proved)
Any lower-bound argument for Deg_PC(m, t) that works by restricting all but one block (as in
the closure note's Lemma X reduction, or a one-block "pigeonhole/univariate" reduction) is
capped at C(m) = O_m(1) and cannot show growth with t. A t-growing lower bound must use at least
two free blocks jointly. For r ≥ 2 free blocks the Graeffe argument fails: the product over the
2^r sign patterns multiplies each block's degree by 2^{r−1} per level, so the exponents (and NF
degree) are no longer bounded by a function of m alone; the generic upper bound it yields is
O(t)·r, which is no better than the trivial one.

## What is NOT proved
- No lower bound Deg_PC(m,t) ≥ f(t) with f → ∞, for any fixed m ≥ 2 (the theorem that would
  close PKM). Candidate route, not carried out: a two-block expansion/"design" argument in the
  Fourier basis of μ_{2^t} (as in Buss–Grigoriev–Impagliazzo–Pitassi for Tseitin mod q), where the
  tower encoding makes the degree the popcount of the character index.
- No t-independent upper bound for two or more free blocks. Measured (instrument 1):
  full m = 2 (two free blocks, no aux): 3,4,4,4,4,5,5,5,5 for t = 1..9, ≥ 6 at t = 10 (one degree
  per ≈ 4 t-steps); restr2 m = 4 (blocks 2,3 fixed; two free blocks + 2 aux): 5 for t = 1..6,
  ≥ 6 at t = 8; restr2 m = 3: 4 at t = 2,3,4. So with two free blocks the degree does grow, but
  slowly; with one free block it is flat (Prop. R). Growth in the full chain therefore comes from
  the joint behaviour of ≥ 2 free blocks, which is exactly where no lower-bound technique was
  carried through here.
