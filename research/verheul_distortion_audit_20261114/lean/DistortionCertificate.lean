import ConcreteDistortion

namespace VerheulDistortionAudit
noncomputable section

/-- A source-qualified disproof of the printed coordinate formula, with the
actual corrected point-group automorphism and explicit boundary controls. -/
theorem actual_distortion_formula_certificate :
    (∀ (K : Type) [Field K] (a x y w : K), w^3=1 → w≠1 → y≠0 →
      (model a).Equation x y → ¬ (model a).Equation x (w*y)) ∧
    (∀ (K : Type) [Field K] (a x y w : K), w^3=1 →
      ((model a).Equation (w*x) y ↔ (model a).Equation x y)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (a w : K)
      (hΔ : (model a).Δ≠0) (hw : w^3=1),
      Function.Bijective (correctedPoint a w hΔ hw) ∧
      (∀ P Q : (model a).Point,correctedPoint a w hΔ hw (P+Q)=
        correctedPoint a w hΔ hw P+correctedPoint a w hΔ hw Q) ∧
      (∀ P : (model a).Point,correctedPoint a w hΔ hw
        (correctedPoint a w hΔ hw (correctedPoint a w hΔ hw P))=P)) ∧
    (Nat.Prime 29 ∧ Nat.Prime 5 ∧ 3<29 ∧ 3<5 ∧ 29%3=2 ∧ Nat.Coprime 5 29 ∧
      ¬5∣29-1 ∧ 5∣29^2-1 ∧ Nat.card Extension=29^2 ∧
      IsPrimitiveRoot omega 3 ∧ (∀ r : Base,omega≠algebraMap Base Extension r)) ∧
    ((model (1 : Base)).Δ≠0 ∧ (model (1 : Extension)).Δ≠0 ∧
      Nat.card (model (1 : Base)).Point=30 ∧
      (29 : ℤ)+1-(Nat.card (model (1 : Base)).Point : ℤ)=0 ∧
      addOrderOf baseP=5 ∧ addOrderOf P=5) ∧
    (¬ (model (1 : Extension)).Equation 4 (omega*6) ∧
      (model (1 : Extension)).Equation (omega*4) 6 ∧
      ¬ (∃ D : (model (1 : Extension)).Point → (model (1 : Extension)).Point,
        ∃ h : (model (1 : Extension)).Nonsingular 4 (omega*6),D P=.some 4 (omega*6) h)) ∧
    (addOrderOf Q=5 ∧ Q∉AddSubgroup.zmultiples P ∧
      (∀ R : (model (1 : Extension)).Point,R∈AddSubgroup.zmultiples P → R≠0 →
        repaired R∉AddSubgroup.zmultiples P)) ∧
    (addOrderOf threeControl=3 ∧ threeControl≠0 ∧ repaired threeControl=threeControl) := by
  refine ⟨?_,?_,?_,?_,?_,?_,?_,?_⟩
  · intro K _ a x y w hw h1 hy hP
    exact printed_map_fails a x y w hw h1 hy hP
  · intro K _ a x y w hw
    exact corrected_equation a x y w hw
  · intro K _ _ a w hΔ hw
    exact ⟨(correctedEquiv a w hΔ hw).bijective,
      correctedPoint_add a w hΔ hw,correctedPoint_cube a w hΔ hw⟩
  · obtain ⟨hn1,hd2,hp,hl,hp3,hl3,hpmod,hcop⟩ := cyclotomic_degree_two
    exact ⟨hp,hl,hp3,hl3,hpmod,hcop,hn1,hd2,extension_card,omega_primitive,omega_not_in_base⟩
  · exact ⟨base_discriminant,extension_discriminant,actual_base_point_card,
      actual_source_trace_zero,baseP_order,P_order⟩
  · exact ⟨printed_formula_not_on_curve,corrected_formula_on_curve,no_point_map_with_printed_coordinates⟩
  · exact ⟨Q_order,Q_not_in_cyclic,repaired_distorts_cyclic⟩
  · exact ⟨threeControl_order,by decide +kernel,threeControl_fixed⟩

#print axioms actual_distortion_formula_certificate
end
end VerheulDistortionAudit
