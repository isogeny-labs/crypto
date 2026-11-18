import JVLocalization

namespace JVLocalMultiplicityAudit
noncomputable section
open MvPolynomial JVHilbertAudit

theorem h_cube_nonzero : h ^ 3 ≠ 0 := by
  simpa [actual_basis_apply, basisVector_coordinates] using actualBasis.ne_zero 7

theorem h_four_zero : h ^ 4 = 0 := quotient_quartic_relations.2.2

theorem h_nonzero : h ≠ 0 := by
  intro he
  apply h_cube_nonzero
  rw [he, zero_pow (by decide)]

theorem actual_not_reduced : ¬ IsReduced A := by
  intro hr
  let : IsReduced A := hr
  exact h_nonzero coordinate_nilpotent.2.2.eq_zero

def chartIdeal : Ideal A := Ideal.span ({h - 1} : Set A)
abbrev FirstChart := A ⧸ chartIdeal

theorem first_chart_h : Ideal.Quotient.mk chartIdeal h = 1 := by
  have hz : Ideal.Quotient.mk chartIdeal (h - 1) = 0 :=
    Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.subset_span (by simp))
  simpa only [map_sub, map_one, sub_eq_zero] using hz

theorem first_chart_subsingleton : Subsingleton FirstChart := by
  apply Ideal.Quotient.subsingleton_iff.mpr
  apply chartIdeal.eq_top_of_isUnit_mem (x := h - 1)
  · exact Ideal.subset_span (by simp)
  · apply unit_of_origin_nonzero
    simp

theorem first_chart_length : Module.length FirstChart FirstChart = 0 := by
  let : Subsingleton FirstChart := first_chart_subsingleton
  exact Module.length_eq_zero

theorem first_chart_not_local : ¬ IsLocalRing FirstChart := by
  intro hl
  obtain ⟨a, b, hab⟩ := hl.toNontrivial.exists_pair_ne
  exact hab (first_chart_subsingleton.elim a b)

abbrev FirstOpen := Localization.Away h

theorem first_open_subsingleton : Subsingleton FirstOpen := by
  apply IsLocalization.subsingleton (M := Submonoid.powers h)
  exact ⟨4, h_four_zero⟩

theorem first_open_length : Module.length FirstOpen FirstOpen = 0 := by
  let : Subsingleton FirstOpen := first_open_subsingleton
  exact Module.length_eq_zero

theorem first_open_not_local : ¬ IsLocalRing FirstOpen := by
  intro hl
  obtain ⟨a, b, hab⟩ := hl.toNontrivial.exists_pair_ne
  exact hab (first_open_subsingleton.elim a b)

def secondF0 {R : Type*} [CommRing R] (u v w : R) : R :=
  59*u^2 + 59*u*v + 57*u*w + v^2 + 80*v*w + 77*w^2
def secondF1 {R : Type*} [CommRing R] (u v w : R) : R :=
  78*u^2 + 14*u*v + 20*u*w + 89*v*w + 11*w^2
def secondF2 {R : Type*} [CommRing R] (u v w : R) : R :=
  61*u^2 + 69*u*v + 40*u*w + 40*v*w + 12*w^2

theorem second_source_root : secondF0 (69 : F) 75 1 = 0 ∧
    secondF1 (69 : F) 75 1 = 0 ∧ secondF2 (69 : F) 75 1 = 0 := by
  decide +kernel

def secondPolynomials : Fin 3 → Poly :=
  ![secondF0 (X 0) (X 1) (X 2), secondF1 (X 0) (X 1) (X 2),
    secondF2 (X 0) (X 1) (X 2)]
def secondIdeal : Ideal Poly := Ideal.span (Set.range secondPolynomials)
abbrev SecondAlgebra := Poly ⧸ secondIdeal
def secondPolynomialEvaluation : Poly →ₐ[F] F := MvPolynomial.aeval ![69,75,1]

theorem second_kills_source : ∀ p ∈ secondIdeal, secondPolynomialEvaluation p = 0 := by
  have hi : secondIdeal ≤ RingHom.ker secondPolynomialEvaluation.toRingHom := by
    rw [secondIdeal, Ideal.span_le]
    rintro p ⟨i, rfl⟩
    change secondPolynomialEvaluation (secondPolynomials i) = 0
    fin_cases i
    · simpa [secondPolynomialEvaluation, secondPolynomials, secondF0] using second_source_root.1
    · simpa [secondPolynomialEvaluation, secondPolynomials, secondF1] using second_source_root.2.1
    · simpa [secondPolynomialEvaluation, secondPolynomials, secondF2] using second_source_root.2.2
  exact fun _ hp => hi hp

def secondEvaluation : SecondAlgebra →ₐ[F] F :=
  Ideal.Quotient.liftₐ secondIdeal secondPolynomialEvaluation second_kills_source
def secondH : SecondAlgebra := Ideal.Quotient.mk secondIdeal (X 2)

theorem second_h_value : secondEvaluation secondH = 1 := by
  change secondPolynomialEvaluation (X 2) = 1
  simp [secondPolynomialEvaluation]

def secondChartIdeal : Ideal SecondAlgebra := Ideal.span ({secondH - 1} : Set SecondAlgebra)
abbrev SecondChart := SecondAlgebra ⧸ secondChartIdeal

def secondChartEvaluation : SecondChart →+* F :=
  Ideal.Quotient.lift secondChartIdeal secondEvaluation.toRingHom (by
    have hi : secondChartIdeal ≤ RingHom.ker secondEvaluation.toRingHom := by
      rw [secondChartIdeal, Ideal.span_le]
      intro a ha
      rcases Set.mem_singleton_iff.mp ha with rfl
      change secondEvaluation (secondH - 1) = 0
      rw [map_sub, second_h_value, map_one, sub_self]
    exact fun _ ha => hi ha)

theorem second_chart_nontrivial : Nontrivial SecondChart := by
  apply nontrivial_of_ne (0 : SecondChart) 1
  intro he
  have hv := congrArg secondChartEvaluation he
  exact (zero_ne_one : (0 : F) ≠ 1) (by simpa only [map_zero, map_one] using hv)

abbrev SecondOpen := Localization.Away secondH
def secondOpenEvaluation : SecondOpen →+* F :=
  IsLocalization.Away.lift secondH (g := secondEvaluation.toRingHom)
    (by change IsUnit (secondEvaluation secondH); rw [second_h_value]; exact isUnit_one)

theorem second_open_nontrivial : Nontrivial SecondOpen := by
  apply nontrivial_of_ne (0 : SecondOpen) 1
  intro he
  have hv := congrArg secondOpenEvaluation he
  exact (zero_ne_one : (0 : F) ≠ 1) (by simpa only [map_zero, map_one] using hv)

#print axioms h_cube_nonzero
#print axioms first_chart_subsingleton
#print axioms first_open_subsingleton
#print axioms second_source_root
#print axioms second_chart_nontrivial
#print axioms second_open_nontrivial
end
end JVLocalMultiplicityAudit
