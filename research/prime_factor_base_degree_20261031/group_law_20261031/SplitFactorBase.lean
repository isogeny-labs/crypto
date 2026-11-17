import SecantGroupLaw

noncomputable section
open scoped BigOperators
open WeierstrassCurve
namespace SplitFactorBase
variable {K : Type*} [Field K]

def cubic (A B : K) : Polynomial K :=
  Polynomial.X^3+Polynomial.C A*Polynomial.X+Polynomial.C B

/-- An actual finite complete-splitting identity, not a root-membership oracle. -/
def CompleteSplit (A B : K) (h : Polynomial K) (V : Finset K) (a : K) : Prop :=
  h^2-cubic A B = Polynomial.C (a^2)*∏ t ∈ V, (Polynomial.X-Polynomial.C t)

variable (A B : K)
local notation "W" => WeierstrassCurve.toAffine (SecantGroupLaw.shortCurve A B)

theorem graph_intersection_iff (h : Polynomial K) (V : Finset K) (a : K) (ha : a ≠ 0)
    (hsplit : CompleteSplit A B h V a) (t : K) :
    h.eval t^2 = t^3+A*t+B ↔ t ∈ V := by
  classical
  have he := congrArg (Polynomial.eval t) hsplit
  simp only [cubic, Polynomial.eval_sub, Polynomial.eval_pow, Polynomial.eval_add,
    Polynomial.eval_mul, Polynomial.eval_X, Polynomial.eval_C, Polynomial.eval_prod] at he
  constructor
  · intro ht
    have hz : a^2*∏ v ∈ V, (t-v) = 0 := by rw [← he, ht, sub_self]
    have hp : (∏ v ∈ V, (t-v)) = 0 :=
      (mul_eq_zero.mp hz).resolve_left (pow_ne_zero _ ha)
    obtain ⟨v, hv, hv0⟩ := Finset.prod_eq_zero_iff.mp hp
    have htv : t = v := sub_eq_zero.mp hv0
    simpa only [htv] using hv
  · intro ht
    have hp : (∏ v ∈ V, (t-v)) = 0 := Finset.prod_eq_zero ht (by simp)
    have hz : h.eval t^2-(t^3+A*t+B) = 0 := he.trans (by rw [hp, mul_zero])
    exact sub_eq_zero.mp hz

theorem every_lift_is_signed (h : Polynomial K) (V : Finset K) (a : K) (ha : a ≠ 0)
    (hsplit : CompleteSplit A B h V a) {x y : K} (hx : x ∈ V)
    (hy : y^2 = x^3+A*x+B) :
    ∃ e : K, (e = 1 ∨ e = -1) ∧ y = e*h.eval x := by
  have hh := (graph_intersection_iff A B h V a ha hsplit x).mpr hx
  have hf : (y-h.eval x)*(y+h.eval x) = 0 := by linear_combination hy-hh
  rcases mul_eq_zero.mp hf with hp | hn
  · refine ⟨1, Or.inl rfl, ?_⟩
    simpa only [one_mul] using sub_eq_zero.mp hp
  · refine ⟨-1, Or.inr rfl, ?_⟩
    simpa only [neg_one_mul] using add_eq_zero_iff_eq_neg.mp hn

theorem nonsingular_of_equation (hΔ : (W).Δ ≠ 0) {x y : K}
    (heq : y^2 = x^3+A*x+B) : (W).Nonsingular x y :=
  (Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp
    ((SecantGroupLaw.equation_iff A B x y).mpr heq)

/-- Every raw solver root is an actual base-pair relation, and every actual
    signed base-pair relation yields a root. No membership/sign filter is lost. -/
theorem selector_iff_group_relation (h : Polynomial K) (V : Finset K) (a : K)
    (ha : a ≠ 0) (hsplit : CompleteSplit A B h V a) (hΔ : (W).Δ ≠ 0)
    {r s : K} (hR : (W).Nonsingular r s) (x z e f : K)
    (he : e = 1 ∨ e = -1) (hf : f = 1 ∨ f = -1) :
    (∃ m, SecantGroupLaw.System A r s x z (e*h.eval x) (f*h.eval z) m) ↔
      x ∈ V ∧ z ∈ V ∧
        ∃ (hP : (W).Nonsingular x (e*h.eval x)) (hQ : (W).Nonsingular z (f*h.eval z)),
          Affine.Point.some hP + Affine.Point.some hQ = Affine.Point.some hR := by
  constructor
  · rintro ⟨m, hm⟩
    obtain ⟨hy, hv⟩ := SecantGroupLaw.system_ordinates A hm
    have hcurve : s^2-r^3-A*r-B = 0 := by
      have hh := (SecantGroupLaw.equation_iff A B r s).mp hR.1
      linear_combination hh
    have hpy := SecantIdentity.selected_point_on_curve A B r s x z m
      (e*h.eval x) hcurve hm.1 hm.2.1 hy
    have hsum' : m^2-z-x-r = 0 := by linear_combination hm.1
    have hprod' : z*x-r^2-r*m^2-A-2*m*s = 0 := by
      simpa only [mul_comm z x] using hm.2.1
    have hqv := SecantIdentity.selected_point_on_curve A B r s z x m
      (f*h.eval z) hcurve hsum' hprod' hv
    have hgx : h.eval x^2 = x^3+A*x+B := by
      rcases he with rfl | rfl <;> simpa using hpy
    have hgz : h.eval z^2 = z^3+A*z+B := by
      rcases hf with rfl | rfl <;> simpa using hqv
    have hx := (graph_intersection_iff A B h V a ha hsplit x).mp hgx
    have hz := (graph_intersection_iff A B h V a ha hsplit z).mp hgz
    have hP := nonsingular_of_equation A B hΔ hpy
    have hQ := nonsingular_of_equation A B hΔ hqv
    exact ⟨hx, hz, hP, hQ, SecantGroupLaw.system_implies_add A B hP hQ hR hm⟩
  · rintro ⟨_, _, hP, hQ, hadd⟩
    exact SecantGroupLaw.add_implies_system A B hP hQ hR hadd

/-- All ordinate lifts are covered by the union of the four sign branches,
    including repeated points and any duplicate signs at two-torsion. -/
theorem all_four_branches_complete (h : Polynomial K) (V : Finset K) (a : K)
    (ha : a ≠ 0) (hsplit : CompleteSplit A B h V a)
    {r s x z y v : K} (hx : x ∈ V) (hz : z ∈ V)
    (hP : (W).Nonsingular x y) (hQ : (W).Nonsingular z v) (hR : (W).Nonsingular r s) :
    (Affine.Point.some hP + Affine.Point.some hQ = Affine.Point.some hR) ↔
      ∃ e f m : K, (e = 1 ∨ e = -1) ∧ (f = 1 ∨ f = -1) ∧
        y = e*h.eval x ∧ v = f*h.eval z ∧
        SecantGroupLaw.System A r s x z (e*h.eval x) (f*h.eval z) m := by
  constructor
  · intro hadd
    obtain ⟨e, he, hy⟩ := every_lift_is_signed A B h V a ha hsplit hx
      ((SecantGroupLaw.equation_iff A B x y).mp hP.1)
    obtain ⟨f, hf, hv⟩ := every_lift_is_signed A B h V a ha hsplit hz
      ((SecantGroupLaw.equation_iff A B z v).mp hQ.1)
    obtain ⟨m, hm⟩ := SecantGroupLaw.add_implies_system A B hP hQ hR hadd
    refine ⟨e, f, m, he, hf, hy, hv, ?_⟩
    rw [← hy, ← hv]
    exact hm
  · rintro ⟨e, f, m, _, _, hy, hv, hm⟩
    rw [← hy, ← hv] at hm
    exact SecantGroupLaw.system_implies_add A B hP hQ hR hm

/-- Identity relations are exactly inverse pairs, never an omitted finite
    branch of the selector. -/
theorem identity_signed_iff (h : Polynomial K) {x z e f : K}
    (hP : (W).Nonsingular x (e*h.eval x)) (hQ : (W).Nonsingular z (f*h.eval z)) :
    Affine.Point.some hP + Affine.Point.some hQ = 0 ↔
      x = z ∧ e*h.eval x = -(f*h.eval z) :=
  SecantGroupLaw.identity_target_iff A B hP hQ

end SplitFactorBase

#print axioms SplitFactorBase.graph_intersection_iff
#print axioms SplitFactorBase.selector_iff_group_relation
#print axioms SplitFactorBase.all_four_branches_complete
#print axioms SplitFactorBase.identity_signed_iff
