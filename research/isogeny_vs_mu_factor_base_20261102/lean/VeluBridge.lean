import Mathlib
import VeluObstruction
import VeluObstructionOdd

/-!
# Bridges: Vélu's formulas against Mathlib's curves, and density 1 ⇒ monomial

The obstruction theorems (`VeluObstruction.lean`, `VeluObstructionOdd.lean`)
take the shape of a Vélu step as given. This file connects that shape to
the actual objects as far as Mathlib allows.

**(1) ℓ = 2, against Mathlib's Weierstrass curves and group law.**
* `velu2_maps_to_codomain`: for `x ≠ x₀`, Vélu's map
  `(x, y) ↦ (x + v/(x-x₀), y - v·y/(x-x₀)²)` sends an affine point of
  `E : y² = x³ + Ax + B`, in Mathlib's `Equation` predicate, to an affine point
  of `E' : y² = x³ + (A-5v)x + (B-7x₀v)`.
* `velu2_x_kernel_invariant`: the x-map is constant on cosets of the kernel
  `⟨T⟩`, `T = (x₀, 0)`. The `x` of `P + T`, computed with Mathlib's chord
  formula `addX ∘ slope`, has the same image as `x(P)`.
* `velu_dmap_eval`: the pair of forms `velu x₀ v` used by the obstruction
  theorem evaluates to this x-map.

**(2) Odd ℓ.** `veluOddX_eq_div`: Vélu's sum
`x + ∑_{Q∈S} ((6x_Q²+2A)/(x-x_Q) + 4y_Q²/(x-x_Q)²)` is a polynomial divided by
`ψ(x)²`, where `ψ = ∏ (x - x_Q)` is the kernel polynomial. This replaces the
"Kohel form" assumption of `velu_odd_not_monomial` with Vélu's own sum, at the
level of the denominator.

**(3) Density 1 ⇒ monomial (one step).** For a step `u'·(d₁u + d₀) = n₂u² + n₁u + n₀`
with leading monomial `u²` (`n₂ ≠ 0`), reducing `u²` gives
`(d₁/n₂)·u·u' + (d₀/n₂)·u' - (n₁/n₂)·u - n₀/n₂`. This normal form is a single term
exactly when at most one of `d₁, d₀, n₁, n₀` is nonzero.
`density_one_step_monomial` shows that this, together with the step having
degree 2 (nonzero resultant of `N` and `D`), forces the step to be the monomial
map `u' = λu²`.

Not covered here:
* That Vélu's map is a group homomorphism (an isogeny). Mathlib has no
  isogenies, and this file proves only the on-curve and kernel-invariance
  properties.
* That the normal-form computation above is the Gröbner reduction inside the
  whole chain algebra. This is argued in the protocol, not formalised.
* The degree of Vélu's numerator for odd ℓ.
-/

namespace VeluObstruction.Bridge

open WeierstrassCurve

variable {K : Type*} [Field K]

/-- The short Weierstrass curve `y² = x³ + A x + B` as a Mathlib curve. -/
def shortCurve (A B : K) : WeierstrassCurve K := ⟨0, 0, 0, A, B⟩

theorem short_equation_iff (A B x y : K) :
    (shortCurve A B).toAffine.Equation x y ↔ y ^ 2 = x ^ 3 + A * x + B := by
  simp [Affine.equation_iff, shortCurve]

/-- Vélu's 2-isogeny x-map with kernel abscissa `x₀`, `v = 3x₀² + A`. -/
def veluX (x₀ v x : K) : K := x + v / (x - x₀)

/-- Vélu's 2-isogeny y-map. -/
def veluY (x₀ v x y : K) : K := y - v * y / (x - x₀) ^ 2

/-- The forms used by the obstruction theorem evaluate to `veluX`. -/
theorem velu_dmap_eval (x₀ v x : K) (hx : x ≠ x₀) :
    ((velu x₀ v).N.A * x ^ 2 + (velu x₀ v).N.B * x + (velu x₀ v).N.C)
      / ((velu x₀ v).D.A * x ^ 2 + (velu x₀ v).D.B * x + (velu x₀ v).D.C) = veluX x₀ v x := by
  have ht : x - x₀ ≠ 0 := sub_ne_zero.mpr hx
  simp only [velu, veluX]
  rw [show (0 : K) * x ^ 2 + 1 * x + -x₀ = x - x₀ by ring, div_eq_iff ht, add_mul,
    div_mul_cancel₀ _ ht]
  ring

/-- **Vélu's 2-isogeny maps `E` to `E'`.** -/
theorem velu2_maps_to_codomain (A B x₀ x y : K) (hroot : x₀ ^ 3 + A * x₀ + B = 0)
    (hx : x ≠ x₀) (hP : (shortCurve A B).toAffine.Equation x y) :
    (shortCurve (A - 5 * (3 * x₀ ^ 2 + A)) (B - 7 * x₀ * (3 * x₀ ^ 2 + A))).toAffine.Equation
      (veluX x₀ (3 * x₀ ^ 2 + A) x) (veluY x₀ (3 * x₀ ^ 2 + A) x y) := by
  rw [short_equation_iff] at hP ⊢
  have ht : x - x₀ ≠ 0 := sub_ne_zero.mpr hx
  have key : y ^ 2 * ((x - x₀) ^ 2 - (3 * x₀ ^ 2 + A)) ^ 2
      = (x - x₀) * ((x * (x - x₀) + (3 * x₀ ^ 2 + A)) ^ 3
          + (A - 5 * (3 * x₀ ^ 2 + A)) * (x * (x - x₀) + (3 * x₀ ^ 2 + A)) * (x - x₀) ^ 2
          + (B - 7 * x₀ * (3 * x₀ ^ 2 + A)) * (x - x₀) ^ 3) := by
    linear_combination ((x - x₀) ^ 2 - (3 * x₀ ^ 2 + A)) ^ 2 * hP
      + (((x - x₀) ^ 2 - (3 * x₀ ^ 2 + A)) ^ 2 - (x - x₀) ^ 4) * hroot
  have lhs : veluY x₀ (3 * x₀ ^ 2 + A) x y ^ 2
      = y ^ 2 * ((x - x₀) ^ 2 - (3 * x₀ ^ 2 + A)) ^ 2 / (x - x₀) ^ 4 := by
    simp only [veluY]; field_simp <;> ring
  have rhs : veluX x₀ (3 * x₀ ^ 2 + A) x ^ 3
        + (A - 5 * (3 * x₀ ^ 2 + A)) * veluX x₀ (3 * x₀ ^ 2 + A) x
        + (B - 7 * x₀ * (3 * x₀ ^ 2 + A))
      = (x - x₀) * ((x * (x - x₀) + (3 * x₀ ^ 2 + A)) ^ 3
          + (A - 5 * (3 * x₀ ^ 2 + A)) * (x * (x - x₀) + (3 * x₀ ^ 2 + A)) * (x - x₀) ^ 2
          + (B - 7 * x₀ * (3 * x₀ ^ 2 + A)) * (x - x₀) ^ 3) / (x - x₀) ^ 4 := by
    simp only [veluX]; field_simp <;> ring
  rw [lhs, rhs, key]

/-- **The x-map is invariant under translation by the kernel point `T = (x₀, 0)`.**
`x(P + T)` is computed with Mathlib's chord formula. -/
theorem velu2_x_kernel_invariant [DecidableEq K] (A B x₀ x y : K)
    (hroot : x₀ ^ 3 + A * x₀ + B = 0) (hv : 3 * x₀ ^ 2 + A ≠ 0) (hx : x ≠ x₀)
    (hP : (shortCurve A B).toAffine.Equation x y) :
    veluX x₀ (3 * x₀ ^ 2 + A)
        ((shortCurve A B).toAffine.addX x x₀ ((shortCurve A B).toAffine.slope x x₀ y 0))
      = veluX x₀ (3 * x₀ ^ 2 + A) x := by
  rw [short_equation_iff] at hP
  have ht : x - x₀ ≠ 0 := sub_ne_zero.mpr hx
  rw [Affine.slope_of_X_ne hx]
  obtain ⟨l, hl⟩ : ∃ l : K, l = (y - 0) / (x - x₀) := ⟨_, rfl⟩
  rw [← hl]
  have hlt : l * (x - x₀) = y := by rw [hl, sub_zero, div_mul_cancel₀ y ht]
  have hadd : (shortCurve A B).toAffine.addX x x₀ l = l ^ 2 - x - x₀ := by
    simp [Affine.addX, shortCurve]
  rw [hadd]
  have e : (l ^ 2 - x - x₀ - x₀) * (x - x₀) = 3 * x₀ ^ 2 + A := by
    have h0 : ((l ^ 2 - x - x₀ - x₀) * (x - x₀) - (3 * x₀ ^ 2 + A)) * (x - x₀) = 0 := by
      linear_combination (l * (x - x₀) + y) * hlt + hP + hroot
    rcases mul_eq_zero.mp h0 with h | h
    · linear_combination h
    · exact absurd h ht
  have hne : l ^ 2 - x - x₀ - x₀ ≠ 0 := by
    intro h; rw [h, zero_mul] at e; exact hv e.symm
  have h3 : (3 * x₀ ^ 2 + A) / (l ^ 2 - x - x₀ - x₀) = x - x₀ := by
    rw [div_eq_iff hne]; linear_combination -e
  have h4 : (3 * x₀ ^ 2 + A) / (x - x₀) = l ^ 2 - x - x₀ - x₀ := by
    rw [div_eq_iff ht]; linear_combination -e
  simp only [veluX]
  rw [h3, h4]
  ring

/-- Vélu's odd-degree x-map:
`x + ∑_{q ∈ S} ((6q² + 2A)/(x - q) + 4(q³ + Aq + B)/(x - q)²)`. Here `S` holds
the abscissas of one point from each `±` pair of nonzero kernel points, and
`4(q³ + Aq + B) = 4y_Q²`. -/
def veluOddX (A B : K) (S : Finset K) (x : K) : K :=
  x + ∑ q ∈ S, ((6 * q ^ 2 + 2 * A) / (x - q) + 4 * (q ^ 3 + A * q + B) / (x - q) ^ 2)

/-- Its numerator over the kernel polynomial squared. -/
noncomputable def veluOddN (A B : K) (S : Finset K) : Polynomial K :=
  Polynomial.X * ∏ q ∈ S, (Polynomial.X - Polynomial.C q) ^ 2
    + ∑ q ∈ S, (Polynomial.C (6 * q ^ 2 + 2 * A) * (Polynomial.X - Polynomial.C q)
        + Polynomial.C (4 * (q ^ 3 + A * q + B)))
        * ∏ q' ∈ S.erase q, (Polynomial.X - Polynomial.C q') ^ 2

/-- **Vélu's odd-degree x-map is `N/ψ²`**, with `ψ = ∏_{q ∈ S} (X - q)` the
kernel polynomial (away from its roots). -/
theorem veluOddX_eq_div (A B : K) (S : Finset K) (x : K) (hx : ∀ q ∈ S, x ≠ q) :
    veluOddX A B S x
      = (veluOddN A B S).eval x
          / ((∏ q ∈ S, (Polynomial.X - Polynomial.C q)).eval x) ^ 2 := by
  have hq : ∀ q ∈ S, x - q ≠ 0 := fun q h => sub_ne_zero.mpr (hx q h)
  have hP : (∏ q ∈ S, (x - q)) ≠ 0 := Finset.prod_ne_zero_iff.mpr hq
  simp only [veluOddN, veluOddX, Polynomial.eval_add, Polynomial.eval_mul, Polynomial.eval_X,
    Polynomial.eval_prod, Polynomial.eval_finset_sum, Polynomial.eval_pow, Polynomial.eval_sub,
    Polynomial.eval_C]
  rw [add_div, Finset.sum_div]
  congr 1
  · rw [Finset.prod_pow, mul_div_assoc, div_self (pow_ne_zero 2 hP), mul_one]
  · apply Finset.sum_congr rfl
    intro q hqS
    have h1 : x - q ≠ 0 := hq q hqS
    have hE : ∏ q' ∈ S.erase q, (x - q') ≠ 0 :=
      Finset.prod_ne_zero_iff.mpr (fun q' h => hq q' (Finset.mem_of_mem_erase h))
    have hsplit : (x - q) * ∏ q' ∈ S.erase q, (x - q') = ∏ q' ∈ S, (x - q') :=
      Finset.mul_prod_erase S (fun q' => x - q') hqS
    rw [Finset.prod_pow, ← hsplit]
    field_simp <;> ring

/-- **Density 1 forces a monomial step.** Take a step `u'(d₁u + d₀) = n₂u² + n₁u + n₀`
of degree 2 (`n₂ ≠ 0` and nonzero resultant of `N` and `D`). If the normal form of
`u²` is a single term (at most one of `d₁, d₀, n₁, n₀` nonzero, written as
"all pairwise products vanish"), then `d₁ = n₁ = n₀ = 0` and `d₀ ≠ 0`. So the step
is `u' = (n₂/d₀)u²`, which is `IsMonomial`. -/
theorem density_one_step_monomial (n₂ n₁ n₀ d₁ d₀ : K) (hn₂ : n₂ ≠ 0)
    (hres : n₂ * d₀ ^ 2 - n₁ * d₀ * d₁ + n₀ * d₁ ^ 2 ≠ 0)
    (h10 : d₁ * d₀ = 0) (h1n1 : d₁ * n₁ = 0) (h1n0 : d₁ * n₀ = 0)
    (h0n1 : d₀ * n₁ = 0) (h0n0 : d₀ * n₀ = 0) (hn1n0 : n₁ * n₀ = 0) :
    d₁ = 0 ∧ n₁ = 0 ∧ n₀ = 0 ∧ d₀ ≠ 0 ∧
      IsMonomial (⟨⟨n₂, n₁, n₀⟩, ⟨0, d₁, d₀⟩⟩ : DMap K) := by
  by_cases hd₀ : d₀ = 0
  · exfalso
    apply hres
    rw [hd₀]
    have : n₀ * d₁ = 0 := by rw [mul_comm]; exact h1n0
    linear_combination d₁ * this
  · have hd₁ : d₁ = 0 := (mul_eq_zero.mp h10).resolve_right hd₀
    have hn₁ : n₁ = 0 := (mul_eq_zero.mp h0n1).resolve_left hd₀
    have hn₀ : n₀ = 0 := (mul_eq_zero.mp h0n0).resolve_left hd₀
    refine ⟨hd₁, hn₁, hn₀, hd₀, d₀, n₂ / d₀, hd₀, div_ne_zero hn₂ hd₀, Or.inl ⟨?_, ?_⟩⟩
    · ext
      · simp only; field_simp
      · simp only [hn₁]
      · simp only [hn₀]
    · ext
      · rfl
      · simp only [hd₁]
      · rfl

end VeluObstruction.Bridge
