import FieldInterchange
import DegreeDropControl

namespace ResultantInterchangeAudit
open Polynomial
noncomputable section
variable {R : Type*} [CommRing R] [IsDomain R]

theorem fixed_interchange_domain (p : Polynomial R) (q : Polynomial (Polynomial R))
    (r : Polynomial R) (dx dy : ℕ) (hq : q.natDegree ≤ dy)
    (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx) :
    leftFixed p q r dx dy = rightFixed p q r dx dy := by
  let f : R →+* FractionRing R := algebraMap R (FractionRing R)
  have hf : Function.Injective f := IsFractionRing.injective R (FractionRing R)
  apply hf
  rw [← leftFixed_map f hf, ← rightFixed_map f hf]
  apply fixed_interchange
  · exact natDegree_map_le.trans hq
  · rw [swap_mapBiv]
    exact natDegree_map_le.trans hx

theorem innerY_degree_domain (p : Polynomial R) (q : Polynomial (Polynomial R))
    (dx dy : ℕ) (hq : q.natDegree ≤ dy)
    (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx) :
    (innerY p q dy).natDegree ≤ p.natDegree * dx := by
  let f : R →+* FractionRing R := algebraMap R (FractionRing R)
  have hf : Function.Injective f := IsFractionRing.injective R (FractionRing R)
  have h := innerY_degree_field (p.map f) (mapBiv f q) dx dy
    (natDegree_map_le.trans hq) (by rw [swap_mapBiv]; exact natDegree_map_le.trans hx)
  simpa only [innerY_map f hf, natDegree_map_eq_of_injective hf] using h

theorem innerX_degree_domain (q : Polynomial (Polynomial R)) (r : Polynomial R)
    (dx dy : ℕ) (hq : q.natDegree ≤ dy)
    (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx) :
    (innerX q r dx).natDegree ≤ dy * r.natDegree := by
  let f : R →+* FractionRing R := algebraMap R (FractionRing R)
  have hf : Function.Injective f := IsFractionRing.injective R (FractionRing R)
  have h := innerX_degree_field (mapBiv f q) (r.map f) dx dy
    (natDegree_map_le.trans hq) (by rw [swap_mapBiv]; exact natDegree_map_le.trans hx)
  simpa only [innerX_map f hf, natDegree_map_eq_of_injective hf] using h

/-- Exact intermediate degrees are an explicit sufficient condition for using defaults. -/
theorem natural_interchange_of_exact_degrees (p : Polynomial R)
    (q : Polynomial (Polynomial R)) (r : Polynomial R) (dx dy : ℕ)
    (hq : q.natDegree ≤ dy) (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx)
    (hy : (innerY p q dy).natDegree = p.natDegree * dx)
    (hz : (innerX q r dx).natDegree = dy * r.natDegree) :
    leftNatural p q r dy = rightNatural p q r dx := by
  unfold leftNatural rightNatural
  rw [hy, hz]
  exact fixed_interchange_domain p q r dx dy hq hx

theorem resultant_interchange_with_degree_drop_control :
    (∀ (R : Type) [CommRing R] [IsDomain R]
      (p : Polynomial R) (q : Polynomial (Polynomial R)) (r : Polynomial R)
      (dx dy : ℕ), q.natDegree ≤ dy →
      (Polynomial.Bivariate.swap q).natDegree ≤ dx →
      leftFixed p q r dx dy = rightFixed p q r dx dy ∧
      (innerY p q dy).natDegree ≤ p.natDegree * dx ∧
      (innerX q r dx).natDegree ≤ dy * r.natDegree) ∧
    controlQ.natDegree = 1 ∧ (Polynomial.Bivariate.swap controlQ).natDegree = 1 ∧
    (innerY controlP controlQ 1).natDegree < controlP.natDegree * 1 ∧
    leftNatural controlP controlQ controlR 1 ≠ rightNatural controlP controlQ controlR 1 ∧
    leftFixed controlP controlQ controlR 1 1 = rightFixed controlP controlQ controlR 1 1 := by
  constructor
  · intro R _ _ p q r dx dy hq hx
    exact ⟨fixed_interchange_domain p q r dx dy hq hx,
      innerY_degree_domain p q dx dy hq hx, innerX_degree_domain q r dx dy hq hx⟩
  · exact ⟨control_bidegrees.1, control_bidegrees.2, control_actual_degree_drop,
      natural_interchange_counterexample⟩

#print axioms resultant_interchange_with_degree_drop_control

end
end ResultantInterchangeAudit
