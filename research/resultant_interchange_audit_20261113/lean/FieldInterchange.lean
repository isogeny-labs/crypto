import SplitInterchange
import Mathlib.FieldTheory.IsAlgClosed.AlgebraicClosure

namespace ResultantInterchangeAudit
open Polynomial
noncomputable section

section Mapping
variable {K L : Type*} [CommRing K] [CommRing L]

def mapBiv (f : K →+* L) (q : Polynomial (Polynomial K)) : Polynomial (Polynomial L) :=
  q.map (mapRingHom f)

theorem lift_map (f : K →+* L) (p : Polynomial K) :
    lift (p.map f) = mapBiv f (lift p) := by
  simp only [lift, mapBiv, map_map]
  congr 1
  ext a
  simp

theorem swap_mapBiv (f : K →+* L) (q : Polynomial (Polynomial K)) :
    Polynomial.Bivariate.swap (mapBiv f q) = mapBiv f (Polynomial.Bivariate.swap q) := by
  unfold mapBiv
  induction q using Polynomial.induction_on' with
  | add p q hp hq => rw [Polynomial.map_add, map_add, hp, hq, map_add, Polynomial.map_add]
  | monomial n a =>
    rw [Polynomial.map_monomial, Polynomial.Bivariate.swap_monomial,
      Polynomial.Bivariate.swap_monomial]
    change lift (a.map f) * C (X ^ n) = mapBiv f (lift a * C (X ^ n))
    rw [lift_map]
    simp [mapBiv]

theorem innerY_map (f : K →+* L) (hf : Function.Injective f)
    (p : Polynomial K) (q : Polynomial (Polynomial K)) (dy : ℕ) :
    innerY (p.map f) (mapBiv f q) dy = (innerY p q dy).map f := by
  unfold innerY
  rw [lift_map, natDegree_map_eq_of_injective hf]
  exact resultant_map_map _ _ _ _ (mapRingHom f)

theorem innerX_map (f : K →+* L) (hf : Function.Injective f)
    (q : Polynomial (Polynomial K)) (r : Polynomial K) (dx : ℕ) :
    innerX (mapBiv f q) (r.map f) dx = (innerX q r dx).map f := by
  unfold innerX
  rw [lift_map, swap_mapBiv, natDegree_map_eq_of_injective hf]
  exact resultant_map_map _ _ _ _ (mapRingHom f)

theorem leftFixed_map (f : K →+* L) (hf : Function.Injective f)
    (p : Polynomial K) (q : Polynomial (Polynomial K)) (r : Polynomial K) (dx dy : ℕ) :
    leftFixed (p.map f) (mapBiv f q) (r.map f) dx dy = f (leftFixed p q r dx dy) := by
  unfold leftFixed
  rw [innerY_map f hf, natDegree_map_eq_of_injective hf,
    natDegree_map_eq_of_injective hf]
  exact resultant_map_map _ _ _ _ f

theorem rightFixed_map (f : K →+* L) (hf : Function.Injective f)
    (p : Polynomial K) (q : Polynomial (Polynomial K)) (r : Polynomial K) (dx dy : ℕ) :
    rightFixed (p.map f) (mapBiv f q) (r.map f) dx dy = f (rightFixed p q r dx dy) := by
  unfold rightFixed
  rw [innerX_map f hf, natDegree_map_eq_of_injective hf,
    natDegree_map_eq_of_injective hf]
  exact resultant_map_map _ _ _ _ f

end Mapping

variable {K : Type*} [Field K]

theorem fixed_interchange (p : Polynomial K) (q : Polynomial (Polynomial K))
    (r : Polynomial K) (dx dy : ℕ) (hq : q.natDegree ≤ dy)
    (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx) :
    leftFixed p q r dx dy = rightFixed p q r dx dy := by
  let f : K →+* AlgebraicClosure K := algebraMap K (AlgebraicClosure K)
  apply f.injective
  rw [← leftFixed_map f f.injective, ← rightFixed_map f f.injective]
  apply fixed_interchange_of_splits
  · exact IsAlgClosed.splits _
  · exact IsAlgClosed.splits _
  · exact natDegree_map_le.trans hq
  · rw [swap_mapBiv]
    exact natDegree_map_le.trans hx

theorem innerY_degree_field (p : Polynomial K) (q : Polynomial (Polynomial K))
    (dx dy : ℕ) (hq : q.natDegree ≤ dy)
    (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx) :
    (innerY p q dy).natDegree ≤ p.natDegree * dx := by
  let f : K →+* AlgebraicClosure K := algebraMap K (AlgebraicClosure K)
  have h := innerY_degree (p.map f) (mapBiv f q) dx dy (IsAlgClosed.splits _)
    (natDegree_map_le.trans hq) (by rw [swap_mapBiv]; exact natDegree_map_le.trans hx)
  simpa only [innerY_map f f.injective, natDegree_map_eq_of_injective f.injective] using h

theorem innerX_degree_field (q : Polynomial (Polynomial K)) (r : Polynomial K)
    (dx dy : ℕ) (hq : q.natDegree ≤ dy)
    (hx : (Polynomial.Bivariate.swap q).natDegree ≤ dx) :
    (innerX q r dx).natDegree ≤ dy * r.natDegree := by
  let f : K →+* AlgebraicClosure K := algebraMap K (AlgebraicClosure K)
  have h := innerX_degree (mapBiv f q) (r.map f) dx dy (IsAlgClosed.splits _)
    (natDegree_map_le.trans hq) (by rw [swap_mapBiv]; exact natDegree_map_le.trans hx)
  simpa only [innerX_map f f.injective, natDegree_map_eq_of_injective f.injective] using h

#print axioms fixed_interchange

end
end ResultantInterchangeAudit
