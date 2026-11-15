# Upper bounds for the last fall degree `d_F(W, t)`: what is proved, what is open (2026-11-15)

**Class.** A derivation (desk mathematics). No run, no measurement, no compute beyond compiling a
core-Lean file. Notation follows `../README.md` §0 and `research/prime_fall_degree_20261027/DERIVED.md`
(D1–D8). The proofs below are in prose. `lean/UpperBound.lean` kernel-checks only Lemma A, Lemma T,
the single-ring gluing `closure_glue`, the abstract lift-plus-defect step `closure_lift_defect`, `wrap_pop`
and `orbit_all_ones` (§8). U1's subring gluing and the cross-ring degree-shift lift in U2 and
Proposition S are prose only. No pcc claim is registered for anything here, so every statement is
`[pcc:unverified]` as a ledger matter, and measured values quoted from other notes stay
`[pcc:unverified]`.

**Question (task, 2026-11-15).** Prove the upper half of Conjecture G, [pcc:unverified]
`d_F(W, t) ≤ ffd(W) + max(0, ⌊(t − s_W)/4⌋)`, for `W = 1, 2` (`m = 2`). Failing that, prove a weaker
explicit upper bound (linear in `t` with an explicit slope, or `d_F(W, t+4) ≤ d_F(W, t) + 1`), or show
exactly where a four-level lifting fails.

## Answer

**The upper half of Conjecture G is not proved.** Proved here (every statement for `m = 2`): [pcc:unverified]

1. **Theorem U1 (slope 1).** `d_F(f, t) ≤ t + 2` for **every** unit `f` whose `z_2`-degree is at most 2 [pcc:unverified]
   and whose total popcount degree is at most 2. This covers the whole width-1 and width-2 boxes and the
   Kummer tower generator `S₃`, with no genericity assumption, for `t ≥ 1` (`W = 1`) or `t ≥ 2`
   (`W = 2`, `S₃`). It halves the slope of the only previous upper bound, D4's `2t + 2`. The proof glues
   the one-variable D8 refutations on the fibres `z_1 = x`, `x ∈ μ_K`, with the fibre idempotents.
   Those idempotents have degree `t`.
2. **Exact values at the first cells.** With Theorem 4 of `../README.md`, U1 gives `d_F(f, 2) = 4` for a
   generic width-2 generator and for `S₃` under D3's hypotheses. It also gives `d_F(f, 1) = 3` for every
   width-1 unit with `c_{11} ≠ 0`. These are the values of Conjecture G at `(W, t) = (2, 2)` and `(1, 1)`.
   This is the only part of Conjecture G's upper half proved here.
3. **Theorem U2 (one-level lift, cost 2).** `d_F(f, t+1) ≤ 2 + max_λ d_F(f_λ, t)`, where `f_λ(z) =
   f(λ_1 z_1, λ_2 z_2)` and `λ_i^K = ±1`. For generic generators of a box this reads
   `d_F^gen(W, t+1) ≤ d_F^gen(W, t) + 2`, so `d_F^gen(W, t+4) ≤ d_F^gen(W, t) + 8`.
4. **Theorem E (where the growth lives: an exact identity).** Fix `f` with `deg f ≥ 1` and exponents
   `< 2^t`, and let `k ≥ 1`. Let `κ_{t,k}(f)` be the least `c` with `z_1^K − 1, z_2^K − 1 ∈ V_c(f)` at level `t + k`.
   Then

       d_F(f, t + k) = max( d_F(f, t), κ_{t,k}(f) ).

   The hypothesis `deg f ≥ 1` is needed: for constant `f` one has `d_F(f, t+k) = 0` but `κ_{t,k}(f) = 1`,
   so the identity would read `0 = 1` (`d_F = 0`, `κ = 1`).
   So, for `deg f ≥ 1`, the four-level lift `d_F(t+4) ≤ d_F(t) + 1` holds **iff** the two degree-1 elements `z_i^K − 1`
   (the lift defects) enter the level-`(t+4)` closure by degree `d_F(t) + 1`. Theorem 1 of
   `../README.md` is the easy half of this identity. Proposition S pins `κ_{t,1}` to within 1, using
   the twisted half-fibres.
5. **Lemma B (the fibre-gluing barrier).** A non-zero element that annihilates a fibre ideal
   `z_1^{2^s} − ω` has degree at least `t − s`. So, for Lemma-G-style gluing (additive bookkeeping
   `δ = deg e` with an annihilator `e` as multiplier), gluing fibres over the top `k` levels of a
   coordinate pays at least `k` degrees. That is slope at least 1 in the levels glued. U1 and U2
   are optimal for that mechanism, and a period-4 bound (slope 1/4) cannot come from it. The shift
   transport `z ↦ z^{16}` (Lemma S4) is the other natural route. Proposition N rules out only the
   polynomial identity `f | f(z^{16})` in `F[z_1, z_2]` (for the `W = 1` Möbius form); it does not decide
   whether `f(z^{16}) ∈ V_{c'}(f)` at level `t + 4`.

**Open.** These remain open: the upper half of Conjecture G for `t ≥ 2` (`W = 1`) and `t ≥ 3`
(`W = 2`); any bound of slope below 1; and the four-level lift, which by Theorem E (for `deg f ≥ 1`) is
the single statement `κ_{t,4}(f) ≤ d_F(f, t) + 1` (§7).

---

## 0. Setting and conventions

- **Rings.** `R_t = F[z_1, z_2]/(z_1^K − 1, z_2^K − 1)`, `K = 2^t`, `char F = p ∤ 2`.
- **Degree and variables.** A normal-form monomial `z^e`, `e = (e_1, e_2) ∈ [0, K)²`, has degree
  `pop(e) = pop(e_1) + pop(e_2)`. The tower variables are `y_{i,j} = z_i^{2^j}`, `j < t`. Products
  never raise degree (D2): `deg(uv) ≤ deg u + deg v`.
- **Closure.** `V_c(G)` is the closure of a generator set `G` (PREREGISTRATION §2). It is the smallest
  subspace of `R_{≤c}` containing every `g ∈ G` with `deg g ≤ c` and containing `y·u` for every
  tower variable `y` and every `u` in it with `deg u < c`. We write `V_c(f) = V_c({f})`.
  `d_F(f) = min{c : 1 ∈ V_c(f)}`, for `f` a unit.
- **Twisted rings.** `R^ω = F[z]/(z_1^K − ω_1, z_2^K − ω_2)` has the same monomials, degree, variables
  and closure. Proposition 1 of `../lower_bound/README.md` gives `Θ: R^ω → R_t`, `z_i ↦ λ_i z_i`
  (`λ_i^K = ω_i`). It is a degree-preserving isomorphism with `Θ(V^ω_c(f)) = V_c(f_λ)`, so
  `1 ∈ V^ω_c(f) ⇔ 1 ∈ V_c(f_λ)`.
- **Asymmetric heights.** `R_{(a,b)}` is the ring with `z_1^{2^a} = 1` and `z_2^{2^b} = 1`. All
  definitions carry over, and `d_F^{(a,b)}` denotes its last fall degree.
- **The lift `ι`.** `ι: R_t → R_{t+k}` sends a normal form to the same polynomial (exponents `< K`).
  It is linear and preserves degree, but it is not multiplicative.

## 1. Basic lemmas (proved)

**Lemma A (monomial multiples).** If `g ∈ V_c(G)` and `z^e` is a monomial with `deg g + pop(e) ≤ c`,
then `z^e g ∈ V_c(G)`.

*Proof.* `z^e = Π_{(i,j) ∈ bits(e)} y_{i,j}`. These are distinct bits, so there is no carry. Multiply
`g` by these `pop(e)` variables one at a time. Before the `r`-th multiplication the element has
degree `≤ deg g + r − 1 < c`. ∎

**Lemma M (monotone in `c`).** If `max_{g ∈ G} deg g ≤ c ≤ c'`, then `V_c(G) ⊆ V_{c'}(G)`. *Proof.*
`V_{c'}(G) ∩ R_{≤c}` contains `G` and is closed for the degree-`c` rule. ∎

**Lemma T (transitivity).** Let `c ≤ c'` and let every `g' ∈ G'` with `deg g' ≤ c` lie in `V_{c'}(G)`.
Then `V_c(G') ⊆ V_{c'}(G)`. *Proof.* `V_{c'}(G) ∩ R_{≤c}` contains those `g'` and is closed: if
`deg u < c ≤ c'` then `yu ∈ V_{c'}(G)` and `deg(yu) ≤ c`. ∎

**Lemma F (saturation).** If `1 ∈ V_d(G)`, then `V_d(G) = R_{≤d}`. *Proof.* By Lemma A with `g = 1`. ∎

**Lemma BC (base change).** Let `F'/F` be a field extension. Then
`V_c^{F'}(G) = V_c^F(G) ⊗_F F'`, and `1 ∈ V_c^{F'}(G) ⇔ 1 ∈ V_c^F(G)`.

*Proof.* Write `V = V_c^F(G)`.
- `V ⊗ F'` contains `G`.
- An element of `(V ⊗ F') ∩ R'_{<c}` lies in `(V ∩ R_{<c}) ⊗ F'`, because intersections of
  `F`-subspaces commute with `⊗ F'`. So it is `Σ λ_i v_i` with `v_i ∈ V ∩ R_{<c}`, and multiplying by
  `y` keeps it in `V ⊗ F'`. Hence `V^{F'} ⊆ V ⊗ F'`.
- `V^{F'} ∩ R` is closed and contains `G`, so `V ⊆ V^{F'}`. Hence `V ⊗ F' ⊆ V^{F'}`.

Finally `(V ⊗ F') ∩ R = V`. ∎

**Lemma G (gluing).** Let `e ∈ R` with `deg e ≤ δ`. Let `G'` be a set, and suppose `e g' ∈ V_{c+δ}(G)`
for every `g' ∈ G'` with `deg g' ≤ c`. Then `e·V_c(G') ⊆ V_{c+δ}(G)`.

*Proof.* `W = {u ∈ R_{≤c} : eu ∈ V_{c+δ}(G)}` is a subspace containing those `g'`. If `u ∈ W` and
`deg u < c`, then `deg(eu) ≤ δ + c − 1 < c + δ`, so `y(eu) = e(yu) ∈ V_{c+δ}(G)`. Also
`deg(yu) ≤ c`. So `W` is closed, and minimality gives `V_c(G') ⊆ W`. ∎

The same proof works when `G'` lives in a subring, or in another ring mapped linearly into `R`, provided
the variable steps match up to elements that `e` kills. That is how Lemma G is used in §§3–5.

## 2. One-variable refutation in degree 2 (Lemma O; D8, restated with full proof)

**Lemma O.** Let `R¹ = F[z]/(z^K − 1)`, with `t ≥ 1` and `y_j = z^{2^j}`. Let
`q = α + β y_0 + γ y_1` be a unit of `R¹`, i.e. it has no root on `μ_K`. (When `t = 1`, read
`y_1 = 1`.) Then `1 ∈ V_2(q)`.

*Proof.*
- **Case `q` constant.** `q ≠ 0`, so `1 ∈ V_c(q)` for every `c ≥ 0`.
- **Case `q` linear.** This is the case `γ = 0, β ≠ 0`, or `t = 1`. Write `q = α' + β y_0` and
  `ρ = −α'/β`. Then `ρ^K ≠ 1`. Put `L_j = y_j − ρ^{2^j}` for `0 ≤ j ≤ t`, with `y_t := 1`.
  `L_0 = q/β ∈ V_2`. Since `y_j² = y_{j+1}`, we get `y_j L_j + ρ^{2^j} L_j = L_{j+1}`, and every `L_j`
  has degree `≤ 1 < 2`. Hence `L_t = 1 − ρ^K ∈ V_2`, a non-zero constant.
- **Case `γ ≠ 0`, `t ≥ 2`.**
  - *Setup.* Let `r_j = a_j + b_j z` be the remainder of `z^{2^j}` modulo `q` (as a polynomial of
    `z`-degree 2), and put `L_j = y_j − a_j − b_j y_0` for `0 ≤ j ≤ t` with `y_t := 1`. Then
    `L_0 = 0` and `L_1 = q/γ ∈ V_2`.
  - *Induction, `1 ≤ j ≤ t − 1`.* Using `y_0 y_0 = y_1` and `y_j y_j = y_{j+1}`, then substituting
    `y_j = L_j + a_j + b_j y_0` and `y_1 = L_1 + a_1 + b_1 y_0`:

        y_j L_j + b_j·y_0 L_j + a_j L_j + b_j² L_1 = y_{j+1} − (a_j² + b_j² a_1) − (2a_j b_j + b_j² b_1) y_0.

    The right side is `L_{j+1}`, because `r_{j+1} ≡ r_j² ≡ a_j² + 2a_jb_j z + b_j²(a_1 + b_1 z)`. The
    left side lies in `V_2`, because `L_j` and `L_1` have degree `≤ 1`.
  - *Conclusion.* `M := L_t = 1 − a_t − b_t y_0 ∈ V_2`, and `M ≡ 1 − z^K (mod q)`.
    - `M ≠ 0`: otherwise every root of `q` is in `μ_K`.
    - If `b_t = 0`, `M` is a non-zero constant.
    - Otherwise `M` has the root `ρ = (1 − a_t)/b_t`, and `q(ρ) ≠ 0`, since `q(ρ) = 0` would force
      `ρ^K = 1`. The elements `q`, `M`, `y_0 M` lie in `V_2` (because `deg M ≤ 1`). They lie in
      `span{1, y_0, y_1}`, with coefficient determinant `m_1² q(ρ) ≠ 0`, where `m_1 = −b_t`. So they
      span `1`.

∎

## 3. Theorem U1: `d_F ≤ t + 2` (proved)

**Theorem U1.** Let `f ∈ R_t` be a unit of degree `≤ D` whose `z_2`-exponents all lie in `{0, 1, 2}`.
Assume `t ≥ 2` if some `z_2`-exponent is 2, and `t ≥ 1` otherwise. Then

    d_F(f, t) ≤ t + max(2, D).

In particular, every unit of the width-1 box (`t ≥ 1`), of the width-2 box (`t ≥ 2`), and the tower
generator `S₃` (`t ≥ 2`) has `d_F ≤ t + 2`.

*Proof.*
1. By Lemma BC, work over `F' = F(μ_K)`. Put `c = max(2, D)`.
2. For `x ∈ μ_K`, let `e_x = K^{-1} Σ_{i<K} x^{−i} z_1^i`. Then `(z_1 − x)e_x = 0` (the sum
   telescopes, using `z_1^K = 1 = x^K`), and `Σ_x e_x = 1` (orthogonality of characters).
   `deg e_x ≤ t`, because `pop(i) ≤ t` for `i < K`.
3. Let `f_x(z_2) = f(x, z_2) ∈ R¹ = F'[z_2]/(z_2^K − 1) ⊂ R_t`, the subring of `z_1`-free elements.
   It is a unit of `R¹`, since `f` has no zero on `μ_K²`. It has the form `α + β y_{2,0} + γ y_{2,1}`.
   `f − f_x ∈ (z_1 − x)R_t`, so `e_x f_x = e_x f`.
4. `e_x f ∈ V_{c+t}(f)`. Indeed, `e_x f = Σ_i K^{-1} x^{−i} z_1^i f`, and each `z_1^i f` lies in
   `V_{D + pop(i)}(f) ⊆ V_{c+t}(f)` by Lemmas A and M.
5. Apply Lemma G with `e = e_x`, `δ = t` and `G' = {f_x}`, where `V_c(G')` is the closure inside
   `R¹`, using only the variables `y_{2,j}`. The set `W ∩ R¹` in the proof of Lemma G is closed under
   those variables, and `deg(e_x u) ≤ t + deg u` because `z_1^i z_2^b` has degree `pop(i) + pop(b)`.
   This gives `e_x·V¹_c(f_x) ⊆ V_{c+t}(f)`.
6. By Lemma O, `1 ∈ V¹_2(f_x) ⊆ V¹_c(f_x)`, so `e_x ∈ V_{c+t}(f)`. Summing over `x`,
   `1 = Σ_x e_x ∈ V_{c+t}(f)`.

∎

**Corollary U1′ (cells where Conjecture G's upper half is proved).** [pcc:unverified]
- (a) Every unit `f` of the width-1 box with `c_{11} ≠ 0` has `d_F(f, 1) = 3`. The lower bound is
  `d_F ≥ ffd` (if `1 ∈ V_d \ V_{d−1}` then `V_d ∩ R_{≤d−1} ≠ V_{d−1}`), together with the Lemma of
  `../README.md` §4: `ffd = 3`, valid at `t = 1` because `y_{1,0}² = 1` has degree 0 `< 2`.
- (b) A generic width-2 generator has `d_F(f, 2) = 4`. So does `S₃` under D3's hypotheses. The lower
  bound is `ffd = 4` (Theorem 4 / D3).

Both values equal `ffd(W) + max(0, ⌊(t − s_W)/4⌋)`. Part (b) agrees with the measured `t = 2` entries
of `../README.md` §5 `[pcc:unverified]`. Part (a) is supported by the recorded `t = 1` tower cell
`d_F = 3` (`research/pkm_falldegree_unblock_20261107/RESULTS.md` Table 1, `[pcc:unverified]`); the `t = 1`
reduction of `S₃` is a width-1-box unit. Conjecture G was fitted on `t = 2..11`, so `t = 1` is an
extrapolated cell. Both `(1, 1)` and `(2, 2)` are proved cells.

**Comparison with the fitted law** (measured values `[pcc:unverified]`, from `../README.md` §5):

| `t` | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| U1 bound `t + 2` | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 |
| `W = 1` measured | 3 | 3 | 4 | 4 | 4 | 4 | 5 | 5 | 5 | 5 | ≥ 6 |
| `W = 2` measured | 4 | 4 | 4 | 4 | 5 | 5 | 5 | 5 | 6 | 6 | — |

No measured value exceeds the bound. That is a consistency check of the theorem, not evidence for it.

**Why the method stops at slope 1.** The fibre idempotent `e_x` costs `t` degrees. Lemma B (§6) shows
that any element killing `z_1 − x` costs at least that much, so for Lemma-G-style gluing the cost cannot
be reduced. One can also fibre over only the top
`t − a` levels of `z_1`, which costs `t − a` and leaves twisted rings of heights `(a, t)`. That gives
`d_F(f, t) ≤ (t − a) + max_λ d_F^{(a,t)}(f_λ)`. Any uniform constant bound `d_F^{(a,b)} ≤ C_a` on
strips would therefore give `t + C_a − a`, which is still slope 1.

## 4. Theorem U2: one tower level costs at most 2 (proved)

**Theorem U2.** Let `f` have exponents `< K = 2^t`. Then

    d_F(f, t + 1) ≤ 2 + max { d_F(f_λ, t) : λ_i^K ∈ {±1} }.

*Proof.*
1. At level `t + 1`, `y_{i,t} = z_i^K` and `y_{i,t}² = 1`. For `ω ∈ {±1}²`, put
   `e_ω = ¼(1 + ω_1 y_{1,t})(1 + ω_2 y_{2,t})`. Then `deg e_ω ≤ 2`, `Σ_ω e_ω = 1`, and
   `e_ω(y_{i,t} − ω_i) = 0`.
2. Let `ι: R^ω → R_{t+1}` be the lift of normal forms. For a variable `y_{i,j}` (`j < t`) and a
   monomial `z^e`, a wrap in `R^ω` (`e_i + 2^j ≥ K`) gives `ω_i z^{e'}`, with
   `e'_i = e_i + 2^j − K`. In `R_{t+1}` the same product is `y_{i,t} z^{e'}`. So
   `ι(y u) = y ι(u) + (ω_i − y_{i,t})·(…)`, and therefore `e_ω ι(y u) = y·(e_ω ι(u))`.
3. Let `c ≥ deg f`. The set `T = {v ∈ R^ω_{≤c} : e_ω ι(v) ∈ V_{c+2}(f)}` contains `f`, by Lemma A
   (`e_ω f` is a combination of `m f` with `pop(m) ≤ 2`). It is closed, as in Lemma G. So
   `1 ∈ V^ω_c(f)` implies `e_ω ∈ V_{c+2}(f)`.
4. Take `c = max_ω d_F^ω(f)`, where each value is at least `deg f`. Summing over the four `ω` gives
   `1 ∈ V_{c+2}(f)`. Finally `d_F^ω(f) = d_F(f_λ, t)` by Proposition 1.

∎

**Corollary U2′ (generic lift).** For the generic value of a box (Theorem 3 of `../README.md`),
`d_F^gen(W, t+1) ≤ d_F^gen(W, t) + 2`, and hence `d_F^gen(W, t+4) ≤ d_F^gen(W, t) + 8`.

*Proof.*
- For each of the finitely many `λ`, the map `f ↦ f_λ` is a linear automorphism of the box's
  coefficient space. It maps units at level `t + 1` to units at level `t`, which is the fact used
  (a zero of `f_λ` on `μ_K²` is a zero of `f` on `λμ_K²`, and `λμ_K² ⊆ μ_{2K}²` since `λ_i^K = ±1`).
- So the generic open sets at levels `t + 1` and `t`, pulled back along all `f ↦ f_λ`, meet in a dense
  open set.

This step depends on Theorem 3 of `../README.md` (the generic value of a box; prose, `[pcc:unverified]`).
Theorem U2 and the statement of U2′ also assume that `f` is a unit at level `t + 1` (otherwise the
maximum is infinite and the bound is vacuous).

∎

Conditionally, used with the measured `d_F(1, 11) = 5` `[pcc:unverified]`, U2′ would give
`d_F(1, 12) ≤ 7`. This is conditional on three things: the two `t = 11` draws lie in the generic stratum
(Theorem 3 of `../README.md` gives no probability bound); the measurement is correct; and `d_F` of the
four twists `f_λ` of the drawn `f` at `t = 11` is known, which was not measured. Conjecture G
predicts 6, and the falsification run read `≥ 6` `[pcc:unverified]`. U2′ is weaker than U1 asymptotically.

## 5. Theorem E: the growth is exactly the membership degree of `z_i^K − 1` (proved)

**Theorem E.** Let `f` have `deg f ≥ 1` and all exponents `< K = 2^t`, so that it is the same polynomial
at levels `t` and `t + k`, and let `k ≥ 1`. Let `κ = κ_{t,k}(f)` be the least `c` with
`y_{1,t} − 1, y_{2,t} − 1 ∈ V_c(f)` at level `t + k` (here `y_{i,t} = z_i^K` is a tower variable).
Then

    d_F(f, t + k) = max( d_F(f, t), κ_{t,k}(f) ).

*Proof.*

- **`≥`.** `d_F(f, t+k) ≥ d_F(f, t)` is Theorem 1 of `../README.md`. If `1 ∈ V_d` at level `t + k`,
  then `V_d = R_{≤d}` by Lemma F, and `d ≥ deg f ≥ 1`, so `R_{≤d}` contains the degree-1 elements
  `y_{i,t} − 1`. So `d_F(f, t+k) ≥ κ`.
- **`≤`.**
  1. Let `c = d_F(f, t)`, `c' = max(c, κ)` and `Y = V_{c'}(f)` at level `t + k`. Both `c` and `κ` are
     at least `deg f`, because `V_{c''} = 0` for `c'' < deg f`.
  2. Wrap identity. For `j < t` and a level-`t` monomial `z^e` whose product with `y_{i,j}` wraps (bits
     `j … t − 1` of `e_i` all set; Theorem 2), let `e'` be `e` with `e'_i = e_i + 2^j − K`. Then
     `ι(y_{i,j} z^e) = z^{e'}` and `y_{i,j} ι(z^e) = y_{i,t} z^{e'}`. Hence

         ι(y u) = y ι(u) − (y_{i,t} − 1)·ι(E_{i,j} u),

     where `E_{i,j} u` collects the wrapped monomials `z^{e'}`. They satisfy
     `pop(e') = pop(e) − (t − j)` (Lean: `wrap_pop`), hence `pop(e') ≤ deg u − 1`.
  3. Let `T = {v ∈ R_{t,≤c} : ι(v) ∈ Y}`. It contains `f`, since `ι f = f` and `deg f ≤ c ≤ c'`.
  4. `T` is closed. If `v ∈ T` and `deg v < c`, then `y ι(v) ∈ Y`, since `deg ι(v) = deg v < c'`. The
     defect `(y_{i,t} − 1) z^{e'}` has `pop(e') ≤ c − 2`. It lies in `Y` by Lemma A applied to
     `y_{i,t} − 1 ∈ V_κ ⊆ Y` (Lemma M), because `1 + (c − 2) ≤ c'`.
  5. So `T ⊇ V_c(f)` at level `t`, which contains `1`. Hence `1 = ι(1) ∈ Y`.

∎

*Remark (hypothesis `deg f ≥ 1`).* For constant `f ≠ 0`, `d_F(f, t+k) = 0` but `κ_{t,k}(f) = 1`, so the
identity fails (`d_F = 0`, `κ = 1`).

**Corollary E1 (the four-level lift, restated exactly).** For `deg f ≥ 1`, all exponents `< 2^t`, and
`c = d_F(f, t)`:
`d_F(f, t+4) ≤ c + 1` iff `z_1^K − 1` and `z_2^K − 1` lie in `V_{c+1}(f)` at level `t + 4`. The upper
half of Conjecture G for `W`, for `t ≥ s_W`, therefore **follows from** two statements, by induction
in steps of 4. Theorem E needs all exponents `< 2^t`, so the base range starts at
`t_0 = max(s_W, ⌈log₂(W+1)⌉)`. For `W = 1` (`s_1 = 0`) the base cells are `t = 1..4` (not `0..3`, since
exponent 1 is not `< 2^0`); for `W = 2` they are `t = 2..5`.
- (i) the upper bound at the base cells `t ∈ [t_0, t_0 + 3]`;
- (ii) for generic `f` and all `t ≥ t_0`, `κ_{t,4}(f) ≤ d_F(f, t) + 1`.

Conversely, Conjecture G with both halves implies (ii), because then
`κ_{t,4} ≤ d_F(t+4) = d_F(t) + 1`. The upper half alone does not imply (ii). At present (i) is proved
only at `(1, 1)` and `(2, 2)` (U1′(a) and (b)). The other base cells are measured `[pcc:unverified]`.

**Proposition S (one level, sandwich).** Let `R^{(i,−)}` be the ring of heights `t` in coordinate `i`
and `t + 1` in the other, twisted by `z_i^K = −1`. Write `d^{(i,−)}` for its last fall degree of `f`.
Then

    max_i d^{(i,−)}(f) ≤ κ_{t,1}(f) ≤ 1 + max_i d^{(i,−)}(f).

By Proposition 1 (applied in one coordinate), `d^{(i,−)}(f) = d_F^{(t, t+1)}(f(λ z_i, ·))` with
`λ^K = −1`.

*Proof.*
- **Lower bound.**
  - The quotient `π_i: R_{t+1} → R^{(i,−)}` (modulo `y_{i,t} + 1`) is a ring map. It sends each
    variable to a variable, or `y_{i,t}` to the scalar `−1`.
  - It does not raise degree, since `z_i^{e_i}` with `e_i ≥ K` maps to `−z_i^{e_i − K}`.
  - The proof of Theorem 1 (`../README.md` §1) therefore applies verbatim:
    `π_i(V_c(f)) ⊆ V_c^{(i,−)}(f)`.
  - Since `π_i(y_{i,t} − 1) = −2`, `y_{i,t} − 1 ∈ V_κ` gives `1 ∈ V_κ^{(i,−)}`.
- **Upper bound.** Take `e = (1 − y_{i,t})/2`, which has degree 1 and kills `y_{i,t} + 1`. Lift
  `R^{(i,−)} → R_{t+1}` by `ι` as in U2; the only wraps are in coordinate `i`. The set-`T` argument
  gives `e ∈ V_{c+1}(f)` for `c = d^{(i,−)}`, so `y_{i,t} − 1 = −2e ∈ V_{c+1}(f)`.

∎

So one tower level costs at most the twisted half-fibre value plus 1. The open question is entirely
about the twisted asymmetric rings, or equivalently about `κ`.

## 6. Why the natural four-level liftings fail (proved obstructions)

**Lemma B (fibre-gluing barrier).** Let `0 ≠ e ∈ R_t` satisfy `e·(y_{1,s} − ω) = 0` for some `s < t`
and `ω ∈ F^×`. Then `deg e ≥ t − s`. If moreover `e·(y_{2,s'} − ω') = 0`, then
`deg e ≥ (t − s) + (t − s')`.

*Proof.*
1. Multiplication by `y_{1,s}` permutes monomials: `(a, b) ↦ (a + 2^s mod K, b)`.
2. Write `e = Σ c_{a,b} z^{(a,b)}`. Comparing coefficients in `e y_{1,s} = ω e` gives
   `c_{a,b} = ω c_{a + 2^s, b}`.
3. So if `c_{a,b} ≠ 0`, the coefficient is non-zero on the whole orbit
   `{(a mod 2^s) + h·2^s : h < 2^{t−s}}`.
4. That orbit contains the exponent whose bits `s … t − 1` are all set, of popcount `≥ t − s` (Lean:
   `orbit_all_ones`).
5. For the second statement, move along the `z_2`-orbit from that monomial in the same way.

∎

*Consequence (for Lemma-G-style gluing only).* Lemma G with a fibre annihilator converts a fibre [pcc:unverified]
certificate of degree `c` into a global one of degree at least `c + (number of levels fibred)`. Every
annihilator of the fibre ideal `(y_{i,s} − ω)` is a multiple of the fibre idempotent, and Lemma B
bounds its degree from below. Hence U1 (`k = t` levels, cost `t`) and U2 (one level per coordinate,
cost 1 each) are optimal for Lemma-G gluing with annihilator multipliers. Lemma B does not constrain
other gluing arguments. A period-4 proof of that kind would have to absorb the defects `z_i^K − 1`
by another route, for instance through falls of the closure itself (Theorem E); that is not proved.

**Lemma S4 (shift transport).** `σ: R_t → R_{t+4}`, `z ↦ z^{16}`, is an injective ring map. It sends
`y_{i,j} ↦ y_{i,j+4}` and preserves degree, so `σ(V_c(g)) ⊆ V_c(σ g)`, by the proof of Theorem 1 with
variables mapped to variables. With Lemma T:

    if  g(z^{16}) ∈ V_{c'}(f) at level t+4,  deg g ≤ c ≤ c',  and  1 ∈ V_c(g) at level t,  then  d_F(f, t+4) ≤ c'.

**Proposition N (the shifted generator is not a multiple).** Let `f = c_{11}(z_1z_2 + αz_1 + βz_2 + γ)`
with `c_{11} ≠ 0`, `γ ≠ αβ` and `β ≠ 0`. Then `f ∤ f(z_1^{16}, z_2^{16})` in `F[z_1, z_2]`.

*Proof.*
1. On the curve, `z_2 = φ(z_1) = −(αz_1 + γ)/(z_1 + β)`. Divisibility would give
   `φ(z_1)^{16} = φ(z_1^{16})` as rational functions.
2. The left side has exactly one finite pole, at `z_1 = −β`, of order 16. The numerator there is
   `γ − αβ ≠ 0`.
3. The right side has a pole at each of the 16 distinct roots of `z_1^{16} = −β`. They are distinct
   because `p ∤ 16`, and the numerator there is `γ − αβ ≠ 0`.
4. This is a contradiction.

∎

So Lemma S4 with `g = f` needs the closure to manufacture `f(z^{16})` at level `t + 4` through wraps.
By Theorem 2, wraps happen only in the top `c' − 1` levels. Proposition N rules out only the polynomial
identity `f | f(z^{16})` in `F[z_1, z_2]`; whether `f(z^{16}) ∈ V_{c'}(f)` at level `t + 4` for
`c' = d_F(t) + 1` is not decided here. The exploratory data say the same thing (`../lower_bound/README.md` §6.3, §8;
Rust re-run §10; `[pcc:unverified]`). There, the first degree-2 multiple of `f` involving level 4 has
the shape `κ w_1w_2 + w_1 ℓ_1 + w_2 ℓ_2 + (levels 0..3)`, with `w_i = z_i^{16}` and `ℓ_i` non-constant
degree-≤1 forms. It is a Möbius relation one block up, but with coefficients in the low algebra rather
than in `F`. A lifting lemma would therefore have to run S4 over the low 16×16 block as a coefficient
module, not over `F`. That relative transport is not formulated as a theorem here.

## 7. What remains conjectural (stated exactly)

- **Upper half of Conjecture G.** Open for `W = 1`, `t ≥ 2` and `W = 2`, `t ≥ 3`. By Theorem E (for
  `deg f ≥ 1`) it follows from Corollary E1 (i) and (ii), and the full Conjecture G implies (ii). The weakest open statement that would give slope 1/4 is
  `κ_{t,4}(f) ≤ d_F(f, t) + 1` for generic `f`.
- **Any slope `< 1`.** Not proved. Lemma B shows that Lemma-G-style gluing cannot give it; another route
  (for instance Theorem E's defect absorption) is not established.
- **Asymmetric strips.** Open: a uniform bound `d_F^{(a,b)}(f) ≤ C_a` for `b ≥ a`. If it holds,
  §3 gives `d_F ≤ t + C_a − a`, which changes only the constant.
- **Other widths.** U1 extends to any box once each one-variable fibre `f(x, z_2)` has a `t`-uniform
  refutation degree `b_W`: then `d_F ≤ t + max(D_W, b_W)`. For `W ≤ 2`, Lemma O gives `b_W = 2`. For
  `W = 3, 4` the fibre has `z_2`-degree 3 or 4, and Lemma O's degree-2 chain does not apply as written.
  A constant `b_W` is plausible (the quotient `F[z]/(q)` has dimension `≤ W`) but not proved
  `[pcc:unverified]`.

**Counterexample risks.**
1. U1, U2, E (for `deg f ≥ 1`), S, B and N are unconditional theorems (prose). U2′ needs genericity and
   Theorem 3 of `../README.md`; E and U2 also assume `f` is a unit at the higher level. A counterexample to
   them would mean an error in a proof. Every measured value is consistent with U1.
2. Conjecture G's upper half could fail if the periodic step pattern breaks at larger `t`, i.e. if
   `κ_{t,4} − d_F(t) ≥ 2` somewhere. The data have two observed steps for `W = 1, 2` and one for
   `W = 3` (plus the `W = 1`, `t = 12` lower read).
3. It could also fail for special generators: Theorem 3 of `../README.md` makes the law a statement
   about a dense open set only. `S₃` is one point, not proved generic beyond its support.
4. U1′(b) uses Theorem 4 at `W = 2`, which rests on D3's prose proof `[pcc:unverified]`.

## 8. Formalization

`lean/UpperBound.lean` is core Lean 4.35.0, no Mathlib, standalone (it copies `FilteredData`, `Closure`
and the `pop` lemmas from `../lean/GrowthLaw.lean`). Gate: `tools/lean/proof-gate.sh --out
…/upper_bound/lean/receipts/20261115 …/UpperBound.lean` returned PASS: 22 theorems audited (12
hand-written, `grep -c '^theorem '`), 0 trusted, axioms ⊆ {`propext`, `Quot.sound`}. The receipt and
source SHA-256 are in `lean/receipts/20261115/` `[pcc:unverified]`.

| result | Lean | status |
|:--|:--|:--|
| Lemma A | `closure_mulv_list` (abstract: each variable raises degree ≤ 1) | checked |
| Lemma T | `closure_trans` | checked |
| Lemma G | `closure_glue` (abstract: `M` linear, commutes with variables, raises degree ≤ δ) | checked |
| Theorem E, `≤` core | `closure_lift_defect` (lift = variable step + defect in target closure); `wrap_pop` (`pop(e + 2^j − 2^t) + (t − j) = pop e`) | abstract step and defect arithmetic checked |
| Lemma B, combinatorial core | `orbit_all_ones` | checked |
| Lemmas M, F, BC; Lemma O; U1, U2, E (concrete), S, N | — | prose. They need the concrete group algebra, idempotents, roots of unity and base change (Mathlib). Lemma O and the idempotent identities are the most mechanical next targets. |

## 9. Sources checked

`../README.md` (§0–§7: Theorems 1–4, Conjecture G, open items, proof map); `../lower_bound/README.md`
(Proposition 1 used for twisted fibres; §§6–8 and §10 for the level-4 relation and the threshold data,
quoted as `[pcc:unverified]`); `research/prime_fall_degree_20261027/DERIVED.md` (D1–D4, D8: the
previous upper bound `2t + 2` and the one-variable chain restated as Lemma O);
`research/prime_fall_degree_20261027/PREREGISTRATION.md` §2 (closure definition); `../lean/GrowthLaw.lean`
(abstract closure layer reused). **KB unavailable**: the `research-kb` MCP server was not loaded for this
subagent, so no `kb_status`/`search_knowledge` receipt exists, and the novelty of U1, Theorem E and
Lemma B is unchecked against the corpus. Fibre gluing via idempotents and the defect-absorption form of
a lifting lemma are standard proof patterns, presented as tools, not as new.

## Red-team review (2026-11-15)

Independent adversarial review (Red Team, core tier). It challenges; it does not change a status or
declare a result supported or refuted. The Coordinator and Validator decide and record.

**What was reviewed, and its standing.** The review was dispatched on working-tree files: `upper_bound/`
was untracked at HEAD `c696cd874a`, and `README.md` hashed (SHA-256) to
`bfccbf5b03454ac50ce7d33dea363d6272698cbd484651f20f088684470afd32`. During the review another actor (not
this reviewer) did two things:
- appended ` [pcc:unverified]` to the ends of lines 10, 17, 19 and 176;
- committed the note as `741217c652` on branch `claude/prime-field-index-calculus-atxvot`
  ("red-team review pending").

At that commit `README.md` is `ca76d5f5e0b6ca641eb0dadd18fa0c29794834856e6826e530ab4e2b3a531120`.
Removing the four tags gives back `bfccbf5b…` exactly. `lean/UpperBound.lean` is
`a759fa648a4ff6bce2e3893f0ed7b9780d2ac2dc367f6b608ff84a8a38e24819`, identical to the reviewed file and to
the receipt's `SOURCE.sha256`. **So this review applies to the committed text of `741217c652`.** This
section is an uncommitted working-tree addition on top of that commit. No pcc claim references
`upper_bound/`; every result here remains `[pcc:unverified]`.

**Checks run by the reviewer.**
- `tools/lean/proof-gate.sh` on `lean/UpperBound.lean` (core Lean 4.35.0 from `~/.elan`, receipt
  written to a scratch directory, not here). Result: PASS, 22 theorems, 0 trusted, rc 0. `#print axioms`:
  `orbit_all_ones`, `wrap_pop`, `closure_mulv_list`, `closure_glue`, `closure_lift_defect` use
  `[propext, Quot.sound]`; `closure_trans` uses none. This reproduces the producer's receipt.
- Every recorded `m = 2` cell in `../../devcheck/*.jsonl` and `../falsify/*.jsonl` in scope for U1 (tower
  arm and null arm with `W ∈ {1, 2}`; multiples arm and planted targets excluded). That is 160 completed
  rows (tower 104, null `W = 2` 30, null `W = 1` 26). None exceeds `t + 2`. 16 rows are tight
  (`d_F = 4` at `t = 2`). The 6 one-sided rows (`≥ 6` at `t ≥ 10`, `> 5` at `t = 12`) sit below the bound.
  The external tower ladder `research/pkm_falldegree_unblock_20261107/RESULTS.md` Table 1 also obeys
  `t + 2`, and is tight at `t = 1` (3) and `t = 2` (4). Every measured value is `[pcc:unverified]`.
- A reviewer scratch closure in Python (not admitted, not committed). It implements §0 directly over
  `F_12289`, with 2 random units per cell. It reproduced the recorded ladders: `W = 1`: 3, 3, 4, 4 and
  `W = 2`: 4, 4, 4, 4 at `t = 2..5`, plus `W = 1`, `t = 1`: 3. Theorem E held on all 20 tested
  `(W, t, k)` instances (`t + k ≤ 5`), and so did Proposition S and Theorem U2 on all 12 `k = 1`
  instances. Both ends of S's sandwich occur: `κ = 1 + max d^{(i,−)}` at `W = 1`, `t = 3`, and
  `κ = max d^{(i,−)}` elsewhere. The constant generator gives `d_F = 0` but `κ_{2,1} = 1` (finding 2).

### Per-claim verdicts

| claim | verdict | reason |
|:--|:--|:--|
| Lemmas A, M, T, F, BC, G | established | Each checked. G's closure step needs `deg(eu) ≤ δ + c − 1 < c + δ`, which follows from subadditivity of popcount degree (D2). BC: intersection with `R_{<c}` commutes with `⊗ F'`. The abstract cores of A, T and G are kernel-checked. |
| Lemma O | established | Checked the recurrence `a_{j+1} = a_j² + b_j² a_1`, `b_{j+1} = 2a_j b_j + b_j² b_1`, the case split, and the determinant `m_1² q(ρ)`. Needs no roots of unity in `F`. |
| Theorem U1 | **established** (prose) | Needs only `p` odd. `p ≡ 1 (mod 2^t)` is **not** needed: `e_x ∈ F(μ_K)[z_1]`, not `F_p[z]` in general, and Lemma BC descends `1 ∈ V_{c+t}` to `F`. (For the pilot prime `2013265921 = 15·2^27 + 1`, `e_x` is `F_p`-rational for `t ≤ 27` anyway.) `deg e_x = t` exactly: the coefficient of `z_1^{K−1}` is `K^{-1}x ≠ 0`. The unit hypothesis is used only to make each fibre `f_x` a unit of `R¹`. No genericity is used. Data consistent (above). |
| U1′(a) | established | Upper bound: U1 at `t = 1` gives 3. Lower bound: the §4 Lemma at `t = 1` (`y_{1,0}² = 1` has degree 0 < 2). Wording issue: finding 8. |
| U1′(b) | established, conditional on Theorem 4 (`W = 2`) / D3 | D3 was re-derived at `t = 2`: `gr R` has `y² = 0` because `y_{i,1}² = 1`. It needs `det M ≠ 0` and `y_{11} f_2 ≠ 0` (explicit conditions, not vague genericity). D3 is prose `[pcc:unverified]`. |
| Theorem U2 | established (prose) | Checked: the wrap identity, `e_ω(y_{i,t} − ω_i) = 0`, the set-`T` closure, and that twisted units follow from a level-`(t+1)` unit by CRT. Implicit hypothesis: `f` is a unit at level `t + 1` (otherwise the max is infinite and the bound is vacuous). |
| U2′ | established up to wording; the numeric use is **unsupported** | Findings 5 and 6. |
| Theorem E | **false as stated** (constant `f`); established for `deg f ≥ 1` | Finding 2. The `≤` proof checked; `wrap_pop` matches the defect bound. |
| Corollary E1 | the equivalence is established; the induction scheme has a **gap** | Finding 3. |
| Proposition S | established | Both bounds checked. The lower bound needs `−2` invertible (`p` odd). |
| Lemma B | established as stated; its consequences are a **gap** | Finding 4. |
| Lemma S4 | established | |
| Proposition N | established as stated (`W = 1` Möbius form, `β ≠ 0`, `γ ≠ αβ`, `p` odd); its use is overstated | Finding 4. |
| Lean file | gate PASS reproduced; coverage is narrower than one prose sentence says | Finding 7. |

**Status against the task's question.** The upper half of Conjecture G is not proved, and the note says [pcc:unverified]
so. The task's fallback "weaker explicit bound, linear in `t`" is delivered (U1, slope 1). The
four-level lift is reformulated exactly (Theorem E / E1, subject to findings 2 and 3). "Where the
natural liftings fail" is shown only relative to the method used (finding 4).

### Findings, by severity

1. **No pcc claim (process).** The note is now committed (`741217c652`), but nothing is registered in [pcc:unverified]
   the ledger. The gate receipt was written before the commit ("uncommitted working tree"). Its source
   hash does match the committed Lean file, and the reviewer re-ran the gate. *Cheapest fix:* register a
   pcc claim for U1 (and for E with the hypothesis `deg f ≥ 1`) citing `741217c652`.
2. **Theorem E is wrong as stated, for a degenerate case** (§5, "`≥`" bullet). "`V_d = R_{≤d}`, which
   contains the degree-1 elements `y_{i,t} − 1`" needs `d ≥ 1`. For constant `f` (exponents `< 2^t`, as
   the statement allows): `d_F(f, t+k) = 0` but `κ_{t,k}(f) = 1`, so the identity reads `0 = 1`. The
   scratch check confirms it. *Fix:* add the hypothesis `deg f ≥ 1`, which every unit of the boxes with a
   non-constant term satisfies. The proof is otherwise sound.
3. **Off-by-one in Corollary E1's base cells for `W = 1`** (gap). Theorem E needs all exponents
   `< 2^t`. For `W = 1` with `s_1 = 0`, the base range `[s_W, s_W + 3] = [0, 3]` would need the step
   `0 → 4`, which Theorem E does not provide, because exponent 1 is not `< 2^0`. The base cells for
   `W = 1` must be `t ∈ {1, 2, 3, 4}`. (`W = 2`, `[2, 5]`, is fine.) E1 also says "(i) is proved only at
   `(2, 2)`", while U1′(a) proves `(1, 1)`. *Fix:* use the range
   `[max(s_W, ⌈log₂(W+1)⌉), that + 3]` and list `(1, 1)` as proved.
4. **The barrier statements claim more than is proved** (unsupported). Answer item 5, the §6
   "Consequence", and §7 "Lemma B shows that it needs falls inside the fibres" go beyond what is proved.
   - Lemma B bounds the degree of one annihilator from below. It therefore limits only the Lemma-G
     mechanism, with its additive bookkeeping `δ = deg e`. It is not a statement about every gluing
     proof, so "a period-4 bound cannot come from it" holds only for that mechanism.
   - Proposition N rules out only the polynomial identity `f | f(z^16)` in `F[z_1, z_2]`, for the
     `W = 1` form. At every finite level `f` is a unit, so it divides `f(z^16)` there. N says nothing
     about whether `f(z^16) ∈ V_{c'}(f)` at level `t + 4` for `c' = d_F(t) + 1`, nothing about `W = 2` or
     `S₃` ("for generic `f`" in item 5 drops the `W = 1` restriction), and nothing about `g ≠ f`. So
     "S4 fails because…" is not established.

   *Cheapest test:* compute the least `c'` with `f(z^16) ∈ V_{c'}(f)` at level `t + 4` for `W = 1`,
   `t = 2..4` (pilot engine, `t + 4 ≤ 8`), and compare it with `d_F(t) + 1`. Also restate the barrier as
   "for Lemma-G gluing with annihilator multipliers".
5. **The U2′ number is conditional** (unsupported as stated, §4). "Used with the measured
   `d_F(1, 11) = 5`, U2′ gives `d_F^gen(1, 12) ≤ 7`" assumes three things:
   - that the two `F_p` draws at `t = 11` lie in the generic stratum (Theorem 3 of `../README.md` gives
     no probability bound);
   - that the measurement is correct (it is `[pcc:unverified]`);
   - for the drawn `f` itself, that `d_F` of its four twists `f_λ` at `t = 11` is known (U2 needs it,
     and it was not measured).

   *Fix:* state it as a conditional.
6. **U2′ wording and an unlisted dependency** (gap). The proof says `f ↦ f_λ` "preserves units". As
   written that is wrong: it maps units at level `t + 1` to units at level `t`, which is the fact
   actually used, and the parenthetical states it correctly. Separately, "Counterexample risks" 1 says
   "U2′ needs only genericity", but U2′ also rests on Theorem 3 of `../README.md` (prose,
   `[pcc:unverified]`). Theorem E and U2 also leave implicit the hypothesis that `f` is a unit at the
   higher level.
7. **The Lean file covers less than one sentence claims** (gap). The §8 table is accurate: U1, U2 and S
   are listed as prose. But the header sentence "Their abstract closure steps … are kernel-checked"
   overstates:
   - `closure_glue` uses one `FilteredData` for both closures (same ring, same variable set), while U1
     step 5 uses a subring closure with fewer variables.
   - U2 and S compose a lift between different rings with a multiplier that raises degree by `δ = 2` or
     1. `closure_glue` cannot express that (it needs the same `D`), and neither can
     `closure_lift_defect` (its `hdeg` forbids raising degree).
   - All statements are over an axiom-free `FilteredData`. `Closure = V_c` is not formalized (an open
     item inherited from `../README.md`).

   *Fix:* add a lemma `closure_lift_glue` (lift, degree shift `δ`, defect killed by the multiplier),
   plus monotonicity in the variable set.
8. **Minor wording in U1′.** "So does part (a), where no `t = 1` entry was measured": a `t = 1` tower
   cell exists (`research/pkm_falldegree_unblock_20261107/RESULTS.md` Table 1, `d_F = 3 / 3`,
   `[pcc:unverified]`). The `t = 1` reduction of `S₃` is a width-1-box unit, so that cell corroborates
   (a) and should be cited. Conjecture G was fitted on `t = 2..11`, so `t = 1` is an extrapolated cell.
9. **Sourcing (process, minor).** The knowledge base was not consulted (the note says so). "The only
   previous upper bound, D4's `2t + 2`" is scoped to this repository; DERIVED.md D7 also records
   `d_F ≤ D` (solving degree), which is not explicit in `t`. No novelty is claimed.

Corrections applied 2026-11-15: findings 2, 3, 5, 6, 7, 8 (see the body); 1, 4, 9 remain open.
