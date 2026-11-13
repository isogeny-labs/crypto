# The growth law of the last fall degree in the support width: what is proved, what is fitted (2026-11-12)

**Class.** A derivation. No new run; the data are the rows already committed in
`research/prime_ic_leads_20261110/devcheck/` (every measured value there is `[pcc:unverified]`).
It builds on `research/prime_fall_degree_20261027/DERIVED.md` (D1–D8), whose notation it uses, and
extends it; nothing there is restated as new.

**Question (user, 2026-11-12).** Derive the law for how the HKY last fall degree `d_F` of a PKM
Kummer system grows with the support width `W` of the generator and with the tower height `t`.

**Answer.** Four results are proved here; they explain the structure of the width ladder and reduce
the growth law to one open statement. (1) `d_F` is non-decreasing in `t` for a fixed system
(Theorem 1), which is the monotonicity seen on every ladder. (2) A degree-`c` closure uses the top
relation `y_{t−1}² = 1` only through the top `c − 1` tower variables (Theorem 2), which locates
where any growth-proving pseudo-solution must live. (3) `d_F` has a generic value determined by the
support alone, and the "null equals tower" observation is the statement that `S₃` lies in the
generic stratum of the width-2 box, whose support it fills exactly (Theorem 3). (4) The first fall
degree is `3, 4, 5, 5` for `W = 1, 2, 3, 4` (Theorem 4), equal to the measured values, and it
explains why `W = 3` and `W = 4` read alike. The growth itself is **not** derived: on all completed
cells the data fit `d_F = ffd(W) + max(0, ⌊(t − s_W)/4⌋)` with `s_W = 0, 2, 5` for `W = 1, 2, 3`, a
period of four tower levels at every width. That law is stated as a conjecture with three dated
predictions that falsify it. No pcc claim is registered for these proofs `[pcc:unverified]`.

## 0. Setting and notation (from D1–D2)

`m = 2` summands, Kummer tower of height `t`, `K = 2^t`, `x_i = g·z_i`. The tower quotient is
`R = R_t^{⊗2}` with `R_t = F_p[z]/(z^K − 1)`; the tower variable `y_j = z^{2^j}`; the normal-form
monomial `z^e` (`0 ≤ e < K`) has degree `pop(e)` (binary digit sum); multiplication adds exponents
mod `K`, and carries are the degree falls (D2). `gr R = F_p[y]/(y²)` (D3). For a generator `f ∈ R`
the closure `V_c(f)` is the smallest subspace of `R_{≤c}` containing `f` when `deg f ≤ c` and, for
every `g` in it with `deg g < c` and every tower variable `y`, the normal form of `y·g`
(`PREREGISTRATION.md` §2). `d_F(f)` is the least `c` with `1 ∈ V_c(f)` (defined when `f` has no zero
on the grid, i.e. is a unit, D1); `ffd(f)` is the least `c` with `V_c ∩ R_{≤c−1} ≠ V_{c−1}`.

The width-`W` box is the set of generators `f = Σ_{a,b ≤ W} c_{ab} z_1^a z_2^b`; the pilot's
`--null-width W` draws its coefficients uniformly. Its degree is `D_W = 2·max_{k ≤ W} pop(k)`:
`D_1 = D_2 = 2`, `D_3 = D_4 = 4`.

## 1. Theorem 1 (monotone in the tower height)

**Statement.** Fix `p`, the curve, the target, the coset representative `g` and hence the polynomial
`f(z_1, z_2)` of `z`-degree below `K`. Let `f^{(t)}` be its normal form at height `t`. If `f^{(t+1)}`
is inconsistent, so is `f^{(t)}`, and `d_F(f^{(t)}) ≤ d_F(f^{(t+1)})`. The same holds for any `m`, the
chain presentation's free unknowns being left unchanged.

**Proof.** Inconsistency descends because `μ_K ⊂ μ_{2K}`. Let `φ : R_{t+1} → R_t` be reduction
modulo `z_i^K − 1` (a ring homomorphism, since `z^K − 1` divides `z^{2K} − 1`). On monomials,
`φ(z^e) = z^{e mod K}`, and `pop(e mod K) ≤ pop(e)`, so `deg φ(u) ≤ deg u` for every `u`. On
variables, `φ(y_j) = y_j` for `j < t` and `φ(y_t) = 1`. And `φ(f^{(t+1)}) = f^{(t)}`. Let
`S = {u ∈ V_c(f^{(t+1)}) : φ(u) ∈ V_c(f^{(t)})}`. `S` is a subspace of `R_{≤c}`; it contains
`f^{(t+1)}` when that has degree at most `c` (its image has degree at most `c`); and for `u ∈ S` with
`deg u < c`, `φ(y·u) = φ(y)·φ(u)` is either `y·φ(u)` with `deg φ(u) < c`, hence in `V_c(f^{(t)})`, or
`φ(u)` itself. So `S` satisfies the defining closure property, and minimality gives
`S = V_c(f^{(t+1)})`. If `1 ∈ V_c(f^{(t+1)})` then `1 = φ(1) ∈ V_c(f^{(t)})`. ∎

**Generic form.** With Theorem 3, the generic value `d_F^gen(supp, t)` is non-decreasing in `t`:
pick `f` in the intersection of the dense open generic sets at heights `t` and `t + 1`.

**Scope.** The ladders in `devcheck/` draw a fresh `g` and fresh coefficients at each `t`, so they
compare generic values, not one fixed system; the theorem is what makes "the ladder never decreases"
expected rather than observed.

## 2. Theorem 2 (the top window)

**Statement.** For `0 ≤ e < K` and `0 ≤ j < t`, the product `y_j·z^e = z^{e + 2^j}` uses the top
relation `z^K = 1` if and only if bits `j, …, t − 1` of `e` are all 1; then `pop(e) ≥ t − j`. Hence a
degree-`c` closure applies the top relation only when multiplying by one of the top `c − 1` variables
`y_{t−c+1}, …, y_{t−1}`.

**Proof.** `e + 2^j ≥ 2^t` exactly when the carry started at bit `j` runs out of bit `t − 1`, i.e.
when bits `j … t − 1` of `e` are all set. The rule multiplies only elements of degree `< c`, whose
monomials have `pop(e) ≤ c − 1`, so `t − j ≤ c − 1`. ∎

**Consequence.** Replace `R_t` by the twisted algebra `F_p(ζ)[z]/(z^K − ζ)`: same monomial basis,
same grading, same rewriting except `y_{t−1}² = ζ`. A degree-`c` closure computation that never
touches the top window is identical in both. Over the twisted algebra, any point of the curve
`f = 0` with `z_i^K = ζ_i` is a true solution, so `1` never enters. A lower bound `d_F > c` therefore
needs a functional that kills `V_c(f)`, not `1`, and fails to be multiplicative only through the
top `c − 1` levels. This is the precise form of the open problem D2 pointed at; it is not solved
here `[pcc:unverified]`.

## 3. Theorem 3 (the support determines a generic value; "null equals tower")

**Statement.** For fixed `(p, t, m)` and a fixed monomial support `Σ`, there is a value
`d_F^gen(Σ)` such that every coefficient vector in a Zariski-dense open subset of `F̄_p^{|Σ|}` gives a
unit generator with `d_F = d_F^gen(Σ)`; likewise for `ffd`. For the Kummer `m = 2` system, the
normal form of `S₃(g z_1, g z_2, x_R)` has support exactly the width-2 box `{0, 1, 2}²` whenever its
nine coefficients are non-zero (they are `g⁴`, `−2x_R g³` (twice), `x_R² g²` (twice),
`−2(x_R² + a)g²`, `−(2a x_R + 4b)g` (twice), `a² − 4b x_R`), which holds outside a proper closed set
of `(a, b, x_R, g)`.

**Proof.** The closure is computed by a finite sequence of rank decisions on matrices whose entries
are polynomials in the coefficients (span, intersect with `R_{<c}`, multiply, repeat to a fixpoint, for
`c` up to `2t + D` by D4). Each branch of the decision tree is a constructible set, the branches
partition the irreducible affine space, so exactly one branch contains a non-empty open set; on it
`d_F` and `ffd` are constant. Units are the open set where `f` is non-zero at every grid point. The
`S₃` coefficients are read off its expansion. ∎

**What it explains, and what it does not.** The null control on `S₃`'s support is a uniform draw from
the width-2 box, which lands in the generic stratum with high probability; the tower is one point,
`S₃`, of a four-parameter family inside that box. The measured equality on all 40 cells
(`../README.md` §6b) is the statement that `S₃` avoids the exceptional closed set at these sizes. The
theorem makes that the expected outcome; it does not prove it, and it gives no usable bound on the
probability (the degree of the defining polynomials is not controlled here) `[pcc:unverified]`.

## 4. Theorem 4 (first fall degree by width)

**Lemma.** Let `f` be a unit of degree `D` whose top form `f_D ∈ gr_D R` has every monomial divisible
by one tower variable `y`. Then `ffd(f) = D + 1`.

**Proof.** For `c < D`, `V_c = 0`; `V_D = ⟨f⟩` and `V_D ∩ R_{≤D−1} = 0`, so no fall at or below `D`.
At `c = D + 1`, `y·f ∈ V_{D+1}`, and its degree-`(D+1)` part is `y·f_D = 0` in `gr R` (`y² = 0`), so
`deg(y·f) ≤ D`. If `y·f ∈ V_D = ⟨f⟩` then `(y − λ)f = 0`, so `y = λ` because `f` is a unit; impossible.
So `V_{D+1} ∩ R_{≤D} ≠ V_D`. ∎

**Theorem 4.** For a generic generator in the width-`W` box at `m = 2` (and `t ≥ 3` for `W = 4`):
`ffd = 3` (`W = 1`), `4` (`W = 2`), `5` (`W = 3`), `5` (`W = 4`).

**Proof.**
- `W = 1`: `D = 2`, top form `c_{11} y_{10} y_{20}`; the lemma with `y = y_{10}` gives 3.
- `W = 3` and `W = 4`: `D = 4`. The only exponent in `[0, W]` with popcount 2 is `3 = 11₂`, so the top
  form is the single monomial `c_{33} y_{10} y_{11} y_{20} y_{21}`; the lemma gives 5. (At `t = 2`,
  `z^4 = 1` and the `W = 4` box collapses.)
- `W = 2`: `D = 2`, top form `Σ_{a,b ∈ {1,2}} c_{ab} y_{1,a−1} y_{2,b−1}`, a bilinear form with a generic
  invertible `2 × 2` matrix; D3's proof applies word for word (it uses only `det M ≠ 0` and
  `y_{11}·f_2 ≠ 0`) and gives 4. ∎

**Measured check** (`devcheck/*.jsonl`, every row): `first_fall_degree` is 3 on every `W = 1` row, 4 on
every `W = 2` and tower row, 5 on every completed `W = 3` and `W = 4` row at `m = 2`. Theorem 4
explains why `W = 3` and `W = 4` coincide on every cell measured: both have the same single-monomial
top form, and `W = 4` adds only `z^4 = y_2`, a degree-1 monomial below the top.

## 5. The growth law: conjecture, fitted to every completed cell

**Data** (`devcheck/README.md`, width-law table; `m = 2`, both draws agree unless censored):

| `W` | `ffd` (Thm 4) | `d_F` at `t = 2 … 11` | fit |
|--:|--:|:--|:--|
| 1 | 3 | 3, 3, 4, 4, 4, 4, 5, 5, 5, 5 | `3 + ⌊t/4⌋` |
| 2 (= tower) | 4 | 4, 4, 4, 4, 5, 5, 5, 5, 6, 6 | `4 + ⌊(t − 2)/4⌋` (C-step, Amendment 6) |
| 3 | 5 | 5, 5, 5, 5, 5, 5, 5, 6, ≥ 6 | `5 + max(0, ⌊(t − 5)/4⌋)` |
| 4 | 5 | 5, 5, 5, 5, 5, 5, 5, 6 / ≥ 6, ≥ 6 | same as `W = 3` on every cell |

**Conjecture G (growth law, `m = 2`).** For a generic generator in the width-`W` box,

    d_F(W, t) = ffd(W) + max(0, ⌊(t − s_W)/4⌋),   s_1 = 0, s_2 = 2, s_3 = s_4 = 5,

i.e. the refutation degree is the first fall degree plus one for every completed block of four tower
levels above a width-dependent offset. In unknown bits (`N = 2t`) the slope is `1/8`, the value §3 of
the main note compared with the parity thresholds.

**Status.** Fitted, not derived. The period 4 and the offsets have no proof; Theorems 1 and 4 fix the
monotonicity and the intercepts, and the fit uses the observed step positions (`W = 1`: `t = 4, 8`;
`W = 2`: `t = 6, 10`; `W = 3`: `t = 9`, one step only).

**Predictions that falsify it** (cells not yet run; each needs the pilot's closure at `cmax 7`):
- `W = 1` reads 6 at `t = 12` and 5 at `t = 11` (5 is already measured at `t = 11`).
- `W = 3` reads 6 at `t = 10, 11, 12` and 7 at `t = 13`.
- `W = 2` reads 7 at `t = 14` (the C-step prediction of Amendment 6, already registered).

## 6. What remains open, stated exactly

- **A lower bound that grows with `t`.** By Theorem 2 it needs, for `c = d_F − 1`, a functional on
  `R_{≤c}` that vanishes on `V_c(f)` and not on `1`, multiplicative except through the top `c − 1`
  levels. Twisted-algebra points (§2) are multiplicative everywhere but are not grid points; the
  missing step is to correct them in the top window at a cost that stays below degree `c` until
  `t` exceeds a threshold growing like `4c`.
- **The period 4.** Why one block of four levels per degree at every width is not explained; nor is
  the offset `s_W`.
- **Isogeny towers and `m ≥ 3`.** Theorems 1 and 2 hold verbatim for the Kummer chain presentation;
  Theorem 4's lemma applies wherever the top form has a common variable. Nothing here covers the
  2-isogeny tower.

## 7. Proof map and formalization

| result | statement | Lean | status |
|:--|:--|:--|:--|
| Theorem 1 | monotone in `t` (fixed system); generic form via Thm 3 | none | proved in prose |
| Theorem 2 | top window | none | proved in prose |
| Theorem 3 | generic value; `S₃` support = width-2 box | none | proved in prose (no probability bound) |
| Lemma, Thm 4 | `ffd = D + 1` under a common top variable; `ffd(W)` | none | proved in prose; `W = 2` by D3 |
| Conjecture G | growth law | — | fitted, open |

Formalization was not done in this session. The container has no Lean toolchain (`lean`, `lake` and
`elan` are not installed), and the repository's Mathlib-based projects
(`research/guarded_fall_degree_20261028/formal`) would need a Mathlib cache larger than the remaining
disk allowance. The faithful Lean statements are small: Theorem 1 is a lemma about the least
submodule closed under a monotone rule and its image under a filtered ring homomorphism that sends
variables to variables or to 1; the Lemma of §4 is a statement in a commutative ring with a
filtration whose associated graded ring has `y² = 0`. Both are candidates for the next session with
the Lean toolchain (the `crypto-lean` gate).

## 8. Sources checked

`research/prime_fall_degree_20261027/DERIVED.md` (D1–D8, the "not derived" list) and
`PREREGISTRATION.md` §2 (definitions); `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`
§3.3; `research/first_fall_closure_20261111/README.md` (binary first fall degree; no overlap);
`research/prime_ic_leads_20261110/devcheck/README.md` (all rows used). Knowledge base unavailable
(`research-kb` not loaded in this session).
