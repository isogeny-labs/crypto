import JVTailFlags

namespace JVLocalMultiplicityAudit
noncomputable section
open MvPolynomial JVHilbertAudit

def originPolynomial : Poly →ₐ[F] F := MvPolynomial.aeval fun _ => 0

theorem origin_kills_source : ∀ p ∈ JVMacaulayAudit.sourceIdeal, originPolynomial p = 0 := by
  have hi : JVMacaulayAudit.sourceIdeal ≤ RingHom.ker originPolynomial.toRingHom := by
    rw [JVMacaulayAudit.sourceIdeal, Ideal.span_le]
    intro p hp
    change originPolynomial p = 0
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
    rcases hp with rfl | rfl | rfl
    · simp [originPolynomial, JVMacaulayAudit.f0]
    · simp [originPolynomial, JVMacaulayAudit.f1]
    · simp [originPolynomial, JVMacaulayAudit.f2]
  intro p hp
  exact hi hp

def originEvaluation : A →ₐ[F] F :=
  Ideal.Quotient.liftₐ JVMacaulayAudit.sourceIdeal originPolynomial origin_kills_source

theorem origin_quotient (p : Poly) : originEvaluation (quotientMap p) = originPolynomial p := rfl

@[simp] theorem origin_constant (c : F) : originEvaluation (algebraMap F A c) = c :=
  originEvaluation.commutes c

theorem origin_coordinate (s : Fin 3) : originEvaluation (quotientMap (X s)) = 0 := by
  rw [origin_quotient]
  simp [originPolynomial]

@[simp] theorem origin_x : originEvaluation x = 0 := origin_coordinate 0
@[simp] theorem origin_y : originEvaluation y = 0 := origin_coordinate 1
@[simp] theorem origin_h : originEvaluation h = 0 := origin_coordinate 2

theorem origin_basis (i : Fin 8) : originEvaluation (actualBasis i) = if i = 0 then 1 else 0 := by
  rw [actual_basis_apply, basisVector_coordinates]
  fin_cases i <;> simp

theorem origin_repr (a : A) : originEvaluation a = actualBasis.repr a 0 := by
  calc
    originEvaluation a = originEvaluation (∑ i, actualBasis.repr a i • actualBasis i) :=
      congrArg originEvaluation (actualBasis.sum_repr a).symm
    _ = actualBasis.repr a 0 := by
      rw [map_sum]
      simp_rw [map_smul, origin_basis]
      simp

theorem tail_one_iff (a : A) : a ∈ tailSpan 1 ↔ originEvaluation a = 0 := by
  constructor
  · have hi : tailSpan 1 ≤ LinearMap.ker originEvaluation.toLinearMap := by
      apply Submodule.span_le.mpr
      rintro z ⟨i, hi, rfl⟩
      change 1 ≤ i.val at hi
      change originEvaluation (actualBasis i) = 0
      have hn : i ≠ 0 := by
        intro he
        have hv : i.val = 0 := congrArg Fin.val he
        omega
      rw [origin_basis, ite_eq_right hn]
    exact fun ha => hi ha
  · intro ha
    have hc : actualBasis.repr a 0 = 0 := (origin_repr a).symm.trans ha
    rw [← actualBasis.sum_repr a]
    apply (tailSpan 1).sum_mem
    intro i _
    by_cases hi : i = 0
    · subst i
      rw [hc, zero_smul]
      exact (tailSpan 1).zero_mem
    · apply (tailSpan 1).smul_mem
      apply (basis_mem_tail_iff 1 i).mpr
      have hv : i.val ≠ 0 := by intro he; exact hi (Fin.ext he)
      omega

theorem coordinate_nilpotent : IsNilpotent x ∧ IsNilpotent y ∧ IsNilpotent h := by
  have van := JVMacaulayAudit.all_degree_four_vanish x y h
    JVMacaulayAudit.actual_quotient_equations.1
    JVMacaulayAudit.actual_quotient_equations.2.1
    JVMacaulayAudit.actual_quotient_equations.2.2
  exact ⟨⟨4, by simpa using van 4 0 0 (by decide)⟩,
    ⟨4, by simpa using van 0 4 0 (by decide)⟩,
    ⟨4, quotient_quartic_relations.2.2⟩⟩

theorem positive_basis_nilpotent (i : Fin 8) (hi : 1 ≤ i.val) : IsNilpotent (actualBasis i) := by
  have hx := coordinate_nilpotent.1
  have hy := coordinate_nilpotent.2.1
  have hh := coordinate_nilpotent.2.2
  have hxx : IsNilpotent (h ^ 2) := by
    simpa only [pow_two] using (Commute.all h h).isNilpotent_mul_right hh
  have hxxx : IsNilpotent (h ^ 3) := by
    simpa only [pow_succ] using (Commute.all (h ^ 2) h).isNilpotent_mul_right hxx
  rw [actual_basis_apply, basisVector_coordinates]
  fin_cases i
  · norm_num at hi
  · exact hx
  · exact hy
  · exact hh
  · exact (Commute.all x h).isNilpotent_mul_right hx
  · exact (Commute.all y h).isNilpotent_mul_right hy
  · exact hxx
  · exact hxxx

theorem tail_one_nilpotent {a : A} (ha : a ∈ tailSpan 1) : IsNilpotent a := by
  induction ha using Submodule.span_induction with
  | mem z hz =>
      obtain ⟨i, hi, rfl⟩ := hz
      exact positive_basis_nilpotent i hi
  | zero => exact ⟨1, zero_pow (by decide)⟩
  | add z w _ _ hz hw => exact (Commute.all z w).isNilpotent_add hz hw
  | smul c z _ hz => exact hz.smul c

theorem scalar_remainder_nilpotent (a : A) :
    IsNilpotent (a - algebraMap F A (originEvaluation a)) := by
  apply tail_one_nilpotent
  apply (tail_one_iff _).mpr
  rw [map_sub, origin_constant, sub_self]

theorem unit_of_origin_nonzero (a : A) (ha : originEvaluation a ≠ 0) : IsUnit a := by
  have hn := scalar_remainder_nilpotent a
  have hu : IsUnit (algebraMap F A (originEvaluation a)) :=
    (isUnit_iff_ne_zero.mpr ha).map (algebraMap F A)
  have he : a = (a - algebraMap F A (originEvaluation a)) + algebraMap F A (originEvaluation a) :=
    (sub_add_cancel _ _).symm
  rw [he]
  exact hn.isUnit_add_right_of_commute hu (Commute.all _ _)

theorem unit_or_nilpotent (a : A) : IsUnit a ∨ IsNilpotent a := by
  by_cases ha : originEvaluation a = 0
  · exact Or.inr (tail_one_nilpotent ((tail_one_iff a).mpr ha))
  · exact Or.inl (unit_of_origin_nonzero a ha)

theorem actual_nontrivial : Nontrivial A :=
  ⟨⟨actualBasis 0, 0, actualBasis.ne_zero 0⟩⟩

theorem actual_local : IsLocalRing A := by
  let : Nontrivial A := actual_nontrivial
  apply IsLocalRing.of_isUnit_or_isUnit_one_sub_self
  intro a
  rcases unit_or_nilpotent a with hu | hn
  · exact Or.inl hu
  · exact Or.inr hn.isUnit_one_sub

theorem origin_surjective : Function.Surjective originEvaluation :=
  fun c => ⟨algebraMap F A c, origin_constant c⟩

def originIdeal : Ideal A := RingHom.ker originEvaluation.toRingHom

theorem origin_maximal : originIdeal.IsMaximal :=
  RingHom.ker_isMaximal_of_surjective originEvaluation.toRingHom origin_surjective

def originResidueEquiv : A ⧸ originIdeal ≃+* F :=
  originEvaluation.toRingHom.quotientKerEquivOfSurjective origin_surjective

theorem origin_ideal_nilpotent (a : A) (ha : a ∈ originIdeal) : IsNilpotent a :=
  tail_one_nilpotent ((tail_one_iff a).mpr ha)

#print axioms actual_local
#print axioms origin_maximal
#print axioms originResidueEquiv
end
end JVLocalMultiplicityAudit
