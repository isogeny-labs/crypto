import Mathlib
import VeluObstruction

/-!
# Odd degree: no Vélu ℓ-isogeny x-map is Möbius-conjugate to a monomial map

For odd ℓ ≥ 3, Vélu's formulas (in Kohel's form) give the x-map of an
ℓ-isogeny as `x ↦ N(x)/ψ(x)²`. Here `ψ` is the monic kernel polynomial,
`∏_{Q ∈ S} (x - x(Q))`, where `S` holds one point from each pair `±Q` of
nonzero kernel points, so `deg ψ = (ℓ-1)/2 ≥ 1`. Homogenised, the map is
`(X : Z) ↦ (N(X,Z) : Z·ψ(X,Z)²)`.

The obstruction for ℓ = 2 (`VeluObstruction.lean`) needs two consecutive steps.
For ℓ ≥ 3 one step is already enough: **no Möbius coordinates on source and
target make a single Vélu ℓ-step monomial** (`u ↦ λu^ℓ` or `u ↦ λ/u^ℓ`).

The argument, for any field `K` in which `ℓ` is a unit:

* The fibre form over `∞` is `-Z·ψ²`. It has a *simple* zero at `(1 : 0)`,
  because `ψ` is monic, and a zero of order ≥ 2 at every root of `ψ`.
* Every fibre form of a monomial map is a binomial `αX^ℓ + βZ^ℓ`. Such a
  binomial never has both a zero of order ≥ 2 and a simple zero:
  - if `αβ ≠ 0`, a double zero forces the point to be `(0, 0)`;
  - if `α = 0` or `β = 0`, every zero has order `ℓ ≥ 2`.
* A Möbius change of coordinates turns fibres into fibres and keeps the order
  of zeros. Orders are defined by restricting a form to lines,
  `t ↦ F(p + t q)`, and this restriction commutes with linear substitution.

The modelling input is Kohel's form of Vélu's theorem: the denominator of the
odd-degree x-map is the square of the kernel polynomial. It is not formalised
(Mathlib has no isogenies) and is stated in `velu_odd_not_monomial` as the
shape of the map.
-/

namespace VeluObstruction

variable {K : Type*} [Field K]

/-- Restriction of a binary form to the line `t ↦ p + t q`. -/
noncomputable def lineR (F : MvPolynomial (Fin 2) K) (p q : Fin 2 → K) : Polynomial K :=
  MvPolynomial.aeval (fun i => Polynomial.C (p i) + Polynomial.C (q i) * Polynomial.X) F

/-- Linear substitution `X ↦ aX + bZ`, `Z ↦ cX + dZ` of a binary form. -/
noncomputable def lsubst (a b c d : K) (F : MvPolynomial (Fin 2) K) : MvPolynomial (Fin 2) K :=
  MvPolynomial.aeval
    ![MvPolynomial.C a * MvPolynomial.X 0 + MvPolynomial.C b * MvPolynomial.X 1,
      MvPolynomial.C c * MvPolynomial.X 0 + MvPolynomial.C d * MvPolynomial.X 1] F

/-- The matrix `[[a, b], [c, d]]` applied to a point. -/
def app (a b c d : K) (p : Fin 2 → K) : Fin 2 → K :=
  ![a * p 0 + b * p 1, c * p 0 + d * p 1]

theorem eval0_lineR (F : MvPolynomial (Fin 2) K) (p q : Fin 2 → K) :
    (lineR F p q).eval 0 = MvPolynomial.eval p F := by
  induction F using MvPolynomial.induction_on
  next x => simp [lineR]
  next f g hf hg =>
    simp only [lineR, map_add, Polynomial.eval_add] at hf hg ⊢
    rw [hf, hg]
  next f i hf =>
    simp only [lineR, map_mul, Polynomial.eval_mul, MvPolynomial.aeval_X,
      MvPolynomial.eval_X] at hf ⊢
    rw [hf]
    simp

/-- Line restriction commutes with linear substitution. -/
theorem lineR_subst (a b c d : K) (F : MvPolynomial (Fin 2) K) (p q : Fin 2 → K) :
    lineR (lsubst a b c d F) p q = lineR F (app a b c d p) (app a b c d q) := by
  induction F using MvPolynomial.induction_on
  next x => simp [lineR, lsubst]
  next f g hf hg =>
    simp only [lineR, lsubst, map_add] at hf hg ⊢
    rw [hf, hg]
  next f i hf =>
    simp only [lineR, lsubst, map_mul, MvPolynomial.aeval_X] at hf ⊢
    rw [hf]
    congr 1
    fin_cases i <;> simp [app] <;> ring

theorem lineR_C_mul (c : K) (F : MvPolynomial (Fin 2) K) (p q : Fin 2 → K) :
    lineR (MvPolynomial.C c * F) p q = Polynomial.C c * lineR F p q := by
  simp [lineR]

/-- Applying the adjugate undoes `M`, scaled by `det⁻¹`. -/
theorem app_adj_inv (m : Mob K) (hm : m.det ≠ 0) (v₀ v₁ : K) :
    app m.d (-m.b) (-m.c) m.a
      ![m.det⁻¹ * (m.a * v₀ + m.b * v₁), m.det⁻¹ * (m.c * v₀ + m.d * v₁)] = ![v₀, v₁] := by
  have h := inv_mul_cancel₀ hm
  simp only [Mob.det] at h
  funext i
  fin_cases i
  · simp [app, Mob.det]
    linear_combination v₀ * h
  · simp [app, Mob.det]
    linear_combination v₁ * h

/-- A degree-`ℓ` map of `ℙ¹`, given by two binary forms. -/
structure LMap (K : Type*) [Field K] where
  N : MvPolynomial (Fin 2) K
  D : MvPolynomial (Fin 2) K

/-- Fibre form over the target `(T : W)`: `W•N - T•D`. -/
noncomputable def fibreL (F : LMap K) (T W : K) : MvPolynomial (Fin 2) K :=
  MvPolynomial.C W * F.N - MvPolynomial.C T * F.D

/-- `M' ∘ F ∘ M⁻¹`, using the adjugate of `M`. -/
noncomputable def conjL (F : LMap K) (m m' : Mob K) : LMap K :=
  ⟨MvPolynomial.C m'.a * lsubst m.d (-m.b) (-m.c) m.a F.N
      + MvPolynomial.C m'.b * lsubst m.d (-m.b) (-m.c) m.a F.D,
   MvPolynomial.C m'.c * lsubst m.d (-m.b) (-m.c) m.a F.N
      + MvPolynomial.C m'.d * lsubst m.d (-m.b) (-m.c) m.a F.D⟩

theorem fibreL_conj (F : LMap K) (m m' : Mob K) (T₀ W₀ : K) :
    fibreL (conjL F m m') (m'.a * T₀ + m'.b * W₀) (m'.c * T₀ + m'.d * W₀)
      = MvPolynomial.C m'.det * lsubst m.d (-m.b) (-m.c) m.a (fibreL F T₀ W₀) := by
  simp only [fibreL, conjL, lsubst, Mob.det, map_add, map_sub, map_mul, MvPolynomial.aeval_C,
    MvPolynomial.algebraMap_eq]
  ring

/-- The binomial `αX^ℓ + βZ^ℓ`. -/
noncomputable def binom (ℓ : ℕ) (α β : K) : MvPolynomial (Fin 2) K :=
  MvPolynomial.C α * MvPolynomial.X 0 ^ ℓ + MvPolynomial.C β * MvPolynomial.X 1 ^ ℓ

/-- `G` is monomial of degree `ℓ`: up to a nonzero scalar, `u ↦ λu^ℓ` or `u ↦ λ/u^ℓ`. -/
def IsMonomialL (ℓ : ℕ) (G : LMap K) : Prop :=
  ∃ κ l : K, κ ≠ 0 ∧ l ≠ 0 ∧
    ((G.N = MvPolynomial.C (κ * l) * MvPolynomial.X 0 ^ ℓ ∧
        G.D = MvPolynomial.C κ * MvPolynomial.X 1 ^ ℓ) ∨
     (G.N = MvPolynomial.C (κ * l) * MvPolynomial.X 1 ^ ℓ ∧
        G.D = MvPolynomial.C κ * MvPolynomial.X 0 ^ ℓ))

theorem monomial_fibre_binom {ℓ : ℕ} {G : LMap K} (hG : IsMonomialL ℓ G) (T W : K) :
    ∃ α β : K, fibreL G T W = binom ℓ α β := by
  obtain ⟨κ, l, -, -, ⟨hN, hD⟩ | ⟨hN, hD⟩⟩ := hG
  · exact ⟨W * (κ * l), -(T * κ), by
      simp only [fibreL, binom, hN, hD, map_mul, map_neg]; ring⟩
  · exact ⟨-(T * κ), W * (κ * l), by
      simp only [fibreL, binom, hN, hD, map_mul, map_neg]; ring⟩

theorem binom_val (ℓ : ℕ) (α β : K) (p q : Fin 2 → K) :
    (lineR (binom ℓ α β) p q).eval 0 = α * p 0 ^ ℓ + β * p 1 ^ ℓ := by
  rw [eval0_lineR]; simp [binom]

theorem binom_slope (ℓ : ℕ) (α β : K) (p q : Fin 2 → K) :
    (Polynomial.derivative (lineR (binom ℓ α β) p q)).eval 0
      = (ℓ : K) * (α * p 0 ^ (ℓ - 1) * q 0 + β * p 1 ^ (ℓ - 1) * q 1) := by
  simp only [binom, lineR, map_add, map_mul, map_pow, MvPolynomial.aeval_C,
    MvPolynomial.aeval_X, Polynomial.algebraMap_eq]
  simp [Polynomial.derivative_pow] <;> ring

/-- A binomial `αX^ℓ + βZ^ℓ` (with `ℓ ≥ 2` a unit) cannot vanish to order ≥ 2 at
one point and to order exactly 1 at another. -/
theorem binom_contra {ℓ : ℕ} (hℓ : 2 ≤ ℓ) (hℓK : (ℓ : K) ≠ 0) (α β : K)
    {P R Q₀ : Fin 2 → K} (hP : P ≠ 0)
    (h2 : ∀ q, (Polynomial.derivative (lineR (binom ℓ α β) P q)).eval 0 = 0)
    (hRv : (lineR (binom ℓ α β) R Q₀).eval 0 = 0)
    (hRs : (Polynomial.derivative (lineR (binom ℓ α β) R Q₀)).eval 0 ≠ 0) : False := by
  have hl1 : ℓ - 1 ≠ 0 := by omega
  have e0 : (ℓ : K) * (α * P 0 ^ (ℓ - 1)) = 0 := by
    have h := h2 ![1, 0]
    rw [binom_slope] at h
    simp only [Matrix.cons_val_zero, Matrix.cons_val_one, Matrix.head_cons, mul_one, mul_zero, add_zero, zero_add] at h
    linear_combination h
  have e1 : (ℓ : K) * (β * P 1 ^ (ℓ - 1)) = 0 := by
    have h := h2 ![0, 1]
    rw [binom_slope] at h
    simp only [Matrix.cons_val_zero, Matrix.cons_val_one, Matrix.head_cons, mul_one, mul_zero, add_zero, zero_add] at h
    linear_combination h
  have hα := (mul_eq_zero.mp e0).resolve_left hℓK
  have hβ := (mul_eq_zero.mp e1).resolve_left hℓK
  rw [binom_val] at hRv
  rw [binom_slope] at hRs
  by_cases hp0 : P 0 = 0
  · have hp1 : P 1 ≠ 0 := by
      intro hp1
      apply hP
      funext i
      fin_cases i <;> simp [hp0, hp1]
    have hb0 : β = 0 := (mul_eq_zero.mp hβ).resolve_right (pow_ne_zero _ hp1)
    subst hb0
    have hs : (ℓ : K) * (α * R 0 ^ (ℓ - 1) * Q₀ 0) ≠ 0 := by
      intro h; apply hRs; linear_combination h
    have hR0 : R 0 ≠ 0 := by
      intro h0; apply hs; rw [h0, zero_pow hl1]; ring
    have hα0 : α ≠ 0 := by
      intro h0; apply hs; rw [h0]; ring
    have hv : α * R 0 ^ ℓ = 0 := by linear_combination hRv
    exact mul_ne_zero hα0 (pow_ne_zero ℓ hR0) hv
  · have ha0 : α = 0 := (mul_eq_zero.mp hα).resolve_right (pow_ne_zero _ hp0)
    subst ha0
    have hs : (ℓ : K) * (β * R 1 ^ (ℓ - 1) * Q₀ 1) ≠ 0 := by
      intro h; apply hRs; linear_combination h
    have hR1 : R 1 ≠ 0 := by
      intro h0; apply hs; rw [h0, zero_pow hl1]; ring
    have hβ0 : β ≠ 0 := by
      intro h0; apply hs; rw [h0]; ring
    have hv : β * R 1 ^ ℓ = 0 := by linear_combination hRv
    exact mul_ne_zero hβ0 (pow_ne_zero ℓ hR1) hv

/-- The fibre over `∞` of a map with denominator `Z·ψ²`, restricted to a line. -/
theorem lineR_fibInf (N ψ : MvPolynomial (Fin 2) K) (p q : Fin 2 → K) :
    lineR (fibreL ⟨N, MvPolynomial.X 1 * ψ ^ 2⟩ 1 0) p q
      = -((Polynomial.C (p 1) + Polynomial.C (q 1) * Polynomial.X) * (lineR ψ p q) ^ 2) := by
  simp [fibreL, lineR]

/-- Every root of `ψ` is a zero of order ≥ 2 of the fibre over `∞`. -/
theorem fibInf_double (N ψ : MvPolynomial (Fin 2) K) (p q : Fin 2 → K)
    (hp : MvPolynomial.eval p ψ = 0) :
    (Polynomial.derivative (lineR (fibreL ⟨N, MvPolynomial.X 1 * ψ ^ 2⟩ 1 0) p q)).eval 0 = 0 := by
  have h0 : (lineR ψ p q).eval 0 = 0 := by rw [eval0_lineR, hp]
  rw [lineR_fibInf]
  simp [Polynomial.derivative_mul, Polynomial.derivative_pow, h0]

/-- `(1 : 0)` is a zero of the fibre over `∞`. -/
theorem fibInf_simple_val (N ψ : MvPolynomial (Fin 2) K) :
    (lineR (fibreL ⟨N, MvPolynomial.X 1 * ψ ^ 2⟩ 1 0) ![1, 0] ![0, 1]).eval 0 = 0 := by
  rw [lineR_fibInf]
  simp

/-- ... and a simple one, when `ψ(1, 0) ≠ 0` (e.g. `ψ` monic). -/
theorem fibInf_simple_slope (N ψ : MvPolynomial (Fin 2) K)
    (hψ : MvPolynomial.eval ![1, 0] ψ ≠ 0) :
    (Polynomial.derivative
      (lineR (fibreL ⟨N, MvPolynomial.X 1 * ψ ^ 2⟩ 1 0) ![1, 0] ![0, 1])).eval 0 ≠ 0 := by
  have e : (Polynomial.derivative
      (lineR (fibreL ⟨N, MvPolynomial.X 1 * ψ ^ 2⟩ 1 0) ![1, 0] ![0, 1])).eval 0
        = -(MvPolynomial.eval ![1, 0] ψ) ^ 2 := by
    rw [lineR_fibInf]
    simp [Polynomial.derivative_mul, Polynomial.derivative_pow, eval0_lineR] <;> ring
  rw [e]
  exact neg_ne_zero.mpr (pow_ne_zero 2 hψ)

/-- **Core theorem.** Let a map of `ℙ¹` have denominator form `Z·ψ²`, where
`ψ(1, 0) ≠ 0` and `ψ` has a root `(r : 1)`. If `ℓ ≥ 2` is a unit in `K`, no
Möbius coordinates make the map monomial of degree `ℓ`. -/
theorem not_monomial_of_sq_denominator {ℓ : ℕ} (hℓ : 2 ≤ ℓ) (hℓK : (ℓ : K) ≠ 0)
    (N ψ : MvPolynomial (Fin 2) K) (r : K)
    (hψinf : MvPolynomial.eval ![1, 0] ψ ≠ 0) (hψr : MvPolynomial.eval ![r, 1] ψ = 0)
    (m m' : Mob K) (hm : m.det ≠ 0) (hm' : m'.det ≠ 0) :
    ¬ IsMonomialL ℓ (conjL ⟨N, MvPolynomial.X 1 * ψ ^ 2⟩ m m') := by
  intro hG
  obtain ⟨α, β, hB⟩ := monomial_fibre_binom hG (m'.a * 1 + m'.b * 0) (m'.c * 1 + m'.d * 0)
  have hfc := fibreL_conj (⟨N, MvPolynomial.X 1 * ψ ^ 2⟩ : LMap K) m m' 1 0
  rw [hB] at hfc
  have key : ∀ p q, lineR (binom ℓ α β) p q
      = Polynomial.C m'.det * lineR (fibreL ⟨N, MvPolynomial.X 1 * ψ ^ 2⟩ 1 0)
          (app m.d (-m.b) (-m.c) m.a p) (app m.d (-m.b) (-m.c) m.a q) := by
    intro p q
    rw [hfc, lineR_C_mul, lineR_subst]
  have hPapp := app_adj_inv m hm r 1
  have hRapp := app_adj_inv m hm 1 0
  have hQapp := app_adj_inv m hm 0 1
  apply binom_contra hℓ hℓK α β
    (P := ![m.det⁻¹ * (m.a * r + m.b * 1), m.det⁻¹ * (m.c * r + m.d * 1)])
    (R := ![m.det⁻¹ * (m.a * 1 + m.b * 0), m.det⁻¹ * (m.c * 1 + m.d * 0)])
    (Q₀ := ![m.det⁻¹ * (m.a * 0 + m.b * 1), m.det⁻¹ * (m.c * 0 + m.d * 1)])
  · intro h
    have h1 := congrFun hPapp 1
    rw [h] at h1
    simp [app] at h1
  · intro q
    rw [key, hPapp, Polynomial.derivative_C_mul, Polynomial.eval_mul, Polynomial.eval_C,
      fibInf_double N ψ _ _ hψr, mul_zero]
  · rw [key, hRapp, hQapp, Polynomial.eval_mul, Polynomial.eval_C, fibInf_simple_val, mul_zero]
  · rw [key, hRapp, hQapp, Polynomial.derivative_C_mul, Polynomial.eval_mul, Polynomial.eval_C]
    exact mul_ne_zero hm' (fibInf_simple_slope N ψ hψinf)

/-- **Main theorem (odd ℓ).** Let `ℓ ≥ 3` be a unit in `K`. Take a Vélu ℓ-isogeny
x-map in Kohel's form `x ↦ N(x)/ψ(x)²`, with kernel polynomial
`ψ = ∏_{x ∈ S} (x - x_Q)` having a `K`-rational root `r ∈ S` (true over `K̄`, or
whenever a kernel point has rational abscissa). Then no Möbius coordinates
`M, M'` make the step monomial (`u ↦ λu^{±ℓ}`). The numerator `N` is arbitrary. -/
theorem velu_odd_not_monomial {ℓ : ℕ} (hℓ : 3 ≤ ℓ) (hℓK : (ℓ : K) ≠ 0)
    (S : Finset K) (r : K) (hr : r ∈ S) (N : MvPolynomial (Fin 2) K)
    (m m' : Mob K) (hm : m.det ≠ 0) (hm' : m'.det ≠ 0) :
    ¬ IsMonomialL ℓ (conjL ⟨N, MvPolynomial.X 1 *
        (∏ x ∈ S, (MvPolynomial.X 0 - MvPolynomial.C x * MvPolynomial.X 1)) ^ 2⟩ m m') := by
  apply not_monomial_of_sq_denominator (by omega) hℓK N _ r _ _ m m' hm hm'
  · simp [map_prod]
  · rw [map_prod]
    exact Finset.prod_eq_zero hr (by simp)

/-- Positive control: `u ↦ u^ℓ` in the identity coordinates is monomial. -/
theorem power_isMonomial (ℓ : ℕ) :
    IsMonomialL ℓ (conjL (⟨MvPolynomial.X 0 ^ ℓ, MvPolynomial.X 1 ^ ℓ⟩ : LMap K) Mob.id Mob.id) := by
  refine ⟨1, 1, one_ne_zero, one_ne_zero, Or.inl ⟨?_, ?_⟩⟩ <;>
    simp [conjL, lsubst, Mob.id]

end VeluObstruction
