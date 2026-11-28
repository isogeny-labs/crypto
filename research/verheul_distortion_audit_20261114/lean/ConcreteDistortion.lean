import Field29

namespace VerheulDistortionAudit
noncomputable section
private instance prime5_concrete : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance prime3_concrete : Fact (Nat.Prime 3) := ⟨by decide⟩

abbrev repaired := correctedHom (1 : Extension) omega extension_discriminant omega_cube
abbrev repairedEquiv := correctedEquiv (1 : Extension) omega extension_discriminant omega_cube
def Q : (model (1 : Extension)).Point := repaired P

theorem printed_formula_not_on_curve :
    ¬ (model (1 : Extension)).Equation 4 (omega*6) := by
  apply printed_map_fails 1 4 6 omega omega_cube omega_ne_one (by decide +kernel)
  rw [equation_iff]
  decide +kernel

theorem corrected_formula_on_curve : (model (1 : Extension)).Equation (omega*4) 6 := by
  apply (corrected_equation 1 4 6 omega omega_cube).mpr
  rw [equation_iff]
  decide +kernel

theorem Q_ne_zero : Q≠0 := by decide +kernel

theorem Q_five : (5 : ℕ) • Q=0 := by
  change 5 • repaired P=0
  rw [← map_nsmul,P_five,map_zero]

theorem Q_order : addOrderOf Q=5 := addOrderOf_eq_prime Q_five Q_ne_zero

theorem Q_not_small_multiple (i : Fin 5) : Q ≠ i.val • P := by
  fin_cases i <;> decide +kernel

theorem Q_not_nsmul (n : ℕ) : Q ≠ n • P := by
  have hm : (n%5) • P=n • P := by simpa only [P_order] using mod_addOrderOf_nsmul P n
  rw [← hm]
  exact Q_not_small_multiple ⟨n%5,Nat.mod_lt _ (by omega)⟩

theorem P_finite_order : IsOfFinAddOrder P :=
  isOfFinAddOrder_iff_nsmul_eq_zero.mpr ⟨5,by omega,P_five⟩

theorem Q_not_in_cyclic : Q ∉ AddSubgroup.zmultiples P := by
  intro h
  obtain ⟨n,hn,he⟩ := Finset.mem_image.mp (P_finite_order.mem_zmultiples_iff_mem_range_addOrderOf.mp h)
  exact Q_not_nsmul n he.symm

theorem repaired_nonzero_multiple_not_small (i j : Fin 5) (hi : i≠0) :
    repaired (i.val • P) ≠ j.val • P := by
  fin_cases i
  · exact (hi rfl).elim
  all_goals fin_cases j <;> decide +kernel

theorem repaired_distorts_cyclic (R : (model (1 : Extension)).Point)
    (hR : R ∈ AddSubgroup.zmultiples P) (h0 : R≠0) :
    repaired R ∉ AddSubgroup.zmultiples P := by
  obtain ⟨i,hi,hiR⟩ := Finset.mem_image.mp
    (P_finite_order.mem_zmultiples_iff_mem_range_addOrderOf.mp hR)
  simp only [Finset.mem_range,P_order] at hi
  have hi0 : (⟨i,hi⟩ : Fin 5)≠0 := by
    intro he
    have hei : i=0 := congrArg Fin.val he
    exact h0 (by rw [← hiR,hei,zero_nsmul])
  intro hD
  obtain ⟨j,hj,he⟩ := Finset.mem_image.mp
    (P_finite_order.mem_zmultiples_iff_mem_range_addOrderOf.mp hD)
  simp only [Finset.mem_range,P_order] at hj
  exact repaired_nonzero_multiple_not_small ⟨i,hi⟩ ⟨j,hj⟩ hi0 (by simpa only [hiR] using he.symm)

def threeControl : (model (1 : Extension)).Point := .some 0 1 (by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero extension_discriminant).mp
  rw [equation_iff]
  decide +kernel)

theorem threeControl_order : addOrderOf threeControl=3 := by
  apply addOrderOf_eq_prime
  · decide +kernel
  · decide +kernel

theorem threeControl_fixed : repaired threeControl=threeControl := by decide +kernel

theorem cyclotomic_degree_two :
    ¬ 5∣29-1 ∧ 5∣29^2-1 ∧ Nat.Prime 29 ∧ Nat.Prime 5 ∧ 3<29 ∧ 3<5 ∧
      29%3=2 ∧ Nat.Coprime 5 29 := by decide +kernel

/-- The literal printed formula cannot be an endomorphism, already on the
actual source-qualified point of prime order five. -/
theorem no_point_map_with_printed_coordinates :
    ¬ (∃ D : (model (1 : Extension)).Point → (model (1 : Extension)).Point,
      ∃ h : (model (1 : Extension)).Nonsingular 4 (omega*6), D P=.some 4 (omega*6) h) := by
  rintro ⟨D,h,he⟩
  exact printed_formula_not_on_curve h.1

end
end VerheulDistortionAudit
