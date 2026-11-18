module
public import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
public import Mathlib.RingTheory.Valuation.ExtendToLocalization
public import Mathlib.Algebra.Order.GroupWithZero.Canonical
public import Mathlib.Tactic

@[expose] public section
namespace MillerNormValuationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open scoped WithZero
variable {K : Type*} [Field K]

instance coordinateFinite (W : WeierstrassCurve.Affine K) : Module.Finite K[X] W.CoordinateRing :=
  Module.Finite.of_basis (CoordinateRing.basis W)

def degreeValue : WithBot ℕ → ℤᵐ⁰
  | none => 0
  | some n => WithZero.exp (n : ℤ)

theorem degreeValue_mono : Monotone degreeValue := by
  intro a b h
  cases a <;> cases b <;> simp_all [degreeValue]

theorem degreeValue_add (a b : WithBot ℕ) :
    degreeValue (a + b) = degreeValue a * degreeValue b := by
  cases a with
  | bot => simp [degreeValue]
  | coe a =>
    cases b with
    | bot => simp [degreeValue]
    | coe b =>
      rw [← WithBot.coe_add]
      change WithZero.exp ((a + b : ℕ) : ℤ) = WithZero.exp (a : ℤ) * WithZero.exp (b : ℤ)
      rw [Nat.cast_add, WithZero.exp_add]

theorem degreeValue_max (a b : WithBot ℕ) :
    degreeValue (max a b) = max (degreeValue a) (degreeValue b) := by
  rcases le_total a b with h | h
  · rw [max_eq_right h, max_eq_right (degreeValue_mono h)]
  · rw [max_eq_left h, max_eq_left (degreeValue_mono h)]

theorem degreeValue_zero_iff (a : WithBot ℕ) : degreeValue a = 0 ↔ a = ⊥ := by
  cases a <;> simp [degreeValue]

def normDegree (W : WeierstrassCurve.Affine K) (f : W.CoordinateRing) : WithBot ℕ :=
  (Algebra.norm K[X] f).degree

theorem norm_degree_basis (W : WeierstrassCurve.Affine K) (p q : K[X]) :
    normDegree W (p • 1 + q • mk W X) = max (2 • p.degree) (2 • q.degree + 3) :=
  degree_norm_smul_basis p q

theorem norm_degree_zero (W : WeierstrassCurve.Affine K) : normDegree W 0 = ⊥ := by
  simp [normDegree]

theorem norm_degree_one (W : WeierstrassCurve.Affine K) : normDegree W 1 = 0 := by
  simp [normDegree]

theorem norm_degree_mul (W : WeierstrassCurve.Affine K) (f g : W.CoordinateRing) :
    normDegree W (f * g) = normDegree W f + normDegree W g := by
  simp only [normDegree, map_mul, degree_mul]

theorem twice_degree_add_le (p r : K[X]) (d : WithBot ℕ) :
    2 • (p + r).degree + d ≤ max (2 • p.degree + d) (2 • r.degree + d) := by
  have h := nsmul_le_nsmul_right (degree_add_le p r) 2
  rcases le_total p.degree r.degree with hpr | hrp
  · rw [max_eq_right hpr] at h
    exact (add_le_add h (le_refl d)).trans (le_max_right _ _)
  · rw [max_eq_left hrp] at h
    exact (add_le_add h (le_refl d)).trans (le_max_left _ _)

/-- The actual rank-two norm degree satisfies the ultrametric triangle bound. -/
theorem norm_degree_add_le (W : WeierstrassCurve.Affine K) (f g : W.CoordinateRing) :
    normDegree W (f + g) ≤ max (normDegree W f) (normDegree W g) := by
  obtain ⟨p, q, rfl⟩ := exists_smul_basis_eq f
  obtain ⟨r, s, rfl⟩ := exists_smul_basis_eq g
  have he : (p • (1 : W.CoordinateRing) + q • mk W X) + (r • 1 + s • mk W X) =
      (p + r) • 1 + (q + s) • mk W X := by
    simp only [add_smul]
    abel
  rw [he, norm_degree_basis, norm_degree_basis, norm_degree_basis]
  have hp : 2 • (p + r).degree ≤ max (2 • p.degree) (2 • r.degree) := by
    simpa only [add_zero] using twice_degree_add_le p r 0
  have hq := twice_degree_add_le q s 3
  apply max_le
  · apply hp.trans
    exact max_le ((le_max_left _ _).trans (le_max_left _ _))
      ((le_max_left _ _).trans (le_max_right _ _))
  · apply hq.trans
    exact max_le ((le_max_right _ _).trans (le_max_left _ _))
      ((le_max_right _ _).trans (le_max_right _ _))

theorem norm_degree_bot_iff (W : WeierstrassCurve.Affine K) (f : W.CoordinateRing) :
    normDegree W f = ⊥ ↔ f = 0 := by
  rw [normDegree, degree_eq_bot, Algebra.norm_eq_zero_iff]

/-- An actual valuation of the actual affine coordinate ring, constructed from its algebra norm. -/
def coordinateValuation (W : WeierstrassCurve.Affine K) : Valuation W.CoordinateRing ℤᵐ⁰ where
  toFun f := degreeValue (normDegree W f)
  map_zero' := by rw [norm_degree_zero]; rfl
  map_one' := by rw [norm_degree_one]; rfl
  map_mul' f g := by rw [norm_degree_mul, degreeValue_add]
  map_add_le_max' f g := by
    exact (degreeValue_mono (norm_degree_add_le W f g)).trans_eq (degreeValue_max _ _)

theorem coordinate_valuation_zero_iff (W : WeierstrassCurve.Affine K) (f : W.CoordinateRing) :
    coordinateValuation W f = 0 ↔ f = 0 := by
  change degreeValue (normDegree W f) = 0 ↔ f = 0
  rw [degreeValue_zero_iff, norm_degree_bot_iff]

#print axioms norm_degree_add_le
#print axioms coordinate_valuation_zero_iff
end
end MillerNormValuationAudit
