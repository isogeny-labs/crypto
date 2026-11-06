/-
  Lift-dense factor bases: the twisted-line theorem, formalised.

  Study `research/lift_dense_factor_base_20261027/`; report statements are
  mapped to these declarations in `PROOF_MAP.md`. Toolchain and Mathlib are
  the repository pins (leanprover/lean4:v4.34.1, Mathlib d13f23b7).
-/
import Mathlib

namespace LiftDense

open Algebra

section Tower

variable {K L M : Type*} [Field K] [Field L] [Field M]
  [Algebra K L] [Algebra L M] [Algebra K M] [IsScalarTower K L M]
  [FiniteDimensional K L] [FiniteDimensional L M]

/-- Trace tower for a scalar multiple: for `y ∈ L` and `u ∈ M`,
`Tr_{M/K}(y u) = Tr_{L/K}(y · Tr_{M/L}(u))`. (Report: Lemma 1.) -/
theorem trace_mul_algebraMap (y : L) (u : M) :
    trace K M (algebraMap L M y * u) = trace K L (y * trace L M u) := by
  rw [← Algebra.trace_trace (R := K) (S := L) (T := M)]
  congr 1
  rw [← Algebra.smul_def, LinearMap.map_smul, smul_eq_mul]

/-- Twisted-line theorem, trace form (Report: Theorem 2). If
`Tr_{M/L}(u) = 0` and `Tr_{M/L}(c u⁻¹) = 0`, then for every `y ∈ L` the
absolute trace of `x + a + c/x` at `x = y u` equals `Tr_{M/K}(a)`. (No
`y ≠ 0` hypothesis is needed: with `0⁻¹ = 0` the case `y = 0` also holds.) -/
theorem twisted_line_trace_const (u c a : M) (hu : trace L M u = 0)
    (hcu : trace L M (c * u⁻¹) = 0) (y : L) :
    trace K M (algebraMap L M y * u + a + c * (algebraMap L M y * u)⁻¹)
      = trace K M a := by
  have h1 : trace K M (algebraMap L M y * u) = 0 := by
    rw [trace_mul_algebraMap, hu, mul_zero, map_zero]
  have h2 : c * (algebraMap L M y * u)⁻¹ = algebraMap L M y⁻¹ * (c * u⁻¹) := by
    rw [mul_inv, map_inv₀]; ring
  have h3 : trace K M (c * (algebraMap L M y * u)⁻¹) = 0 := by
    rw [h2, trace_mul_algebraMap, hcu, mul_zero, map_zero]
  rw [map_add, map_add, h1, h3, zero_add, add_zero]

end Tower

section CharTwo

variable {M : Type*} [Field M] [Finite M] [Algebra (ZMod 2) M]

local notation "Tr" => Algebra.trace (ZMod 2) M

/-- The absolute trace is invariant under squaring (Frobenius).
(Report: obligation O2, first half.) -/
theorem trace_sq (z : M) : Tr (z ^ 2) = Tr z := by
  have h := Algebra.trace_eq_of_algEquiv
    (FiniteField.frobeniusAlgEquivOfAlgebraic (ZMod 2) M) z
  rw [FiniteField.coe_frobeniusAlgEquivOfAlgebraic, ZMod.card] at h
  exact h

omit [Finite M] in
theorem charTwo : CharP M 2 :=
  charP_of_injective_algebraMap (algebraMap (ZMod 2) M).injective 2

/-- `Tr(c²/x²) = Tr(c/x)`. (Report: obligation O2.) -/
theorem trace_sq_div (c x : M) : Tr (c ^ 2 / x ^ 2) = Tr (c / x) := by
  rw [← div_pow, trace_sq]

omit [Finite M] in
/-- The additive Artin–Schreier map `z ↦ z² + z`, as a `ZMod 2`-linear map. -/
noncomputable def asMap : M →ₗ[ZMod 2] M where
  toFun z := z ^ 2 + z
  map_add' a b := by
    have := charTwo (M := M)
    have : Fact (Nat.Prime 2) := ⟨Nat.prime_two⟩
    rw [add_pow_char a b]; ring
  map_smul' r z := by
    have hA : (algebraMap (ZMod 2) M r) ^ 2 = algebraMap (ZMod 2) M r := by
      rw [← map_pow, (by fin_cases r <;> rfl : r ^ 2 = r)]
    simp only [RingHom.id_apply, Algebra.smul_def, mul_pow]
    linear_combination z ^ 2 * hA

omit [Finite M] in
theorem asMap_apply (z : M) : asMap z = z ^ 2 + z := rfl

/-- Easy half of Artin–Schreier: an image of `z ↦ z² + z` has trace zero. -/
theorem trace_asMap (z : M) : Tr (asMap z) = 0 := by
  rw [asMap_apply, map_add, trace_sq]
  exact CharTwo.add_self_eq_zero _

omit [Finite M] in
/-- The kernel of `z ↦ z² + z` is `{0, 1}` = the image of `ZMod 2`. -/
theorem ker_asMap : LinearMap.ker (asMap (M := M)) = (ZMod 2) ∙ (1 : M) := by
  have := charTwo (M := M)
  ext z
  simp only [LinearMap.mem_ker, asMap_apply, Submodule.mem_span_singleton]
  constructor
  · intro h
    have h' : z * (z + 1) = 0 := by rw [← h]; ring
    rcases mul_eq_zero.mp h' with h0 | h1
    · exact ⟨0, by simp [h0]⟩
    · refine ⟨1, ?_⟩
      have : z = -1 := eq_neg_of_add_eq_zero_left h1
      rw [this, CharTwo.neg_eq, one_smul]
  · rintro ⟨r, rfl⟩
    have hA : (algebraMap (ZMod 2) M r) ^ 2 = algebraMap (ZMod 2) M r := by
      rw [← map_pow, (by fin_cases r <;> rfl : r ^ 2 = r)]
    rw [Algebra.smul_def, mul_one, hA]
    exact CharTwo.add_self_eq_zero _

/-- Artin–Schreier criterion in characteristic 2 (Report: obligation O1):
`z² + z = w` is solvable in `M` iff `Tr_{M/F₂}(w) = 0`. -/
theorem artin_schreier (w : M) : (∃ z : M, z ^ 2 + z = w) ↔ Tr w = 0 := by
  have := Fintype.ofFinite M
  constructor
  · rintro ⟨z, rfl⟩; exact trace_asMap z
  · intro hw
    -- range asMap ≤ ker Tr, and both have dimension n - 1
    have hle : LinearMap.range (asMap (M := M)) ≤ LinearMap.ker (Tr) := by
      rintro _ ⟨z, rfl⟩; exact trace_asMap z
    have hsurj : Function.Surjective (Tr) := by
      have hne : (Tr) ≠ 0 := Algebra.trace_ne_zero (ZMod 2) M
      obtain ⟨x, hx⟩ : ∃ x, Tr x ≠ 0 := by
        by_contra! h; exact hne (LinearMap.ext h)
      intro t
      refine ⟨(t * (Tr x)⁻¹) • x, ?_⟩
      rw [LinearMap.map_smul, smul_eq_mul, mul_assoc, inv_mul_cancel₀ hx, mul_one]
    have hr1 : Module.finrank (ZMod 2) (LinearMap.range (asMap (M := M)))
        + 1 = Module.finrank (ZMod 2) M := by
      have := LinearMap.finrank_range_add_finrank_ker (asMap (M := M))
      rwa [ker_asMap, finrank_span_singleton one_ne_zero] at this
    have hr2 : 1 + Module.finrank (ZMod 2) (LinearMap.ker (Tr))
        = Module.finrank (ZMod 2) M := by
      have := LinearMap.finrank_range_add_finrank_ker (Tr)
      rwa [LinearMap.range_eq_top.mpr hsurj, finrank_top, Module.finrank_self] at this
    have heq : LinearMap.range (asMap (M := M)) = LinearMap.ker (Tr) :=
      Submodule.eq_of_le_of_finrank_eq hle (by omega)
    have : w ∈ LinearMap.range (asMap (M := M)) := by
      rw [heq]; exact hw
    obtain ⟨z, hz⟩ := this
    exact ⟨z, hz⟩

/-- Lift criterion: for `x ≠ 0`, the abscissa `x` lifts to
`y² + x y = x³ + a x² + b` over `M` iff `Tr(x + a + b/x²) = 0`. -/
theorem lifts_iff (a b x : M) (hx : x ≠ 0) :
    (∃ y : M, y ^ 2 + x * y = x ^ 3 + a * x ^ 2 + b) ↔
      Tr (x + a + b / x ^ 2) = 0 := by
  rw [← artin_schreier]
  have hx2 : x ^ 2 ≠ 0 := pow_ne_zero 2 hx
  constructor
  · rintro ⟨y, hy⟩
    refine ⟨y / x, ?_⟩
    field_simp
    linear_combination hy
  · rintro ⟨z, hz⟩
    refine ⟨x * z, ?_⟩
    have := congrArg (· * x ^ 2) hz
    field_simp at this
    linear_combination this

/-- Twisted-line theorem, lift form (Report: Theorem 2). Let `L` be an
intermediate field, `b = c²`, and let `u` satisfy `Tr_{M/L}(u) = 0` and
`Tr_{M/L}(c u⁻¹) = 0`. Then for every nonzero `y ∈ L` the abscissa
`x = y u` lifts to `E : y² + x y = x³ + a x² + b` iff `Tr_{M/F₂}(a) = 0`;
in particular the whole line `u L` lifts when `Tr(a) = 0` and lies on the
twist when `Tr(a) = 1`. -/
theorem twisted_line_lifts {L : Type*} [Field L] [Algebra (ZMod 2) L] [Algebra L M]
    [IsScalarTower (ZMod 2) L M] [FiniteDimensional (ZMod 2) L]
    [FiniteDimensional L M]
    (u c a : M) (hu : trace L M u = 0) (hcu : trace L M (c * u⁻¹) = 0)
    (y : L) (hy : y ≠ 0) (huz : u ≠ 0) :
    (∃ Y : M, Y ^ 2 + (algebraMap L M y * u) * Y
        = (algebraMap L M y * u) ^ 3 + a * (algebraMap L M y * u) ^ 2 + c ^ 2)
      ↔ Tr a = 0 := by
  have hx : algebraMap L M y * u ≠ 0 :=
    mul_ne_zero ((map_ne_zero _).mpr hy) huz
  rw [lifts_iff a (c ^ 2) _ hx, map_add, map_add, trace_sq_div, ← map_add, ← map_add]
  have hdiv : c / (algebraMap L M y * u) = c * (algebraMap L M y * u)⁻¹ := div_eq_mul_inv _ _
  rw [hdiv, twisted_line_trace_const (K := ZMod 2) u c a hu hcu y]

end CharTwo

end LiftDense
