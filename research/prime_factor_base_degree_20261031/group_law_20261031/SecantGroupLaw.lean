import Mathlib.AlgebraicGeometry.EllipticCurve.Group
import Mathlib.Tactic.LinearCombination
import SecantIdentity

/-! Exact equivalence with Mathlib's actual nonsingular-point abelian group.
    Neither a slope equality nor the desired addition relation is assumed. -/
noncomputable section
namespace SecantGroupLaw
open WeierstrassCurve
variable {K : Type*} [Field K]

def shortCurve (A B : K) : WeierstrassCurve K := ⟨0, 0, 0, A, B⟩

variable (A B : K)
local notation "W" => WeierstrassCurve.toAffine (shortCurve A B)

@[simp] theorem equation_iff (x y : K) :
    (W).Equation x y ↔ y^2 = x^3+A*x+B := by
  simp [Affine.equation_iff, shortCurve]

@[simp] theorem nonsingular_iff (x y : K) :
    (W).Nonsingular x y ↔ y^2 = x^3+A*x+B ∧ (3*x^2+A ≠ 0 ∨ 2*y ≠ 0) := by
  rw [Affine.nonsingular_iff']
  rw [equation_iff]
  simp only [shortCurve, zero_mul, mul_zero, zero_add, add_zero, zero_sub, neg_ne_zero]

@[simp] theorem negY_eq (x y : K) : (W).negY x y = -y := by
  simp [Affine.negY, shortCurve]

/-- All four original equations, with no discarded branch or diagonal. -/
def System (r s x z y v m : K) : Prop :=
  m^2-x-z-r = 0 ∧ x*z-r^2-r*m^2-A-2*m*s = 0 ∧
    y-m*(x-r)+s = 0 ∧ v-m*(z-r)+s = 0

lemma system_ordinates {r s x z y v m : K} (h : System A r s x z y v m) :
    y = m*(x-r)-s ∧ v = m*(z-r)-s := by
  constructor
  · linear_combination h.2.2.1
  · linear_combination h.2.2.2

/-- A diagonal root enforces tangency, not just collinearity. -/
theorem diagonal_tangent {r s x y m : K}
    (hsum : m^2-x-x-r = 0) (hprod : x*x-r^2-r*m^2-A-2*m*s = 0)
    (hy : y = m*(x-r)-s) : 2*m*y = 3*x^2+A := by
  linear_combination (2*x-r)*hsum + hprod + 2*m*hy

/-- The supplied slope is the canonical secant/tangent slope. Vertical and
    zero-ordinate diagonal roots are excluded by actual nonsingularity. -/
theorem canonical_slope {r s x z y v m : K} (hP : (W).Nonsingular x y)
    (h : System A r s x z y v m) :
    ¬ (x = z ∧ y = (W).negY z v) ∧ (W).slope x z y v = m := by
  obtain ⟨hy, hv⟩ := system_ordinates A h
  by_cases hx : x = z
  · subst z
    have hyv : y = v := by rw [hy, hv]
    have ht := diagonal_tangent A h.1 h.2.1 hy
    have hy2 : 2*y ≠ 0 := by
      intro hz
      have hder : 3*x^2+A = 0 := by
        calc
          _ = 2*m*y := ht.symm
          _ = m*(2*y) := by ring
          _ = 0 := by rw [hz, mul_zero]
      rcases ((nonsingular_iff A B x y).mp hP).2 with hn | hn
      · exact hn hder
      · exact hn hz
    have hyneg : y ≠ -y := by
      intro heq
      apply hy2
      linear_combination heq
    have hn : y ≠ (W).negY x v := by
      intro hh
      apply hyneg
      simpa only [negY_eq, ← hyv] using hh
    refine ⟨fun hh => hn hh.2, ?_⟩
    rw [Affine.slope_of_Y_ne rfl hn]
    have hd : y-(-y) ≠ 0 := by simpa [sub_neg_eq_add, two_mul] using hy2
    simp only [Affine.negY, shortCurve, zero_mul, mul_zero, zero_add, add_zero, sub_zero]
    apply (div_eq_iff hd).mpr
    linear_combination -ht
  · refine ⟨fun hh => hx hh.1, ?_⟩
    rw [Affine.slope_of_X_ne hx]
    apply (div_eq_iff (sub_ne_zero.mpr hx)).mpr
    linear_combination hy-hv

/-- Soundness as actual elliptic-curve point addition, including tangents,
    x=r, z=r, and all-abscissae-equal flex configurations. -/
theorem system_implies_add {r s x z y v m : K}
    (hP : (W).Nonsingular x y) (hQ : (W).Nonsingular z v) (hR : (W).Nonsingular r s)
    (h : System A r s x z y v m) :
    Affine.Point.some hP + Affine.Point.some hQ = Affine.Point.some hR := by
  obtain ⟨hnv, hm⟩ := canonical_slope A B hP h
  obtain ⟨hy, _⟩ := system_ordinates A h
  have hxsum : (W).addX x z ((W).slope x z y v) = r := by
    rw [hm]
    simp only [Affine.addX, shortCurve, zero_mul, add_zero, sub_zero]
    linear_combination h.1
  have hysum : (W).addY x z y ((W).slope x z y v) = s := by
    rw [Affine.addY, Affine.negAddY, negY_eq, hxsum, hm]
    linear_combination -hy
  rw [Affine.Point.add_some hnv]
  simp only [Affine.Point.some.injEq]
  exact ⟨hxsum, hysum⟩

lemma slope_difference {x z y v : K} (hP : (W).Nonsingular x y) (hQ : (W).Nonsingular z v)
    (hnv : ¬ (x = z ∧ y = (W).negY z v)) :
    y-v = (W).slope x z y v*(x-z) := by
  by_cases hx : x = z
  · have hy : y = v := Affine.Y_eq_of_Y_ne hP.1 hQ.1 hx (fun hh => hnv ⟨hx, hh⟩)
    simp [hx, hy]
  · rw [Affine.slope_of_X_ne hx, div_mul_cancel₀ _ (sub_ne_zero.mpr hx)]

/-- Completeness: actual finite addition supplies a slope satisfying every
    residual. The g2 relation comes from the full cubic coefficient, with no
    division by a target-abscissa difference. -/
theorem add_implies_system {r s x z y v : K}
    (hP : (W).Nonsingular x y) (hQ : (W).Nonsingular z v) (hR : (W).Nonsingular r s)
    (hadd : Affine.Point.some hP + Affine.Point.some hQ = Affine.Point.some hR) :
    ∃ m, System A r s x z y v m := by
  have hnv : ¬ (x = z ∧ y = (W).negY z v) := by
    rintro ⟨hx, hy⟩
    have hz : Affine.Point.some hP + Affine.Point.some hQ = 0 :=
      Affine.Point.add_of_Y_eq hx hy
    exact Affine.Point.some_ne_zero hR (hadd.symm.trans hz)
  have hcoords := hadd
  rw [Affine.Point.add_some hnv, Affine.Point.some.injEq] at hcoords
  let m := (W).slope x z y v
  have hsum : m^2-x-z-r = 0 := by
    have hx := hcoords.1
    simp only [Affine.addX, shortCurve, zero_mul, add_zero, sub_zero] at hx
    change m^2-x-z = r at hx
    linear_combination hx
  have hy : y = m*(x-r)-s := by
    have hh := hcoords.2
    rw [Affine.addY, Affine.negAddY, negY_eq, hcoords.1] at hh
    change -(m*(r-x)+y) = s at hh
    linear_combination -hh
  have hv : v = m*(z-r)-s := by
    have hd := slope_difference A B hP hQ hnv
    change y-v = m*(x-z) at hd
    linear_combination hy-hd
  have hpoly := Affine.addPolynomial_slope hP.1 hQ.1 hnv
  rw [hcoords.1, Affine.addPolynomial_eq, neg_inj,
    Cubic.prod_X_sub_C_eq, Cubic.toPoly_injective] at hpoly
  have hc := congrArg Cubic.c hpoly
  simp only [shortCurve, zero_mul, mul_zero, zero_sub, zero_add, add_zero, sub_zero, neg_zero] at hc
  change 2*x*m^2+(-(2*y))*m+A = x*z+x*r+z*r at hc
  have hprod : x*z-r^2-r*m^2-A-2*m*s = 0 := by
    linear_combination -hc-2*m*hy+r*hsum
  refine ⟨m, hsum, hprod, ?_, ?_⟩
  · linear_combination hy
  · linear_combination hv

/-- The complete finite-target group-law equivalence. -/
theorem finite_target_iff {r s x z y v : K}
    (hP : (W).Nonsingular x y) (hQ : (W).Nonsingular z v) (hR : (W).Nonsingular r s) :
    (∃ m, System A r s x z y v m) ↔
      Affine.Point.some hP + Affine.Point.some hQ = Affine.Point.some hR := by
  constructor
  · rintro ⟨m, hm⟩
    exact system_implies_add A B hP hQ hR hm
  · exact add_implies_system A B hP hQ hR

/-- The identity target has exactly the inverse-pair relations, including
    possible two-torsion. It is separate from the finite-slope system. -/
theorem identity_target_iff {x z y v : K} (hP : (W).Nonsingular x y) (hQ : (W).Nonsingular z v) :
    Affine.Point.some hP + Affine.Point.some hQ = 0 ↔ x = z ∧ y = -v := by
  rw [add_eq_zero_iff_eq_neg, Affine.Point.neg_some, Affine.Point.some.injEq, negY_eq]

end SecantGroupLaw

#print axioms SecantGroupLaw.canonical_slope
#print axioms SecantGroupLaw.system_implies_add
#print axioms SecantGroupLaw.add_implies_system
#print axioms SecantGroupLaw.finite_target_iff
#print axioms SecantGroupLaw.identity_target_iff
