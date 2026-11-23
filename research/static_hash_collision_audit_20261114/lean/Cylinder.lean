import WalkWeightedGeneral

namespace StaticHashCollisionAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit
variable {G I Q : Type*} [Fintype G] [Fintype I] [Fintype Q]
  [DecidableEq G] [DecidableEq I]

def queryKernel (v : Q → G) (w : Q → I) (x : G) (j : I) : ℝ :=
  ∏ a : Q, coordinateGate (v a) (w a) x j

omit [Fintype G] [Fintype I] in
theorem queryKernel_at_image (v : Q → G) (hv : Function.Injective v) (w : Q → I)
    (a : Q) (j : I) : queryKernel v w (v a) j = delta j (w a) := by
  classical
  unfold queryKernel
  calc
    _ = coordinateGate (v a) (w a) (v a) j := by
      apply Fintype.prod_eq_single a
      intro b hba
      have hab : v a ≠ v b := fun h => hba (hv h).symm
      simp [coordinateGate, hab]
    _ = _ := by simp [coordinateGate]

omit [Fintype G] [Fintype I] in
theorem queryKernel_off_image (v : Q → G) (w : Q → I) (x : G)
    (hx : ¬∃ a, v a = x) (j : I) : queryKernel v w x j = 1 := by
  apply Fintype.prod_eq_one
  intro a
  have hxa : x ≠ v a := fun h => hx ⟨a, h.symm⟩
  simp [coordinateGate, hxa]

omit [Fintype I] in
theorem queryKernel_product (v : Q → G) (w : Q → I) (h : Hash G I) :
    (∏ x : G, queryKernel v w x (h x)) = ∏ a : Q, delta (h (v a)) (w a) := by
  unfold queryKernel
  rw [Finset.prod_comm]
  simp only [coordinate_product]

omit [Fintype G] in
theorem queryKernel_average (p : I → ℝ) (hp : ∑ i, p i = 1)
    (v : Q → G) (hv : Function.Injective v) (w : Q → I) (x : G) :
    (∑ j : I, p j * queryKernel v w x j) =
      Function.extend v (fun a => p (w a)) (fun _ => 1) x := by
  classical
  by_cases hx : ∃ a, v a = x
  · obtain ⟨a, rfl⟩ := hx
    rw [hv.extend_apply]
    simp only [queryKernel_at_image v hv]
    simp [delta, mul_ite]
  · simp only [queryKernel_off_image v w x hx, mul_one]
    rw [hp, Function.extend_apply' _ _ _ hx]

theorem cylinder_mass (p : I → ℝ) (hp : ∑ i, p i = 1)
    (v : Q → G) (hv : Function.Injective v) (w : Q → I) :
    expectation p (fun h : Hash G I => if ∀ a, h (v a) = w a then 1 else 0) =
      ∏ a : Q, p (w a) := by
  classical
  calc
    _ = expectation p (fun h : Hash G I => ∏ x : G, queryKernel v w x (h x)) := by
      congr 1
      funext h
      rw [queryKernel_product]
      simp only [delta, Fintype.prod_ite_zero, Finset.prod_const_one]
    _ = ∏ x : G, ∑ j : I, p j * queryKernel v w x j :=
      expectation_product p (queryKernel v w)
    _ = ∏ x : G, Function.extend v (fun a => p (w a)) (fun _ => 1) x := by
      apply Fintype.prod_congr
      intro x
      exact queryKernel_average p hp v hv w x
    _ = ∏ a : Q, p (w a) :=
      Finset.prod_extend_one_of_injective (fun a => p (w a)) hv (fun _ => Finset.mem_univ _)

#print axioms cylinder_mass
end
end StaticHashCollisionAudit
