import SplitFactorBase

noncomputable section
open scoped BigOperators
open WeierstrassCurve
namespace PencilSelector
variable {K : Type*} [Field K] {k : ℕ}

def Boolean (u : Fin k → K) : Prop := ∀ i, u i = 0 ∨ u i = 1

theorem boolean_iff_bool (u : Fin k → K) :
    Boolean u ↔ ∃ bits : Fin k → Bool, ∀ i, u i = if bits i then 1 else 0 := by
  classical
  constructor
  · intro hu
    refine ⟨fun i => decide (u i = 1), ?_⟩
    intro i
    rcases hu i with hi | hi <;> simp [hi]
  · rintro ⟨bits, hbits⟩ i
    rw [hbits i]
    cases bits i <;> simp

theorem boolean_iff_equations (u : Fin k → K) :
    Boolean u ↔ ∀ i, u i^2-u i = 0 := by
  simp only [Boolean]
  apply forall_congr'
  intro i
  constructor
  · rintro (h | h) <;> simp [h]
  · intro h
    have hz : u i*(u i-1) = 0 := by linear_combination h
    rcases mul_eq_zero.mp hz with hz | hz
    · exact Or.inl hz
    · exact Or.inr (sub_eq_zero.mp hz)

def offset (t0 : K) (w u : Fin k → K) : K := t0+∑ i, w i*u i
def graph (a b c t0 : K) (w u : Fin k → K) (x : K) : K :=
  a*x^2+b*x+c+offset t0 w u

/-- Exact finite-label membership; rational curve membership is separate. -/
def InBase (a b c t0 : K) (w : Fin k → K) (x y : K) : Prop :=
  ∃ e : K, ∃ u : Fin k → K, (e = 1 ∨ e = -1) ∧ Boolean u ∧
    y = e*graph a b c t0 w u x

def System (A a b c t0 r s e f x z m : K) (w u v : Fin k → K) : Prop :=
  Boolean u ∧ Boolean v ∧
    SecantGroupLaw.System A r s x z
      (e*graph a b c t0 w u x) (f*graph a b c t0 w v z) m

variable (A B : K)
local notation "W" => WeierstrassCurve.toAffine (SecantGroupLaw.shortCurve A B)

/-- Raw roots produce actual nonsingular base points. No full splitting,
    assumed root oracle, or discarded field/sign branch is needed. -/
theorem system_implies_base_relation (a b c t0 : K) (w u v : Fin k → K)
    {r s e f x z m : K} (he : e = 1 ∨ e = -1) (hf : f = 1 ∨ f = -1)
    (hΔ : (W).Δ ≠ 0) (hR : (W).Nonsingular r s)
    (h : System A a b c t0 r s e f x z m w u v) :
    ∃ (y q : K), InBase a b c t0 w x y ∧ InBase a b c t0 w z q ∧
      ∃ (hP : (W).Nonsingular x y) (hQ : (W).Nonsingular z q),
        Affine.Point.some hP + Affine.Point.some hQ = Affine.Point.some hR := by
  obtain ⟨hu, hv, hs⟩ := h
  obtain ⟨hy, hq⟩ := SecantGroupLaw.system_ordinates A hs
  have hcurve : s^2-r^3-A*r-B = 0 := by
    have hh := (SecantGroupLaw.equation_iff A B r s).mp hR.1
    linear_combination hh
  have hpy := SecantIdentity.selected_point_on_curve A B r s x z m
    (e*graph a b c t0 w u x) hcurve hs.1 hs.2.1 hy
  have hsum' : m^2-z-x-r = 0 := by linear_combination hs.1
  have hprod' : z*x-r^2-r*m^2-A-2*m*s = 0 := by
    simpa only [mul_comm z x] using hs.2.1
  have hqz := SecantIdentity.selected_point_on_curve A B r s z x m
    (f*graph a b c t0 w v z) hcurve hsum' hprod' hq
  have hP := SplitFactorBase.nonsingular_of_equation A B hΔ hpy
  have hQ := SplitFactorBase.nonsingular_of_equation A B hΔ hqz
  exact ⟨_, _, ⟨e, u, he, hu, rfl⟩, ⟨f, v, hf, hv, rfl⟩,
    hP, hQ, SecantGroupLaw.system_implies_add A B hP hQ hR hs⟩

/-- All labels in both columns stay variable, and all signs and lifts are
    retained. This includes diagonal and repeated-target-abscissa cases. -/
theorem complete_selector_iff (a b c t0 : K) (w : Fin k → K)
    {r s x z y q : K} (hP : (W).Nonsingular x y)
    (hQ : (W).Nonsingular z q) (hR : (W).Nonsingular r s) :
    (InBase a b c t0 w x y ∧ InBase a b c t0 w z q ∧
      Affine.Point.some hP + Affine.Point.some hQ = Affine.Point.some hR) ↔
      ∃ e f m : K, ∃ u v : Fin k → K,
        (e = 1 ∨ e = -1) ∧ (f = 1 ∨ f = -1) ∧
        y = e*graph a b c t0 w u x ∧ q = f*graph a b c t0 w v z ∧
        System A a b c t0 r s e f x z m w u v := by
  constructor
  · rintro ⟨⟨e, u, he, hu, hy⟩, ⟨f, v, hf, hv, hq⟩, hadd⟩
    obtain ⟨m, hm⟩ := SecantGroupLaw.add_implies_system A B hP hQ hR hadd
    refine ⟨e, f, m, u, v, he, hf, hy, hq, hu, hv, ?_⟩
    rw [← hy, ← hq]
    exact hm
  · rintro ⟨e, f, m, u, v, he, hf, hy, hq, hu, hv, hm⟩
    refine ⟨⟨e, u, he, hu, hy⟩, ⟨f, v, hf, hv, hq⟩, ?_⟩
    rw [← hy, ← hq] at hm
    exact SecantGroupLaw.system_implies_add A B hP hQ hR hm

theorem identity_target_iff {x z y q : K}
    (hP : (W).Nonsingular x y) (hQ : (W).Nonsingular z q) :
    Affine.Point.some hP + Affine.Point.some hQ = 0 ↔ x = z ∧ y = -q :=
  SecantGroupLaw.identity_target_iff A B hP hQ

end PencilSelector

#print axioms PencilSelector.boolean_iff_equations
#print axioms PencilSelector.system_implies_base_relation
#print axioms PencilSelector.complete_selector_iff
#print axioms PencilSelector.identity_target_iff
