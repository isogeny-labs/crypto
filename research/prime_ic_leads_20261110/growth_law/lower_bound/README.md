# Lower bound for `d_F(1, t)`: what is proved, what is computed, what is open (2026-11-13)

**Class.** A derivation, plus exact toy-size computations. No pilot run and no admitted measurement.
Every computed value here is over the toy prime `p = 786433 = 3·2^18 + 1`, not the pilot's
`P1 = 2013265921`, and is suggestive only for the pilot's generic draws. No pcc claim is registered
`[pcc:unverified]`. Notation follows `../README.md` §0 and `research/prime_fall_degree_20261027/DERIVED.md`.

**The question (§6 of `../README.md`).** For a generic width-1 generator
`f = c00 + c10 z1 + c01 z2 + c11 z1 z2` on the Kummer `m = 2` tower of height `t`, is there a lower
bound on the last fall degree `d_F` that grows with `t`? The fitted law is `d_F(1, t) = 3 + ⌊t/4⌋`,
which says `1 ∉ V_c(f)` exactly when `t ≥ 4(c − 2)`.


> **Status of the computations in this note (main session, 2026-11-13).** The proofs (Propositions 1
> and 2, Theorem 3, Lemmas 4 and 5) stand on their own. Every item labelled "computed" (§1 check, §2(b)
> check, §3 check, §4 context, §6, and the evidence quoted for §7) was produced by exploratory Python
> scripts written by the delegated agent. AGENTS.md ("Implementation language: no Python") does not
> admit Python as a research-result generator in this repository, so those scripts and their outputs
> are **not committed and not admitted as evidence**. They are kept outside the repository with their
> SHA-256 recorded in §9 so their provenance is not concealed. The observations are reported as
> exploratory leads only; each needs a native (Rust) reimplementation, for example as an option of
> `examples/pkm_tower_pilot.rs` (fixed coefficients across `t`, the `λ_f` test), before it counts.
> Every number below is `[pcc:unverified]`.

## Answer

**No growing lower bound is proved here.** This note proves five results that narrow the problem.
They are stated with full proofs in §§1–5: `[pcc:unverified]`

1. **Twisted points are vacuous** (Proposition 1). The README's twisted algebra `z_i^K = ζ_i` is
   isomorphic, as a filtered algebra with its tower variables, to the untwisted one. The isomorphism
   rescales the generator, `f(z) ↦ f(λ1 z1, λ2 z2)`. A curve point with `z_i^K = ζ_i` is exactly a grid
   zero of the rescaled generator. So a twisted-point functional only restates that non-units never
   refute. The route sketched in `../README.md` §2 and §6 cannot work as stated.
2. **An explicit pseudo-solution candidate, in closed form** (Proposition 2). Let
   `λ_f(g) = Σ_{x ∈ μ_K²} x1 x2 g(x)/f(x)`, which is `K²` times the coefficient of
   `T = z1^{K−1} z2^{K−1}` in `g·f⁻¹`. By two residue computations it equals
   `−(K/c11) Σ_{s ∈ μ_K} s·g(r_s, s)/((s + α)(r_s^K − 1))`. This is a weighted sum of evaluations at
   the `K` points of the curve `f = 0` whose `z2` coordinate is a `K`-th root of unity. Their `z1`
   coordinates `r_s` are not roots of unity. It kills every product `m·f` with `m ≠ T`. Also
   `λ_f(1) ≠ 0` for generic `f` (proved).
3. **A two-layer support criterion** (Theorem 3), valid for any unit generator of the `m = 2` tower. If
   a set of monomials `S0 ∋ 1` and `S1 = S0 ∪ y·S0` satisfy "every `h ∈ span S1` with
   `deg(h f) ≤ c − 1` lies in `span S0`", and `supp(f⁻¹) ⊄ S0`, then `d_F > c`.
4. **Low-degree multiples in the infinite tower** (Lemma 4). No non-zero multiple of `f` in
   `F[z1, z2]` has popcount degree `≤ 1`, in any characteristic.
5. **Carry-free high bits are paid for in degree** (Lemma 5). Take a multiplier `h` whose low parts
   (levels `< s`) never have an all-ones exponent. Then every high-level monomial of `h` has at most
   `deg(h f) − 2` bits. Every fall that moves information up the tower is therefore a carry through
   an all-ones low exponent. `[pcc:unverified]`

**Exploratory observations, not admitted** (§6; Python, outside the repository; see the status note above):

- **Toy ladder.** The `W = 1` closure ladder reproduces `3, 3, 4, 4, 4, 4` at `t = 2..7`, and
  `1 ∉ V_4` at `t = 8, 9`. With Theorem 1 of `../README.md` this gives an exact instance statement:
  for `f1 = 3 + 1234 z1 + 5678 z2 + 9012 z1 z2` over `F_786433`, `1 ∉ V_3(f1)` for every `t ≥ 4` and
  `1 ∉ V_4(f1)` for every `t ≥ 8`.
- **`λ_f` behaves as a pseudo-solution.** On every computed cell with `1 ∉ V_c`, `λ_f` kills
  `V_c(f)` and `λ_f(1) ≠ 0`. That is 22 cells: 14 at `W = 1` over three generators, and 8 at
  `W = 2, 3`. On the 6 computed cells with `1 ∈ V_c`, it does not.
- **The multipliers follow a level pattern.** At `c = 3`, the multipliers `V_c·f⁻¹` use exactly the
  monomials with at most one bit at levels `≥ 3`. At `c = 4`, `t = 8, 9`, the multipliers of degree-3
  elements use levels `0..6` freely, plus at most one further bit under an exact rule (§6.3).

**Open.** Two conjectures would give the lower half of Conjecture G for `W = 1`: Conjecture Λ (`λ_f`
kills `V_c(f)` whenever `1 ∉ V_c(f)`) and Conjecture C (the "core-width" pattern of the multipliers,
§7). The period-4 upper bound was not reached either. §8 lists what any proof has to handle.

---

## 0. Setting

`R = R_t = F[z1, z2]/(z1^K − 1, z2^K − 1)`, `K = 2^t`. Here `F` is a field of characteristic
`p ∤ 2`, and the grid `μ_K ⊂ F̄` may need an extension. The normal-form monomial `z^{(a,b)}`
(`0 ≤ a, b < K`) has degree `pop(a) + pop(b)`. The tower variables are `y_{i,j} = z_i^{2^j}`.
`V_c(f)` is the closure of `../README.md` §0. `P = F[z1, z2]` is the infinite tower: no top relation,
the same popcount degree, and `ι : R → P` lifts a normal form to its polynomial with exponents `< K`.

Write `f = c11 (z1 z2 + α z1 + β z2 + γ)` with `c11 ≠ 0`. Put `A = z1 + β` and `B = α z1 + γ`, so
`f = c11 (A z2 + B)`. The curve `f = 0` is the graph `z2 = φ(z1) = −B/A`. It is irreducible iff
`γ ≠ αβ`, i.e. `c00 c11 ≠ c10 c01`. `f` is a unit of `R` iff it has no zero on `μ_K²` (D1).

## 1. Proposition 1 (twist = rescaling)

**Statement.** Let `ζ1, ζ2 ∈ F^×` and let `λ1, λ2 ∈ F` satisfy `λ_i^K = ζ_i`. Let
`R^ζ = F[z]/(z1^K − ζ1, z2^K − ζ2)`, with the same normal-form monomials, degree and tower variables.
Define `Θ : R^ζ → R` by `z_i ↦ λ_i z_i`.

- (a) `Θ` is an algebra isomorphism. It sends each normal-form monomial to a non-zero multiple of
  itself, so it preserves degree, and it sends `y_{i,j}` to `λ_i^{2^j} y_{i,j}`.
- (b) `Θ(V^ζ_c(f)) = V_c(Θ f)` for every `f` and `c`, where `Θf(z) = f(λ1 z1, λ2 z2)` has the same
  support as `f`.
- (c) A point `P` with `f(P) = 0` and `P_i^K = ζ_i` exists iff `Θ f` has a zero on the grid `μ_K²`,
  namely `P/λ`.

**Proof.**

- (a) `(λ_i z_i)^K − ζ_i = ζ_i(z_i^K − 1) = 0` in `R`, so `Θ` is well defined. `z_i ↦ λ_i⁻¹ z_i`
  inverts it. On a normal form, `Θ(z^{(a,b)}) = λ1^a λ2^b z^{(a,b)}` with `a, b < K`.
- (b) `Θ(V^ζ_c(f))` is a subspace of `R_{≤c}` containing `Θ f`. If `g' = Θ g` has degree `< c`, then
  `deg g = deg g' < c` and `y_{i,j} g' = λ_i^{2^j} Θ(y_{i,j} g) ∈ Θ(V^ζ_c(f))`. So it is closed, and
  `V_c(Θ f) ⊆ Θ(V^ζ_c(f))`. The same argument with `Θ⁻¹` gives the reverse inclusion.
- (c) `Θ f(x) = f(λx)`, and `(λ_i x_i)^K = ζ_i` iff `x_i^K = 1`. ∎

**Consequence.** Theorem 2 of `../README.md` is correct. What it suggests, a lower bound from
twisted-algebra points, would amount to showing that the closure cannot tell the unit `f` from the
non-unit rescaled generator `Θ f`. That is not implied by anything proved so far. Computed agreement
(`out/run1.txt`, `twist.py`; `f1`, `t = 2..7`): with a generic twist `ζ = (5, 7)` the twisted `d_F`
equals the untwisted one at every `t`. With `ζ` on the image curve `{(x^K, φ(x)^K)}` (point
`x = 777`), `1` never enters up to the `cmax` used. This is consistent with (b) and (c).
Specializations `z1 = x1 ∈ μ_K` (D6) give only degree 2 (D8), so no one-point or one-variable
functional works either. `[pcc:unverified]`

## 2. Proposition 2 (the top-coefficient functional and its residue form)

Define `λ_f : R → F̄` by `λ_f(g) = Σ_{x ∈ μ_K²} x1 x2 g(x)/f(x)` for a unit `f`.

**(a) Coefficient form.** For every `g ∈ R`, `λ_f(g) = K²·[T](g f⁻¹)`, where `T = z1^{K−1} z2^{K−1}`.
Hence `λ_f(m f) = K²[T](m) = 0` for every normal-form monomial `m ≠ T`. Also
`λ_f(1) = K²[T] f⁻¹`.

*Proof.* `R ≅ F̄^{μ_K²}` (D1). The coefficient of `z^e` in `u` is `K^{-2} Σ_x u(x) x^{−e}`. Take
`u = g f⁻¹` and `e = (K − 1, K − 1)`, so that `x^{−e} = x1 x2`. ∎

**(b) Residue form.** Assume:

- `c11 ≠ 0` and `γ ≠ αβ` (irreducible curve);
- `f` is a unit;
- `(−β)^K ≠ 1`, i.e. `A` has no root on `μ_K`;
- `α^K ≠ 1`.

For `s ∈ μ_K` put `r_s = −(γ + β s)/(s + α)`, so that `Q_s = (r_s, s)` is the point of the curve with
`z2 = s`. Then for every normal form `g` (all exponents `< K`):

    λ_f(g) = −(K/c11) · Σ_{s ∈ μ_K}  s · g(r_s, s) / ((s + α)(r_s^K − 1)).

*Proof.*

1. Fix `x1 ∈ μ_K` and put `H(z) = g(x1, z)/f(x1, z)`. Here `f(x1, z) = c11 A(x1)(z − φ(x1))` and
   `A(x1) ≠ 0`.
2. For `x ∈ μ_K`, `Res_{z=x} H(z)/(z^K − 1) = H(x)/(K x^{K−1}) = x H(x)/K`. The rational function
   `H/(z^K − 1)` has `z`-degree `≤ (K − 1) − 1 − K = −2`, so its residue at `∞` is `0`.
3. The residue theorem on `P¹` holds over any field. It gives
   `Σ_{x2 ∈ μ_K} x2 H(x2) = −K·Res_{z=φ(x1)} = −K g(x1, φ(x1))/(c11 A(x1)(φ(x1)^K − 1))`.
   Note `φ(x1)^K ≠ 1`, because `f` is a unit.
4. Write `Ĝ = A^{K−1}·g(z, −B/A)`, a polynomial of degree `≤ 2K − 2`, and `D = B^K − A^K`. Then
   `g(z, φ)/(A(φ^K − 1)) = Ĝ/D`. Hence `λ_f(g) = −(K/c11) Σ_{x1 ∈ μ_K} x1 Ĝ(x1)/D(x1)`.
5. The same residue step applies to `Ĝ/(D(z^K − 1))`. Its degree is `≤ 2K − 2 − 2K = −2`, so there
   is again no residue at `∞`. This gives
   `λ_f(g) = (K²/c11) Σ_{D(r)=0} Ĝ(r)/(D'(r)(r^K − 1))`.
6. `D` has degree exactly `K`, with leading coefficient `α^K − 1`, and its roots are the `r` with
   `(−B(r)/A(r))^K = 1`. These are exactly the `K` distinct `r_s`, distinct because `φ` is a Möbius
   map, so the roots are simple. At `r = r_s`, `B = −sA` gives
   `D'(r) = K(α B^{K−1} − A^{K−1}) = −K A^{K−1}(s + α)/s` and `Ĝ(r) = A^{K−1} g(r_s, s)`.
7. Note `r_s^K ≠ 1`, since `Q_s` is not a grid point, and `s + α ≠ 0`, since `(−α)^K = α^K ≠ 1`.
   Substituting step 6 into step 5 gives the formula. ∎

The `z1 ↔ z2` symmetry gives the same functional as a weighted sum over the `K` curve points with
`z1 ∈ μ_K`. *Computed check* (`residue_check.py`, `out/run1.txt`): both sides agree for 3 random `g`
at each of `t = 2, 3, 4, 5` (`f1`, `p = 786433`).

**(c) `λ_f(1) ≠ 0` for generic `f`.** On the unit locus of the width-1 box, `[T] f⁻¹` has the form
`N(c)/Π_{x ∈ μ_K²} f_c(x)` with `N` a polynomial. At `c = (1, 0, 0, ε)`, with `ε ≠ 0` and `ε^K ≠ 1`,
we have `f = 1 + ε u`, `u = z1 z2`, `u^K = 1`. Then
`f⁻¹ = (1 − ε^K)^{-1} Σ_{k<K} (−ε u)^k`, whose `T = u^{K−1}` coefficient is
`(−ε)^{K−1}/(1 − ε^K) ≠ 0`. So `N ≢ 0`, and `λ_f(1) ≠ 0` on a dense open subset of the box. ∎

**What it means.** By (a), `λ_f` kills `V_c(f)` iff no element of `V_c(f)` has a multiplier
`h = g f⁻¹` with a non-zero `T` coefficient. In that case `λ_f/λ_f(1)` is a pseudo-solution and
`d_F > c`. By (b), this is the "combination of curve evaluations that cancels the top-relation
defects" that the open problem asks for.

- Each `ev_{Q_s}` is exactly multiplicative in `z2`, because `s^K = 1`.
- In `z1`, `ev_{Q_s}(y_{1,j} g) = r_s^{2^j} ev_{Q_s}(g) − (r_s^K − 1) ev_{Q_s}(E_{1,j} g)`. Here
  `E_{1,j} g` is the part of `g` with bits `j … t − 1` of the `z1` exponent all set, with those bits
  removed (Theorem 2 of `../README.md`).
- The factor `r_s^K − 1` cancels against the weight. So `λ_f(y_{1,j} g)` is a fixed residue-type
  functional of `E_{1,j} g`.

Proving that these defects vanish on `V_c` is the remaining step (Conjecture Λ, §7).

## 3. Theorem 3 (two-layer support criterion)

**Statement.** Let `f` be a unit of `R_t` with `deg f ≤ c`. Let `S0` be a set of normal-form
monomials with `1 ∈ S0`, and let `S1 = S0 ∪ {y_{i,j}·m : m ∈ S0, all i, j}`. A product of monomials is
again a monomial. Suppose:

- **(H)** every `h ∈ span S1` with `deg(h f) ≤ c − 1` lies in `span S0`.

Then `V_c(f) ⊆ Y := {h f : h ∈ span S1} ∩ R_{≤c}`, and `V_c(f) ∩ R_{≤c−1} ⊆ f·span S0`. In
particular, if `supp(f⁻¹) ⊄ S0` then `1 ∉ V_c(f)`, i.e. `d_F(f) > c`.

**Proof.**

1. `Y` contains `f`, since `1 ∈ S1` and `deg f ≤ c`.
2. Let `g ∈ Y` with `deg g < c`. Its multiplier `h = g f⁻¹` is unique and lies in `span S1`, and
   `deg(hf) ≤ c − 1`. By (H), `h ∈ span S0`.
3. Then `y h ∈ span S1`, so `y g = (y h) f ∈ Y`, because `deg(y g) ≤ c`. Thus `Y` is closed, and
   minimality gives `V_c ⊆ Y`.
4. If `1 ∈ V_c`, then its multiplier `f⁻¹` lies in `span S1` and `deg 1 = 0 ≤ c − 1`, so by (H)
   `f⁻¹ ∈ span S0`. ∎

**How it was tested.** `S0` was taken as the union of the supports of the multipliers of
`V_c ∩ R_{<c}`, computed from the closure (`thmB.py`, `out/run3.txt`, `f1`). (H) was then checked by
comparing `dim{h ∈ span S_k : deg(hf) ≤ c − 1}` for `k = 0, 1`.

The check is sound in one direction. The random projection can only overestimate kernel dimensions,
and `dim ≥ dim(V_c ∩ R_{<c})` holds for `S0` by construction. So an equality in the table below
proves (H) for that instance. `[pcc:unverified]`

| `t` | `c` | `dim V_c ∩ R_{<c}` | `|S0|` | kernel on `S0` | kernel on `S1` | (H) |
|--:|--:|--:|--:|--:|--:|:--|
| 4 | 3 | 14 | 64 | 14 | 22 | fails |
| 5 | 3 | 14 | 64 | 14 | 14 | holds |
| 6 | 3 | 14 | 64 | 14 | 14 | holds |
| 8 | 4 | 342 | 20480 | 342 | 474 | fails |

With these `S0`, (H) fails exactly at the threshold cells `t = 4(c − 2)` and holds one level above.
Any proof by this criterion therefore either uses a finer `S0` at the threshold, or settles for
`t ≥ 4(c − 2) + 1`. The latter would still be a growing bound, `d_F ≥ 3 + ⌊(t − 1)/4⌋`.

## 4. Lemma 4 (no multiple of popcount degree ≤ 1 in the infinite tower)

**Statement.** Under the hypotheses `c11 ≠ 0` and `γ ≠ αβ`, `f P ∩ P_{≤1} = 0`, over any field and
with no bound on the exponents.

**Proof.** Let `g = γ0 + Σ_j a_j z1^{2^j} + Σ_j b_j z2^{2^j}` vanish on the curve. Substitute
`z2 = φ(z1) = −B/A`. The term `b_j φ^{2^j} = b_j (−B)^{2^j}/A^{2^j}` has a pole of order exactly `2^j`
at `z1 = −β`, because `B(−β) = γ − αβ ≠ 0`. The other terms are polynomials. If some `b_j ≠ 0`, the
largest such `j` gives a pole order that nothing cancels. So all `b_j = 0`, and then the polynomial
`γ0 + Σ a_j z1^{2^j}` is `0`, which forces all coefficients to vanish. ∎

*Computed context* (`lowmult.py`, `out/run1.txt`, `f1`): `dim(fP ∩ P_{≤2})` restricted to exponents
`< 2^s` is `4, 9, 14, 15, 15, 15` for `s = 2..7`. It stabilizes at 15, with largest exponent 16.
`fP ∩ P_{≤3}` grows without bound in `s` (`8, 27, 64, 121, 194` for `s = 2..6`), because it contains
`y·(fP ∩ P_{≤2})`. So no proof can simply bound the set of low-degree multiples. It must use the
closure's reachability.

## 5. Lemma 5 (carry-free high bits cost degree)

**Statement.** Let `1 ≤ s ≤ t` and write `h ∈ R_t` as `h = Σ_m m·h_m`. Here `m` ranges over the
monomials in the variables `y_{i,j}` with `j ≥ s`, and `h_m ∈ span{z1^a z2^b : a, b < 2^s}`. Assume
every `h_m` has all `z1` and `z2` exponents `≤ 2^s − 2`, i.e. no low exponent is all ones. Then:

- `deg(h f) = max_{h_m ≠ 0} (deg m + deg(h_m f))`;
- `deg(h_m f) ≥ 2` whenever `h_m ≠ 0`;
- hence every `m` occurring in `h` has `deg m ≤ deg(h f) − 2`.

**Proof.**

1. `h_m f` has exponents `≤ 2^s − 1`. It equals the product in `P`, with no wrap because
   `2^s − 1 < K`.
2. The exponents of `m` are sums of distinct powers `2^j`, `s ≤ j < t`. Adding a low exponent `< 2^s`
   causes no carry and stays below `K`. So `m·(h_m f)` has monomials `z^{e_m + e}` with popcount
   `deg m + pop(e)`, and different `m` give disjoint sets of monomials.
3. Hence `hf = Σ_m m·(h_m f)` with disjoint supports, and its degree is the maximum.
4. `h_m f ≠ 0` lies in `fP`, so by Lemma 4 its degree is `≥ 2`. ∎

**Reading.** High bits in a multiplier are paid one degree each unless a low part reaches an
all-ones exponent `2^s − 1`, whose product with `f` carries into level `s`. The closure's falls
exploit exactly these carries. The computed cores (§6.3) are what such carry chains build per unit
of degree.

## 6. Exploratory computations, not admitted (Python; exact over `F_786433`; float64 linear algebra with `p²·N < 2^53`)

All of these run in `scripts/`, from that directory. Each run took under 2 minutes, except
`hsupp2.py 9 4`, which took about 7.5 minutes on the shared host. Peak memory stayed under 1 GB.

Generators:

- `f1 = 3 + 1234 z1 + 5678 z2 + 9012 z1 z2`. It has no zero on `μ_{2^9}²`, which was checked with
  `gridcheck.py`.
- `f2 = 17 + 40000 z1 + 123457 z2 + 654321 z1 z2` and `f3 = 101 + 7 z1 + 99991 z2 + 31337 z1 z2`. Both
  are units at `t ≤ 8`.
- Width 2 and 3: seeded random coefficients (`topfun_w.py`, seed 1), the same polynomial at every `t`.

### 6.1 The `W = 1` ladder (`closure.py`, `filt.py`, `lowsupp.py`; `out/run1.txt`, `out/run3.txt`)

| `t` | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| `d_F(f1)` | 3 | 3 | 4 | 4 | 4 | 4 | ≥ 5 | ≥ 5 |

At `t = 8, 9` only `c ≤ 4` was run: `c = 5` at `t = 8` needs a 6,885-column dense closure, which
exceeds this note's resource limit. The values agree with the pilot's `W = 1` ladder at `P1`
(`../README.md` §5).

**Instance statement (exact).** `f1` is the same polynomial at every height. So Theorem 1 of
`../README.md` turns the computed `1 ∉ V_3` at `t = 4` and `1 ∉ V_4` at `t = 8` into the following:
`1 ∉ V_3(f1)` for all `t ≥ 4`, and `1 ∉ V_4(f1)` for all `t ≥ 8`. This is for this `f1` and this
`p`. Genericity over the box is not established by one point.

Closure dimensions by pivot degree:

- At `c = 3`, `V_3 ∩ R_{≤2}` has dimension 14 for every `t = 4..7`. It has no element of degree
  `≤ 1`, and its support has exponents `≤ 8` in each variable (levels `0..3`).
- `dim V_3 = 14 + 28(t − 2)`.
- At `c = 4`: `[0, 0, 15, 327, 1768]` at `t = 8` and `[0, 0, 15, 357, 2437]` at `t = 9`. The
  degree-2 part, 15-dimensional, is then all of `fP ∩ P_{≤2}`.

### 6.2 `λ_f` as a pseudo-solution (`topfun.py`, `topfun_w.py`; `out/run2.txt`)

"Kills `V_c`" means `λ_f(g) = 0` for a basis of `V_c(f)`. On every row of the table with
`1 ∉ V_c`, `λ_f` kills `V_c` and `λ_f(1) ≠ 0`. On every row with `1 ∈ V_c`, it does not.

| generator | cells `(t, c)` with `1 ∉ V_c`: `λ_f` kills `V_c` | cells with `1 ∈ V_c` |
|:--|:--|:--|
| `f1` | (3,2) (4,3) (5,3) (6,3) (7,3) (8,4) (9,4) | (3,3) (6,4) (7,4) |
| `f2` | (3,2) (4,3) (5,3) (6,3) (8,4) | (3,3) (7,4) |
| `f3` | (4,3) (8,4) | — |
| `W = 2`, seed 1 | (2,3) (3,3) (5,3) (6,4) (7,4) | (5,4) |
| `W = 3`, seed 1 | (4,4) (6,4) (7,4) | — |

The `W = 2` toy ladder reads `d_F = 4` at `t ≤ 5` and `≥ 5` at `t = 6, 7`, which matches
`4 + ⌊(t − 2)/4⌋`.

### 6.3 The support of the multipliers `H_c = V_c·f⁻¹` (`hsupp2.py`, `s0pat.py`; `out/run3.txt`, `out/hsupp2_t9_c4.txt`)

A monomial is *unused* if no element of `V_c` has a multiplier containing it. In every case below the
unused set is an up-set under bitwise inclusion. Bit strings are printed most significant bit first.

| `(t, c)` | used / `K²` | minimal unused `(z1 bits, z2 bits)` | description |
|:--|:--|:--|:--|
| (4,3) | 192 / 256 | `({3},{3})` | at most one bit at levels `≥ 3` |
| (5,3) | 320 / 1024 | any two bits at levels `{3,4}` (6 minimal elements) | at most one bit at levels `≥ 3` |
| (6,3) | 448 / 4096 | any two bits at levels `{3,4,5}` (15 minimal elements) | at most one bit at levels `≥ 3` |
| (8,4) | 52992 / 65536 | `({7, k}, {7, k'})`, `k, k' ∈ {4,5,6}` (9 minimal elements) | — |
| (9,4) | 94208 / 262144 | 31 minimal elements (`out/hsupp2_t9_c4.txt`) | not described here |

At `(8, 4)`, split by the degree of the element:

- Degree `≤ 2` elements (15-dimensional): multipliers lie in levels `0..3`, 256 monomials.
- Degree `≤ 3` elements (342-dimensional): multipliers avoid exactly the up-set generated by "a
  variable has bit 7 and a bit in `{4,5,6}`" and "both variables have bit 7". So levels `0..6` are
  free in both variables, plus bit 7 in at most one variable, and only without bits `4..6` there.

At `(9, 4)` the degree `≤ 3` multipliers (372 elements; `s0pat.py 9 4 3`, `out/s0pat_t9_c4.txt`)
use 20992 monomials. Their complement has 24 minimal elements, and exactly the following rule
reproduces all 24, counted by hand against the list. Levels `0..6` are free in both variables. At
most one bit lies at levels `≥ 7`. A bit at level 7 forbids bits `4..6` in its own variable, and a bit
at level 8 forbids bits `4..6` in both variables. The rule also matches the 7 minimal elements at
`t = 8` (no level 8 there). A level-8 bit thus behaves like a free variable on top of the degree-2
core (levels `0..3`). Level 7 is part of a degree-3 core that spans levels `0..7` with the stated
restriction.

The degree-2 multiples (`mult2.py 2 6`, `out/mult2_d2_s6.txt`): the monomials of the 15-dimensional
`fP ∩ P_{≤2}` with an exponent `≥ 16` are exactly `z1^{16} z2^b` and `z1^b z2^{16}` with
`b ∈ {0, 1, 2, 4, 8}`, together with `z1^{16} z2^{16}`.

## 7. Conjectures (open; the evidence is §6 only)

**Conjecture Λ.** For a generic generator in a width-`W` box, `1 ∉ V_c(f)` iff `λ_f(V_c(f)) = 0`.
Equivalently, `d_F(f)` is the least `c` for which some element of `V_c(f)` has a multiplier `g f⁻¹`
with a non-zero coefficient at `T = z1^{K−1} z2^{K−1}`. The "if" direction is Proposition 2 plus
§2(c). Evidence: 22 cells with `1 ∉ V_c` and 6 with `1 ∈ V_c`, §6.2, all consistent.

**Conjecture C (core width, `W = 1`).** For `t ≥ 4(c − 2)`, the multipliers of the degree-`(c − 1)`
elements of `V_c(f)` lie in a set `S0^{(c)}` of the following shape:

- a *core region*, levels `0 … 4(c − 2) − 2` in both variables, plus a bit at level `4(c − 2) − 1`
  under a restriction;
- at most one further bit, whose allowed position depends on which lower core it is attached to.

`S0^{(c)}` also satisfies (H) of Theorem 3 for `t ≥ 4(c − 2) + 1`. Evidence: `c = 3`, core levels
`0..2` (`|S0| = 64`, (H) verified at `t = 5, 6`); `c = 4`, core levels `0..6` plus the
level-7 rule (§6.3, exact at `t = 8, 9`). The general shape of `S0^{(c)}` is not determined by these
two cases. `[pcc:unverified]`

**Conjecture C + §2(c) would give the lower half of Conjecture G for `W = 1`** (heuristic count, not
a proof). The top monomial `T` has `2t` bits. A degree-`(c − 1)` element contributes at most
`2(4(c − 2) − 1)` core bits plus one free bit to a degree-`c` multiplier. So `T ∈ supp(H_c)` needs
`2t ≤ 8(c − 2) − 1`, i.e. `t ≤ 4(c − 2) − 1`. With Conjecture Λ, `1 ∈ V_c` iff `t ≤ 4c − 9`, which
is `d_F = 3 + ⌊t/4⌋`. The computed maxima agree with the count. The degree-`(c − 1)` multipliers
reach popcount 6 at `c = 3` and 14 at `c = 4` (`s0pat.py` at `t = 8, 9`). `H_3` at `t = 4` reaches 7,
below `2t = 8`. Turning this count into a proof needs Conjecture C as an exact support
statement, i.e. (H) of Theorem 3 at `t ≥ 4(c − 2)` (or `+1`) for an explicit `S0^{(c)}`.

## 8. What any proof has to handle (the obstruction, stated exactly)

1. **Not a character, not a specialization.**
   - Twisted-point evaluations are equivalent to non-unit generators (Prop. 1).
   - One-variable specializations refute at degree 2 (D8).
   - The natural functional, `λ_f`, is a combination of `K` curve-point evaluations that are not grid
     points. Each is multiplicative in only one variable. Its defects in the other variable cancel by
     a residue identity that holds only for the exact weights of §2(b).
2. **Reachability, not divisibility.** The set of multiples of `f` of degree `≤ 3` in the infinite
   tower is infinite-dimensional (§4). The closure at `c = 3` reaches 14 of the 15 degree-2
   multiples, those in levels `0..3`. The 15th, with exponent 16, appears only at `c = 4`. A proof
   must use the closure's dynamics.
3. **Carries are the only engine** (Lemma 5). Every extra high bit of a multiplier at fixed degree
   comes from an all-ones low exponent carrying into the next level. The period 4 is the statement
   that one unit of degree buys a carry chain four levels long. Data: the free core grows from levels
   `0..2` (`c = 3`) to `0..6` (`c = 4`).
4. **The threshold is sharp for simple invariants.** Theorem 3 with the closure's own supports fails
   at `t = 4(c − 2)`, for `(4, 3)` and `(8, 4)`, and holds one level above. Either refine `S0` at the
   threshold, or accept a growing bound that is off by one level.
5. **Genericity.** All verifications are at single points over `F_786433`. A theorem for generic
   coefficients at `P1` needs either a symbolic argument, or exhibiting `dim(V_c ∩ R_{<c})`
   independent elements generically (e.g. the 14 at `c = 3`) so that the rank conditions in (H)
   become open conditions.

**Upper bound with period 4 (task item 2): not achieved.** The only lead is §6.1's observation that
the 15th degree-2 multiple has the shape `κ w1 w2 + w1·ℓ1 + w2·ℓ2 + (levels 0..3)`. Here
`w_i = z_i^{16}` and `ℓ_i` are degree-`≤ 1` forms in the other variable's levels `0..3`. It is a
Möbius-type relation one block of four levels up, with non-constant coefficients. Whether the closure
at degree `c + 1` simulates the degree-`c` closure of this level-4 relation is unexamined.

## 9. Files and provenance

The exploratory Python scripts and their outputs were moved out of the repository (main-session
scratchpad archive `lower_bound_python_archive/`; the agent's working copies in `agent-lb/`). SHA-256:

```
7eec8ca82015409b93165ab2200f085cb4ef6a8fabb22462134a9f78bfb95fb1  ./out/hsupp2_t9_c4.txt
2eb589e03802150f6dbcc4550ffeb068253d56512b9bf7843f32f4ed7ef51123  ./out/mult2_d2_s6.txt
18146165983930bf112b875a94e0a2db5521bae8017e51d7883d65e0e4ea683f  ./out/run1.txt
a4f7f59b97f9055f67daae7fa3fb3d7a6451769757a6eeab82d7c9ef6f204985  ./out/run2.txt
cc2b42d6488f6561e3d8b0b31e3eb85f5771cf59ca03af4065befa2a872188c4  ./out/run3.txt
54be49b84291362d624fef37fc16a8d456f0fc26aa966e968ef4c3352f81021c  ./out/s0pat_t9_c4.txt
73c7137df80f10497a906ce0312733d2a02e633f9af0dc61fcf93566a208435d  ./scripts/closure.py
5e3bfb724747d6127f1ada7de14e435cb9d89a1f2eed1180742097844ec6ef76  ./scripts/filt.py
4d40240f817535dacaecc8bb8651cb50ba068317fff978851914ef8a34079ea0  ./scripts/gridcheck.py
1d06a6cd4786a567623e68219d8a118db0ab5d02969346c6c9986e2e507eca51  ./scripts/hsupp2.py
896b78d5a846a8c48025cb9929602950335bce0a9fbd8d6a22767b3d8c6caebc  ./scripts/lowmult.py
fc50b3725076ea9e46608597d5521582cd313a107c50cbb37731f6c95a61d57c  ./scripts/lowsupp.py
3daea83ed927b590f410883a05712418619a9635bcb29accd7b2e841644d94c8  ./scripts/mult2.py
dbd6dc79a6dd4e9caa42683861adae18e31323c340cd8b6edaa7cb7f3e1f93a3  ./scripts/residue_check.py
1d436659360b67faca61ed086ec76d312b9e0d52d3dcb9938be8713e809909ec  ./scripts/s0pat.py
cd810a3df98f872490d5f44dd3c81ead840945c6864cef241cd2fe1d5761016f  ./scripts/thmB.py
cd9c09d1d7947bdec6e0f4a962208c8a194690889e02af9e5742b78373c6dd53  ./scripts/topfun.py
b54f4984d1e98cf46ab0dc7b5e504797dbba3faa135ab4d05776468c7e43e368  ./scripts/topfun_w.py
72821d5932a01f6dc367c891476f3b09baee7a0d4e8526f1e0a8c3c8ec317e8e  ./scripts/twist.py
73c7137df80f10497a906ce0312733d2a02e633f9af0dc61fcf93566a208435d  agent-lb/closure.py
5e3bfb724747d6127f1ada7de14e435cb9d89a1f2eed1180742097844ec6ef76  agent-lb/filt.py
4d40240f817535dacaecc8bb8651cb50ba068317fff978851914ef8a34079ea0  agent-lb/gridcheck.py
ad8e27c79272b19454678557b9f22c01a443e07a28f5fa5ae6b1f15b2de45f2c  agent-lb/hspace.py
2d32df3bb5a76d9e96fbe7055532e59a7ae8a309c0c380c5dc6816d7b4de1ee9  agent-lb/hsupp.py
1d06a6cd4786a567623e68219d8a118db0ab5d02969346c6c9986e2e507eca51  agent-lb/hsupp2.py
896b78d5a846a8c48025cb9929602950335bce0a9fbd8d6a22767b3d8c6caebc  agent-lb/lowmult.py
d60859f81c338f97e8089e364868ca996cb7a43b4567fd939451126d77c917b8  agent-lb/lowpart.py
fc50b3725076ea9e46608597d5521582cd313a107c50cbb37731f6c95a61d57c  agent-lb/lowsupp.py
3daea83ed927b590f410883a05712418619a9635bcb29accd7b2e841644d94c8  agent-lb/mult2.py
dbd6dc79a6dd4e9caa42683861adae18e31323c340cd8b6edaa7cb7f3e1f93a3  agent-lb/residue_check.py
429c50b78f6830de3f3f0fd5feafa728fd59f60f94aa727899804ae73def53ff  agent-lb/s0pat.py
cd810a3df98f872490d5f44dd3c81ead840945c6864cef241cd2fe1d5761016f  agent-lb/thmB.py
cd9c09d1d7947bdec6e0f4a962208c8a194690889e02af9e5742b78373c6dd53  agent-lb/topfun.py
b54f4984d1e98cf46ab0dc7b5e504797dbba3faa135ab4d05776468c7e43e368  agent-lb/topfun_w.py
72821d5932a01f6dc367c891476f3b09baee7a0d4e8526f1e0a8c3c8ec317e8e  agent-lb/twist.py
```

Nothing in this note depends on them except the items marked exploratory.

## Sources checked

`../README.md` (Theorems 1–4, Conjecture G, §6); `research/prime_fall_degree_20261027/DERIVED.md`
(D1–D8, "What is not derived"); `PREREGISTRATION.md` §2 (closure, `d_F`, `ffd`);
`research/prime_ic_leads_20261110/devcheck/README.md` (width ladder). The `research-kb` MCP server
was not available to this subagent, so no knowledge-base receipt is recorded. The novelty of
Proposition 2 and Theorem 3 is therefore unchecked against the corpus. Theorem 3 is the
polynomial-calculus degree-lower-bound pattern ("R-operator"/pseudo-solution duality) specialized to
a support-closed subspace; it is presented as a tool, not as new.
