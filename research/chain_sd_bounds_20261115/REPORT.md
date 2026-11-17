# Grevlex solving degree of the normalized two-summand Kummer chain systems over F_p

**Study:** `research/chain_sd_bounds_20261115/` (2026-11-15). **Standing:** every statement below is
`[pcc:unverified]` (no pcc claim filed in this session). Lean theorems are kernel-checked on the
standard foundations (`propext`, `Classical.choice`, `Quot.sound`); computed values carry replay
certificates verified by an independent C checker where stated.

**Task.** Rigorous bounds `ck − C ≤ sd(I_k) ≤ Ak + B` on the grevlex solving degree of the [pcc:unverified]
repository's normalized two-summand elliptic-curve chain systems (depth `k`, `2^(k+1) | p − 1`),
with pinned definitions, an admissible unit family, proved transfers, finite-depth certificates
beyond depth 14, and Lean proofs.

## 0. Results at a glance

| # | statement | status |
|---|---|---|
| T1 | Quotient reduction: `V_{F,c} = NF(V_{F,c}) ⊕ J_{≤c}`, and `NF(V_{F,c}) = U_c`, the weight-`c` closure in `F_p[(ℤ/2^k)²]` | proved (§2, desk) |
| T2 | For the unit ideal, pinned solving degree = degree-bounded refutation degree = Huang–Kosters–Yeo last fall degree: `sd(I_k) = ref(I_k) = d_F(I_k) = LFD(k)` | proved (§3; Lean `lastFallDegree_eq_solvingDegree_chain` on the concrete chain algebra, reachability included) |
| T3 | Admissible unit family: `p ≡ 1 (mod 2^(k+1))`, `E: y² = x³ + n²(n−1)x`, `t = n`, `n` a non-square; unit for every such `k`; infinitely many `p` for every `k` | proved (§4; Lean `family_S3_ne_zero`, `S3_ne_zero_of_cm`) |
| T4 | Depth monotonicity: `LFD(k') ≤ LFD(k)` for `k' ≤ k`, same `(p, A, B, t)` | proved (§5; Lean `one_mem_closure_of_le`, `solvingDegree_mono`) |
| T5 | ℤ-exponent (no-wrap) model: `π_k(W_c) ⊆ U_c(k)`; observed equality in all 6 tested cells | containment proved (§5), equality empirical |
| T6 | Constant lower bound `LFD(k) ≥ 4` for all `k ≥ 2` (needs `t ≠ 0`, `A ≠ −3t²`); `LFD(k) ≥ 3` for `k ≥ 2` and all `(A, B, t)` | `≥ 4` proved (§7, desk); `≥ 3` Lean (`one_not_mem_closure_two`, `two_lt_solvingDegree`) |
| T7 | Upper bound `LFD(k) ≤ 2k + 1` for every unit instance, `k ≥ 1`: `A = 2, B = 1` | proved (§6, desk) |
| T8 | Finite depths: exact `LFD(k) = 4, 4, 4, 4, 5, 5, 5, 5` for `k = 2..9` on the admissible instance `p = 12289, n = 11`, each with an upper and a lower replay certificate; `LFD(k) ≥ 6` for `10 ≤ k ≤ 15` on `p = 65537, n = 3` by one certified closure at `k = 10` plus T4 | certified (§8; all 17 certificates verified by the independent checker) |
| T9 | Linear lower bound `ck − C ≤ LFD(k)` with `c > 0` | **open** (§7.3: the missing lemma is stated exactly; the lane-P slope-1/14 proof is not re-derived here) |

Separate reports, as requested: finite-depth results in §8, constant bounds in §6–7 (`4 ≤ LFD(k) ≤ 2k + 1`), linear growth in §7.3 (open above; the proved linear *upper* growth is `2k + 1`). [pcc:unverified]

## 1. The pinned objects

**Ring and order.** `k ≥ 1`, `N = 2^k`, `p` prime with `2^(k+1) | p − 1`. The ring is
`R = F_p[y_{1,0}, …, y_{1,k−1}, y_{2,0}, …, y_{2,k−1}]` (`2k` variables) with the graded reverse
lexicographic order for `y_{1,0} > y_{1,1} > … > y_{1,k−1} > y_{2,0} > … > y_{2,k−1}`. Nothing
below depends on the tie-break within a degree; every statement is about a degree-compatible order.

**Generators.** `F_k = {y_{s,j+1} − y_{s,j}² (s = 1,2; 0 ≤ j ≤ k−2), y_{s,k−1}² − 1 (s = 1,2),
S_3(y_{1,0}, y_{2,0}, t)}`, with Semaev's third summation polynomial for `E: y² = x³ + Ax + B`,

```
S_3(a, b, c) = (a − b)² c² − 2((a + b)(ab + A) + 2B) c + (ab − A)² − 4B(a + b).
```

`t = x(R)` is the target abscissa. This is the "normalized" presentation: the chain ends at
`y_{s,k−1}² = 1` (factor base `V = μ_N = {x : x^N = 1}`, no scaling `g`), the same as the
`kummer` rules of the repository engines (`tower_engine.py`, N8). The tower part `J_k = (F_k ∖
{S_3})` is a radical zero-dimensional ideal with `V(J_k) = μ_N × μ_N ⊂ F_p²` (all points rational
because `N | p − 1`). `I_k = (F_k)`.

**Field equations.** None are added. For `c < p` they are inert: `V_{F,c}` only admits generators of
degree `≤ c`, and `deg(y^p − y) = p > 2k + 2 ≥` every degree considered (`p ≥ 2^(k+1) + 1`). In
particular `V_{F ∪ FE, c} = V_{F,c}` for all `c ≤ 2k + 2`.

**Algorithm.** The degree-by-degree Macaulay closure of Caminata–Gorla (MutantXL; the repository's
`hky_last_fall_degree` engine): for `c = 1, 2, …` compute

```
V_{F,c} = the smallest F_p-subspace with  F ∩ R_{≤c} ⊆ V_{F,c}  and  g ∈ V_{F,c}, deg(hg) ≤ c ⇒ hg ∈ V_{F,c},
```

and stop at the first `c` for which `V_{F,c}` contains a grevlex Gröbner basis of `I_k`.
**Solving degree** `sd(I_k)` := that `c`, the maximum degree reached. For the unit ideal the stopping
rule is `1 ∈ V_{F,c}` (any Gröbner basis of `(1)` contains a nonzero constant, and `{1}` is one).

**Degree-bounded refutation degree** `ref(I_k) := min{c : 1 ∈ V_{F,c}}` (the Clegg–Edmonds–Impagliazzo
polynomial-calculus refutation degree with the generators as axioms, over `F_p`).

**Last fall degree** (HKY, the repository's `d_F`): `d_F = min{c : ∀ f ∈ I_k, f ∈ V_{F, max(c, deg f)}}`.

## 2. Theorem 1: reduction to the chain algebra

Let `Q_k = F_p[(ℤ/N)²]` be the group algebra with basis `e_v`, `v = (a, b)`, `e_v e_w = e_{v+w}`
(the character `x_1^a x_2^b` on the grid), **weight** `wt(a, b) = popcount(a) + popcount(b)`,
`Q_{≤c} = span{e_v : wt v ≤ c}`, and the `2k` **shifts** `e_{(2^j,0)}·`, `e_{(0,2^j)}·`. The
**generator** is `g = S_3(e_{(1,0)}, e_{(0,1)}, t) = Σ_{(a,b)∈{0,1,2}²} g_{ab} e_{(a,b)}`, of weight 2.
Define the **weight-`c` closure** `U_c(k)` as the smallest subspace containing `g` with
`u ∈ U_c, wt u ≤ c − 1 ⇒ e_s u ∈ U_c` for every shift `s` (`U_0 = U_1 = {0}` if `c < 2`; `U_c ⊆ Q_{≤c}`
since `wt(e_s u) ≤ wt u + 1`).

**Theorem 1.** (a) `F_k ∖ {S_3}` is a Gröbner basis of `J_k` for every degree-compatible order, with
leading monomials `y_{s,j}²`; the standard monomials are the squarefree monomials, and the normal
form map `NF : R → span(squarefree) ≅ Q_k`, `∏ y_{s,j}^{ε_{s,j}} ↦ e_{(Σ_j ε_{1,j} 2^j, Σ_j ε_{2,j} 2^j)}`,
is a degree-non-increasing surjective linear map with kernel `J_k`, and `deg(NF m) = wt(NF m)`.
(b) For every `c`: `V_{F,c} = NF(V_{F,c}) ⊕ J_{≤c}` and `NF(V_{F,c}) = U_c(k)` (identifying
`span(squarefree)` with `Q_k`). Hence `1 ∈ V_{F,c} ⇔ 1 ∈ U_c(k)`.

*Proof.* (a) The leading monomials `y_{s,j}²` are pairwise coprime, so all S-polynomials reduce to
zero (Buchberger's first criterion) and the generators form a Gröbner basis; this holds for every
monomial order in which `LM(y_{s,j+1} − y_{s,j}²) = y_{s,j}²`, i.e. every degree-compatible order.
Reduction replaces `y_{s,j}²` by `y_{s,j+1}` (degree −1) or `y_{s,k−1}²` by `1` (degree −2), so
`deg NF(f) ≤ deg f`. A squarefree monomial is the product of the `y_{s,j}` with `ε_{s,j} = 1`, and in
`Q_k` it is `x_s^{Σ ε 2^j}`: its degree is the number of set bits, the weight. `NF` is multiplicative
modulo `J_k` and `NF(y_{s,j} · m) = e_{(2^j,0)} NF(m)` (resp. the second coordinate): adding `2^j`
to a binary exponent, with its carries, is exactly the reduction chain `y_{s,j}² → y_{s,j+1} → …`,
and `y_{s,k−1}² → 1` is the reduction mod `N`.

(b) `J_{≤c} ⊆ V_{F,c}`: an element of `J_{≤c}` is `Σ u_j h_j` with `deg(u_j h_j) ≤ c` (the standard
representation of a degree-compatible Gröbner basis), and each `u_j h_j ∈ V_{F,c}` by the closure
rule applied to the generator `h_j`. `NF(V_{F,c}) ⊆ V_{F,c}`: `NF(f) = f − Σ u_j h_j` with
`deg(u_j h_j) ≤ deg f ≤ c`. So `V_{F,c} = NF(V_{F,c}) ⊕ J_{≤c}`.

`U_c ⊆ NF(V_{F,c})`: `g = NF(S_3) ∈ NF(V_{F,c})`; if `ũ = NF(u) ∈ NF(V_{F,c})` with `wt ũ ≤ c − 1`
then `y_{s,j} ũ ∈ V_{F,c}` (closure rule: `ũ ∈ V_{F,c}`, `deg(y ũ) ≤ c`) and `NF(y ũ) = e_s ũ`.
`NF(V_{F,c}) ⊆ U_c`: by induction over the construction of `V_{F,c}`. Generators: `NF(S_3) = g`,
`NF(h_j) = 0`. Closure step: `f = ũ + j` with `ũ ∈ U_c`, `j ∈ J_{≤c}`, and `deg(h f) ≤ c` for a
monomial `h`. Then `h j ∈ J_{≤c}`, and `deg ũ ≤ deg f` (NF is computed from `f` alone and is
degree-non-increasing), so `deg(h ũ) ≤ deg h + deg f ≤ c`; writing `h` as a product of variables and
shifting one variable at a time, every intermediate `NF` has weight `≤ c − 1` before the last step
(`wt NF(y ũ') ≤ wt ũ' + 1`), so `NF(h ũ) ∈ U_c` by the `U_c` rule. ∎

**Terminology for the rest.** `LFD(k) := min{c : 1 ∈ U_c(k)}`. By Theorem 1, `ref(I_k) = LFD(k)`.

## 3. Theorem 2: solving degree, refutation degree, last fall degree

**Theorem 2.** For a unit instance (`I_k = (1)`): `sd(I_k) = ref(I_k) = d_F(I_k) = LFD(k)`.

*Proof.* `sd = ref` is the stopping rule (§1). `d_F ≤ ref`: let `1 ∈ V_{F,c₀}`. For `f ∈ R` of degree
`d`, put `c = max(c₀, d)`; `V_{F,c} ⊇ V_{F,c₀} ∋ 1` (monotonicity in `c`, Lean `closure_mono`), and
the closure rule applied to `1` gives every monomial of degree `≤ d ≤ c`, hence `f ∈ V_{F,c}`.
`ref ≤ d_F`: take `f = 1` (degree 0): `1 ∈ V_{F, max(d_F, 0)} = V_{F,d_F}`. ∎

Lean: `ChainSd.ShiftSystem.lastFallDegrees_eq_refutationDegrees`, [pcc:unverified]
`lastFallDegree_eq_solvingDegree` (abstract shift system, under the reachability hypothesis
"`1 ∈ V_c` implies `V_{c'} = R_{≤c'}` for `c' ≥ c`"), and for the concrete chain algebra
`ChainSd.lastFallDegree_eq_solvingDegree_chain` (file `Reach.lean`: the reachability hypothesis is
proved by adding the bits of the two exponents in increasing order, `single_mem_closure_of_one_mem`,
`weightSub_le_closure_of_one_mem`, using `popcount_add_pow_of_testBit_false`), plus
`lt_solvingDegree_of_not_mem`, `solvingDegree_le_of_mem` (what lower and upper certificates prove).

## 4. Theorem 3: the admissible family and its unitness

**Definition (admissible instance).** `(p, k, A, B, t)` with `p` prime, `2^(k+1) | p − 1`,
`4A³ + 27B² ≠ 0`, `t ≠ 0`, `t³ + At + B` a nonzero square (rational target), and **unit**:
`S_3(v_1, v_2, t) ≠ 0` for all `v_1, v_2 ∈ μ_N`. Unit ⇔ `I_k = (1)` ⇔ `g` is a unit of `Q_k`:
`Q_k ≅ F_p^{μ_N × μ_N}` by evaluation (CRT, `x^N − 1` splits with distinct roots in `F_p`), and
`g` is invertible iff it vanishes at no grid point; since `V(J_k) = μ_N²` is reduced and rational,
`I_k = (1)` iff `S_3` has no zero on the grid (Nullstellensatz for the radical ideal `J_k`).

**Theorem 3 (unitness).** Let `2^(k+1) | p − 1`, `E: y² = x³ + a x` with `a ≠ 0`, and `t ∈ F_p` a
non-square with `t³ + a t` a nonzero square. Then `S_3(v_1, v_2, t) ≠ 0` for all `v_1, v_2 ∈ μ_N`.
In particular, for every non-square `n ∈ F_p` the instance `A = n²(n − 1)`, `B = 0`, `t = n` is
unit at every depth `k` with `2^(k+1) | p − 1` (`t³ + At = n⁴`).

*Proof* (Lean `ChainSd.S3_ne_zero_of_cm`, `family_S3_ne_zero`; sketch). Every `v ∈ μ_N` is a square
(`v^{(p−1)/2} = (v^{2^k})^{(p−1)/2^{k+1}} = 1`, Euler's criterion; Lean `isSquare_of_pow_eq_one`).
Over an algebraic closure `K` pick `y_i² = v_i³ + a v_i`. The factorisation
`(v_1 − v_2)² S_3(v_1, v_2, c) = ∏_± (c(v_1 − v_2)² − ((y_1 ∓ y_2)² − (v_1 + v_2)(v_1 − v_2)²))`
(`S3_factor`) shows that `S_3(v_1, v_2, t) = 0` with `v_1 ≠ v_2` forces `t = addX` of
`(v_1, y_1) ± (v_2, y_2)`; by Mathlib's `WeierstrassCurve.Affine.equation_negAdd` the ordinate `y_3`
of that point satisfies `y_3² = t³ + a t`, a nonzero square of `F_p`, so `y_3 ∈ F_p^*`. From
`(y_1 − y_2)² ∈ F_p` one gets `y_1 y_2 ∈ F_p`, hence `y_1 y_3, y_2 y_3 ∈ F_p`, hence `y_1, y_2 ∈ F_p`.
The 2-descent identity `v_1 v_2 · addX · (v_1 − v_2)² = (v_1 y_2 − v_2 y_1)²` (`descent_identity`)
then makes `t` a square: contradiction. The diagonal `v_1 = v_2 = v` gives `4(v³ + av) t = (v² − a)²`
and `t = addX` of the doubling, with the same rationality argument (`isSquare_of_doubling`). ∎

**Non-degeneracy of the family.** `t = n ≠ 0`; the constant term of `g` is `A² − 4Bt = A² ≠ 0`;
`A + 3t² = n²(n + 2)`, so the exceptional tangent class `A = −3t²` (§7) is avoided iff `n ≠ −2`.

**Unbounded depth.** For every `k` there are infinitely many primes `p ≡ 1 (mod 2^(k+1))`
(Dirichlet; Mathlib `Nat.setOf_prime_and_eq_mod_infinite`), and every odd `p` has a non-square
`n ∉ {−2}` (there are `(p−1)/2 ≥ 2` non-squares for `p ≥ 5`). So `(p_k, E_k, t_k)` exists for all
`k`; the prime varies with `k`. Concretely `p = 65537` (`2^16 | p − 1`) covers `k ≤ 15` with `n = 3`,
`p = 786433 = 3·2^18 + 1` covers `k ≤ 17` with `n = 5`, `p = 12289` covers `k ≤ 11` with `n = 11`
(checked unit by grid enumeration at k = 15, 17, 11 respectively; engine `family`).

**A refuted alternative, preserved.** A *twist* target (`t³ + At + B` a non-square) is not unit:
sums of two twist points are twist points, so such a target is decomposable about as often as a
random one (≈ `N²/p` grid zeros; measured 16,168–16,570 at `k = 15`, `p = 65537`). The CM curve with a
*square* target is not unit either (`653–688` grid zeros at `k = 11`, `p = 12289`, `n ∈ {7, 13, 14, 17}`
squares), exactly as the descent predicts.

## 5. Theorems 4 and 5: transfers

**Theorem 4 (depth monotonicity).** For `k' ≤ k` and the same `(p, A, B, t)`: if `1 ∈ U_c(k)` then
`1 ∈ U_c(k')`. Hence `LFD(k') ≤ LFD(k)`.

*Proof* (Lean `ChainSd.one_mem_closure_of_le`, `solvingDegree_mono`). The group homomorphism
`ℤ/2^k → ℤ/2^{k'}` induces a ring homomorphism `φ: Q_k → Q_{k'}` with `φ(e_v) = e_{v mod 2^{k'}}`.
It does not increase weight (`popcount(a mod 2^{k'})` counts the low `k'` bits of `a`; Lean
`bitWeight_castZ_le` via `Nat.testBit_mod_two_pow`), sends the shift `2^j` to the shift `2^j` for
`j < k'` and to `1` for `j ≥ k'`, and `φ(g_k) = g_{k'}`. By the abstract transfer theorem
(`closure_transfer`: the preimage of a closed subspace under such a map is closed) `φ(U_c(k)) ⊆
U_c(k')`, and `φ(1) = 1`. ∎

**Theorem 5 (ℤ-exponent model).** Let `P = F_p[x_1, x_2]`, weight = popcount of the integer
exponents, and `W_c` the closure of `S_3(x_1, x_2, t)` under all shifts `x_i^{2^j}` (`j ≥ 0`) within
weight `≤ c`. Then `W_c ⊆ (S_3) ⊂ P` and `π_k(W_c) ⊆ U_c(k)` for every `k`, where `π_k` reduces
exponents mod `N`.

*Proof.* `π_k` is a ring homomorphism, weight-non-increasing, maps shifts to shifts or to `1`, and
`π_k(S_3) = g`; apply `closure_transfer`. ∎

**Observation (H-nowrap), not proved.** In every tested cell the containment is an equality: [pcc:unverified]
`rank π_k(W_c^{(k)}) = dim U_c(k)` for `(k, c) ∈ {(4,4), (5,4), (6,4), (7,4), (6,5), (7,5)}`
(engine `nowrap`; `W_c^{(k)}` uses shifts `j < k` and exponent width `k + 2`; exponents up to
`452 < 4N` occurred). The derivations in `Q_k` therefore never exploited the wrap `x^N = 1` beyond
what the integer closure already provides, including in the cells where `1` is derived.

**Coefficient field.** `U_c(k)` is defined by linear algebra over `F_p`; its dimension is invariant
under extension of the coefficient field, and the `F_p`-structure of the instance enters only
through `g`. Observed: the dimension profiles of `U_c(k)` below the LFD coincide for the N8 instance
(`p 12289, A 4021, B 9917, t 4408`), the family instance `p 12289, n 11`, and the family instance
`p 65537, n 3` at every tested `(k, c)` (e.g. `dim U_4(8) = 1061` with weight profile `(2, 117, 942)`
in all three). No theorem is claimed for this.

## 6. Theorem 7: the upper bound

**Lemma (reachability).** If `1 ∈ U_c(k)` then `U_{c'}(k) = Q_{≤c'}` for every `c' ≥ c`: a monomial
`e_v` with `wt v ≤ c'` is reached from `1` by adding the bits of `v` one at a time; every
intermediate monomial has weight `< wt v ≤ c'`, so each step is admitted. (This is the hypothesis
`hreach` of the Lean theorem `lastFallDegree_eq_solvingDegree`.)

**Theorem 7.** For every unit instance and `k ≥ 1`: `LFD(k) ≤ 2k + 1`.

*Proof.* Since `g` is a unit of `Q_k`, `1 = Σ_v γ_v e_v g` with `γ = g^{−1}` (all `v ∈ (ℤ/N)²`). It
suffices that `e_v g ∈ U_{2k+1}` for every `v`. Reach `e_v g` from `g` by adding the bits of `v` one at
a time (`2k` possible bits). If `wt v ≤ 2k − 1`, every proper partial sum `v'` has `wt v' ≤ 2k − 2`, so
`wt(e_{v'} g) ≤ wt v' + 2 ≤ 2k`, each step is admitted at budget `c − 1 = 2k`, and `wt(e_v g) ≤ 2k + 1`.
For `v = (N − 1, N − 1)` the last intermediate `v' = (N − 1 − 2^j, N − 1)` (or symmetric) has
`wt(e_{v'} g) ≤ 2k`: its monomials `(N − 1 − 2^j + a, N − 1 + b)`, `a, b ∈ {0, 1, 2}`, have weights
`popcount(N−1−2^j+a) + popcount(N−1+b)` with `popcount(N−1+b) ∈ {k, 0, 1}` and
`popcount(N−1−2^j+a) ≤ k − j + 1 ≤ k` for `a = 2, j ≥ 1`, `= k − j` for `a = 1`, `= k − 1` for `a = 0`,
and `popcount(N) = 0` for `j = 0, a = 2`; the maximum is `2k`. Finally `wt(e_{(N−1,N−1)} g) = 2k`.
Hence all `e_v g ∈ U_{2k+1}` and `1 ∈ U_{2k+1}`. ∎

So `A = 2, B = 1` (the simpler count `wt(e_{v'} g) ≤ wt v' + 2` gives `2k + 2`). The repository's
earlier record `prime_field_m2_lfd_bound` (ml-cryptanalysis, not readable here) states the same bound.

## 7. Lower bounds

### 7.1 Theorem 6: constant lower bounds for every depth

**Theorem 6.** Let `k ≥ 2`, `t ≠ 0`, `A ≠ −3t²` (any `B`). Then `LFD(k) ≥ 4`. (And `LFD(k) ≥ 3`
for every `k ≥ 2`, `A² − 4Bt` arbitrary.)

*Proof.* `U_2 = span{g}` (no element of weight `≤ 1` to shift), and `1 ∉ span{g}` because `g` has the
weight-2 term `e_{(2,2)}` with coefficient `1`. At `c = 3` the admitted shifts are those of `g`
(weight 2): `U_3 ⊇ span{g, e_s g : s}`. Each `e_s g` has weight exactly 3 and its weight-3 part is:
for `s = (2^j, 0)`, `j ≥ 2`: the four monomials `(2^j + a, b)`, `a, b ∈ {1, 2}` with coefficients
`g_{ab}` (`g_{22} = 1`); for `j = 1`: `g_{11} e_{(3,1)} + g_{12} e_{(3,2)}`; for `j = 0`:
`g_{21} e_{(3,1)} + g_{22} e_{(3,2)}`; symmetrically for the `x_2`-shifts. These `2k` weight-3 parts
have pairwise disjoint supports except the pair `j ∈ {0, 1}` in each coordinate, which is
independent iff `det [[g_{21}, g_{22}], [g_{11}, g_{12}]] = (−2t)(−2t) − (−2t² − 2A)·1 = 6t² + 2A ≠ 0`,
i.e. `A ≠ −3t²`. Hence `Σ c_s e_s g + c g` has weight `≤ 2` only if all `c_s = 0`: `U_3 ∩ Q_{≤2} = span{g}`,
no further shift is admitted, `U_3 = span{g, e_s g}`, and `1 ∉ U_3` since `1 ∉ span{g}`. ∎

For the family, `A + 3t² = n²(n + 2)`, so Theorem 6 applies for `n ≠ −2`. The bound is sharp:
`LFD(k) = 4` for `k = 2..5` (§8). The exceptional class `A = −3t²` is the "tangent" class
`3t² + A = 0` of the repository's lane-P notes.

### 7.2 Certified lower bounds at larger depth

By Theorem 4, a certified `1 ∉ U_c(k₀)` on an instance gives `LFD(k) ≥ c + 1` for every `k ≥ k₀`
with the same `(p, A, B, t)`. Table in §8.

### 7.3 The linear lower bound: open, and the exact missing lemma

The primary theorem asks for `ck − C ≤ LFD(k)` with `c > 0`. **This is not proved here.** What is [pcc:unverified]
established: `LFD` is nondecreasing in `k` on a fixed instance (T4); `LFD(k) ≥ 4` for `k ≥ 2` (T6);
`LFD(k) ≥ 5` for `6 ≤ k ≤ 11` on `p = 12289, n = 11` and `LFD(k) ≥ 6` for `10 ≤ k ≤ 15` on
`p = 65537, n = 3` (§8, certified); and the exact values
`LFD(k) = ⌊(k + 14)/4⌋` for `k ≤ 9` here (and for `k ≤ 15` in lane P, not re-verified).

**Missing Lemma (locality).** There are `α > 0`, `β` such that for every admissible unit instance and [pcc:unverified]
all `k`, `c` with `c ≤ αk − β`: `1 ∉ U_c(k)`. Any of the following would prove it:

1. *Dual (co-closed) form.* A family `Λ_c ⊆ Q_{≤c}^*` of functionals with `λ(g) = 0` for all `λ ∈ Λ_c`,
   co-closed on `Q_{≤c−1}` (for every `λ ∈ Λ_c` and shift `s`, `(λ ∘ e_s)|_{Q_{≤c−1}} ∈ Λ_c|_{Q_{≤c−1}}`),
   and containing some `λ` with `λ(1) ≠ 0`. Then `U_c ⊆ Λ_c^⊥` by induction and `1 ∉ U_c`. The natural
   candidates — evaluations at the points of the Semaev curve `S_3 = 0` with one coordinate in `μ_N`,
   lifted to exponents in `[0, N)` — annihilate `U_c` at `k = 7, 8` for `c ≤ 4` but **not** at
   `k = 6, c = 4` (188 of 605 basis rows), where nevertheless `1 ∉ U_4`; balanced lifts are worse
   (415 of 605). So the exact annihilator is not this family (counterexample preserved in
   `reports/evaltest.md`).
2. *ℤ-model form.* (H-nowrap) `U_c(k) = π_k(W_c)` for `c < LFD(k)` (observed, §5), together with:
   no `f ∈ W_c` satisfies `f ≡ 1 (mod x_1^N − 1, x_2^N − 1)` when `c ≤ αk − β`. Since `W_c ⊂ (S_3)`,
   this is a statement about low-binary-weight polynomials vanishing on the Semaev curve and their
   exponent spread; the lane-P note (ml-cryptanalysis `docs/proofs/N8-growth.md`, quoted in
   `research/leads_20261112/BRIEFING.md`) asserts `LFD ≥ ⌊(k − 10)/14⌋ + 1` by a characteristic-`p`
   abc/Wronskian argument of this type; the note was not accessible to this session and is not
   re-derived.
3. *Counting form.* `dim U_c(k) ≤ C(2k, ≤c) − 1` for `c ≤ αk − β`. The recorded dimensions are
   polynomial in `k` for fixed `c` below the LFD: `dim U_3(k) = 2k + 1` (`k = 2..9`),
   `dim U_4(k) = 4k² + 172k − 571` (`k = 6..12`, exact on all seven values), while the ambient
   dimension is `C(2k, ≤c)`.

Which `α` the data allow: the lane-P law gives slope `1/4`; the proved lower bounds here are [pcc:unverified]
constant; the proved upper bound has slope 2.

## 8. Finite-depth results and certificates

Instance **F1**: `p = 12289, n = 11` (`A = 1210, B = 0, t = 11`), admissible for `k ≤ 11`. Instance [pcc:unverified]
**F2**: `p = 65537, n = 3` (`A = 18, B = 0, t = 3`), admissible for `k ≤ 15`. Both are unit by Theorem 3
and by grid enumeration. `dim` = rank of the closure, `cols = C(2k, ≤D)`. Every row marked ✓ has a
replay certificate in `certs/` verified by `native/checker/chaincheck` (independent C
implementation; exit 0, `"verified":true`). Lower certificates include the full closure sweep.

| k | instance | lower certificate: `1 ∉ V_D` (dim/cols) | upper certificate: `1 ∈ V_D` | LFD(k) |
|---|---|---|---|---|
| 2 | F1 | D = 3: 5/15 ✓ | D = 4: 16/16 ✓ | 4 |
| 3 | F1 | D = 3: 7/42 ✓ | D = 4: 57/57 ✓ | 4 |
| 4 | F1 | D = 3: 9/93 ✓ | D = 4: 163/163 ✓ | 4 |
| 5 | F1 | D = 3: 11/176 ✓ | D = 4: 386/386 ✓ | 4 |
| 6 | F1 | D = 4: 605/794 ✓ | D = 5: 1586/1586 ✓ | 5 |
| 7 | F1 | D = 4: 829/1471 ✓ | D = 5: 3473/3473 ✓ | 5 |
| 8 | F1 | D = 4: 1061/2517 ✓ | D = 5: 6885/6885 ✓ | 5 |
| 9 | F1 | D = 4: 1301/4048 ✓ | D = 5: 12616/12616 ✓ | 5 |
| 10 | F2 | D = 5: 18569/21700 ✓ (closure sweep: 3,163 rows × 20 shifts, 0 nonzero) | — | ≥ 6 |
| 11..15 | F2 | transferred from k = 10 by Theorem 4 (Lean) | — | ≥ 6 |

Additional computed values without certificate files (engine runs, `verify_closed: bad = 0`):
`dim U_3(k) = 2k + 1` for `k = 4..9` and `dim U_4(k) = 605, 829, 1061, 1301, 1549, 1805, 2069` for
`k = 6..12` on the N8 instance (`p 12289, A 4021, B 9917, t 4408`; unit for `k ≤ 7`, proper with 2,
16, 100, …, 1399 grid zeros for `k = 8..12`); LFD = 4, 4, 4, 4, 5, 5, 5 on F2 for `k = 2..8` (engine
`lfd`), identical to F1.

**Depth-15 milestone, honest statement.** For the same admissible instance F2 beyond depth 14: [pcc:unverified]
lower certificate `LFD(15) ≥ 6` (the certified closure `1 ∉ V_5(10)` on F2 and Theorem 4, i.e. the
reduction homomorphism `Q_15 → Q_10` which is Lean-proved to carry `V_5(15)` into `V_5(10)`);
upper bound `LFD(15) ≤ 31` by Theorem 7 (a proof, not a computational certificate). The expected
exact value is 7 (lane-P law and the lane-P GPU cell `≥ 7` at `k = 15`, not re-verified here). A direct
upper certificate at `D = 7` needs the closure on `C(30, ≤7) = 2,629,575` columns, and a direct lower
certificate at `D = 6` on `768,212` columns; both are out of reach of this 4-core, 15 GB host (the
dense closure at `k = 10, D = 5`, 21,700 columns, takes about an hour here), and the derivation of `1`
found by back-tracing uses 75–85 % of the closure rows at `k ≤ 7`, so no short derivation is
available either. Those two cells are the concrete next computations.

**What a certificate proves.** `1 ∉ V_D` (checked closure) ⇒ `sd(I_k) > D` (Lean [pcc:unverified]
`lt_solvingDegree_of_not_mem`); `1 ∈ V_D` ⇒ `sd(I_k) ≤ D` (`solvingDegree_le_of_mem`); together with
Theorem 1 these are statements about the pinned polynomial-ring computation.

## 9. Dependence on the characteristic

* Definitions and Theorems 1, 2, 4, 5, 7 hold for every prime `p` with `2^(k+1) | p − 1` (Theorem 1
  needs only `N | p − 1` for rationality; the closure itself is defined for any `p`).
* Theorem 3 needs `p` odd (automatic) and uses `2^(k+1) | p − 1` exactly once: `μ_N ⊆ (F_p^*)²`.
  With only `2^k | p − 1` the grid is still rational but its elements need not be squares, and the
  descent argument fails (the data at `p = 1048583 ≡ 3 mod 4` in `research/leads_followup_20261114/
  item09_velu_tower/` are closures on non-rational grids).
* Theorem 6 needs `t ≠ 0`, `A ≠ −3t²`, and `p ≠ 2, 3` only through `6t² + 2A ≠ 0`.
* Dimensions of the closures are field-extension invariant (linear algebra) and were observed equal
  across `p ∈ {12289, 65537}` and across instances (§5); no theorem.

## 10. Proof map

| statement | Lean declaration (`lean/ChainSd/`) | hypotheses as formalised | status |
|---|---|---|---|
| closure monotone in `c` | `ShiftSystem.closure_mono` | abstract shift system | kernel-checked, std axioms |
| closure ⊆ weight-`c` part | `ShiftSystem.closure_le_F` | `g ∈ F c`, shifts raise weight ≤ 1 | checked |
| transfer (T4, T5 engine) | `ShiftSystem.closure_transfer`, `not_mem_closure_of_transfer` | `φ` linear, weight-non-increasing, intertwines or trivialises shifts, `φ g ∈` target closure | checked |
| T2 (abstract) | `ShiftSystem.lastFallDegrees_eq_refutationDegrees`, `lastFallDegree_eq_solvingDegree` | `1 ∈ F 0`, reachability hypothesis | checked |
| T2 (chain system) | `lastFallDegree_eq_solvingDegree_chain`, `single_mem_closure_of_one_mem`, `weightSub_le_closure_of_one_mem` (`Reach.lean`), `popcount_add_pow_of_testBit_false`, `popcount_mod_pow_le` (`Popcount.lean`) | none beyond `p` prime | checked |
| certificates ⇒ bounds | `ShiftSystem.lt_solvingDegree_of_not_mem`, `solvingDegree_le_of_mem` | finiteness of the refutation degrees for the strict bound | checked |
| T3 unitness | `S3_ne_zero_of_cm`, `family_S3_ne_zero`, `isSquare_of_pow_eq_one` | `p ≠ 2`, `a ≠ 0`, `¬IsSquare t`, `t³+at` nonzero square, `x_i` nonzero squares / `x_i^{2^k} = 1` with `2^(k+1) ∣ p−1` | checked |
| T3 bridge grid ⇒ unit ideal | — | CRT for `x^N − 1` over `F_p` | paper only (§4) |
| T4 depth monotonicity | `one_mem_closure_of_le`, `solvingDegree_mono` (concrete chain algebra, `AddMonoidAlgebra (ZMod p) (ZMod 2^k × ZMod 2^k)`, weight via `Nat.testBit`) | `k' ≤ k`, same `A B t` | checked |
| T1 quotient reduction | — | Gröbner basis of the tower ideal | paper only (§2) |
| T6, weak form `1 ∉ V_2` (`LFD ≥ 3`) | `one_not_mem_closure_two`, `two_lt_solvingDegree` (`LowerThree.lean`) | `k ≥ 2`, any `A B t` | checked |
| T6 (`LFD ≥ 4`), T7 (`LFD ≤ 2k+1`) | — | | paper only (§6, §7.1) |
| T9 linear lower bound | — | | open |

Build: `cd lean && lake build` (Lean 4.34.1, Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612` = tag
`v4.34.1`; `lakefile.toml`, `lake-manifest.json`, `lean-toolchain` pinned; "Build completed
successfully (8931 jobs)"). Axioms: `lake env lean ChainSd/Axioms.lean` prints only `propext`,
`Classical.choice`, `Quot.sound` for all 17 listed theorems (recorded in `reports/axioms.txt`). No
`sorry`, no `native_decide`, no custom axiom. Files: `Closure.lean` (abstract), `Popcount.lean`,
`GroupAlgebra.lean` (concrete algebra, depth monotonicity), `Reach.lean` (reachability, T2 concrete),
`Unit.lean` (T3), `LowerThree.lean` (`1 ∉ V_2`).

## 11. Reproduction

```sh
cd research/chain_sd_bounds_20261115/native && cargo build --release
./target/release/chainsd family 65537 15 3                  # unit check of the k=15 instance (O(N) + brute force for k<=9)
./target/release/chainsd lfd 12289 7 1210 0 11 6            # LFD by the closure (D = 2.. until 1 enters)
CHAINSD_CERT=../certs/x.cert ./target/release/chainsd closure 12289 8 1210 0 11 4 verify
cc -O2 -o checker/chaincheck checker/chaincheck.c && ./checker/chaincheck ../certs/x.cert
./target/release/chainsd nowrap 12289 7 1210 0 11 5         # Z-model containment/equality test
./target/release/chainsd closure 12289 7 1210 0 11 4 evaltest   # boundary-evaluation annihilator test
```

Engine: `native/src/main.rs` (Rust, exact `F_p` with `p < 2^23`, lazily reduced 64-bit rows,
weight-descending echelon closure, full `verify_closed` sweep). Checker: `native/checker/chaincheck.c`.
Plots: `reports/plot_lfd.py` (lays out recorded values from `reports/values.json`).

## 12. Sources checked

Repository: `research/notes/index-calculus/RESEARCH_FALL_DEGREE_BOUNDS.md` (definitions of `V_{F,c}`,
`d_F`, `sd`, Caminata–Gorla and HKY citations), `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`,
`research/prime_field_ic_closure_20261112/finish/theorem/PROOF.md` (family (T), Lemma 4, Lemma 7),
`research/leads_20261112/BRIEFING.md`, `research/leads_20261112/lanes/lane_P_prime.md`,
`research/leads_chase_20261112/PROTOCOL.md` (lane-P values and the `UnitInstance` recipe),
`research/leads_followup_20261114/item09_velu_tower/` (engine rules, parity values),
`research/guarded_fall_degree_20261028/` (a different, guarded presentation), AGENTS.md
("Implementation language: no Python", "Mathematical results should include Lean proofs").
Mathlib: `WeierstrassCurve.Affine.equation_negAdd`, `slope_of_X_ne`, `slope_of_Y_ne`,
`ZMod.euler_criterion`, `Nat.testBit_mod_two_pow`, `AddMonoidAlgebra.mapDomainAlgHom`.
Not reachable in this session: the `research-kb` and `ecdb` MCP servers (not configured), the lead
atlas (sibling checkout absent), and `isogeny-labs/ml-cryptanalysis` (`add_repo` denied), where the
N8 proof, `U1-upper-bounds.md` and `experiments/ic-bounds-k14/` live; their values are quoted from
the crypto-repository summaries and labelled unverified.
